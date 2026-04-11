/**
 * Basic usage: list remotes, get version, and browse a remote.
 *
 * Run: npx tsx examples/basic.ts
 * Requires: rclone rcd --rc-no-auth
 */
import createRCDClient from '../src/index.js'

const rcd = createRCDClient({ baseUrl: 'http://localhost:5572' })

// Get rclone version
const { data: version } = await rcd.POST('/core/version')
console.log('rclone', version?.version)

// List configured remotes
const { data: remotes } = await rcd.POST('/config/listremotes')
console.log('Remotes:', remotes?.remotes)

// Browse the first remote (if any)
const first = remotes?.remotes?.[0]
if (first) {
    const { data: listing } = await rcd.POST('/operations/list', {
        body: { fs: `${first}:`, remote: '' },
    })
    console.log(`\nContents of ${first}: (${listing?.list?.length ?? 0} items)`)
    for (const item of listing?.list?.slice(0, 10) ?? []) {
        console.log(`  ${item.IsDir ? '📁' : '📄'} ${item.Name} (${item.Size} bytes)`)
    }
}
