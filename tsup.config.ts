import { defineConfig } from 'tsup'

export default defineConfig({
    entry: {
        index: 'src/index.ts',
        query: 'src/query.ts',
        swr: 'src/swr.ts',
    },
    format: ['esm'],
    dts: true, // generate *.d.ts alongside each entry
    sourcemap: true,
    clean: true,
    target: 'es2020',
    platform: 'neutral',
    treeshake: true,
    minify: false, // keep readable; let apps minify if they want
})
