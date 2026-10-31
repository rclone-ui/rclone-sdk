// Type test: compiles real SDK calls against the spec's types.
// Each @ts-expect-error fails `npm run check` if that call starts compiling.

import createRCDClient, {
    type AsyncJobResponse,
    type ConfigOpt,
    type ConfigStep,
    type RCDArgs,
    type RCDData,
    type RCDPath,
} from '../src/index.js'
import createRCDQueryClient from '../src/query.js'
import createRCDSWR from '../src/swr.js'

const rcd = createRCDClient({ baseUrl: 'http://localhost:5572' })

export async function requiredFields() {
    // @ts-expect-error jobid is missing
    await rcd.POST('/job/status')
    // @ts-expect-error jobid is missing
    await rcd.POST('/job/status', {})
    // @ts-expect-error jobid is missing
    await rcd.POST('/job/status', { body: {} })

    await rcd.POST('/job/status', { body: { jobid: 1 } })
    await rcd.POST('/core/version')
    await rcd.POST('/core/version', { body: { _group: 'g' } })
}

export async function extraFields() {
    // @ts-expect-error rclone ignores recurse at the top level
    await rcd.POST('/operations/list', { body: { fs: '/tmp', remote: '', recurse: true } })
    // @ts-expect-error rclone ignores unknown fields inside opt too
    await rcd.POST('/operations/list', { body: { fs: '/tmp', remote: '', opt: { bogus: true } } })
    // @ts-expect-error _async isn't documented on routes, use ASYNC
    await rcd.POST('/sync/copy', { body: { srcFs: 'a:', dstFs: 'b:', _async: true } })
    // @ts-expect-error query params aren't documented
    await rcd.POST('/job/status', { body: { jobid: 1 }, params: { query: { jobid: 1 } } })
    // @ts-expect-error unknown top-level option
    await rcd.POST('/core/version', { bogus: 1 })

    await rcd.POST('/operations/list', { body: { fs: '/tmp', remote: '', opt: { recurse: true } } })
    await rcd.POST('/operations/list', {
        body: { fs: { type: 'local', _root: '/tmp' }, remote: '' },
    })
    await rcd.POST('/rc/noop', { body: { anything: 1 } })
    await rcd.POST('/serve/start', { body: { type: 'http', fs: '/tmp', read_only: true } })
    await rcd.POST('/job/batch', { body: { inputs: [{ _path: 'rc/noop', _async: true }] } })
    await rcd.POST('/core/version', {
        signal: new AbortController().signal,
        headers: { 'X-A': 'b' },
    })
}

export async function syncAndAsync(form: FormData) {
    const sync = await rcd.POST('/core/version')
    const version: string | undefined = sync.data?.version
    // @ts-expect-error POST never returns a job ID
    const notAJob = sync.data?.jobid

    await rcd.POST('/sync/copy', {
        body: { srcFs: 'a:', dstFs: 'b:' },
        // @ts-expect-error POST waits for the result, so it doesn't take Prefer
        params: { header: { Prefer: 'respond-async' } },
    })

    const job = await rcd.ASYNC('/sync/copy', { body: { srcFs: 'a:', dstFs: 'b:' } })
    const id: number | undefined = job.data?.jobid
    const executeId: string | undefined = job.data?.executeId
    const typed: AsyncJobResponse | undefined = job.data
    await rcd.ASYNC('/core/version')

    // @ts-expect-error srcFs and dstFs are missing
    await rcd.ASYNC('/sync/copy')
    // @ts-expect-error extra fields fail here too
    await rcd.ASYNC('/sync/copy', { body: { srcFs: 'a:', dstFs: 'b:', bogus: 1 } })
    // @ts-expect-error uploads can't run in the background
    await rcd.ASYNC('/operations/uploadfile', {
        params: { query: { fs: 'a:', remote: 'x' } },
        body: form,
    })

    return [version, notAJob, id, executeId, typed]
}

export async function stringOnlyRoutes() {
    await rcd.POST('/vfs/refresh', { body: { _group: 'g', dir: '/a', dir2: '/b' } })
    // @ts-expect-error the extra keys on this route are strings, so the schema leaves _config out
    await rcd.POST('/vfs/refresh', { body: { _config: { Transfers: 2 } } })
}

export async function parseAs() {
    const text = await rcd.POST('/core/version', { parseAs: 'text' })
    const value: string | undefined = text.data
    const json = await rcd.POST('/job/status', { body: { jobid: 1 } })
    const duration: number | undefined = json.data?.duration
    return [value, duration]
}

export async function uploads(form: FormData) {
    // @ts-expect-error fs and remote are missing
    await rcd.POST('/operations/uploadfile', { body: form })
    // @ts-expect-error the body is missing
    await rcd.POST('/operations/uploadfile', { params: { query: { fs: 'a:', remote: 'x' } } })
    await rcd.POST('/operations/uploadfile', {
        params: { query: { fs: 'a:', remote: 'x' } },
        // @ts-expect-error rclone would return 200 and write nothing
        body: { file: 'hello' },
    })

    await rcd.POST('/operations/uploadfile', {
        params: { query: { fs: 'a:', remote: 'x', _group: 'g' } },
        body: form,
    })
}

export async function configOverrides(unknownOpt: ConfigOpt) {
    const step = await rcd.configCreate({ name: 'x', type: 's3', opt: { nonInteractive: true } })
    const state: ConfigStep['State'] = step.State

    const empty = await rcd.configCreate({ name: 'x', type: 'alias', parameters: { remote: '/' } })
    const none: Record<string, never> = empty

    const either = await rcd.configUpdate({ name: 'x', opt: unknownOpt })
    // @ts-expect-error opt isn't known at compile time, so this could be a step
    const notAlwaysEmpty: Record<string, never> = either

    // @ts-expect-error bogus isn't an option
    await rcd.configUpdate({ name: 'x', opt: { bogus: true } })
    // @ts-expect-error bogus isn't an option, even next to a real one
    await rcd.configCreate({ name: 'x', type: 's3', opt: { nonInteractive: true, bogus: 1 } })

    return [state, none, notAlwaysEmpty]
}

// The wrapper from the README
const post = rcd.POST as unknown as (
    path: string,
    init?: unknown
) => Promise<{ data?: unknown; error?: unknown }>

export async function rclone<Path extends RCDPath>(path: Path, ...init: RCDArgs<Path>) {
    const { data, error } = await post(path, init[0])
    if (error) throw new Error(typeof error === 'string' ? error : JSON.stringify(error))
    return data as RCDData<Path>
}

export async function wrapper() {
    const remotes = await rclone('/config/listremotes')
    const names: string[] = remotes.remotes
    const status = await rclone('/job/status', { body: { jobid: 1 } })
    const finished: boolean = status.finished
    // @ts-expect-error jobid is missing
    await rclone('/job/status')
    // @ts-expect-error extra fields fail in the wrapper too
    await rclone('/job/status', { body: { jobid: 1, bogus: 1 } })
    return [names, finished]
}

export function hooks() {
    const rq = createRCDQueryClient({ baseUrl: 'http://localhost:5572' })
    const a = rq.useQuery('post', '/config/listremotes')
    const remotes: string[] | undefined = a.data?.remotes
    const b = rq.useQuery('post', '/operations/about', { body: { fs: 'a:' } })
    const used: number | undefined = b.data?.used
    // @ts-expect-error fs is missing
    rq.useQuery('post', '/operations/about', { body: {} })

    const swr = createRCDSWR({ baseUrl: 'http://localhost:5572' })
    const c = swr.useQuery('/config/listremotes')
    const swrRemotes: string[] | undefined = c.data?.remotes
    const d = swr.useQuery('/operations/about', { body: { fs: 'a:' } })
    const swrUsed: number | undefined = d.data?.used
    // @ts-expect-error fs is missing
    swr.useQuery('/operations/about', { body: {} })
    // @ts-expect-error swr-openapi rejects extra fields itself
    swr.useQuery('/operations/about', { body: { fs: 'a:', bogus: 1 } })
    swr.useImmutable('/config/listremotes')

    const mutate = swr.useMutate()
    mutate(['/operations/about', { body: { fs: 'a:' } }])

    return [remotes, used, swrRemotes, swrUsed]
}
