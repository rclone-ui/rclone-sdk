/**
 * Temporary HTTP serve: start a server, download a file, then stop.
 *
 * Run: npx tsx examples/serve-download.ts <fs> <path>
 * Example: npx tsx examples/serve-download.ts dummy: hello.txt
 * Requires: rclone rcd --rc-no-auth
 */
import { writeFileSync } from 'node:fs'
import createRCDClient from '../src/index.js'

const rcd = createRCDClient({ baseUrl: 'http://localhost:5572' })

const [fs, remotePath] = process.argv.slice(2)
if (!fs || !remotePath) {
    console.error('Usage: npx tsx examples/serve-download.ts <fs> <path>')
    process.exit(1)
}

// Start a temporary HTTP server on a random port
const { data: serve } = await rcd.POST('/serve/start', {
    body: { type: 'http', fs, addr: ':0' },
})

if (!serve?.id) {
    console.error('Failed to start serve')
    process.exit(1)
}

console.log(`Serve started at ${serve.addr} (id: ${serve.id})`)

try {
    // Download the file
    const url = `http://${serve.addr}/${remotePath}`
    console.log(`Fetching ${url}`)

    const response = await fetch(url)
    if (!response.ok) {
        throw new Error(`${response.status} ${response.statusText}`)
    }

    const filename = remotePath.split('/').pop() ?? 'download'
    const bytes = Buffer.from(await response.arrayBuffer())
    writeFileSync(filename, bytes)
    console.log(`Saved ${filename} (${bytes.length} bytes)`)
} finally {
    // Always stop the server
    await rcd.POST('/serve/stop', { body: { id: serve.id } })
    console.log('Serve stopped')
}
