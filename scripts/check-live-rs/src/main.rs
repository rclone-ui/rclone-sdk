// The Rust half of scripts/check-live.mjs. Any failed check panics, which fails that script.

use rclone_sdk::{Client, multipart, types};
use serde_json::json;
use std::{env, fs, path::Path, time::Duration};

fn var(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{name} isn't set, run this through check-live.mjs"))
}

// Builds a body from JSON, for the fields a check doesn't care about.
fn body<T: serde::de::DeserializeOwned>(value: serde_json::Value) -> T {
    serde_json::from_value(value).expect("the JSON matches the generated type")
}

#[tokio::main]
async fn main() {
    let url = var("CHECK_URL");
    let src = var("CHECK_SRC");
    let dst = var("CHECK_DST");
    let client = Client::with_auth(&url, &var("CHECK_USER"), &var("CHECK_PASS"));

    // Normal calls, including a body with no required fields
    let version = client.core_version(&Default::default()).await.unwrap();
    assert_eq!(version.into_inner().version, var("CHECK_VERSION"));

    // A failed login is plain text, and the message survives
    let denied = Client::without_auth(&url)
        .core_version(&Default::default())
        .await
        .unwrap_err();
    assert!(denied.to_string().contains("401 Unauthorized"), "{denied}");

    // `https://` reaches the network, which shows that TLS is built in
    let tls = Client::without_auth("https://127.0.0.1:1")
        .core_version(&Default::default())
        .await
        .unwrap_err();
    assert!(!tls.to_string().contains("scheme is not http"), "{tls}");

    // `fs` takes a string, and the root of a remote is an empty string
    let listing = client
        .operations_list(&types::OperationsListBody {
            config: Default::default(),
            filter: Default::default(),
            group: None,
            fs: src.as_str().into(),
            opt: None,
            remote: String::new(),
        })
        .await
        .unwrap()
        .into_inner();
    assert_eq!(listing.list.len(), 1);

    // post_async runs in the background and takes a generated body
    let job = client
        .post_async(
            "sync/copy",
            &types::SyncCopyBody {
                config: Default::default(),
                filter: Default::default(),
                group: None,
                create_empty_src_dirs: None,
                src_fs: src.as_str().into(),
                dst_fs: dst.as_str().into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(job.status().as_u16(), 202);
    let job = job.into_inner();
    assert!(!job.execute_id.is_empty());

    let mut finished = false;
    for _ in 0..100 {
        tokio::time::sleep(Duration::from_millis(50)).await;
        let status = client
            .job_status(&body(json!({ "jobid": job.jobid })))
            .await
            .unwrap()
            .into_inner();
        if status.finished {
            assert!(status.success, "{:?}", status.error);
            finished = true;
            break;
        }
    }
    assert!(finished, "the copy didn't finish");
    assert_eq!(
        fs::read_to_string(Path::new(&dst).join("a.txt")).unwrap(),
        "hello\n"
    );

    // Uploads write the file, into the directory that `remote` names
    let files = [(
        "up.txt".to_string(),
        multipart::Part::bytes(b"uploaded".to_vec()),
    )];
    client
        .operations_uploadfile(None, None, Some("g"), &dst, "sub", files)
        .await
        .unwrap();
    assert_eq!(
        fs::read_to_string(Path::new(&dst).join("sub/up.txt")).unwrap(),
        "uploaded"
    );

    // Config steps come back in the generated type
    let plain = client
        .config_create(&body(json!({
            "name": "al",
            "type": "alias",
            "parameters": { "remote": src },
        })))
        .await
        .unwrap()
        .into_inner();
    assert!(plain.option.is_none());
    let step = client
        .config_create(&body(json!({
            "name": "q",
            "type": "s3",
            "parameters": {},
            "opt": { "nonInteractive": true, "all": true },
        })))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(step.option.expect("a question").name, "provider");

    // An unknown remote is an empty object, not an error
    let missing = client
        .config_get(&body(json!({ "name": "nope" })))
        .await
        .unwrap()
        .into_inner();
    assert!(missing.type_.is_none());

    // Open routes keep the fields that the spec doesn't list
    let batch = client
        .job_batch(&body(json!({
            "inputs": [{ "_path": "core/obscure", "clear": "secret" }],
        })))
        .await
        .unwrap()
        .into_inner();
    let results = serde_json::to_value(&batch).unwrap();
    assert!(results["results"][0]["obscured"].is_string(), "{results}");

    // `_group` and `group` are two fields with their own names on the wire
    let stats: types::CoreStatsBody = body(json!({ "_group": "a", "group": "b" }));
    assert_eq!(stats.group_.as_deref(), Some("a"));
    assert_eq!(stats.group.as_deref(), Some("b"));
    let wire = serde_json::to_value(&stats).unwrap();
    assert_eq!(wire, json!({ "_group": "a", "group": "b" }));
    client.core_stats(&stats).await.unwrap();
}
