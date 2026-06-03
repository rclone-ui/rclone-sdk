import createFetchClient, { type Client, type ClientOptions } from 'openapi-fetch'
import type { MediaType } from 'openapi-typescript-helpers'
import {
    createImmutableHook,
    createInfiniteHook,
    createMutateHook,
    createQueryHook,
} from 'swr-openapi'

import { type SyncPaths, preferAsyncMiddleware } from './shared.js'

type SWRHooks<P extends {}, M extends MediaType, Prefix extends string> = {
    useQuery: ReturnType<typeof createQueryHook<P, M, Prefix>>
    useImmutable: ReturnType<typeof createImmutableHook<P, M, Prefix>>
    useInfinite: ReturnType<typeof createInfiniteHook<P, M, Prefix>>
    useMutate: ReturnType<typeof createMutateHook<P, M>>
}

/**
 * Creates SWR hooks that speak to the Rclone RC daemon.
 * Quick start:
 *   const swr = createRCDSWR({ baseUrl: "http://localhost:5572" });
 *   const { data } = swr.useQuery("/operations/about");
 * Note: Rclone RC routes are exposed as POST calls, even for read operations.
 */
export default function createRCDSWR(
    options: ClientOptions = {}
): SWRHooks<SyncPaths, `${string}/${string}`, 'rclone-swr'> {
    const client: Client<SyncPaths> = createFetchClient<SyncPaths>(options)
    client.use(preferAsyncMiddleware)

    return {
        useQuery: createQueryHook(client, 'rclone-swr'),
        useImmutable: createImmutableHook(client, 'rclone-swr'),
        useInfinite: createInfiniteHook(client, 'rclone-swr'),
        useMutate: createMutateHook(client, 'rclone-swr', (a, b) => a === b),
    }
}
