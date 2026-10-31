// Rust generator: 3.1→3.0 down-convert + format-adapters + declarative overrides
// → cargo progenitor → post-gen fixups (group rename, fs conversions, wire in overrides module).

import { execSync } from 'node:child_process'
import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { Converter } from '@apiture/openapi-down-convert'

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..')
const GEN_FOLDER = path.resolve(ROOT, 'rs')
const SOURCE_SPEC = path.resolve(ROOT, 'node_modules/rclone-openapi/openapi.json')
const PROGENITOR_VERSION = '0.15.1'
const OUTPUT_SPEC = path.resolve(ROOT, 'openapi-3.0.json')
const KEYWORDS = ['rclone', 'sdk', 'openapi', 'client']
const CATEGORIES = ['api-bindings', 'web-programming::http-client']
const HOMEPAGE = 'https://rcloneui.com'
const REPOSITORY = 'https://github.com/rclone-ui/rclone-sdk'
const README = '../README.md'

// --- Override registry: facts the OpenAPI spec can't express --------------

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
    from: 'group',
    to: 'group_',
}

function loadSourceSpec(filePath) {
    if (!fs.existsSync(filePath)) {
        throw new Error(`Source spec not found at ${filePath}`)
    }
    return JSON.parse(fs.readFileSync(filePath, 'utf8'))
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

// progenitor 0.15 allows one success type and one error type per operation, and
// can't generate multipart. Rust is the sync client, and overrides.rs has the rest:
// - Background calls go through `post_async`, so the 202 and `Prefer` go. A generated
//   method that sent `Prefer` would get a 202 it can't read.
// - The text/plain 401 goes, so a 401 comes back as `Error::InvalidResponsePayload`.
// - Uploads go through a hand-written `operations_uploadfile`.
function adaptRoutes(doc) {
    delete doc.paths['/operations/uploadfile']
    for (const pathItem of Object.values(doc.paths ?? {})) {
        if (!pathItem || typeof pathItem !== 'object') continue
        for (const op of Object.values(pathItem)) {
            if (!op || typeof op !== 'object' || !op.responses) continue
            delete op.responses['202']
            delete op.responses['401']
            op.parameters = op.parameters?.filter(
                (param) => param.$ref !== '#/components/parameters/PreferAsyncHeader'
            )
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

// Calls `visitor` on every schema node in the components and in the routes.
function forEachSchemaNode(doc, visitor) {
    const walkContent = (content) => {
        for (const media of Object.values(content ?? {})) walkSchemaNodes(media.schema, visitor)
    }
    for (const schema of Object.values(doc.components?.schemas ?? {})) {
        walkSchemaNodes(schema, visitor)
    }
    for (const response of Object.values(doc.components?.responses ?? {})) {
        walkContent(response.content)
    }
    for (const pathItem of Object.values(doc.paths ?? {})) {
        walkContent(pathItem.post?.requestBody?.content)
        for (const response of Object.values(pathItem.post?.responses ?? {})) {
            walkContent(response.content)
        }
    }
}

// typify ignores `additionalProperties: true` on an object that has properties, which would
// leave no way to send or read the other fields. With `{}` the struct gets an `extra` map.
function keepOpenFields(node) {
    if (node.properties && node.additionalProperties === true) node.additionalProperties = {}
}

// --- Apply overrides ------------------------------------------------------

// C: replace a named schema wholesale.
function applySchemaReplace(doc, entry) {
    const schema = doc.components?.schemas?.[entry.schema]
    if (!schema || typeof schema !== 'object') {
        throw new Error(`Schema ${entry.schema} isn't in the spec, so its override can't apply`)
    }
    for (const k of Object.keys(schema)) delete schema[k]
    Object.assign(schema, entry.replacement)
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

    fs.writeFileSync(libPath, content)
}

// `fs`-like params are `string | RemoteDefinition`, and typify makes a separate enum for each one.
// These conversions let callers write `fs: "gdrive:".into()` on all of them.
function addFsConversions(libPath) {
    const content = fs.readFileSync(libPath, 'utf8')
    const enums = content.matchAll(
        /pub enum (\w+) \{\s+String\(::std::string::String\),\s+RemoteDefinition\(RemoteDefinition\),\s+\}/g
    )
    let conversions = ''
    for (const [, name] of enums) {
        conversions += `
impl From<&str> for types::${name} {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::${name} {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
`
    }
    if (!conversions) throw new Error('Found no `string | RemoteDefinition` enums to convert')
    fs.writeFileSync(libPath, content + conversions)
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

// `cargo progenitor --version` sets the crate version, so the tool can't report its own.
// The dependency it writes into the manifest shows which version ran.
function checkProgenitorVersion() {
    const manifest = fs.readFileSync(path.resolve(GEN_FOLDER, 'Cargo.toml'), 'utf8')
    const used = manifest.match(/^progenitor-client = "(.+)"/m)?.[1]
    if (used !== PROGENITOR_VERSION) {
        throw new Error(
            `This needs cargo-progenitor ${PROGENITOR_VERSION}, but ${used} ran. Install it with: ` +
                `cargo install cargo-progenitor --version ${PROGENITOR_VERSION} --locked --force`
        )
    }
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

    // overrides.rs needs multipart and base64, and `https://` URLs need TLS.
    const reqwestIndex = lines.findIndex((line) => line.startsWith('reqwest ='))
    lines[reqwestIndex] = lines[reqwestIndex].replace(
        '"stream"]',
        '"stream", "multipart", "rustls"]'
    )
    lines.splice(lines.indexOf('[dependencies]') + 1, 0, 'base64 = "0.22"')

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
        adaptRoutes(doc)
        forEachSchemaNode(doc, convertAnyOfNullToNullable)
        forEachSchemaNode(doc, keepOpenFields)

        // declarative overrides
        applySchemaReplace(doc, arbitraryJsonValue)

        writeOutput(doc, OUTPUT_SPEC)
        specGenerated = true

        const version = readPackageVersion()
        generateClient(version)
        checkProgenitorVersion()

        // post-gen overrides
        applyGroupRename(libPath, groupCollision)
        addFsConversions(libPath)
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
