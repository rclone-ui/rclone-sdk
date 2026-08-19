import createFetchClient, {
    type Client,
    type ClientOptions,
    type FetchResponse,
    type MaybeOptionalInit,
} from 'openapi-fetch'
import type { PathsWithMethod, RequiredKeysOf } from 'openapi-typescript-helpers'
import type { paths } from 'rclone-openapi'
import { type Overrides, applyOverrides } from './overrides/index.js'
import { type AsyncPaths, type SyncPaths, preferAsyncMiddleware } from './shared.js'

export type {
    Client as OpenApiClient,
    ClientPathsWithMethod as OpenApiClientPathsWithMethod,
    MaybeOptionalInit as OpenApiMaybeOptionalInit,
    MethodResponse as OpenApiMethodResponse,
} from 'openapi-fetch'
export type { RequiredKeysOf as OpenApiRequiredKeysOf } from 'openapi-typescript-helpers'
export type { paths } from 'rclone-openapi'
export type {
    AsyncJobResponse,
    AsyncPaths,
    Strip200,
    Strip202,
    SyncPaths,
} from './shared.js'
export type {
    ConfigStep,
    ConfigOpt,
    ConfigResult,
    ConfigCreateArgs,
    ConfigUpdateArgs,
    Overrides,
} from './overrides/index.js'

type InitParam<Init> = RequiredKeysOf<Init> extends never
    ? [(Init & { [key: string]: unknown })?]
    : [Init & { [key: string]: unknown }]

type DefaultMedia = `${string}/${string}`

export type RCDClient = Client<SyncPaths> & {
    /**
     * POST with forced async execution.
     * Automatically sets `_async: true` and `Prefer: respond-async`.
     * Returns the 202 response type (`{ jobid: number }`).
     */
    ASYNC<
        Path extends PathsWithMethod<SyncPaths, 'post'>,
        Init extends MaybeOptionalInit<paths[Path], 'post'>,
    >(
        url: Path,
        ...init: InitParam<Init>
    ): Promise<FetchResponse<AsyncPaths[Path & keyof AsyncPaths]['post'], Init, DefaultMedia>>
} & Overrides

/**
 * Creates a typed fetch client for the Rclone RC daemon.
 * Quick start:
 *   const rcd = createRCDClient({ baseUrl: "http://localhost:5572" });
 *   const { data } = await rcd.POST("/config/listremotes");       // sync — typed 200 response
 *   const { data } = await rcd.ASYNC("/sync/copy", { body: { srcFs: "a:", dstFs: "b:" } }); // async — typed 202 { jobid }
 *   const step = await rcd.configCreate({ name, type, opt: { nonInteractive: true } }); // typed ConfigStep (flag-dependent)
 * Note: Rclone RC routes are exposed as POST calls, even for read operations.
 */
export default function createRCDClient(options: ClientOptions = {}): RCDClient {
    const client = createFetchClient<SyncPaths>(options)
    client.use(preferAsyncMiddleware)

    const enhanced = client as RCDClient
    enhanced.ASYNC = (url, ...init) => {
        const opts = (init[0] ?? {}) as Record<string, any>
        return client.POST(url, {
            ...opts,
            body: { ...opts.body, _async: true },
        } as any) as any
    }

    Object.assign(enhanced, applyOverrides(client))

    return enhanced
}
