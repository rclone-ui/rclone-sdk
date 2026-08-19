import type { Client } from 'openapi-fetch'
import type { SyncPaths } from '../shared.js'
import { makeConfigCreate, makeConfigUpdate } from './config.js'

export type {
    ConfigStep,
    ConfigOpt,
    ConfigResult,
    ConfigCreateArgs,
    ConfigUpdateArgs,
} from './config.js'

// Per-path typed override methods, composed onto the client by `createRCDClient`.
// To add a path override: write its module and add one line here.
export function applyOverrides(client: Client<SyncPaths>) {
    return {
        configCreate: makeConfigCreate(client),
        configUpdate: makeConfigUpdate(client),
    }
}

/** The typed override methods attached to `RCDClient`. */
export type Overrides = ReturnType<typeof applyOverrides>
