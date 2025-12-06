import createFetchClient, { type ClientOptions } from 'openapi-fetch'
import type { paths } from 'rclone-openapi'
import {
    createImmutableHook,
    createInfiniteHook,
    createMutateHook,
    createQueryHook,
} from 'swr-openapi'

/**
 * Creates SWR hooks that speak to the Rclone RC daemon.
 * Quick start:
 *   const swr = createRCDSWR({ baseUrl: "http://localhost:5572" });
 *   const { data } = swr.useQuery("/operations/about");
 * Note: Rclone RC routes are exposed as POST calls, even for read operations.
 */
export default function createRCDSWR(options: ClientOptions = {}) {
    const client = createFetchClient<paths>(options)

    return {
        useQuery: createQueryHook(client, 'rclone-swr'),
        useImmutable: createImmutableHook(client, 'rclone-swr'),
        useInfinite: createInfiniteHook(client, 'rclone-swr'),
        // simple identity matcher by default; callers can wrap if they need smarter behavior
        useMutate: createMutateHook(client, 'rclone-swr', (a, b) => a === b),
    }
}
