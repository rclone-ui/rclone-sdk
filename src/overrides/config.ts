import type { Client } from 'openapi-fetch'
import type { components, paths } from 'rclone-openapi'
import type { SyncPaths } from '../shared.js'

/** The next step of `config/create` / `config/update` (with `opt.nonInteractive`). */
export type ConfigStep =
    components['responses']['ConfigStepResponse']['content']['application/json']

/** The `opt` param of `config/create` and `config/update`. */
export type ConfigOpt = NonNullable<
    paths['/config/create']['post']['requestBody']['content']['application/json']['opt']
>

type EmptyObject = Record<string, never>

/**
 * `ConfigStep` when `nonInteractive` is set, `{}` when it isn't, and either one when `opt`'s value
 * isn't known at compile time.
 */
export type ConfigResult<O> = O extends { nonInteractive: true }
    ? ConfigStep
    : O extends { nonInteractive?: false }
      ? EmptyObject
      : ConfigStep | EmptyObject

// `O` is inferred, so TypeScript doesn't check it for extra fields on its own.
type OnlyConfigOpt<O> = O & Record<Exclude<keyof O, keyof ConfigOpt>, never>

export interface ConfigCreateArgs<O extends ConfigOpt> {
    name: string
    type: string
    parameters?: Record<string, unknown>
    opt?: OnlyConfigOpt<O>
}

export interface ConfigUpdateArgs<O extends ConfigOpt> {
    name: string
    parameters?: Record<string, unknown>
    opt?: OnlyConfigOpt<O>
}

// `const O` captures the literal `opt` (keeps `nonInteractive: true` as `true`) so
// `ConfigResult` can resolve the return type. Throws the RC error body on failure.
export function makeConfigCreate(client: Client<SyncPaths>) {
    return async function configCreate<const O extends ConfigOpt = EmptyObject>(
        args: ConfigCreateArgs<O>
    ): Promise<ConfigResult<O>> {
        const { data, error } = await client.POST('/config/create', {
            body: {
                name: args.name,
                type: args.type,
                parameters: args.parameters ?? {},
                opt: args.opt,
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
            body: {
                name: args.name,
                parameters: args.parameters ?? {},
                opt: args.opt,
            },
        })
        if (error) throw error
        return (data ?? {}) as unknown as ConfigResult<O>
    }
}
