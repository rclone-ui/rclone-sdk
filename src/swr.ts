import createFetchClient, { type Client, type ClientOptions } from 'openapi-fetch'
import type { MediaType } from 'openapi-typescript-helpers'
import {
    createImmutableHook,
    createInfiniteHook,
    createMutateHook,
    createQueryHook,
} from 'swr-openapi'

import type { SyncPaths } from './shared.js'

/** `swr-openapi` only reads the `get` operation of a route, but rclone's routes are all POST. */
type PostAsGet<P> = {
    [Path in keyof P]: Omit<P[Path], 'get' | 'post'> & {
        get: P[Path] extends { post: infer Operation } ? Operation : never
        post?: never
    }
}

type SWRPaths = PostAsGet<SyncPaths>

type SWRHooks<P extends {}, M extends MediaType, Prefix extends string> = {
    useQuery: ReturnType<typeof createQueryHook<P, M, Prefix>>
    useImmutable: ReturnType<typeof createImmutableHook<P, M, Prefix>>
    useInfinite: ReturnType<typeof createInfiniteHook<P, M, Prefix>>
    useMutate: ReturnType<typeof createMutateHook<P, M>>
}

/** Whether a cached call's options contain everything in `partial`. */
function isMatch(value: unknown, partial: unknown): boolean {
    if (value === partial) return true
    if (!value || !partial || typeof value !== 'object' || typeof partial !== 'object') return false
    return Object.entries(partial).every(([key, item]) =>
        isMatch((value as Record<string, unknown>)[key], item)
    )
}

/**
 * Creates SWR hooks that speak to the Rclone RC daemon.
 * Quick start:
 *   const swr = createRCDSWR({ baseUrl: "http://localhost:5572" });
 *   const { data } = swr.useQuery("/operations/about", { body: { fs: "gdrive:" } });
 * Note: Rclone RC routes are exposed as POST calls, even for read operations.
 */
export default function createRCDSWR(
    options: ClientOptions = {}
): SWRHooks<SWRPaths, `${string}/${string}`, 'rclone-swr'> {
    const fetchClient = createFetchClient<SyncPaths>(options)
    // The hooks call `client.GET`, so point it at POST.
    const client = { ...fetchClient, GET: fetchClient.POST } as unknown as Client<SWRPaths>

    return {
        useQuery: createQueryHook(client, 'rclone-swr'),
        useImmutable: createImmutableHook(client, 'rclone-swr'),
        useInfinite: createInfiniteHook(client, 'rclone-swr'),
        useMutate: createMutateHook(client, 'rclone-swr', isMatch),
    }
}
