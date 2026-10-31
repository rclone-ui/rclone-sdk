import createFetchClient, {
    type Client,
    type ClientOptions,
    type FetchResponse,
    type MaybeOptionalInit,
    type ParseAs,
} from 'openapi-fetch'
import type { PathsWithMethod, RequiredKeysOf } from 'openapi-typescript-helpers'
import { type Overrides, applyOverrides } from './overrides/index.js'
import type { AsyncPaths, SyncPaths } from './shared.js'

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

type DefaultMedia = `${string}/${string}`

/** Every route `POST` accepts. */
export type RCDPath = PathsWithMethod<SyncPaths, 'post'>

/** Every route that can run in the background. Uploads can't. */
export type RCDAsyncPath = {
    [Path in RCDPath]: 202 extends keyof AsyncPaths[Path]['post']['responses'] ? Path : never
}[RCDPath]

/** The options for a call to `Path`. */
export type RCDInit<Path extends RCDPath> = NonNullable<MaybeOptionalInit<SyncPaths[Path], 'post'>>

/**
 * The arguments after the path. `openapi-fetch` infers its options, so TypeScript doesn't check
 * them for extra fields. These options aren't inferred, so a field that rclone would ignore fails.
 */
export type RCDArgs<Path extends RCDPath, As extends ParseAs = 'json'> = RequiredKeysOf<
    RCDInit<Path>
> extends never
    ? [(RCDInit<Path> & { parseAs?: As })?]
    : [RCDInit<Path> & { parseAs?: As }]

/** The body of a successful call to `Path`. */
export type RCDData<Path extends RCDPath> = NonNullable<
    FetchResponse<SyncPaths[Path]['post'], { parseAs: 'json' }, DefaultMedia>['data']
>

export type RCDClient = Omit<Client<SyncPaths>, 'POST'> & {
    /** Runs a call and waits for it to finish. Returns the route's normal 200 response. */
    POST<Path extends RCDPath, As extends ParseAs = 'json'>(
        url: Path,
        ...init: RCDArgs<Path, As>
    ): Promise<FetchResponse<SyncPaths[Path]['post'], { parseAs: As }, DefaultMedia>>
    /**
     * Runs a call in the background by sending `Prefer: respond-async`.
     * Returns the 202 response with the job ID. Poll `/job/status` with it.
     */
    ASYNC<Path extends RCDAsyncPath>(
        url: Path,
        ...init: RCDArgs<Path>
    ): Promise<FetchResponse<AsyncPaths[Path]['post'], { parseAs: 'json' }, DefaultMedia>>
} & Overrides

/**
 * Creates a typed fetch client for the Rclone RC daemon.
 * Quick start:
 *   const rcd = createRCDClient({ baseUrl: "http://localhost:5572" });
 *   const { data } = await rcd.POST("/config/listremotes");       // waits for the result
 *   const { data } = await rcd.ASYNC("/sync/copy", { body: { srcFs: "a:", dstFs: "b:" } }); // runs in the background, returns { jobid, executeId }
 *   const step = await rcd.configCreate({ name, type, opt: { nonInteractive: true } }); // typed ConfigStep (flag-dependent)
 * Note: Rclone RC routes are exposed as POST calls, even for read operations.
 */
export default function createRCDClient(options: ClientOptions = {}): RCDClient {
    const client = createFetchClient<SyncPaths>(options)

    const enhanced = client as unknown as RCDClient
    enhanced.ASYNC = (url, ...init) => {
        const opts = (init[0] ?? {}) as Record<string, any>
        return client.POST(url, {
            ...opts,
            params: { ...opts.params, header: { ...opts.params?.header, Prefer: 'respond-async' } },
        } as any) as any
    }

    Object.assign(enhanced, applyOverrides(client))

    return enhanced
}
