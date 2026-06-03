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
rclone-sdk = "1.74.1"
tokio = { version = "1", features = ["full"] }
```

```rust
use rclone_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("http://localhost:5572");

    // Get rclone version info
    let version = client.core_version(None, None).await?;
    let v = version.into_inner();
    println!("Rclone {} on {}/{}", v.version, v.os, v.arch);

    // List all configured remotes
    let remotes = client.config_listremotes(None, None).await?;
    println!("Remotes: {:?}", remotes.into_inner().remotes);

    // Get storage info for a remote
    let about = client.operations_about(None, None, "gdrive:").await?;
    let info = about.into_inner();
    println!("Storage: {} / {} bytes used", info.used, info.total);

    Ok(())
}
```


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

### Tanstack/React Query

```tsx
import createRCDQueryClient from 'rclone-sdk/query'

const rq = createRCDQueryClient({ baseUrl: 'http://localhost:5572' })

function RemotesList() {
    const { data, isLoading, error } = rq.useQuery('post', '/config/listremotes')

    if (isLoading) return <div>Loading...</div>
    if (error) return <div>Error: {error.message}</div>

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

### SWR

```tsx
import createRCDSWR from 'rclone-sdk/swr'

const swr = createRCDSWR({ baseUrl: 'http://localhost:5572' })

function RemotesList() {
    const { data, error, isLoading } = swr.useQuery('post', '/config/listremotes')

    if (isLoading) return <div>Loading...</div>
    if (error) return <div>Error: {error.message}</div>

    return (
        <ul>
            {data?.remotes?.map(remote => (
                <li key={remote}>{remote}</li>
            ))}
        </ul>
    )
}

function FileList({ remote, path }: { remote: string; path: string }) {
    const { data } = swr.useQuery('post', '/operations/list', {
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

Many rclone endpoints support asynchronous execution. Pass `_async: true` in the request body and the SDK will automatically add the `Prefer: respond-async` header so the server responds with **HTTP 202** and a job ID:

```ts
import createRCDClient, { type AsyncJobResponse } from 'rclone-sdk'

const rcd = createRCDClient({ baseUrl: 'http://localhost:5572' })

// Start an async copy — cast the response since the default types are for sync (200) responses
const { data } = await rcd.POST('/sync/copy', {
    body: { srcFs: 'gdrive:docs', dstFs: 'b2:backup', _async: true },
})
const { jobid } = data as unknown as AsyncJobResponse

// Poll until finished
const { data: status } = await rcd.POST('/job/status', { body: { jobid } })
console.log(status?.finished, status?.success)
```

By default, response types reflect the synchronous (200) response for ergonomic access. If you need the raw OpenAPI types (including 202), they're available as `paths`:

```ts
import { type paths } from 'rclone-sdk'
```

## Tips

Even though the client supports all HTTP methods, **`rclone`** expects everything as a _POST_ request.

If you want to wrap the client to only send POST requests and throw errors automatically, here's a quick snippet (adjust to taste):
```ts
import createRCDClient, {
    type OpenApiMethodResponse,
    type OpenApiClient,
    type OpenApiClientPathsWithMethod,
    type OpenApiMaybeOptionalInit,
    type OpenApiRequiredKeysOf,
    type RCDClient,
} from 'rclone-sdk'

type ClientPaths<T> = T extends OpenApiClient<infer P, any> ? P : never
type Paths = ClientPaths<RCDClient>
type InitParam<Init> = OpenApiRequiredKeysOf<Init> extends never
    ? [(Init & { [key: string]: unknown })?]
    : [Init & { [key: string]: unknown }]

export default async function rclone<
    Path extends OpenApiClientPathsWithMethod<RCDClient, 'post'>,
    Init extends OpenApiMaybeOptionalInit<Paths[Path], 'post'> = OpenApiMaybeOptionalInit<
        Paths[Path],
        'post'
    >,
>(
    path: Path,
    ...init: InitParam<Init>
): Promise<OpenApiMethodResponse<RCDClient, 'post', Path, Init>> {
    const client = createRCDClient({ baseUrl: 'http://localhost:5572' })

    const result = await client.POST(
        path,
        ...(init as InitParam<OpenApiMaybeOptionalInit<Paths[Path], 'post'>>)
    )

    if (result?.error) {
        const message =
            typeof result.error === 'string' ? result.error : JSON.stringify(result.error)

        throw new Error(message)
    }

    const data = result.data as { error?: unknown } | undefined
    if (data?.error) {
        const message = typeof data.error === 'string' ? data.error : JSON.stringify(data.error)

        throw new Error(message)
    }

    if (!result.response.ok) {
        throw new Error(`${result.response.status} ${result.response.statusText}`)
    }

    return result.data as OpenApiMethodResponse<typeof client, 'post', Path, Init>
}
```

Now you'll be able to
- call **`rclone('config/listremotes')`** directly
- get the data as the return value (correctly typed, and without having to de-construct the resulting object)
- have it throw on error with a detailed error message (instead of checking the **`error`** field manually)

## Contributing

Contributions = welcome! Just make sure to check if the PR isn't a better fit for the [**rclone-openapi**](https://github.com/rclone-ui/rclone-openapi) repo.


<br />
<br />

<div align="center">
<sub>Made with ☁️ for the <a href="https://discord.gg/rclone">rclone community</a></sub>
</div>
