import createFetchClient, { type Client, type ClientOptions } from 'openapi-fetch'
import type { paths } from 'rclone-openapi'
export type {
    Client as OpenApiClient,
    ClientPathsWithMethod as OpenApiClientPathsWithMethod,
    MaybeOptionalInit as OpenApiMaybeOptionalInit,
    MethodResponse as OpenApiMethodResponse,
} from 'openapi-fetch'
export type { RequiredKeysOf as OpenApiRequiredKeysOf } from 'openapi-typescript-helpers'

export type RCDClient = Client<paths>

/**
 * Creates a typed fetch client for the Rclone RC daemon.
 * Quick start:
 *   const rcd = createRCDClient({ baseUrl: "http://localhost:5572" });
 *   const response = await rcd.POST("/config/listremotes");
 * Note: Rclone RC routes are exposed as POST calls, even for read operations.
 */
export default function createRCDClient(options: ClientOptions = {}): RCDClient {
    return createFetchClient<paths>(options)
}
