import type { Client } from 'openapi-fetch'
import type { components } from 'rclone-openapi'
import type { SyncPaths } from '../shared.js'

/** Interactive step from `config/create` / `config/update` (with `opt.nonInteractive`). */
export interface ConfigStep {
    State?: string
    Option?: components['schemas']['ConfigProviderOption'] | null
    Error?: string
    Result?: string
    [key: string]: unknown
}

/** Structured `opt` (rclone `UpdateRemoteOpt`); the spec types it as an opaque JSON string. */
export interface ConfigOpt {
    obscure?: boolean
    noObscure?: boolean
    noOutput?: boolean
    nonInteractive?: boolean
    continue?: boolean
    all?: boolean
    state?: string
    result?: string
    edit?: boolean
}

type EmptyObject = Record<string, never>

/** `ConfigStep` when `nonInteractive` is set, `{}` otherwise — resolved from the `opt` passed. */
export type ConfigResult<O> = O extends { nonInteractive: true } ? ConfigStep : EmptyObject

export interface ConfigCreateArgs<O extends ConfigOpt> {
    name: string
    type: string
    parameters?: Record<string, unknown>
    opt?: O
}

export interface ConfigUpdateArgs<O extends ConfigOpt> {
    name: string
    parameters?: Record<string, unknown>
    opt?: O
}

// `const O` captures the literal `opt` (keeps `nonInteractive: true` as `true`) so
// `ConfigResult` can resolve the return type. Throws the RC error body on failure.
export function makeConfigCreate(client: Client<SyncPaths>) {
    return async function configCreate<const O extends ConfigOpt = EmptyObject>(
        args: ConfigCreateArgs<O>
    ): Promise<ConfigResult<O>> {
        const { data, error } = await client.POST('/config/create', {
            params: {
                query: {
                    name: args.name,
                    type: args.type,
                    parameters: JSON.stringify(args.parameters ?? {}),
                    opt: JSON.stringify(args.opt ?? {}),
                },
            },
        })
        if (error) throw error
        return (data ?? {}) as unknown as ConfigResult<O>
    }
}

export function makeConfigUpdate(client: Client<SyncPaths>) {
    return async function configUpdate<const O extends ConfigOpt = EmptyObject>(
        args: ConfigUpdateArgs<O>
    ): Promise<ConfigResult<O>> {
        const { data, error } = await client.POST('/config/update', {
            params: {
                query: {
                    name: args.name,
                    parameters: JSON.stringify(args.parameters ?? {}),
                    opt: JSON.stringify(args.opt ?? {}),
                },
            },
        })
        if (error) throw error
        return (data ?? {}) as unknown as ConfigResult<O>
    }
}
