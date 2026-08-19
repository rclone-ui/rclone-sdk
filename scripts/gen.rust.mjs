// Rust generator: 3.1→3.0 down-convert + format-adapters + declarative overrides
// → cargo progenitor → post-gen fixups (group rename, wire in overrides module).

import { execSync } from 'node:child_process'
import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { Converter } from '@apiture/openapi-down-convert'
import yaml from 'js-yaml'

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..')
const GEN_FOLDER = path.resolve(ROOT, 'rs')
const SOURCE_SPEC = path.resolve(ROOT, 'node_modules/rclone-openapi/openapi.yaml')
const OUTPUT_SPEC = path.resolve(ROOT, 'openapi-3.0.json')
const KEYWORDS = ['rclone', 'sdk', 'openapi', 'client']
const CATEGORIES = ['api-bindings', 'web-programming::http-client']
const HOMEPAGE = 'https://rcloneui.com'
const REPOSITORY = 'https://github.com/rclone-ui/rclone-sdk'
const README = '../README.md'

// --- Override registry: facts the OpenAPI spec can't express --------------

// A: uploadfile has a binary body (spec models it as multipart/unknown).
const uploadFileBinary = {
    target: 'both',
    kind: 'request-body',
    path: '/operations/uploadfile',
    description: 'Binary payload containing the file to upload.',
    mediaType: 'application/octet-stream',
    schema: { type: 'string', format: 'binary' },
}

// C: ConfigProviderOptionAny is arbitrary JSON → empty schema → serde_json::Value
// (was objects-only serde_json::Map, which failed on a string/number Default).
const arbitraryJsonValue = {
    target: 'rust',
    kind: 'schema-replace',
    schema: 'ConfigProviderOptionAny',
    replacement: {},
}

// B: `_group` and a route `group` param both sanitize to Rust field `group`
// (duplicate-field compile error) → rename the `_group`-derived one to `group_`.
const groupCollision = {
    target: 'rust',
    kind: 'rust-rename-post-gen',
    routes: [
        '/core/stats',
        '/core/stats-delete',
        '/core/stats-reset',
        '/core/transferred',
        '/job/stopgroup',
    ],
    wireName: '_group',
    from: 'group',
    to: 'group_',
}

function loadSourceSpec(filePath) {
    if (!fs.existsSync(filePath)) {
        throw new Error(`Source spec not found at ${filePath}`)
    }
    return yaml.load(fs.readFileSync(filePath, 'utf8'))
}

function readPackageVersion() {
    const pkgPath = path.resolve(ROOT, 'package.json')
    const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'))
    if (!pkg.version) {
        throw new Error('package.json does not contain a version')
    }
    return pkg.version
}

// --- Format adapters (not overrides) --------------------------------------

function convertToOpenApi30(openapi31) {
    const converter = new Converter(openapi31, {
        verbose: false,
        deleteExampleWithId: true,
        allOfTransform: false,
    })
    return converter.convert()
}

// Drop the 202 (async) response where a 200 exists — progenitor 0.11 allows ≤1
// success type per operation. Mirrors the TS `Strip202` (Rust is the sync client).
function stripAsyncResponses(doc) {
    for (const pathItem of Object.values(doc.paths ?? {})) {
        if (!pathItem || typeof pathItem !== 'object') continue
        for (const op of Object.values(pathItem)) {
            if (!op || typeof op !== 'object' || !op.responses) continue
            const hasOk = op.responses['200'] !== undefined || op.responses[200] !== undefined
            if (hasOk && op.responses['202'] !== undefined) {
                delete op.responses['202']
            }
        }
    }
}

function isNullSchema(schema) {
    return (
        schema &&
        typeof schema === 'object' &&
        schema.type === 'null' &&
        Object.keys(schema).length === 1
    )
}

function convertAnyOfNullToNullable(target) {
    if (!target || typeof target !== 'object' || !Array.isArray(target.anyOf)) {
        return
    }
    const filtered = target.anyOf.filter((entry) => !isNullSchema(entry))
    if (filtered.length === target.anyOf.length) {
        return
    }
    target.anyOf = undefined
    if (filtered.length === 1) {
        const [single] = filtered
        Object.assign(target, single)
    } else if (filtered.length > 1) {
        target.anyOf = filtered
    }
    target.nullable = true
}

function walkSchemaNodes(node, visitor) {
    if (!node || typeof node !== 'object' || Array.isArray(node)) return
    visitor(node)
    if (node.properties) {
        for (const prop of Object.values(node.properties)) walkSchemaNodes(prop, visitor)
    }
    if (node.items) walkSchemaNodes(node.items, visitor)
    if (node.additionalProperties && typeof node.additionalProperties === 'object') {
        walkSchemaNodes(node.additionalProperties, visitor)
    }
    for (const key of ['anyOf', 'oneOf', 'allOf']) {
        if (Array.isArray(node[key])) node[key].forEach((s) => walkSchemaNodes(s, visitor))
    }
}

function simplifyDeepObjectParameter(param) {
    if (!param || typeof param !== 'object' || param.$ref) return false
    if (param.style !== 'deepObject') return false
    param.style = 'form'
    if (param.explode === undefined) param.explode = true
    if (!param.schema || typeof param.schema !== 'object') {
        param.schema = { type: 'object', additionalProperties: true }
    }
    return true
}

function adjustParameters(doc) {
    const componentParameters = doc.components?.parameters || {}
    for (const key of Object.keys(componentParameters)) {
        simplifyDeepObjectParameter(componentParameters[key])
    }
    if (!doc.paths) return
    for (const pathItem of Object.values(doc.paths)) {
        if (!pathItem || typeof pathItem !== 'object') continue
        if (Array.isArray(pathItem.parameters)) {
            pathItem.parameters.forEach(simplifyDeepObjectParameter)
        }
        for (const operation of Object.values(pathItem)) {
            if (!operation || typeof operation !== 'object') continue
            if (Array.isArray(operation.parameters)) {
                operation.parameters.forEach(simplifyDeepObjectParameter)
            }
        }
    }
}

// --- Apply overrides ------------------------------------------------------

// A: rewrite a route's request body.
function applyRequestBodyOverride(doc, entry) {
    const pathItem = doc.paths?.[entry.path]
    if (!pathItem || typeof pathItem !== 'object') return false
    const op = pathItem.post || pathItem.put || pathItem.patch
    if (!op || typeof op !== 'object') return false
    const originalBody = op.requestBody || {}
    const required = originalBody.required
    op.requestBody = {
        description: originalBody.description || entry.description,
        content: { [entry.mediaType]: { schema: entry.schema } },
        ...(required === undefined ? {} : { required }),
    }
    return true
}

// C: replace a named schema wholesale.
function applySchemaReplace(doc, entry) {
    const schema = doc.components?.schemas?.[entry.schema]
    if (!schema || typeof schema !== 'object') return false
    for (const k of Object.keys(schema)) delete schema[k]
    Object.assign(schema, entry.replacement)
    return true
}

// B: post-gen fix for the `_group`/`group` collision. Pattern-based (from/to
// come from the registry entry).
function applyGroupRename(libPath, entry) {
    if (!fs.existsSync(libPath)) return
    let content = fs.readFileSync(libPath, 'utf8')
    const { from, to } = entry

    // struct field
    content = content.replace(
        new RegExp(`(rename = "_group"[^}]*?)pub ${from}:([^}]*?pub ${from}:)`, 'g'),
        `$1pub ${to}:$2`
    )

    // Default impl
    content = content.replace(
        new RegExp(
            `(\\s+)${from}:\\s*Default::default\\(\\),(\\s+)${from}:\\s*Default::default\\(\\),`,
            'g'
        ),
        `$1${to}: Default::default(),$2${from}: Default::default(),`
    )

    // method signature (rename first of two `<from>` params)
    const methodPattern = /pub async fn (\w+)<'a>\(\s*&'a self,\s*([^)]+)\)/gs
    content = content.replace(methodPattern, (match, methodName, params) => {
        const paramLines = params
            .split(',')
            .map((p) => p.trim())
            .filter((p) => p)
        const groupIndices = []
        paramLines.forEach((param, idx) => {
            if (new RegExp(`^\\s*${from}\\s*:`).test(param)) groupIndices.push(idx)
        })
        if (groupIndices.length === 2) {
            paramLines[groupIndices[0]] = paramLines[groupIndices[0]].replace(
                new RegExp(`^\\s*${from}\\s*:`),
                `${to}:`
            )
            const newParams = paramLines.join(',\n        ')
            return `pub async fn ${methodName}<'a>(\n        &'a self,\n        ${newParams}\n    )`
        }
        return match
    })

    fs.writeFileSync(libPath, content)
}

/** Wire the hand-written override module into the freshly generated crate. */
function wireInOverrides(libPath) {
    if (!fs.existsSync(libPath)) return
    let content = fs.readFileSync(libPath, 'utf8')
    if (content.includes('pub mod overrides;')) return
    const newline = content.includes('\r\n') ? '\r\n' : '\n'
    content +=
        `${newline}${newline}// ---- rclone-sdk hand-written overrides (see src/overrides.rs) ----` +
        `${newline}pub mod overrides;${newline}pub use overrides::*;${newline}`
    fs.writeFileSync(libPath, content)
}

// --- progenitor + crate metadata ------------------------------------------

function writeOutput(doc, destination) {
    fs.writeFileSync(destination, JSON.stringify(doc, null, 2))
}

// cargo-progenitor formats with unstable rustfmt options that need nightly.
function ensureNightlyRustfmt() {
    if (process.env.RUSTFMT) return
    try {
        const nightly = execSync('rustup which --toolchain nightly rustfmt', {
            encoding: 'utf8',
        }).trim()
        if (nightly) process.env.RUSTFMT = nightly
    } catch {
        console.warn(
            'nightly rustfmt not found — progenitor formatting may fail.\n' +
                'Install it with: rustup toolchain install nightly'
        )
    }
}

function generateClient(version) {
    const args = [
        'cargo',
        'progenitor',
        '-i',
        OUTPUT_SPEC,
        '-o',
        GEN_FOLDER,
        '-n',
        'rclone-sdk',
        '-v',
        version,
        '--license-name',
        'MIT',
        '--include-client',
        'false',
    ]
    execSync(args.join(' '), { stdio: 'inherit', cwd: ROOT })
}

function updateCargoManifest(version, description, readme) {
    const manifestPath = path.resolve(GEN_FOLDER, 'Cargo.toml')
    if (!fs.existsSync(manifestPath)) return

    const content = fs.readFileSync(manifestPath, 'utf8')
    const newline = content.includes('\r\n') ? '\r\n' : '\n'
    const lines = content.split(/\r?\n/)

    const sanitizeString = (value) => value.trim()
    const sanitizedDescription = typeof description === 'string' ? sanitizeString(description) : ''
    const sanitizedReadme = typeof readme === 'string' ? sanitizeString(readme) : ''
    const findFieldIndex = (field) =>
        lines.findIndex((line) => line.trim().startsWith(`${field} =`))

    const updateVersion = () => {
        const versionIndex = findFieldIndex('version')
        if (versionIndex !== -1) lines[versionIndex] = `version = "${version}"`
    }

    const metadataFields = [
        sanitizedDescription
            ? {
                  name: 'description',
                  line: `description = ${JSON.stringify(sanitizedDescription)}`,
                  anchors: ['license'],
              }
            : null,
        sanitizedReadme
            ? {
                  name: 'readme',
                  line: `readme = "${sanitizedReadme}"`,
                  anchors: ['description', 'license'],
              }
            : null,
        { name: 'homepage', line: `homepage = "${HOMEPAGE}"`, anchors: ['description', 'license'] },
        {
            name: 'repository',
            line: `repository = "${REPOSITORY}"`,
            anchors: ['homepage', 'description', 'license'],
        },
        {
            name: 'categories',
            line: `categories = [${CATEGORIES.map((entry) => `"${entry}"`).join(', ')}]`,
            anchors: ['repository', 'homepage', 'description', 'license'],
        },
        {
            name: 'keywords',
            line: `keywords = [${KEYWORDS.map((word) => `"${word}"`).join(', ')}]`,
            anchors: ['categories', 'repository', 'homepage', 'description', 'license'],
        },
    ].filter(Boolean)

    const insertAfter = (index, line) => {
        lines.splice(index + 1, 0, line)
    }

    const ensureField = ({ name, line, anchors }) => {
        if (!line) return
        const existingIndex = findFieldIndex(name)
        if (existingIndex !== -1) {
            lines[existingIndex] = line
            return
        }
        for (const anchor of anchors) {
            const anchorIndex = findFieldIndex(anchor)
            if (anchorIndex !== -1) {
                insertAfter(anchorIndex, line)
                return
            }
        }
        const packageIndex = lines.findIndex((lineContent) => lineContent.trim() === '[package]')
        if (packageIndex !== -1) {
            insertAfter(packageIndex, line)
        } else {
            lines.unshift(line)
        }
    }

    updateVersion()
    metadataFields.forEach(ensureField)

    let dependenciesIndex = lines.findIndex((line) => line.trim() === '[dependencies]')
    if (dependenciesIndex > 0) {
        let i = dependenciesIndex - 1
        while (i >= 0 && lines[i].trim() === '') {
            lines.splice(i, 1)
            i--
            dependenciesIndex--
        }
        lines.splice(dependenciesIndex, 0, '')
    }

    fs.writeFileSync(manifestPath, lines.join(newline))
}

function cleanupSpec() {
    if (fs.existsSync(OUTPUT_SPEC)) fs.unlinkSync(OUTPUT_SPEC)
}

export function generateRust() {
    let specGenerated = false
    const libPath = path.resolve(GEN_FOLDER, 'src', 'lib.rs')
    try {
        const source = loadSourceSpec(SOURCE_SPEC)
        const doc = convertToOpenApi30(source)

        // format-adapters
        stripAsyncResponses(doc)
        for (const schema of Object.values(doc.components?.schemas ?? {})) {
            walkSchemaNodes(schema, convertAnyOfNullToNullable)
        }
        adjustParameters(doc)

        // declarative overrides
        applyRequestBodyOverride(doc, uploadFileBinary)
        applySchemaReplace(doc, arbitraryJsonValue)

        writeOutput(doc, OUTPUT_SPEC)
        specGenerated = true

        const version = readPackageVersion()
        generateClient(version)

        // post-gen overrides
        applyGroupRename(libPath, groupCollision)
        wireInOverrides(libPath)

        updateCargoManifest(version, 'Full OpenAPI-based client for the Rclone RC API.', README)
        cleanupSpec()
        console.log('Generated rs client from OpenAPI specification')
    } catch (error) {
        console.error(error.message)
        if (specGenerated && !fs.existsSync(OUTPUT_SPEC)) {
            console.error('Temporary OpenAPI file was removed before error; re-run to regenerate.')
        }
        throw error
    }
}

if (process.argv[1] && fileURLToPath(import.meta.url) === path.resolve(process.argv[1])) {
    try {
        generateRust()
    } catch {
        process.exit(1)
    }
}
