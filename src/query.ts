import createFetchClient, { type ClientOptions } from 'openapi-fetch'
import createRQClient from 'openapi-react-query'

import { type SyncPaths, preferAsyncMiddleware } from './shared.js'

/**
 * Creates a React Query client that talks to the Rclone RC daemon.
 * Quick start:
 *   const rqClient = createRCDQueryClient({ baseUrl: "http://localhost:5572" });
 *   const { data } = rqClient.useQuery("post", "/operations/about");
 * Note: Rclone RC routes are exposed as POST calls, even for read operations.
 */
export default function createRCDQueryClient(options: ClientOptions = {}) {
    const fetchClient = createFetchClient<SyncPaths>(options)
    fetchClient.use(preferAsyncMiddleware)

    return createRQClient(fetchClient)
}
