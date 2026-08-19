import type { Middleware } from 'openapi-fetch'
import type { paths } from 'rclone-openapi'

/**
 * Strips 202 (async job) responses from all operations.
 * openapi-fetch unions all 2xx response bodies into `data`, so without this,
 * every endpoint's `data` becomes `NormalResponse | { jobid: number }`.
 * This gives clean types for synchronous calls.
 */
export type Strip202<P> = {
    [Path in keyof P]: {
        [Method in keyof P[Path]]: P[Path][Method] extends {
            responses: infer R extends Record<string | number, any>
        }
            ? Omit<P[Path][Method], 'responses'> & { responses: Omit<R, 202> }
            : P[Path][Method]
    }
}

// Sync client paths (202 stripped). Response shapes that depend on a request
// field (e.g. config/create) can't be typed here — they get per-path wrappers
// in ./overrides/ instead.
export type SyncPaths = Strip202<paths>

/** Strips 200 responses, keeping only 202 (async) + error codes. */
export type Strip200<P> = {
    [Path in keyof P]: {
        [Method in keyof P[Path]]: P[Path][Method] extends {
            responses: infer R extends Record<string | number, any>
        }
            ? Omit<P[Path][Method], 'responses'> & { responses: Omit<R, 200> }
            : P[Path][Method]
    }
}

/** Paths with only the 202 response for async calls. */
export type AsyncPaths = Strip200<paths>

/** The response body for async job submissions (HTTP 202). */
export type AsyncJobResponse = { jobid: number }

/**
 * Middleware that detects `_async: true` in the request body or query params
 * and automatically injects the `Prefer: respond-async` header.
 *
 * Respects existing `Prefer` header values per RFC 7240 (comma-separated) —
 * appends `respond-async` only if not already present.
 */
export const preferAsyncMiddleware: Middleware = {
    onRequest: async ({ request, params }) => {
        let isAsync = params.query?._async === true

        if (!isAsync && request.body) {
            try {
                const body = await request.clone().json()
                if (body?._async === true) {
                    isAsync = true
                }
            } catch {
                // Not JSON or empty body, skip
            }
        }

        if (!isAsync) return

        const existing = request.headers.get('Prefer') ?? ''
        const alreadySet = existing
            .split(',')
            .some((p) => p.trim().toLowerCase() === 'respond-async')

        if (alreadySet) return

        const headers = new Headers(request.headers)
        headers.set('Prefer', existing ? `${existing}, respond-async` : 'respond-async')
        return new Request(request, { headers })
    },
}
