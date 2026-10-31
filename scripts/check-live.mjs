// Runs both clients against a real rclone server, and checks what types can't show.
//
// It downloads the official rclone release for the version in package.json, starts it on
// loopback with a temporary config and home directory, and fails on any wrong behaviour.
// `npm run check:live` builds `dist` first. The Rust side uses the crate in `rs`, so run
// `npm run gen:rs` first if the generator or the spec changed.
//
// OPENAPI_RCLONE_BIN uses a local binary instead of downloading one, as in rclone-openapi.

import assert from 'node:assert/strict'
import { execFileSync, spawn } from 'node:child_process'
import { createHash, randomBytes } from 'node:crypto'
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs'
import { createServer } from 'node:net'
import { tmpdir } from 'node:os'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import createRCDClient from '../dist/index.js'

const ROOT = join(dirname(fileURLToPath(import.meta.url)), '..')
const CACHE = join(ROOT, '.work', 'cache')
const { version } = JSON.parse(readFileSync(join(ROOT, 'package.json'), 'utf8'))

// --- Getting rclone -----------------------------------------------------------

async function text(url) {
    const r = await fetch(url)
    if (!r.ok) throw new Error(`${url} returned ${r.status}`)
    return r.text()
}

async function getBinary() {
    if (process.env.OPENAPI_RCLONE_BIN) return process.env.OPENAPI_RCLONE_BIN
    const platform = { darwin: 'osx', linux: 'linux', win32: 'windows' }[process.platform]
    const arch = { arm64: 'arm64', x64: 'amd64' }[process.arch]
    const name = `rclone-v${version}-${platform}-${arch}`
    const dir = join(CACHE, name)
    const binary = join(dir, name, process.platform === 'win32' ? 'rclone.exe' : 'rclone')
    if (existsSync(binary)) return binary

    console.log(`Downloading ${name}`)
    const base = `https://downloads.rclone.org/v${version}`
    const sums = await text(`${base}/SHA256SUMS`)
    const expected = sums.match(new RegExp(`^([0-9a-f]{64})\\s+${name}\\.zip$`, 'm'))?.[1]
    if (!expected) throw new Error(`${name}.zip isn't listed in SHA256SUMS`)
    const r = await fetch(`${base}/${name}.zip`)
    if (!r.ok) throw new Error(`${name}.zip returned ${r.status}`)
    const zip = Buffer.from(await r.arrayBuffer())
    const actual = createHash('sha256').update(zip).digest('hex')
    if (actual !== expected) throw new Error(`${name}.zip has the wrong checksum`)

    mkdirSync(dir, { recursive: true })
    const zipPath = join(dir, `${name}.zip`)
    writeFileSync(zipPath, zip)
    execFileSync('unzip', ['-q', '-o', zipPath, '-d', dir])
    rmSync(zipPath)
    return binary
}

const freePort = () =>
    new Promise((resolve, reject) => {
        const s = createServer()
        s.on('error', reject)
        s.listen(0, '127.0.0.1', () => {
            const { port } = s.address()
            s.close(() => resolve(port))
        })
    })

const sleep = (ms) => new Promise((r) => setTimeout(r, ms))

// --- Starting the server ------------------------------------------------------

const binary = await getBinary()
const env = { PATH: process.env.PATH, HOME: tmpdir() }
const reported = execFileSync(binary, ['version', '--check=false'], { encoding: 'utf8', env })
const running = reported.split('\n')[0]
if (running !== `rclone v${version}`) {
    throw new Error(`The SDK targets rclone ${version}, but ${binary} is ${running}`)
}
console.log(`Checking the SDK ${version} against ${running}`)

const work = mkdtempSync(join(tmpdir(), 'rclone-sdk-'))
const home = join(work, 'home')
const SRC = join(work, 'src')
const DST = join(work, 'dst')
const RUST_DST = join(work, 'rust-dst')
for (const d of [home, SRC, DST, RUST_DST]) mkdirSync(d)
writeFileSync(join(SRC, 'a.txt'), 'hello\n')

const user = randomBytes(8).toString('hex')
const pass = randomBytes(16).toString('hex')
const port = await freePort()
const baseUrl = `http://127.0.0.1:${port}`
const server = spawn(
    binary,
    [
        'rcd',
        '--config',
        join(home, 'rclone.conf'),
        '--cache-dir',
        join(home, 'cache'),
        '--rc-addr',
        `127.0.0.1:${port}`,
        '--rc-user',
        user,
        '--rc-pass',
        pass,
    ],
    { cwd: home, env: { PATH: process.env.PATH, HOME: home }, stdio: 'ignore' }
)
process.on('exit', () => {
    server.kill()
    rmSync(work, { recursive: true, force: true })
})

let started = false
for (let i = 0; i < 100 && !started; i++) {
    try {
        await fetch(`${baseUrl}/rc/noop`, { method: 'POST' })
        started = true
    } catch {
        await sleep(100)
    }
}
if (!started) throw new Error('rclone did not start')

// --- TypeScript ---------------------------------------------------------------

const headers = { Authorization: `Basic ${Buffer.from(`${user}:${pass}`).toString('base64')}` }
const rcd = createRCDClient({ baseUrl, headers })

const about = await rcd.POST('/core/version')
assert.equal(about.response.status, 200)
assert.equal(about.data.version, `v${version}`)

// ASYNC runs in the background
const job = await rcd.ASYNC('/sync/copy', { body: { srcFs: SRC, dstFs: DST } })
assert.equal(job.response.status, 202)
assert.equal(job.response.headers.get('preference-applied'), 'respond-async')
assert.equal(typeof job.data.jobid, 'number')
assert.equal(typeof job.data.executeId, 'string')
assert.equal((await rcd.ASYNC('/core/version')).response.status, 202)

let status
for (let i = 0; i < 100 && !status?.finished; i++) {
    await sleep(50)
    status = (await rcd.POST('/job/status', { body: { jobid: job.data.jobid } })).data
}
assert.equal(status.finished, true)
assert.equal(status.success, true)
assert.equal(readFileSync(join(DST, 'a.txt'), 'utf8'), 'hello\n')

// The root of a remote is an empty string
const listing = await rcd.POST('/operations/list', { body: { fs: DST, remote: '' } })
assert.deepEqual(
    listing.data.list.map((item) => item.Name),
    ['a.txt']
)

// Uploads write the file, into the directory that `remote` names
const form = new FormData()
form.append('file', new Blob(['uploaded']), 'up.txt')
const upload = await rcd.POST('/operations/uploadfile', {
    params: { query: { fs: DST, remote: 'sub' } },
    body: form,
})
assert.equal(upload.response.status, 200)
assert.equal(readFileSync(join(DST, 'sub', 'up.txt'), 'utf8'), 'uploaded')

// Config overrides send a body, and the response depends on opt
assert.deepEqual(
    await rcd.configCreate({ name: 'al', type: 'alias', parameters: { remote: SRC } }),
    {}
)
const remote = await rcd.POST('/config/get', { body: { name: 'al' } })
assert.deepEqual(remote.data, { remote: SRC, type: 'alias' })
const question = await rcd.configCreate({
    name: 'q',
    type: 's3',
    opt: { nonInteractive: true, all: true },
})
assert.equal(question.Option?.Name, 'provider')
await rcd.configUpdate({ name: 'al', parameters: { remote: DST } })
const updated = await rcd.POST('/config/get', { body: { name: 'al' } })
assert.equal(updated.data.remote, DST)
await assert.rejects(rcd.configUpdate({ name: 'nope' }))

// A failed login is plain text, and parseAs still works
const noLogin = await createRCDClient({ baseUrl }).POST('/core/version')
assert.equal(noLogin.response.status, 401)
assert.equal(typeof noLogin.error, 'string')
assert.equal(typeof (await rcd.POST('/core/version', { parseAs: 'text' })).data, 'string')

console.log('The TypeScript client works.')

// --- Rust ---------------------------------------------------------------------

const crate = join(ROOT, 'scripts', 'check-live-rs')
execFileSync('cargo', ['run', '--quiet', '--manifest-path', join(crate, 'Cargo.toml')], {
    stdio: 'inherit',
    env: {
        ...process.env,
        CARGO_TARGET_DIR: join(ROOT, '.work', 'check-live-rs'),
        CHECK_URL: baseUrl,
        CHECK_USER: user,
        CHECK_PASS: pass,
        CHECK_VERSION: `v${version}`,
        CHECK_SRC: SRC,
        CHECK_DST: RUST_DST,
    },
})

console.log('The Rust client works.')
process.exit(0)
