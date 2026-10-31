//! Hand-written overrides for the generated client. Survives regeneration; the
//! gen script wires it in via `pub mod overrides;`.

use crate::Client;
use crate::types;
use base64::Engine;
use progenitor_client::{Error, ResponseValue};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

pub use ::reqwest;
pub use ::reqwest::multipart;

/// Job handle from a background call. Poll `job/status` with `jobid`.
#[derive(::serde::Serialize, ::serde::Deserialize, Debug, Clone, Default)]
pub struct AsyncJob {
    #[serde(rename = "executeId")]
    pub execute_id: String,
    pub jobid: i64,
}

// No total timeout, because a call like `sync_copy` can run for as long as its transfer does.
fn client_builder() -> reqwest::ClientBuilder {
    let builder = reqwest::ClientBuilder::new();
    #[cfg(not(target_arch = "wasm32"))]
    let builder = builder.connect_timeout(::std::time::Duration::from_secs(15));
    builder
}

impl Client {
    /// Connects with the credentials from `--rc-user` and `--rc-pass`.
    /// Use this or [`Client::without_auth`] instead of `Client::new`, which gives up on every call
    /// after 15 seconds, and rclone cancels a call when its client disconnects.
    pub fn with_auth(baseurl: &str, user: &str, password: &str) -> Self {
        let credentials =
            base64::engine::general_purpose::STANDARD.encode(format!("{user}:{password}"));
        let mut value = HeaderValue::from_str(&format!("Basic {credentials}"))
            .expect("base64 is always a valid header value");
        value.set_sensitive(true);
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, value);
        let client = client_builder()
            .default_headers(headers)
            .build()
            .expect("the client settings are valid");
        Self::new_with_client(baseurl, client)
    }

    /// Connects to a server that runs with `--rc-no-auth`.
    /// On a server with no credentials and without that flag, most routes return a 403.
    pub fn without_auth(baseurl: &str) -> Self {
        let client = client_builder()
            .build()
            .expect("the client settings are valid");
        Self::new_with_client(baseurl, client)
    }

    /// Runs any RC call in the background by sending `Prefer: respond-async`.
    /// `body` has to serialize to a JSON object. It can be one of the generated body types
    /// or a `serde_json::Value`.
    pub async fn post_async<'a, B>(
        &'a self,
        path: &'a str,
        body: &'a B,
    ) -> Result<ResponseValue<AsyncJob>, Error<types::RcError>>
    where
        B: ::serde::Serialize + ?Sized,
    {
        let url = format!("{}/{}", self.baseurl, path.trim_start_matches('/'));
        let response = self
            .client
            .post(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .header("Prefer", "respond-async")
            .json(body)
            .send()
            .await?;
        match response.status().as_u16() {
            202 => ResponseValue::from_response(response).await,
            400..=599 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Uploads `files` to the directory `remote` on `fs`, as `(file name, content)` pairs.
    /// Rclone only reads multipart bodies and skips parts with no file name, and it returns 200
    /// either way. This method replaces the generated one so that neither can happen.
    pub async fn operations_uploadfile<'a>(
        &'a self,
        config: Option<&'a str>,
        filter: Option<&'a str>,
        group: Option<&'a str>,
        fs: &'a str,
        remote: &'a str,
        files: impl IntoIterator<Item = (String, multipart::Part)>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/operations/uploadfile", self.baseurl);
        let mut query = vec![("fs", fs), ("remote", remote)];
        for (name, value) in [("_config", config), ("_filter", filter), ("_group", group)] {
            if let Some(value) = value {
                query.push((name, value));
            }
        }
        let mut form = multipart::Form::new();
        for (file_name, part) in files {
            form = form.part("file", part.file_name(file_name));
        }
        let response = self
            .client
            .post(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .query(&query)
            .multipart(form)
            .send()
            .await?;
        match response.status().as_u16() {
            200 => ResponseValue::from_response(response).await,
            400..=599 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
