/*
The purpose of this script is to take our OpenAPI v3.1.x spec and
1. Convert it to v3.0.x (using openapi-down-convert)
2. Convert the /operations/uploadfile request body to an octet-stream binary payload supported by progenitor
3. Make the "MetadataInfo" and "ConfigProviderOptionAny" anyOfs from type: 'null' to nullable: true
4. Change the DeepObject fields to something simpler/compatible with progenitor (> cargo progenitor -i openapi-3.0.json -o keeper -n keeper -v 0.1.0 gen fail: UnexpectedFormat("unsupported style of query parameter DeepObject"))
5. Run cargo progenitor using the workspace package version and add crate metadata
6. Clean up temporary artifacts once generation completes
*/

const fs = require('node:fs')
const path = require('node:path')
const { execSync } = require('node:child_process')
const yaml = require('js-yaml')
const { Converter } = require('@apiture/openapi-down-convert')

const ROOT = path.resolve(__dirname, '..')
const GEN_FOLDER = path.resolve(ROOT, 'rs')
const SOURCE_SPEC = path.resolve(ROOT, 'node_modules/rclone-openapi/openapi.yaml')
const OUTPUT_SPEC = path.resolve(ROOT, 'openapi-3.0.json')
const KEYWORDS = ['rclone', 'sdk', 'openapi', 'client']
const CATEGORIES = ['api-bindings', 'web-programming::http-client']
const HOMEPAGE = 'https://rcloneui.com'
const REPOSITORY = 'https://github.com/rclone-ui/rclone-sdk'
const README = '../README.md'

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

function convertToOpenApi30(openapi31) {
    const converter = new Converter(openapi31, {
        verbose: false,
        deleteExampleWithId: true,
        allOfTransform: false,
    })
    return converter.convert()
}

function convertUploadFileRequest(doc) {
    const pathItem = doc.paths?.['/operations/uploadfile']
    if (!pathItem || typeof pathItem !== 'object') {
        return
    }
    const operation = pathItem.post || pathItem.put || pathItem.patch
    if (!operation || typeof operation !== 'object') {
        return
    }
    const originalBody = operation.requestBody || {}
    const description = originalBody.description || 'Binary payload containing the file to upload.'
    const required = originalBody.required

    operation.requestBody = {
        description,
        content: {
            'application/octet-stream': {
                schema: {
                    type: 'string',
                    format: 'binary',
                },
            },
        },
        ...(required === undefined ? {} : { required }),
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

function normalizeArbitraryJsonSchema(schema) {
    if (!schema || typeof schema !== 'object' || !Array.isArray(schema.anyOf)) {
        return
    }
    const nonNull = schema.anyOf.filter((entry) => entry.type && entry.type !== 'null')
    const hasNull = schema.anyOf.some((entry) => entry.type === 'null')
    if (!hasNull || nonNull.length === 0) {
        return
    }
    // Replace with a generic object schema that Progenitor can map to serde_json::Value
    schema.anyOf = undefined
    Object.assign(schema, {
        type: 'object',
        additionalProperties: true,
        nullable: true,
        description: `${schema.description || ''} (arbitrary JSON value)`.trim(),
    })
}

function adjustNullableSchemas(doc) {
    const schemas = doc.components?.schemas
    if (!schemas) {
        return
    }

    const configProvider = schemas.ConfigProvider
    if (configProvider?.properties?.MetadataInfo) {
        convertAnyOfNullToNullable(configProvider.properties.MetadataInfo)
    }

    const optionAny = schemas.ConfigProviderOptionAny
    if (optionAny) {
        normalizeArbitraryJsonSchema(optionAny)
    }
}

function simplifyDeepObjectParameter(param) {
    if (!param || typeof param !== 'object' || param.$ref) {
        return false
    }
    if (param.style !== 'deepObject') {
        return false
    }
    param.style = 'form'
    if (param.explode === undefined) {
        param.explode = true
    }
    if (!param.schema || typeof param.schema !== 'object') {
        param.schema = { type: 'object', additionalProperties: true }
    }
    return true
}

function updateParameterObject(param) {
    simplifyDeepObjectParameter(param)
}

function adjustParameters(doc) {
    const components = doc.components || {}
    const componentParameters = components.parameters || {}
    for (const key of Object.keys(componentParameters)) {
        updateParameterObject(componentParameters[key])
    }

    if (!doc.paths) {
        return
    }

    for (const pathItem of Object.values(doc.paths)) {
        if (!pathItem || typeof pathItem !== 'object') {
            continue
        }
        if (Array.isArray(pathItem.parameters)) {
            pathItem.parameters.forEach(updateParameterObject)
        }
        for (const operation of Object.values(pathItem)) {
            if (!operation || typeof operation !== 'object') {
                continue
            }
            if (Array.isArray(operation.parameters)) {
                operation.parameters.forEach(updateParameterObject)
            }
        }
    }
}

function writeOutput(doc, destination) {
    fs.writeFileSync(destination, JSON.stringify(doc, null, 2))
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
    if (!fs.existsSync(manifestPath)) {
        return
    }

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
        if (versionIndex !== -1) {
            lines[versionIndex] = `version = "${version}"`
        }
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
        if (!line) {
            return
        }
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

function fixGroupParameterCollisions() {
    const libPath = path.resolve(GEN_FOLDER, 'src', 'lib.rs')
    if (!fs.existsSync(libPath)) {
        return
    }

    let content = fs.readFileSync(libPath, 'utf8')

    // Pattern: methods with duplicate `group` parameters
    // We need to rename the first occurrence (which maps to _group) to _group
    const methodPattern = /pub async fn (\w+)<'a>\(\s*&'a self,\s*([^)]+)\)/gs

    content = content.replace(methodPattern, (match, methodName, params) => {
        // Split parameters and check for duplicate 'group'
        const paramLines = params
            .split(',')
            .map((p) => p.trim())
            .filter((p) => p)
        const groupIndices = []

        paramLines.forEach((param, idx) => {
            if (/^\s*group\s*:/.test(param)) {
                groupIndices.push(idx)
            }
        })

        // If we have exactly 2 'group' parameters, rename the first to '_group'
        if (groupIndices.length === 2) {
            const firstGroupIdx = groupIndices[0]
            paramLines[firstGroupIdx] = paramLines[firstGroupIdx].replace(
                /^\s*group\s*:/,
                '_group:'
            )

            // Reconstruct the method signature
            const newParams = paramLines.join(',\n        ')
            return `pub async fn ${methodName}<'a>(\n        &'a self,\n        ${newParams}\n    )`
        }

        return match
    })

    fs.writeFileSync(libPath, content)
}

function cleanupSpec() {
    if (fs.existsSync(OUTPUT_SPEC)) {
        fs.unlinkSync(OUTPUT_SPEC)
    }
}

function main() {
    let specGenerated = false
    try {
        const source = loadSourceSpec(SOURCE_SPEC)
        const openapi30 = convertToOpenApi30(source)
        convertUploadFileRequest(openapi30)
        adjustNullableSchemas(openapi30)
        adjustParameters(openapi30)
        writeOutput(openapi30, OUTPUT_SPEC)
        specGenerated = true

        const version = readPackageVersion()
        generateClient(version)
        fixGroupParameterCollisions()
        const description = 'Full OpenAPI-based client for the Rclone RC API.'
        updateCargoManifest(version, description, README)
        cleanupSpec()
        console.log('Generated rs client from OpenAPI specification')
    } catch (error) {
        console.error(error.message)
        if (specGenerated && !fs.existsSync(OUTPUT_SPEC)) {
            console.error('Temporary OpenAPI file was removed before error; re-run to regenerate.')
        }
        process.exit(1)
    }
}

main()
