import type { components, paths } from 'rclone-openapi'

/**
 * Removes what a normal call doesn't need: the 202 response and the `Prefer` header.
 * Without this, `data` on every route is `NormalResponse | AsyncJobResponse`, and `Prefer`
 * would let a call return a job ID while its type says it returns the normal response.
 * Use `ASYNC` to run a call in the background.
 */
export type Strip202<P> = {
    [Path in keyof P]: {
        [Method in keyof P[Path]]: P[Path][Method] extends {
            parameters: infer Params
            responses: infer R extends Record<string | number, any>
        }
            ? Omit<P[Path][Method], 'parameters' | 'responses'> & {
                  parameters: Omit<Params, 'header'> & { header?: never }
                  responses: Omit<R, 202>
              }
            : P[Path][Method]
    }
}

type UploadPath = '/operations/uploadfile'

// The spec can't describe a form's parts, so its type for this body is `unknown`. Rclone reads
// each part that has a file name and returns 200 for any other body, so only a form makes sense.
type WithFormBody<Route> = Route extends { post: infer Operation }
    ? Omit<Route, 'post'> & {
          post: Omit<Operation, 'requestBody'> & {
              requestBody: { content: { 'multipart/form-data': FormData } }
          }
      }
    : Route

// Sync client paths (202 stripped). Response shapes that depend on a request
// field (e.g. config/create) can't be typed here — they get per-path wrappers
// in ./overrides/ instead.
export type SyncPaths = Omit<Strip202<paths>, UploadPath> & {
    [Path in UploadPath]: WithFormBody<Strip202<paths>[Path]>
}

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
export type AsyncJobResponse =
    components['responses']['AsyncJobResponse']['content']['application/json']
