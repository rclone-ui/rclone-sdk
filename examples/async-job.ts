/**
 * Async operations: start a copy job and poll its status.
 *
 * Run: npx tsx examples/async-job.ts <srcFs> <dstFs>
 * Example: npx tsx examples/async-job.ts dummy: documents:backup/
 * Requires: rclone rcd --rc-no-auth
 */
import createRCDClient from '../src/index.js'

const rcd = createRCDClient({ baseUrl: 'http://localhost:5572' })

const [srcFs, dstFs] = process.argv.slice(2)
if (!srcFs || !dstFs) {
    console.error('Usage: npx tsx examples/async-job.ts <srcFs> <dstFs>')
    process.exit(1)
}

// Start an async copy
const { data: copyResult } = await rcd.POST('/sync/copy', {
    body: { srcFs, dstFs, _async: true },
})

const jobid = copyResult?.jobid
if (!jobid) {
    console.error('No job ID returned')
    process.exit(1)
}

console.log(`Started copy job ${jobid}: ${srcFs} -> ${dstFs}`)

// Poll until finished
let finished = false
while (!finished) {
    await new Promise((r) => setTimeout(r, 500))

    const { data: status } = await rcd.POST('/job/status', {
        body: { jobid },
    })

    if (!status) break

    if (!status.finished) {
        // Check transfer progress
        const { data: stats } = await rcd.POST('/core/stats', {
            body: { group: `job/${jobid}` },
        })
        const pct = stats?.totalBytes
            ? Math.round(((stats.bytes ?? 0) / stats.totalBytes) * 100)
            : 0
        process.stdout.write(`\r  Progress: ${pct}% (${stats?.transfers ?? 0} transfers)`)
    } else {
        finished = true
        console.log(
            status.success
                ? `\nDone in ${status.duration.toFixed(1)}s`
                : `\nFailed: ${status.error}`
        )
    }
}
