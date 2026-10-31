<div align="center">

# Rclone SDK

**Full OpenAPI-based client for the Rclone RC API**

[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?&logo=discord&logoColor=white)](https://discord.gg/rclone)
[![npm version](https://img.shields.io/npm/v/rclone-sdk?color=cb0000&label=npm&logo=npm)](https://www.npmjs.com/package/rclone-sdk)
[![npm downloads](https://img.shields.io/npm/dm/rclone-sdk?color=cb0000&logo=npm)](https://www.npmjs.com/package/rclone-sdk)
[![crates.io](https://img.shields.io/crates/v/rclone-sdk?color=fc8d62&logo=rust)](https://crates.io/crates/rclone-sdk)
[![crates.io downloads](https://img.shields.io/crates/d/rclone-sdk?color=fc8d62&logo=rust)](https://crates.io/crates/rclone-sdk)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

<sub>Built on top of [**rclone-openapi**](https://github.com/rclone-ui/rclone-openapi) (by yours truly) · Works with **Vanilla Fetch** · **React Query** · **SWR** · **Rust**</sub>

</div>

## 🦀 Rust

```sh
cargo add rclone-sdk
```

```toml
[dependencies]
rclone-sdk = "1.75.2"
tokio = { version = "1", features = ["full"] }
```

```rust
use rclone_sdk::{types, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::with_auth("http://localhost:5572", "user", "password");

    // Get rclone version info
    let version = client.core_version(&Default::default()).await?;
    let v = version.into_inner();
    println!("Rclone {} on {}/{}", v.version, v.os, v.arch);

    // List all configured remotes
    let remotes = client.config_listremotes(&Default::default()).await?;
    println!("Remotes: {:?}", remotes.into_inner().remotes);

    // Get storage info for a remote
    let about = client
        .operations_about(&types::OperationsAboutBody {
            fs: "gdrive:".into(),
            config: Default::default(),
            filter: Default::default(),
            group: None,
        })
        .await?;
    let info = about.into_inner();
    println!("Storage: {:?} / {:?} bytes used", info.used, info.total);

    Ok(())
}
```

- **Connecting:** use `Client::with_auth`, or `Client::without_auth` for a server that runs with `--rc-no-auth`. Avoid `Client::new`, which gives up on every call after 15 seconds, and rclone cancels a call when its client disconnects. To use your own settings, pass a `rclone_sdk::reqwest::Client` to `Client::new_with_client`.
- **Bodies:** each method takes one body struct. A body with required fields has no `Default`, so you list every field.
- **Open routes:** routes like `serve/start` accept more than the spec lists. Put those fields in the struct's `extra` map.
- **Background calls:** `client.post_async("sync/copy", &body)` returns the job ID. Poll `job_status` with it.
- **Uploads:** `client.operations_uploadfile(None, None, None, "gdrive:", "photos", files)` takes `(file name, multipart::Part)` pairs. `remote` is the directory to upload into.
- **Failed logins:** a 401 is plain text, so it comes back as `Error::InvalidResponsePayload` with the text inside.


## 📦 JavaScript / TypeScript

```sh
npm install rclone-sdk
```

### Vanilla

```ts
import createRCDClient from 'rclone-sdk'

const rcd = createRCDClient({ baseUrl: 'http://localhost:5572' })

// List all configured remotes
const { data: remotes } = await rcd.POST('/config/listremotes')
console.log(remotes?.remotes) // ['gdrive', 'dropbox', 's3']

// List files in a remote
const { data: files } = await rcd.POST('/operations/list', {
    body: { fs: 'gdrive:', remote: 'Documents' }
})
console.log(files?.list)

// Get storage info for a remote
const { data: about } = await rcd.POST('/operations/about', {
    body: { fs: 'gdrive:' }
})
console.log(`Used: ${about?.used} / ${about?.total}`)
```

TypeScript rejects fields that rclone would ignore, so `{ fs: 'gdrive:', remote: '', recurse: true }` doesn't compile. `recurse` belongs in `opt`.

### Tanstack/React Query

```tsx
import createRCDQueryClient from 'rclone-sdk/query'

const rq = createRCDQueryClient({ baseUrl: 'http://localhost:5572' })

function RemotesList() {
    const { data, isLoading, error } = rq.useQuery('post', '/config/listremotes')

    if (isLoading) return <div>Loading...</div>
    if (error) return <div>Error: {typeof error === 'string' ? error : error.error}</div>

    return (
        <ul>
            {data?.remotes?.map(remote => (
                <li key={remote}>{remote}</li>
            ))}
        </ul>
    )
}

function StorageInfo({ remote }: { remote: string }) {
    const { data } = rq.useQuery('post', '/operations/about', {
        body: { fs: `${remote}:` }
    })

    return <span>{data?.used} / {data?.total} bytes</span>
}
```

These types come from `openapi-react-query`, which doesn't reject extra body fields.

### SWR

```tsx
import createRCDSWR from 'rclone-sdk/swr'

const swr = createRCDSWR({ baseUrl: 'http://localhost:5572' })

function RemotesList() {
    const { data, error, isLoading } = swr.useQuery('/config/listremotes')

    if (isLoading) return <div>Loading...</div>
    if (error) return <div>Error: {typeof error === 'string' ? error : error.error}</div>

    return (
        <ul>
            {data?.remotes?.map(remote => (
                <li key={remote}>{remote}</li>
            ))}
        </ul>
    )
}

function FileList({ remote, path }: { remote: string; path: string }) {
    const { data } = swr.useQuery('/operations/list', {
        body: { fs: `${remote}:`, remote: path }
    })

    return (
        <ul>
            {data?.list?.map(item => (
                <li key={item.Path}>
                    {item.IsDir ? '📁' : '📄'} {item.Name}
                </li>
            ))}
        </ul>
    )
}
```

## Async Operations

Every rclone route except uploads can run in the background. Use `rcd.ASYNC()` instead of `rcd.POST()` to get a properly typed job ID:

```ts
import createRCDClient from 'rclone-sdk'

const rcd = createRCDClient({ baseUrl: 'http://localhost:5572' })

// Start an async copy — ASYNC sends the Prefer: respond-async header
const { data } = await rcd.ASYNC('/sync/copy', {
    body: { srcFs: 'gdrive:docs', dstFs: 'b2:backup' },
})
console.log(data?.jobid) // number — fully typed

// Poll until finished
const { data: status } = await rcd.POST('/job/status', { body: { jobid: data!.jobid } })
console.log(status?.finished, status?.success)
```

`POST()` returns the synchronous (200) response type. `ASYNC()` returns the async (202) response type (`{ jobid: number, executeId: string }`). Both accept the same path and parameters — `ASYNC()` just adds the `Prefer: respond-async` header for you.

React Query and SWR hooks wait for the result, like `POST()`. To start a job from a React app, call `rcd.ASYNC()` inside a mutation.

If you need the raw OpenAPI types (including both 200 and 202), they're available as `paths`:

```ts
import { type paths } from 'rclone-sdk'
```

## Tips

**`rclone`** expects everything as a _POST_ request, even for read operations.

If you want a function that returns the data and throws on errors, here's a quick snippet (adjust to taste):

```ts
import createRCDClient, { type RCDArgs, type RCDData, type RCDPath } from 'rclone-sdk'

const client = createRCDClient({ baseUrl: 'http://localhost:5572' })

// TypeScript can't resolve `client.POST` for a generic `Path`, so this one call uses a simpler type
const post = client.POST as unknown as (
    path: string,
    init?: unknown
) => Promise<{ data?: unknown; error?: unknown }>

export default async function rclone<Path extends RCDPath>(path: Path, ...init: RCDArgs<Path>) {
    const { data, error } = await post(path, init[0])
    if (error) throw new Error(typeof error === 'string' ? error : JSON.stringify(error))
    return data as RCDData<Path>
}
```

Now you'll be able to
- call **`rclone('/config/listremotes')`** directly
- get the data as the return value (correctly typed, and without having to de-construct the resulting object)
- have it throw on error with a detailed error message (instead of checking the **`error`** field manually)

## Known limits

- **Long calls on Node:** `fetch` stops waiting for a response after 5 minutes, and rclone cancels a call when its client disconnects. Use `ASYNC()` for anything long. The Rust client's `with_auth` and `without_auth` have no such limit.
- **`core/command`:** its `returnType` decides between JSON and plain text. In TypeScript, `data` is a union that you narrow, and the `STREAM` types need `parseAs: 'text'`. In Rust, `STREAM` returns `Error::InvalidResponsePayload` with the output inside. In the background, the `STREAM` types run the command but the job's `output` is empty.
- **`Prefer` as a free-form header:** `rcd.POST(path, { headers: { Prefer: 'respond-async' } })` compiles and returns a job ID that its type doesn't show. Use `ASYNC()`.
- **Uploads in TypeScript:** rclone skips form parts with no file name and still returns 200, and types can't see inside a `FormData`. Append each file with a name, as in `form.append('file', blob, 'name.txt')`.
- **`_config` and `_filter` on three routes:** `cache/fetch`, `vfs/forget` and `vfs/refresh` take open-ended keys with string values, and the spec can't allow objects for only these two keys. It leaves them out, so neither client can send them on these routes, although rclone accepts them.

## Contributing

Contributions = welcome! Just make sure to check if the PR isn't a better fit for the [**rclone-openapi**](https://github.com/rclone-ui/rclone-openapi) repo.


<br />
<br />

<div align="center">
<sub>Made with ☁️ for the <a href="https://discord.gg/rclone">rclone community</a></sub>
</div>
