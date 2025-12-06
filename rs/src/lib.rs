#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///`BackendCommandResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "result": {
    ///      "description": "Backend command result payload"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BackendCommandResponse {
        ///Backend command result payload
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<::serde_json::Value>,
    }

    impl ::std::convert::From<&BackendCommandResponse> for BackendCommandResponse {
        fn from(value: &BackendCommandResponse) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for BackendCommandResponse {
        fn default() -> Self {
            Self {
                result: Default::default(),
            }
        }
    }

    ///`ConfigGetResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "provider": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigGetResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub provider: ::std::option::Option<::std::string::String>,
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }

    impl ::std::convert::From<&ConfigGetResponse> for ConfigGetResponse {
        fn from(value: &ConfigGetResponse) -> Self {
            value.clone()
        }
    }

    ///`ConfigListremotesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "remotes"
    ///  ],
    ///  "properties": {
    ///    "remotes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigListremotesResponse {
        pub remotes: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::convert::From<&ConfigListremotesResponse> for ConfigListremotesResponse {
        fn from(value: &ConfigListremotesResponse) -> Self {
            value.clone()
        }
    }

    ///`ConfigPathsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "cache",
    ///    "config",
    ///    "temp"
    ///  ],
    ///  "properties": {
    ///    "cache": {
    ///      "type": "string"
    ///    },
    ///    "config": {
    ///      "type": "string"
    ///    },
    ///    "temp": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigPathsResponse {
        pub cache: ::std::string::String,
        pub config: ::std::string::String,
        pub temp: ::std::string::String,
    }

    impl ::std::convert::From<&ConfigPathsResponse> for ConfigPathsResponse {
        fn from(value: &ConfigPathsResponse) -> Self {
            value.clone()
        }
    }

    ///`ConfigProvider`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "Description",
    ///    "Name",
    ///    "Options",
    ///    "Prefix"
    ///  ],
    ///  "properties": {
    ///    "Aliases": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "CommandHelp": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/ConfigProviderCommandHelp"
    ///      }
    ///    },
    ///    "Description": {
    ///      "type": "string"
    ///    },
    ///    "Hide": {
    ///      "type": "boolean"
    ///    },
    ///    "MetadataInfo": {
    ///      "$ref": "#/components/schemas/ConfigProviderMetadataInfo"
    ///    },
    ///    "Name": {
    ///      "type": "string"
    ///    },
    ///    "Options": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ConfigProviderOption"
    ///      }
    ///    },
    ///    "Prefix": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigProvider {
        #[serde(
            rename = "Aliases",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "CommandHelp",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub command_help: ::std::option::Option<::std::vec::Vec<ConfigProviderCommandHelp>>,
        #[serde(rename = "Description")]
        pub description: ::std::string::String,
        #[serde(
            rename = "Hide",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub hide: ::std::option::Option<bool>,
        #[serde(
            rename = "MetadataInfo",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub metadata_info: ::std::option::Option<ConfigProviderMetadataInfo>,
        #[serde(rename = "Name")]
        pub name: ::std::string::String,
        #[serde(rename = "Options")]
        pub options: ::std::vec::Vec<ConfigProviderOption>,
        #[serde(rename = "Prefix")]
        pub prefix: ::std::string::String,
    }

    impl ::std::convert::From<&ConfigProvider> for ConfigProvider {
        fn from(value: &ConfigProvider) -> Self {
            value.clone()
        }
    }

    ///`ConfigProviderCommandHelp`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "Long": {
    ///      "type": "string"
    ///    },
    ///    "Name": {
    ///      "type": "string"
    ///    },
    ///    "Opts": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": true
    ///    },
    ///    "Short": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigProviderCommandHelp {
        #[serde(
            rename = "Long",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub long: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Name",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Opts",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub opts:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        #[serde(
            rename = "Short",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&ConfigProviderCommandHelp> for ConfigProviderCommandHelp {
        fn from(value: &ConfigProviderCommandHelp) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for ConfigProviderCommandHelp {
        fn default() -> Self {
            Self {
                long: Default::default(),
                name: Default::default(),
                opts: Default::default(),
                short: Default::default(),
            }
        }
    }

    ///`ConfigProviderMetadataInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "Help": {
    ///      "type": "string"
    ///    },
    ///    "System": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/ConfigProviderMetadataSystemEntry"
    ///      }
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigProviderMetadataInfo {
        #[serde(
            rename = "Help",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub help: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "System",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub system: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ConfigProviderMetadataSystemEntry>,
        >,
    }

    impl ::std::convert::From<&ConfigProviderMetadataInfo> for ConfigProviderMetadataInfo {
        fn from(value: &ConfigProviderMetadataInfo) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for ConfigProviderMetadataInfo {
        fn default() -> Self {
            Self {
                help: Default::default(),
                system: Default::default(),
            }
        }
    }

    ///`ConfigProviderMetadataSystemEntry`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "Example": {
    ///      "type": "string"
    ///    },
    ///    "Help": {
    ///      "type": "string"
    ///    },
    ///    "ReadOnly": {
    ///      "type": "boolean"
    ///    },
    ///    "Type": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigProviderMetadataSystemEntry {
        #[serde(
            rename = "Example",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub example: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Help",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub help: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "ReadOnly",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub read_only: ::std::option::Option<bool>,
        #[serde(
            rename = "Type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&ConfigProviderMetadataSystemEntry>
        for ConfigProviderMetadataSystemEntry
    {
        fn from(value: &ConfigProviderMetadataSystemEntry) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for ConfigProviderMetadataSystemEntry {
        fn default() -> Self {
            Self {
                example: Default::default(),
                help: Default::default(),
                read_only: Default::default(),
                type_: Default::default(),
            }
        }
    }

    ///`ConfigProviderOption`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "Advanced",
    ///    "Default",
    ///    "DefaultStr",
    ///    "Exclusive",
    ///    "FieldName",
    ///    "Help",
    ///    "Hide",
    ///    "IsPassword",
    ///    "Name",
    ///    "NoPrefix",
    ///    "Required",
    ///    "Sensitive",
    ///    "Type",
    ///    "Value",
    ///    "ValueStr"
    ///  ],
    ///  "properties": {
    ///    "Advanced": {
    ///      "type": "boolean"
    ///    },
    ///    "Default": {
    ///      "$ref": "#/components/schemas/ConfigProviderOptionAny"
    ///    },
    ///    "DefaultStr": {
    ///      "type": "string"
    ///    },
    ///    "Examples": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ConfigProviderOptionExample"
    ///      }
    ///    },
    ///    "Exclusive": {
    ///      "type": "boolean"
    ///    },
    ///    "FieldName": {
    ///      "type": "string"
    ///    },
    ///    "Help": {
    ///      "type": "string"
    ///    },
    ///    "Hide": {
    ///      "type": "number"
    ///    },
    ///    "IsPassword": {
    ///      "type": "boolean"
    ///    },
    ///    "Name": {
    ///      "type": "string"
    ///    },
    ///    "NoPrefix": {
    ///      "type": "boolean"
    ///    },
    ///    "Provider": {
    ///      "type": "string"
    ///    },
    ///    "Required": {
    ///      "type": "boolean"
    ///    },
    ///    "Sensitive": {
    ///      "type": "boolean"
    ///    },
    ///    "ShortOpt": {
    ///      "type": "string"
    ///    },
    ///    "Type": {
    ///      "$ref": "#/components/schemas/ConfigProviderOptionType"
    ///    },
    ///    "Value": {
    ///      "$ref": "#/components/schemas/ConfigProviderOptionAny"
    ///    },
    ///    "ValueStr": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigProviderOption {
        #[serde(rename = "Advanced")]
        pub advanced: bool,
        #[serde(rename = "Default")]
        pub default: ConfigProviderOptionAny,
        #[serde(rename = "DefaultStr")]
        pub default_str: ::std::string::String,
        #[serde(
            rename = "Examples",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub examples: ::std::vec::Vec<ConfigProviderOptionExample>,
        #[serde(rename = "Exclusive")]
        pub exclusive: bool,
        #[serde(rename = "FieldName")]
        pub field_name: ::std::string::String,
        #[serde(rename = "Help")]
        pub help: ::std::string::String,
        #[serde(rename = "Hide")]
        pub hide: f64,
        #[serde(rename = "IsPassword")]
        pub is_password: bool,
        #[serde(rename = "Name")]
        pub name: ::std::string::String,
        #[serde(rename = "NoPrefix")]
        pub no_prefix: bool,
        #[serde(
            rename = "Provider",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub provider: ::std::option::Option<::std::string::String>,
        #[serde(rename = "Required")]
        pub required: bool,
        #[serde(rename = "Sensitive")]
        pub sensitive: bool,
        #[serde(
            rename = "ShortOpt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_opt: ::std::option::Option<::std::string::String>,
        #[serde(rename = "Type")]
        pub type_: ConfigProviderOptionType,
        #[serde(rename = "Value")]
        pub value: ConfigProviderOptionAny,
        #[serde(rename = "ValueStr")]
        pub value_str: ::std::string::String,
    }

    impl ::std::convert::From<&ConfigProviderOption> for ConfigProviderOption {
        fn from(value: &ConfigProviderOption) -> Self {
            value.clone()
        }
    }

    ///Arbitrary JSON value. (arbitrary JSON value)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Arbitrary JSON value. (arbitrary JSON value)",
    ///  "type": [
    ///    "object",
    ///    "null"
    ///  ],
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct ConfigProviderOptionAny(
        pub ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    );
    impl ::std::ops::Deref for ConfigProviderOptionAny {
        type Target =
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>;
        fn deref(
            &self,
        ) -> &::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        {
            &self.0
        }
    }

    impl ::std::convert::From<ConfigProviderOptionAny>
        for ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    {
        fn from(value: ConfigProviderOptionAny) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&ConfigProviderOptionAny> for ConfigProviderOptionAny {
        fn from(value: &ConfigProviderOptionAny) -> Self {
            value.clone()
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        > for ConfigProviderOptionAny
    {
        fn from(
            value: ::std::option::Option<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
        ) -> Self {
            Self(value)
        }
    }

    ///`ConfigProviderOptionExample`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "Help",
    ///    "Value"
    ///  ],
    ///  "properties": {
    ///    "Help": {
    ///      "type": "string"
    ///    },
    ///    "Provider": {
    ///      "type": "string"
    ///    },
    ///    "Value": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigProviderOptionExample {
        #[serde(rename = "Help")]
        pub help: ::std::string::String,
        #[serde(
            rename = "Provider",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub provider: ::std::option::Option<::std::string::String>,
        #[serde(rename = "Value")]
        pub value: ::std::string::String,
    }

    impl ::std::convert::From<&ConfigProviderOptionExample> for ConfigProviderOptionExample {
        fn from(value: &ConfigProviderOptionExample) -> Self {
            value.clone()
        }
    }

    ///`ConfigProviderOptionType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "Bits",
    ///    "bool",
    ///    "CommaSepList",
    ///    "Duration",
    ///    "Encoding",
    ///    "int",
    ///    "mtime|atime|btime|ctime",
    ///    "SizeSuffix",
    ///    "SpaceSepList",
    ///    "string",
    ///    "stringArray",
    ///    "Time",
    ///    "Tristate"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ConfigProviderOptionType {
        Bits,
        #[serde(rename = "bool")]
        Bool,
        CommaSepList,
        Duration,
        Encoding,
        #[serde(rename = "int")]
        Int,
        #[serde(rename = "mtime|atime|btime|ctime")]
        MtimeAtimeBtimeCtime,
        SizeSuffix,
        SpaceSepList,
        #[serde(rename = "string")]
        String,
        #[serde(rename = "stringArray")]
        StringArray,
        Time,
        Tristate,
    }

    impl ::std::convert::From<&Self> for ConfigProviderOptionType {
        fn from(value: &ConfigProviderOptionType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ConfigProviderOptionType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Bits => f.write_str("Bits"),
                Self::Bool => f.write_str("bool"),
                Self::CommaSepList => f.write_str("CommaSepList"),
                Self::Duration => f.write_str("Duration"),
                Self::Encoding => f.write_str("Encoding"),
                Self::Int => f.write_str("int"),
                Self::MtimeAtimeBtimeCtime => f.write_str("mtime|atime|btime|ctime"),
                Self::SizeSuffix => f.write_str("SizeSuffix"),
                Self::SpaceSepList => f.write_str("SpaceSepList"),
                Self::String => f.write_str("string"),
                Self::StringArray => f.write_str("stringArray"),
                Self::Time => f.write_str("Time"),
                Self::Tristate => f.write_str("Tristate"),
            }
        }
    }

    impl ::std::str::FromStr for ConfigProviderOptionType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Bits" => Ok(Self::Bits),
                "bool" => Ok(Self::Bool),
                "CommaSepList" => Ok(Self::CommaSepList),
                "Duration" => Ok(Self::Duration),
                "Encoding" => Ok(Self::Encoding),
                "int" => Ok(Self::Int),
                "mtime|atime|btime|ctime" => Ok(Self::MtimeAtimeBtimeCtime),
                "SizeSuffix" => Ok(Self::SizeSuffix),
                "SpaceSepList" => Ok(Self::SpaceSepList),
                "string" => Ok(Self::String),
                "stringArray" => Ok(Self::StringArray),
                "Time" => Ok(Self::Time),
                "Tristate" => Ok(Self::Tristate),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ConfigProviderOptionType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ConfigProviderOptionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ConfigProviderOptionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`ConfigProvidersResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "providers"
    ///  ],
    ///  "properties": {
    ///    "providers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ConfigProvider"
    ///      }
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigProvidersResponse {
        pub providers: ::std::vec::Vec<ConfigProvider>,
    }

    impl ::std::convert::From<&ConfigProvidersResponse> for ConfigProvidersResponse {
        fn from(value: &ConfigProvidersResponse) -> Self {
            value.clone()
        }
    }

    ///`CoreBwlimitResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "bytesPerSecond",
    ///    "bytesPerSecondRx",
    ///    "bytesPerSecondTx",
    ///    "rate"
    ///  ],
    ///  "properties": {
    ///    "bytesPerSecond": {
    ///      "type": "integer"
    ///    },
    ///    "bytesPerSecondRx": {
    ///      "type": "integer"
    ///    },
    ///    "bytesPerSecondTx": {
    ///      "type": "integer"
    ///    },
    ///    "rate": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreBwlimitResponse {
        #[serde(rename = "bytesPerSecond")]
        pub bytes_per_second: i64,
        #[serde(rename = "bytesPerSecondRx")]
        pub bytes_per_second_rx: i64,
        #[serde(rename = "bytesPerSecondTx")]
        pub bytes_per_second_tx: i64,
        pub rate: ::std::string::String,
    }

    impl ::std::convert::From<&CoreBwlimitResponse> for CoreBwlimitResponse {
        fn from(value: &CoreBwlimitResponse) -> Self {
            value.clone()
        }
    }

    ///`CoreCommandResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "type": "boolean"
    ///    },
    ///    "result": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "returnType": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreCommandResponse {
        pub error: bool,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "returnType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub return_type: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&CoreCommandResponse> for CoreCommandResponse {
        fn from(value: &CoreCommandResponse) -> Self {
            value.clone()
        }
    }

    ///`CoreDuResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "dir",
    ///    "info"
    ///  ],
    ///  "properties": {
    ///    "dir": {
    ///      "type": "string"
    ///    },
    ///    "info": {
    ///      "type": "object",
    ///      "required": [
    ///        "Available",
    ///        "Free",
    ///        "Total"
    ///      ],
    ///      "properties": {
    ///        "Available": {
    ///          "type": "integer"
    ///        },
    ///        "Free": {
    ///          "type": "integer"
    ///        },
    ///        "Total": {
    ///          "type": "integer"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreDuResponse {
        pub dir: ::std::string::String,
        pub info: CoreDuResponseInfo,
    }

    impl ::std::convert::From<&CoreDuResponse> for CoreDuResponse {
        fn from(value: &CoreDuResponse) -> Self {
            value.clone()
        }
    }

    ///`CoreDuResponseInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "Available",
    ///    "Free",
    ///    "Total"
    ///  ],
    ///  "properties": {
    ///    "Available": {
    ///      "type": "integer"
    ///    },
    ///    "Free": {
    ///      "type": "integer"
    ///    },
    ///    "Total": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreDuResponseInfo {
        #[serde(rename = "Available")]
        pub available: i64,
        #[serde(rename = "Free")]
        pub free: i64,
        #[serde(rename = "Total")]
        pub total: i64,
    }

    impl ::std::convert::From<&CoreDuResponseInfo> for CoreDuResponseInfo {
        fn from(value: &CoreDuResponseInfo) -> Self {
            value.clone()
        }
    }

    ///`CoreGroupListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "groups"
    ///  ],
    ///  "properties": {
    ///    "groups": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreGroupListResponse {
        pub groups: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::convert::From<&CoreGroupListResponse> for CoreGroupListResponse {
        fn from(value: &CoreGroupListResponse) -> Self {
            value.clone()
        }
    }

    ///`CoreObscureResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "obscured"
    ///  ],
    ///  "properties": {
    ///    "obscured": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreObscureResponse {
        pub obscured: ::std::string::String,
    }

    impl ::std::convert::From<&CoreObscureResponse> for CoreObscureResponse {
        fn from(value: &CoreObscureResponse) -> Self {
            value.clone()
        }
    }

    ///`CorePidResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "pid"
    ///  ],
    ///  "properties": {
    ///    "pid": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CorePidResponse {
        pub pid: i64,
    }

    impl ::std::convert::From<&CorePidResponse> for CorePidResponse {
        fn from(value: &CorePidResponse) -> Self {
            value.clone()
        }
    }

    ///Metadata for an item currently undergoing verification.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Metadata for an item currently undergoing
    /// verification.",
    ///  "type": "object",
    ///  "properties": {
    ///    "group": {
    ///      "description": "Stats group name associated with this
    /// verification.",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "Remote path of the object being verified.",
    ///      "type": "string"
    ///    },
    ///    "size": {
    ///      "description": "Total size in bytes of the object.",
    ///      "type": "number"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreStatsChecking {
        ///Stats group name associated with this verification.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub group: ::std::option::Option<::std::string::String>,
        ///Remote path of the object being verified.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<f64>,
    }

    impl ::std::convert::From<&CoreStatsChecking> for CoreStatsChecking {
        fn from(value: &CoreStatsChecking) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for CoreStatsChecking {
        fn default() -> Self {
            Self {
                group: Default::default(),
                name: Default::default(),
                size: Default::default(),
            }
        }
    }

    ///`CoreStatsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "bytes",
    ///    "checks",
    ///    "deletedDirs",
    ///    "deletes",
    ///    "elapsedTime",
    ///    "errors",
    ///    "fatalError",
    ///    "renames",
    ///    "retryError",
    ///    "serverSideCopies",
    ///    "serverSideCopyBytes",
    ///    "serverSideMoveBytes",
    ///    "serverSideMoves",
    ///    "speed",
    ///    "totalBytes",
    ///    "totalChecks",
    ///    "totalTransfers",
    ///    "transferTime",
    ///    "transfers"
    ///  ],
    ///  "properties": {
    ///    "bytes": {
    ///      "type": "number"
    ///    },
    ///    "checking": {
    ///      "description": "Objects currently undergoing verification
    /// operations.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CoreStatsChecking"
    ///      }
    ///    },
    ///    "checks": {
    ///      "type": "number"
    ///    },
    ///    "deletedDirs": {
    ///      "type": "number"
    ///    },
    ///    "deletes": {
    ///      "type": "number"
    ///    },
    ///    "elapsedTime": {
    ///      "type": "number"
    ///    },
    ///    "errors": {
    ///      "type": "number"
    ///    },
    ///    "eta": {
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "fatalError": {
    ///      "type": "boolean"
    ///    },
    ///    "lastError": {
    ///      "type": "string"
    ///    },
    ///    "listed": {
    ///      "type": "number"
    ///    },
    ///    "renames": {
    ///      "type": "number"
    ///    },
    ///    "retryError": {
    ///      "type": "boolean"
    ///    },
    ///    "serverSideCopies": {
    ///      "type": "number"
    ///    },
    ///    "serverSideCopyBytes": {
    ///      "type": "number"
    ///    },
    ///    "serverSideMoveBytes": {
    ///      "type": "number"
    ///    },
    ///    "serverSideMoves": {
    ///      "type": "number"
    ///    },
    ///    "speed": {
    ///      "type": "number"
    ///    },
    ///    "totalBytes": {
    ///      "type": "number"
    ///    },
    ///    "totalChecks": {
    ///      "type": "number"
    ///    },
    ///    "totalTransfers": {
    ///      "type": "number"
    ///    },
    ///    "transferTime": {
    ///      "type": "number"
    ///    },
    ///    "transferring": {
    ///      "description": "Active transfers currently in progress grouped by
    /// stats group.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CoreStatsTransfer"
    ///      }
    ///    },
    ///    "transfers": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreStatsResponse {
        pub bytes: f64,
        ///Objects currently undergoing verification operations.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub checking: ::std::vec::Vec<CoreStatsChecking>,
        pub checks: f64,
        #[serde(rename = "deletedDirs")]
        pub deleted_dirs: f64,
        pub deletes: f64,
        #[serde(rename = "elapsedTime")]
        pub elapsed_time: f64,
        pub errors: f64,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub eta: ::std::option::Option<f64>,
        #[serde(rename = "fatalError")]
        pub fatal_error: bool,
        #[serde(
            rename = "lastError",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_error: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub listed: ::std::option::Option<f64>,
        pub renames: f64,
        #[serde(rename = "retryError")]
        pub retry_error: bool,
        #[serde(rename = "serverSideCopies")]
        pub server_side_copies: f64,
        #[serde(rename = "serverSideCopyBytes")]
        pub server_side_copy_bytes: f64,
        #[serde(rename = "serverSideMoveBytes")]
        pub server_side_move_bytes: f64,
        #[serde(rename = "serverSideMoves")]
        pub server_side_moves: f64,
        pub speed: f64,
        #[serde(rename = "totalBytes")]
        pub total_bytes: f64,
        #[serde(rename = "totalChecks")]
        pub total_checks: f64,
        #[serde(rename = "totalTransfers")]
        pub total_transfers: f64,
        #[serde(rename = "transferTime")]
        pub transfer_time: f64,
        ///Active transfers currently in progress grouped by stats group.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub transferring: ::std::vec::Vec<CoreStatsTransfer>,
        pub transfers: f64,
    }

    impl ::std::convert::From<&CoreStatsResponse> for CoreStatsResponse {
        fn from(value: &CoreStatsResponse) -> Self {
            value.clone()
        }
    }

    ///Progress metrics for an in-flight transfer.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Progress metrics for an in-flight transfer.",
    ///  "type": "object",
    ///  "properties": {
    ///    "bytes": {
    ///      "description": "Bytes transferred so far for this object.",
    ///      "type": "number"
    ///    },
    ///    "eta": {
    ///      "description": "Estimated seconds remaining, when available.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ]
    ///    },
    ///    "group": {
    ///      "description": "Stats group name associated with this transfer.",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "Remote path of the object being transferred.",
    ///      "type": "string"
    ///    },
    ///    "percentage": {
    ///      "description": "Completion percentage from 0-100.",
    ///      "type": "number"
    ///    },
    ///    "size": {
    ///      "description": "Total size in bytes of the object.",
    ///      "type": "number"
    ///    },
    ///    "speed": {
    ///      "description": "Current transfer speed in bytes per second.",
    ///      "type": "number"
    ///    },
    ///    "speedAvg": {
    ///      "description": "Current speed in bytes per second as an
    /// exponentially weighted moving average.",
    ///      "type": "number"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreStatsTransfer {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bytes: ::std::option::Option<f64>,
        ///Estimated seconds remaining, when available.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub eta: ::std::option::Option<f64>,
        ///Stats group name associated with this transfer.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub group: ::std::option::Option<::std::string::String>,
        ///Remote path of the object being transferred.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub percentage: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub speed: ::std::option::Option<f64>,
        #[serde(
            rename = "speedAvg",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub speed_avg: ::std::option::Option<f64>,
    }

    impl ::std::convert::From<&CoreStatsTransfer> for CoreStatsTransfer {
        fn from(value: &CoreStatsTransfer) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for CoreStatsTransfer {
        fn default() -> Self {
            Self {
                bytes: Default::default(),
                eta: Default::default(),
                group: Default::default(),
                name: Default::default(),
                percentage: Default::default(),
                size: Default::default(),
                speed: Default::default(),
                speed_avg: Default::default(),
            }
        }
    }

    ///`CoreTransferredResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "transferred"
    ///  ],
    ///  "properties": {
    ///    "transferred": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "group"
    ///        ],
    ///        "properties": {
    ///          "bytes": {
    ///            "type": "integer"
    ///          },
    ///          "checked": {
    ///            "type": "boolean"
    ///          },
    ///          "completed_at": {
    ///            "description": "ISO8601 timestamp when the transfer
    /// completed.",
    ///            "type": "string"
    ///          },
    ///          "dstFs": {
    ///            "description": "Destination remote or filesystem used for the
    /// transfer.",
    ///            "type": "string"
    ///          },
    ///          "dstRemote": {
    ///            "description": "Destination path within `dstFs`, when
    /// provided.",
    ///            "type": "string"
    ///          },
    ///          "error": {
    ///            "type": "string"
    ///          },
    ///          "group": {
    ///            "description": "Stats group identifier this transfer belonged
    /// to.",
    ///            "type": "string"
    ///          },
    ///          "jobid": {
    ///            "type": "integer"
    ///          },
    ///          "name": {
    ///            "type": "string"
    ///          },
    ///          "size": {
    ///            "type": "integer"
    ///          },
    ///          "srcFs": {
    ///            "description": "Source remote or filesystem used for the
    /// transfer.",
    ///            "type": "string"
    ///          },
    ///          "srcRemote": {
    ///            "description": "Source path within `srcFs`, when provided.",
    ///            "type": "string"
    ///          },
    ///          "started_at": {
    ///            "description": "ISO8601 timestamp when the transfer
    /// started.",
    ///            "type": "string"
    ///          },
    ///          "timestamp": {
    ///            "type": "integer"
    ///          },
    ///          "what": {
    ///            "type": "string",
    ///            "enum": [
    ///              "transferring",
    ///              "deleting",
    ///              "checking",
    ///              "importing",
    ///              "hashing",
    ///              "merging",
    ///              "listing",
    ///              "moving",
    ///              "renaming"
    ///            ]
    ///          }
    ///        },
    ///        "additionalProperties": true
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreTransferredResponse {
        pub transferred: ::std::vec::Vec<CoreTransferredResponseTransferredItem>,
    }

    impl ::std::convert::From<&CoreTransferredResponse> for CoreTransferredResponse {
        fn from(value: &CoreTransferredResponse) -> Self {
            value.clone()
        }
    }

    ///`CoreTransferredResponseTransferredItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "group"
    ///  ],
    ///  "properties": {
    ///    "bytes": {
    ///      "type": "integer"
    ///    },
    ///    "checked": {
    ///      "type": "boolean"
    ///    },
    ///    "completed_at": {
    ///      "description": "ISO8601 timestamp when the transfer completed.",
    ///      "type": "string"
    ///    },
    ///    "dstFs": {
    ///      "description": "Destination remote or filesystem used for the
    /// transfer.",
    ///      "type": "string"
    ///    },
    ///    "dstRemote": {
    ///      "description": "Destination path within `dstFs`, when provided.",
    ///      "type": "string"
    ///    },
    ///    "error": {
    ///      "type": "string"
    ///    },
    ///    "group": {
    ///      "description": "Stats group identifier this transfer belonged to.",
    ///      "type": "string"
    ///    },
    ///    "jobid": {
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "size": {
    ///      "type": "integer"
    ///    },
    ///    "srcFs": {
    ///      "description": "Source remote or filesystem used for the
    /// transfer.",
    ///      "type": "string"
    ///    },
    ///    "srcRemote": {
    ///      "description": "Source path within `srcFs`, when provided.",
    ///      "type": "string"
    ///    },
    ///    "started_at": {
    ///      "description": "ISO8601 timestamp when the transfer started.",
    ///      "type": "string"
    ///    },
    ///    "timestamp": {
    ///      "type": "integer"
    ///    },
    ///    "what": {
    ///      "type": "string",
    ///      "enum": [
    ///        "transferring",
    ///        "deleting",
    ///        "checking",
    ///        "importing",
    ///        "hashing",
    ///        "merging",
    ///        "listing",
    ///        "moving",
    ///        "renaming"
    ///      ]
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreTransferredResponseTransferredItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bytes: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub checked: ::std::option::Option<bool>,
        ///ISO8601 timestamp when the transfer completed.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub completed_at: ::std::option::Option<::std::string::String>,
        ///Destination remote or filesystem used for the transfer.
        #[serde(
            rename = "dstFs",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dst_fs: ::std::option::Option<::std::string::String>,
        ///Destination path within `dstFs`, when provided.
        #[serde(
            rename = "dstRemote",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dst_remote: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<::std::string::String>,
        ///Stats group identifier this transfer belonged to.
        pub group: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub jobid: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<i64>,
        ///Source remote or filesystem used for the transfer.
        #[serde(
            rename = "srcFs",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub src_fs: ::std::option::Option<::std::string::String>,
        ///Source path within `srcFs`, when provided.
        #[serde(
            rename = "srcRemote",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub src_remote: ::std::option::Option<::std::string::String>,
        ///ISO8601 timestamp when the transfer started.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub started_at: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub timestamp: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub what: ::std::option::Option<CoreTransferredResponseTransferredItemWhat>,
    }

    impl ::std::convert::From<&CoreTransferredResponseTransferredItem>
        for CoreTransferredResponseTransferredItem
    {
        fn from(value: &CoreTransferredResponseTransferredItem) -> Self {
            value.clone()
        }
    }

    ///`CoreTransferredResponseTransferredItemWhat`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "transferring",
    ///    "deleting",
    ///    "checking",
    ///    "importing",
    ///    "hashing",
    ///    "merging",
    ///    "listing",
    ///    "moving",
    ///    "renaming"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CoreTransferredResponseTransferredItemWhat {
        #[serde(rename = "transferring")]
        Transferring,
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "checking")]
        Checking,
        #[serde(rename = "importing")]
        Importing,
        #[serde(rename = "hashing")]
        Hashing,
        #[serde(rename = "merging")]
        Merging,
        #[serde(rename = "listing")]
        Listing,
        #[serde(rename = "moving")]
        Moving,
        #[serde(rename = "renaming")]
        Renaming,
    }

    impl ::std::convert::From<&Self> for CoreTransferredResponseTransferredItemWhat {
        fn from(value: &CoreTransferredResponseTransferredItemWhat) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for CoreTransferredResponseTransferredItemWhat {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Transferring => f.write_str("transferring"),
                Self::Deleting => f.write_str("deleting"),
                Self::Checking => f.write_str("checking"),
                Self::Importing => f.write_str("importing"),
                Self::Hashing => f.write_str("hashing"),
                Self::Merging => f.write_str("merging"),
                Self::Listing => f.write_str("listing"),
                Self::Moving => f.write_str("moving"),
                Self::Renaming => f.write_str("renaming"),
            }
        }
    }

    impl ::std::str::FromStr for CoreTransferredResponseTransferredItemWhat {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "transferring" => Ok(Self::Transferring),
                "deleting" => Ok(Self::Deleting),
                "checking" => Ok(Self::Checking),
                "importing" => Ok(Self::Importing),
                "hashing" => Ok(Self::Hashing),
                "merging" => Ok(Self::Merging),
                "listing" => Ok(Self::Listing),
                "moving" => Ok(Self::Moving),
                "renaming" => Ok(Self::Renaming),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for CoreTransferredResponseTransferredItemWhat {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for CoreTransferredResponseTransferredItemWhat
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CoreTransferredResponseTransferredItemWhat {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`CoreVersionResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "arch",
    ///    "decomposed",
    ///    "goTags",
    ///    "goVersion",
    ///    "isBeta",
    ///    "isGit",
    ///    "linking",
    ///    "os",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "arch": {
    ///      "description": "CPU architecture (e.g. amd64, arm64).",
    ///      "type": "string"
    ///    },
    ///    "decomposed": {
    ///      "description": "Version number broken into components.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "goTags": {
    ///      "description": "Space separated Go build tags, if any.",
    ///      "type": "string"
    ///    },
    ///    "goVersion": {
    ///      "description": "Go toolchain version used to build rclone.",
    ///      "type": "string"
    ///    },
    ///    "isBeta": {
    ///      "description": "Indicates whether this build is a beta version.",
    ///      "type": "boolean"
    ///    },
    ///    "isGit": {
    ///      "description": "True when built directly from a git checkout.",
    ///      "type": "boolean"
    ///    },
    ///    "linking": {
    ///      "description": "Linking mode for the binary (static or dynamic).",
    ///      "type": "string"
    ///    },
    ///    "os": {
    ///      "description": "Operating system rclone is running on (e.g. linux,
    /// darwin).",
    ///      "type": "string"
    ///    },
    ///    "osArch": {
    ///      "description": "CPU architecture in use (e.g. arm64 (ARMv8
    /// compatible)).",
    ///      "type": "string"
    ///    },
    ///    "osKernel": {
    ///      "description": "OS Kernel version (e.g. 6.8.0-86-generic
    /// (x86_64)).",
    ///      "type": "string"
    ///    },
    ///    "osVersion": {
    ///      "description": "OS Version (e.g. ubuntu 24.04 (64 bit)).",
    ///      "type": "string"
    ///    },
    ///    "version": {
    ///      "description": "Full semantic version string (e.g. 1.67.0).",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreVersionResponse {
        ///CPU architecture (e.g. amd64, arm64).
        pub arch: ::std::string::String,
        ///Version number broken into components.
        pub decomposed: ::std::vec::Vec<f64>,
        ///Space separated Go build tags, if any.
        #[serde(rename = "goTags")]
        pub go_tags: ::std::string::String,
        ///Go toolchain version used to build rclone.
        #[serde(rename = "goVersion")]
        pub go_version: ::std::string::String,
        ///Indicates whether this build is a beta version.
        #[serde(rename = "isBeta")]
        pub is_beta: bool,
        ///True when built directly from a git checkout.
        #[serde(rename = "isGit")]
        pub is_git: bool,
        ///Linking mode for the binary (static or dynamic).
        pub linking: ::std::string::String,
        ///Operating system rclone is running on (e.g. linux, darwin).
        pub os: ::std::string::String,
        ///CPU architecture in use (e.g. arm64 (ARMv8 compatible)).
        #[serde(
            rename = "osArch",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub os_arch: ::std::option::Option<::std::string::String>,
        ///OS Kernel version (e.g. 6.8.0-86-generic (x86_64)).
        #[serde(
            rename = "osKernel",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub os_kernel: ::std::option::Option<::std::string::String>,
        ///OS Version (e.g. ubuntu 24.04 (64 bit)).
        #[serde(
            rename = "osVersion",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub os_version: ::std::option::Option<::std::string::String>,
        ///Full semantic version string (e.g. 1.67.0).
        pub version: ::std::string::String,
    }

    impl ::std::convert::From<&CoreVersionResponse> for CoreVersionResponse {
        fn from(value: &CoreVersionResponse) -> Self {
            value.clone()
        }
    }

    ///`DebugSetGcPercentResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "existing-gc-percent"
    ///  ],
    ///  "properties": {
    ///    "existing-gc-percent": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DebugSetGcPercentResponse {
        #[serde(rename = "existing-gc-percent")]
        pub existing_gc_percent: i64,
    }

    impl ::std::convert::From<&DebugSetGcPercentResponse> for DebugSetGcPercentResponse {
        fn from(value: &DebugSetGcPercentResponse) -> Self {
            value.clone()
        }
    }

    ///`DebugSetMutexProfileFractionResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "previousRate"
    ///  ],
    ///  "properties": {
    ///    "previousRate": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DebugSetMutexProfileFractionResponse {
        #[serde(rename = "previousRate")]
        pub previous_rate: i64,
    }

    impl ::std::convert::From<&DebugSetMutexProfileFractionResponse>
        for DebugSetMutexProfileFractionResponse
    {
        fn from(value: &DebugSetMutexProfileFractionResponse) -> Self {
            value.clone()
        }
    }

    ///`DebugSetSoftMemoryLimitResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "existing-mem-limit"
    ///  ],
    ///  "properties": {
    ///    "existing-mem-limit": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DebugSetSoftMemoryLimitResponse {
        #[serde(rename = "existing-mem-limit")]
        pub existing_mem_limit: i64,
    }

    impl ::std::convert::From<&DebugSetSoftMemoryLimitResponse> for DebugSetSoftMemoryLimitResponse {
        fn from(value: &DebugSetSoftMemoryLimitResponse) -> Self {
            value.clone()
        }
    }

    ///`FscacheEntriesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "entries"
    ///  ],
    ///  "properties": {
    ///    "entries": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FscacheEntriesResponse {
        pub entries: i64,
    }

    impl ::std::convert::From<&FscacheEntriesResponse> for FscacheEntriesResponse {
        fn from(value: &FscacheEntriesResponse) -> Self {
            value.clone()
        }
    }

    ///`JobBatchInputsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "_path"
    ///  ],
    ///  "properties": {
    ///    "_path": {
    ///      "description": "rc/path",
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JobBatchInputsItem {
        ///rc/path
        #[serde(rename = "_path")]
        pub path: ::std::string::String,
    }

    impl ::std::convert::From<&JobBatchInputsItem> for JobBatchInputsItem {
        fn from(value: &JobBatchInputsItem) -> Self {
            value.clone()
        }
    }

    ///`JobBatchRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "_async": {
    ///      "description": "Run the command asynchronously. Returns a job id
    /// immediately.",
    ///      "type": "boolean"
    ///    },
    ///    "concurrency": {
    ///      "description": "Do this many commands concurrently. Defaults to
    /// --transfers if not set.",
    ///      "type": "integer"
    ///    },
    ///    "inputs": {
    ///      "description": "List of inputs to the commands with an extra _path
    /// parameter.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "_path"
    ///        ],
    ///        "properties": {
    ///          "_path": {
    ///            "description": "rc/path",
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": true
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JobBatchRequest {
        ///Run the command asynchronously. Returns a job id immediately.
        #[serde(
            rename = "_async",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub async_: ::std::option::Option<bool>,
        ///Do this many commands concurrently. Defaults to --transfers if not
        /// set.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub concurrency: ::std::option::Option<i64>,
        ///List of inputs to the commands with an extra _path parameter.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub inputs: ::std::vec::Vec<JobBatchRequestInputsItem>,
    }

    impl ::std::convert::From<&JobBatchRequest> for JobBatchRequest {
        fn from(value: &JobBatchRequest) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for JobBatchRequest {
        fn default() -> Self {
            Self {
                async_: Default::default(),
                concurrency: Default::default(),
                inputs: Default::default(),
            }
        }
    }

    ///`JobBatchRequestInputsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "_path"
    ///  ],
    ///  "properties": {
    ///    "_path": {
    ///      "description": "rc/path",
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JobBatchRequestInputsItem {
        ///rc/path
        #[serde(rename = "_path")]
        pub path: ::std::string::String,
    }

    impl ::std::convert::From<&JobBatchRequestInputsItem> for JobBatchRequestInputsItem {
        fn from(value: &JobBatchRequestInputsItem) -> Self {
            value.clone()
        }
    }

    ///`JobBatchResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "executeId",
    ///    "jobid"
    ///  ],
    ///  "properties": {
    ///    "executeId": {
    ///      "description": "Identifier for this rclone process.",
    ///      "type": "string"
    ///    },
    ///    "jobid": {
    ///      "description": "ID of the async job.",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JobBatchResponse {
        ///Identifier for this rclone process.
        #[serde(rename = "executeId")]
        pub execute_id: ::std::string::String,
        ///ID of the async job.
        pub jobid: i64,
    }

    impl ::std::convert::From<&JobBatchResponse> for JobBatchResponse {
        fn from(value: &JobBatchResponse) -> Self {
            value.clone()
        }
    }

    ///`JobListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "executeId",
    ///    "finishedIds",
    ///    "jobids",
    ///    "runningIds"
    ///  ],
    ///  "properties": {
    ///    "executeId": {
    ///      "description": "Identifier for this rclone process.",
    ///      "type": "string"
    ///    },
    ///    "finishedIds": {
    ///      "description": "Array of integer job ids that are finished.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "jobids": {
    ///      "description": "Job IDs suitable for use with `job/status` and
    /// `job/stop`.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "runningIds": {
    ///      "description": "Array of integer job ids that are running.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JobListResponse {
        ///Identifier for this rclone process.
        #[serde(rename = "executeId")]
        pub execute_id: ::std::string::String,
        ///Array of integer job ids that are finished.
        #[serde(rename = "finishedIds")]
        pub finished_ids: ::std::vec::Vec<i64>,
        ///Job IDs suitable for use with `job/status` and `job/stop`.
        pub jobids: ::std::vec::Vec<f64>,
        ///Array of integer job ids that are running.
        #[serde(rename = "runningIds")]
        pub running_ids: ::std::vec::Vec<i64>,
    }

    impl ::std::convert::From<&JobListResponse> for JobListResponse {
        fn from(value: &JobListResponse) -> Self {
            value.clone()
        }
    }

    ///`JobStatusResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "duration",
    ///    "endTime",
    ///    "error",
    ///    "finished",
    ///    "id",
    ///    "startTime",
    ///    "success"
    ///  ],
    ///  "properties": {
    ///    "duration": {
    ///      "description": "Execution time in seconds.",
    ///      "type": "number"
    ///    },
    ///    "endTime": {
    ///      "description": "Timestamp when the job finished. (e.g.
    /// '2025-12-26T18:50:20.528746884+01:00')",
    ///      "type": "string"
    ///    },
    ///    "error": {
    ///      "description": "Error message, or empty string on success.",
    ///      "type": "string"
    ///    },
    ///    "finished": {
    ///      "description": "True once the job has completed.",
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "description": "Job identifier.",
    ///      "type": "number"
    ///    },
    ///    "output": {
    ///      "description": "Synchronous-style output payload when available."
    ///    },
    ///    "progress": {
    ///      "description": "Progress measurements supplied by the underlying
    /// command."
    ///    },
    ///    "startTime": {
    ///      "description": "Timestamp when the job started. (e.g.
    /// '2025-12-24T18:50:20.5281314+01:00')",
    ///      "type": "string"
    ///    },
    ///    "success": {
    ///      "description": "True if the job completed successfully.",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JobStatusResponse {
        pub duration: f64,
        ///Timestamp when the job finished. (e.g.
        /// '2025-12-26T18:50:20.528746884+01:00')
        #[serde(rename = "endTime")]
        pub end_time: ::std::string::String,
        ///Error message, or empty string on success.
        pub error: ::std::string::String,
        ///True once the job has completed.
        pub finished: bool,
        pub id: f64,
        ///Synchronous-style output payload when available.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub output: ::std::option::Option<::serde_json::Value>,
        ///Progress measurements supplied by the underlying command.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub progress: ::std::option::Option<::serde_json::Value>,
        ///Timestamp when the job started. (e.g.
        /// '2025-12-24T18:50:20.5281314+01:00')
        #[serde(rename = "startTime")]
        pub start_time: ::std::string::String,
        ///True if the job completed successfully.
        pub success: bool,
    }

    impl ::std::convert::From<&JobStatusResponse> for JobStatusResponse {
        fn from(value: &JobStatusResponse) -> Self {
            value.clone()
        }
    }

    ///`MountListmountsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "mountPoints"
    ///  ],
    ///  "properties": {
    ///    "mountPoints": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "Fs",
    ///          "MountPoint",
    ///          "MountedOn"
    ///        ],
    ///        "properties": {
    ///          "Fs": {
    ///            "type": "string"
    ///          },
    ///          "MountPoint": {
    ///            "type": "string"
    ///          },
    ///          "MountedOn": {
    ///            "type": "string",
    ///            "format": "date-time"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MountListmountsResponse {
        #[serde(rename = "mountPoints")]
        pub mount_points: ::std::vec::Vec<MountListmountsResponseMountPointsItem>,
    }

    impl ::std::convert::From<&MountListmountsResponse> for MountListmountsResponse {
        fn from(value: &MountListmountsResponse) -> Self {
            value.clone()
        }
    }

    ///`MountListmountsResponseMountPointsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "Fs",
    ///    "MountPoint",
    ///    "MountedOn"
    ///  ],
    ///  "properties": {
    ///    "Fs": {
    ///      "type": "string"
    ///    },
    ///    "MountPoint": {
    ///      "type": "string"
    ///    },
    ///    "MountedOn": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct MountListmountsResponseMountPointsItem {
        #[serde(rename = "Fs")]
        pub fs: ::std::string::String,
        #[serde(rename = "MountPoint")]
        pub mount_point: ::std::string::String,
        #[serde(rename = "MountedOn")]
        pub mounted_on: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&MountListmountsResponseMountPointsItem>
        for MountListmountsResponseMountPointsItem
    {
        fn from(value: &MountListmountsResponseMountPointsItem) -> Self {
            value.clone()
        }
    }

    ///`MountTypesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "mountTypes"
    ///  ],
    ///  "properties": {
    ///    "mountTypes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MountTypesResponse {
        #[serde(rename = "mountTypes")]
        pub mount_types: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::convert::From<&MountTypesResponse> for MountTypesResponse {
        fn from(value: &MountTypesResponse) -> Self {
            value.clone()
        }
    }

    ///`OperationsAboutResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "free",
    ///    "total",
    ///    "used"
    ///  ],
    ///  "properties": {
    ///    "free": {
    ///      "type": "number"
    ///    },
    ///    "objects": {
    ///      "type": "number"
    ///    },
    ///    "other": {
    ///      "type": "number"
    ///    },
    ///    "total": {
    ///      "type": "number"
    ///    },
    ///    "trashed": {
    ///      "type": "number"
    ///    },
    ///    "used": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsAboutResponse {
        pub free: f64,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub objects: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub other: ::std::option::Option<f64>,
        pub total: f64,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub trashed: ::std::option::Option<f64>,
        pub used: f64,
    }

    impl ::std::convert::From<&OperationsAboutResponse> for OperationsAboutResponse {
        fn from(value: &OperationsAboutResponse) -> Self {
            value.clone()
        }
    }

    ///`OperationsCheckResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "status",
    ///    "success"
    ///  ],
    ///  "properties": {
    ///    "combined": {
    ///      "description": "Combined summary lines when `combined=true` is
    /// requested.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "differ": {
    ///      "description": "Files that differed between source and
    /// destination.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "error": {
    ///      "description": "Entries that produced errors during the check.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "hashType": {
    ///      "description": "Hash algorithm used for comparisons when
    /// applicable.",
    ///      "type": "string"
    ///    },
    ///    "match": {
    ///      "description": "Files that matched on both sides.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "missingOnDst": {
    ///      "description": "Files present on the source but missing from the
    /// destination.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "missingOnSrc": {
    ///      "description": "Files present on the destination but missing from
    /// the source.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "status": {
    ///      "description": "Human readable status string.",
    ///      "type": "string"
    ///    },
    ///    "success": {
    ///      "description": "True when the check completes without differences
    /// or errors.",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsCheckResponse {
        ///Combined summary lines when `combined=true` is requested.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub combined: ::std::vec::Vec<::std::string::String>,
        ///Files that differed between source and destination.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub differ: ::std::vec::Vec<::std::string::String>,
        ///Entries that produced errors during the check.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub error: ::std::vec::Vec<::std::string::String>,
        ///Hash algorithm used for comparisons when applicable.
        #[serde(
            rename = "hashType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub hash_type: ::std::option::Option<::std::string::String>,
        ///Files that matched on both sides.
        #[serde(
            rename = "match",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub match_: ::std::vec::Vec<::std::string::String>,
        ///Files present on the source but missing from the destination.
        #[serde(
            rename = "missingOnDst",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub missing_on_dst: ::std::vec::Vec<::std::string::String>,
        ///Files present on the destination but missing from the source.
        #[serde(
            rename = "missingOnSrc",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub missing_on_src: ::std::vec::Vec<::std::string::String>,
        ///Human readable status string.
        pub status: ::std::string::String,
        ///True when the check completes without differences or errors.
        pub success: bool,
    }

    impl ::std::convert::From<&OperationsCheckResponse> for OperationsCheckResponse {
        fn from(value: &OperationsCheckResponse) -> Self {
            value.clone()
        }
    }

    ///`OperationsFsinfoResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "Features",
    ///    "Hashes",
    ///    "Name",
    ///    "Precision",
    ///    "Root",
    ///    "String"
    ///  ],
    ///  "properties": {
    ///    "Features": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "boolean"
    ///      }
    ///    },
    ///    "Hashes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "MetadataInfo": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": true
    ///    },
    ///    "Name": {
    ///      "type": "string"
    ///    },
    ///    "Precision": {
    ///      "type": "number"
    ///    },
    ///    "Root": {
    ///      "type": "string"
    ///    },
    ///    "String": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsFsinfoResponse {
        #[serde(rename = "Features")]
        pub features: ::std::collections::HashMap<::std::string::String, bool>,
        #[serde(rename = "Hashes")]
        pub hashes: ::std::vec::Vec<::std::string::String>,
        #[serde(
            rename = "MetadataInfo",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub metadata_info:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        #[serde(rename = "Name")]
        pub name: ::std::string::String,
        #[serde(rename = "Precision")]
        pub precision: f64,
        #[serde(rename = "Root")]
        pub root: ::std::string::String,
        #[serde(rename = "String")]
        pub string: ::std::string::String,
    }

    impl ::std::convert::From<&OperationsFsinfoResponse> for OperationsFsinfoResponse {
        fn from(value: &OperationsFsinfoResponse) -> Self {
            value.clone()
        }
    }

    ///`OperationsHashsumResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "hashType",
    ///    "hashsum"
    ///  ],
    ///  "properties": {
    ///    "hashType": {
    ///      "type": "string"
    ///    },
    ///    "hashsum": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsHashsumResponse {
        #[serde(rename = "hashType")]
        pub hash_type: ::std::string::String,
        pub hashsum: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::convert::From<&OperationsHashsumResponse> for OperationsHashsumResponse {
        fn from(value: &OperationsHashsumResponse) -> Self {
            value.clone()
        }
    }

    ///`OperationsListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "list"
    ///  ],
    ///  "properties": {
    ///    "list": {
    ///      "description": "Array of entries equivalent to the items returned
    /// by `rclone lsjson`.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "IsDir",
    ///          "Name",
    ///          "Path"
    ///        ],
    ///        "properties": {
    ///          "Encrypted": {
    ///            "description": "Encrypted entry name when using crypt
    /// remotes.",
    ///            "type": "string"
    ///          },
    ///          "EncryptedPath": {
    ///            "description": "Encrypted path when using crypt remotes.",
    ///            "type": "string"
    ///          },
    ///          "Hashes": {
    ///            "description": "Hash digests keyed by algorithm when
    /// requested.",
    ///            "type": "object",
    ///            "additionalProperties": {
    ///              "type": "string"
    ///            }
    ///          },
    ///          "ID": {
    ///            "description": "Backend-specific identifier when provided.",
    ///            "type": "string"
    ///          },
    ///          "IsBucket": {
    ///            "description": "True for bucket/root entries on bucket-based
    /// remotes.",
    ///            "type": "boolean"
    ///          },
    ///          "IsDir": {
    ///            "description": "True if the entry represents a directory.",
    ///            "type": "boolean"
    ///          },
    ///          "Metadata": {
    ///            "description": "Backend-provided metadata map.",
    ///            "type": "object",
    ///            "additionalProperties": {}
    ///          },
    ///          "MimeType": {
    ///            "description": "MIME type where available.",
    ///            "type": "string"
    ///          },
    ///          "ModTime": {
    ///            "description": "Modification timestamp in RFC3339 format.",
    ///            "type": "string"
    ///          },
    ///          "Name": {
    ///            "description": "Base name of the entry.",
    ///            "type": "string"
    ///          },
    ///          "OrigID": {
    ///            "description": "Original backend identifier when recorded.",
    ///            "type": "string"
    ///          },
    ///          "Path": {
    ///            "description": "Path relative to the requested remote root.",
    ///            "type": "string"
    ///          },
    ///          "Size": {
    ///            "description": "Object size in bytes.",
    ///            "type": "number"
    ///          },
    ///          "Tier": {
    ///            "description": "Storage class or tier, if supplied by the
    /// backend.",
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsListResponse {
        ///Array of entries equivalent to the items returned by `rclone
        /// lsjson`.
        pub list: ::std::vec::Vec<OperationsListResponseListItem>,
    }

    impl ::std::convert::From<&OperationsListResponse> for OperationsListResponse {
        fn from(value: &OperationsListResponse) -> Self {
            value.clone()
        }
    }

    ///`OperationsListResponseListItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "IsDir",
    ///    "Name",
    ///    "Path"
    ///  ],
    ///  "properties": {
    ///    "Encrypted": {
    ///      "description": "Encrypted entry name when using crypt remotes.",
    ///      "type": "string"
    ///    },
    ///    "EncryptedPath": {
    ///      "description": "Encrypted path when using crypt remotes.",
    ///      "type": "string"
    ///    },
    ///    "Hashes": {
    ///      "description": "Hash digests keyed by algorithm when requested.",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "ID": {
    ///      "description": "Backend-specific identifier when provided.",
    ///      "type": "string"
    ///    },
    ///    "IsBucket": {
    ///      "description": "True for bucket/root entries on bucket-based
    /// remotes.",
    ///      "type": "boolean"
    ///    },
    ///    "IsDir": {
    ///      "description": "True if the entry represents a directory.",
    ///      "type": "boolean"
    ///    },
    ///    "Metadata": {
    ///      "description": "Backend-provided metadata map.",
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "MimeType": {
    ///      "description": "MIME type where available.",
    ///      "type": "string"
    ///    },
    ///    "ModTime": {
    ///      "description": "Modification timestamp in RFC3339 format.",
    ///      "type": "string"
    ///    },
    ///    "Name": {
    ///      "description": "Base name of the entry.",
    ///      "type": "string"
    ///    },
    ///    "OrigID": {
    ///      "description": "Original backend identifier when recorded.",
    ///      "type": "string"
    ///    },
    ///    "Path": {
    ///      "description": "Path relative to the requested remote root.",
    ///      "type": "string"
    ///    },
    ///    "Size": {
    ///      "description": "Object size in bytes.",
    ///      "type": "number"
    ///    },
    ///    "Tier": {
    ///      "description": "Storage class or tier, if supplied by the
    /// backend.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsListResponseListItem {
        ///Encrypted entry name when using crypt remotes.
        #[serde(
            rename = "Encrypted",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub encrypted: ::std::option::Option<::std::string::String>,
        ///Encrypted path when using crypt remotes.
        #[serde(
            rename = "EncryptedPath",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub encrypted_path: ::std::option::Option<::std::string::String>,
        ///Hash digests keyed by algorithm when requested.
        #[serde(
            rename = "Hashes",
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub hashes: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Backend-specific identifier when provided.
        #[serde(
            rename = "ID",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<::std::string::String>,
        ///True for bucket/root entries on bucket-based remotes.
        #[serde(
            rename = "IsBucket",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_bucket: ::std::option::Option<bool>,
        ///True if the entry represents a directory.
        #[serde(rename = "IsDir")]
        pub is_dir: bool,
        ///Backend-provided metadata map.
        #[serde(
            rename = "Metadata",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///MIME type where available.
        #[serde(
            rename = "MimeType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<::std::string::String>,
        ///Modification timestamp in RFC3339 format.
        #[serde(
            rename = "ModTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mod_time: ::std::option::Option<::std::string::String>,
        ///Base name of the entry.
        #[serde(rename = "Name")]
        pub name: ::std::string::String,
        ///Original backend identifier when recorded.
        #[serde(
            rename = "OrigID",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub orig_id: ::std::option::Option<::std::string::String>,
        ///Path relative to the requested remote root.
        #[serde(rename = "Path")]
        pub path: ::std::string::String,
        #[serde(
            rename = "Size",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub size: ::std::option::Option<f64>,
        ///Storage class or tier, if supplied by the backend.
        #[serde(
            rename = "Tier",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tier: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&OperationsListResponseListItem> for OperationsListResponseListItem {
        fn from(value: &OperationsListResponseListItem) -> Self {
            value.clone()
        }
    }

    ///`OperationsPubliclinkResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "url"
    ///  ],
    ///  "properties": {
    ///    "url": {
    ///      "type": "string",
    ///      "format": "uri"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsPubliclinkResponse {
        pub url: ::std::string::String,
    }

    impl ::std::convert::From<&OperationsPubliclinkResponse> for OperationsPubliclinkResponse {
        fn from(value: &OperationsPubliclinkResponse) -> Self {
            value.clone()
        }
    }

    ///`OperationsSizeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "bytes",
    ///    "count",
    ///    "sizeless"
    ///  ],
    ///  "properties": {
    ///    "bytes": {
    ///      "type": "number"
    ///    },
    ///    "count": {
    ///      "type": "integer"
    ///    },
    ///    "sizeless": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsSizeResponse {
        pub bytes: f64,
        pub count: i64,
        pub sizeless: i64,
    }

    impl ::std::convert::From<&OperationsSizeResponse> for OperationsSizeResponse {
        fn from(value: &OperationsSizeResponse) -> Self {
            value.clone()
        }
    }

    ///`OperationsStatResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "item"
    ///  ],
    ///  "properties": {
    ///    "item": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "required": [
    ///        "IsDir",
    ///        "MimeType",
    ///        "ModTime",
    ///        "Name",
    ///        "Path",
    ///        "Size"
    ///      ],
    ///      "properties": {
    ///        "Encrypted": {
    ///          "description": "Encrypted entry name when using crypt
    /// remotes.",
    ///          "type": "string"
    ///        },
    ///        "EncryptedPath": {
    ///          "description": "Encrypted path when using crypt remotes.",
    ///          "type": "string"
    ///        },
    ///        "Hashes": {
    ///          "description": "Hash digests keyed by algorithm when
    /// requested.",
    ///          "type": "object",
    ///          "additionalProperties": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "ID": {
    ///          "description": "Backend-specific identifier when provided.",
    ///          "type": "string"
    ///        },
    ///        "IsBucket": {
    ///          "description": "True for bucket/root entries on bucket-based
    /// remotes.",
    ///          "type": "boolean"
    ///        },
    ///        "IsDir": {
    ///          "description": "True if the entry is a directory.",
    ///          "type": "boolean"
    ///        },
    ///        "Metadata": {
    ///          "description": "Backend-provided metadata map.",
    ///          "type": "object",
    ///          "additionalProperties": {}
    ///        },
    ///        "MimeType": {
    ///          "description": "MIME type where available.",
    ///          "type": "string"
    ///        },
    ///        "ModTime": {
    ///          "description": "Modification timestamp in RFC3339 format.",
    ///          "type": "string"
    ///        },
    ///        "Name": {
    ///          "description": "Base name of the entry.",
    ///          "type": "string"
    ///        },
    ///        "OrigID": {
    ///          "description": "Original backend identifier when recorded.",
    ///          "type": "string"
    ///        },
    ///        "Path": {
    ///          "description": "Path relative to the remote root.",
    ///          "type": "string"
    ///        },
    ///        "Size": {
    ///          "description": "Object size in bytes.",
    ///          "type": "number"
    ///        },
    ///        "Tier": {
    ///          "description": "Storage class or tier, if supplied by the
    /// backend.",
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsStatResponse {
        pub item: ::std::option::Option<OperationsStatResponseItem>,
    }

    impl ::std::convert::From<&OperationsStatResponse> for OperationsStatResponse {
        fn from(value: &OperationsStatResponse) -> Self {
            value.clone()
        }
    }

    ///`OperationsStatResponseItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "IsDir",
    ///    "MimeType",
    ///    "ModTime",
    ///    "Name",
    ///    "Path",
    ///    "Size"
    ///  ],
    ///  "properties": {
    ///    "Encrypted": {
    ///      "description": "Encrypted entry name when using crypt remotes.",
    ///      "type": "string"
    ///    },
    ///    "EncryptedPath": {
    ///      "description": "Encrypted path when using crypt remotes.",
    ///      "type": "string"
    ///    },
    ///    "Hashes": {
    ///      "description": "Hash digests keyed by algorithm when requested.",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "ID": {
    ///      "description": "Backend-specific identifier when provided.",
    ///      "type": "string"
    ///    },
    ///    "IsBucket": {
    ///      "description": "True for bucket/root entries on bucket-based
    /// remotes.",
    ///      "type": "boolean"
    ///    },
    ///    "IsDir": {
    ///      "description": "True if the entry is a directory.",
    ///      "type": "boolean"
    ///    },
    ///    "Metadata": {
    ///      "description": "Backend-provided metadata map.",
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "MimeType": {
    ///      "description": "MIME type where available.",
    ///      "type": "string"
    ///    },
    ///    "ModTime": {
    ///      "description": "Modification timestamp in RFC3339 format.",
    ///      "type": "string"
    ///    },
    ///    "Name": {
    ///      "description": "Base name of the entry.",
    ///      "type": "string"
    ///    },
    ///    "OrigID": {
    ///      "description": "Original backend identifier when recorded.",
    ///      "type": "string"
    ///    },
    ///    "Path": {
    ///      "description": "Path relative to the remote root.",
    ///      "type": "string"
    ///    },
    ///    "Size": {
    ///      "description": "Object size in bytes.",
    ///      "type": "number"
    ///    },
    ///    "Tier": {
    ///      "description": "Storage class or tier, if supplied by the
    /// backend.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsStatResponseItem {
        ///Encrypted entry name when using crypt remotes.
        #[serde(
            rename = "Encrypted",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub encrypted: ::std::option::Option<::std::string::String>,
        ///Encrypted path when using crypt remotes.
        #[serde(
            rename = "EncryptedPath",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub encrypted_path: ::std::option::Option<::std::string::String>,
        ///Hash digests keyed by algorithm when requested.
        #[serde(
            rename = "Hashes",
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub hashes: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Backend-specific identifier when provided.
        #[serde(
            rename = "ID",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<::std::string::String>,
        ///True for bucket/root entries on bucket-based remotes.
        #[serde(
            rename = "IsBucket",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_bucket: ::std::option::Option<bool>,
        ///True if the entry is a directory.
        #[serde(rename = "IsDir")]
        pub is_dir: bool,
        ///Backend-provided metadata map.
        #[serde(
            rename = "Metadata",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///MIME type where available.
        #[serde(rename = "MimeType")]
        pub mime_type: ::std::string::String,
        ///Modification timestamp in RFC3339 format.
        #[serde(rename = "ModTime")]
        pub mod_time: ::std::string::String,
        ///Base name of the entry.
        #[serde(rename = "Name")]
        pub name: ::std::string::String,
        ///Original backend identifier when recorded.
        #[serde(
            rename = "OrigID",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub orig_id: ::std::option::Option<::std::string::String>,
        ///Path relative to the remote root.
        #[serde(rename = "Path")]
        pub path: ::std::string::String,
        #[serde(rename = "Size")]
        pub size: f64,
        ///Storage class or tier, if supplied by the backend.
        #[serde(
            rename = "Tier",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tier: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&OperationsStatResponseItem> for OperationsStatResponseItem {
        fn from(value: &OperationsStatResponseItem) -> Self {
            value.clone()
        }
    }

    ///`OptionsBlocksResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "options"
    ///  ],
    ///  "properties": {
    ///    "options": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsBlocksResponse {
        pub options: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::convert::From<&OptionsBlocksResponse> for OptionsBlocksResponse {
        fn from(value: &OptionsBlocksResponse) -> Self {
            value.clone()
        }
    }

    ///`OptionsGetResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "dlna",
    ///    "filter",
    ///    "ftp",
    ///    "http",
    ///    "log",
    ///    "main",
    ///    "mount",
    ///    "nfs",
    ///    "proxy",
    ///    "rc",
    ///    "restic",
    ///    "s3",
    ///    "sftp",
    ///    "vfs",
    ///    "webdav"
    ///  ],
    ///  "properties": {
    ///    "dlna": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "filter": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "ftp": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "http": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "log": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "main": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "mount": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "nfs": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "proxy": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "rc": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "restic": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "s3": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "sftp": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "vfs": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "webdav": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsGetResponse {
        pub dlna: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub ftp: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub http: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub log: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub main: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub mount: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub nfs: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub proxy: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub rc: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub restic: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub s3: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub sftp: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub vfs: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        pub webdav: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    impl ::std::convert::From<&OptionsGetResponse> for OptionsGetResponse {
        fn from(value: &OptionsGetResponse) -> Self {
            value.clone()
        }
    }

    ///`OptionsInfoOption`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "Advanced",
    ///    "Default",
    ///    "DefaultStr",
    ///    "Exclusive",
    ///    "FieldName",
    ///    "Help",
    ///    "Hide",
    ///    "IsPassword",
    ///    "Name",
    ///    "NoPrefix",
    ///    "Required",
    ///    "Sensitive",
    ///    "Type",
    ///    "Value",
    ///    "ValueStr"
    ///  ],
    ///  "properties": {
    ///    "Advanced": {
    ///      "type": "boolean"
    ///    },
    ///    "Default": {
    ///      "description": "Default value for this option.",
    ///      "anyOf": [
    ///        {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        {
    ///          "type": "boolean"
    ///        },
    ///        {
    ///          "type": "number"
    ///        },
    ///        {
    ///          "type": "string"
    ///        },
    ///        {
    ///          "type": "object",
    ///          "required": [
    ///            "Valid",
    ///            "Value"
    ///          ],
    ///          "properties": {
    ///            "Valid": {
    ///              "type": "boolean"
    ///            },
    ///            "Value": {
    ///              "type": "boolean"
    ///            }
    ///          },
    ///          "additionalProperties": false
    ///        }
    ///      ]
    ///    },
    ///    "DefaultStr": {
    ///      "type": "string"
    ///    },
    ///    "Examples": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOptionExample"
    ///      }
    ///    },
    ///    "Exclusive": {
    ///      "type": "boolean"
    ///    },
    ///    "FieldName": {
    ///      "type": "string"
    ///    },
    ///    "Groups": {
    ///      "type": "string"
    ///    },
    ///    "Help": {
    ///      "type": "string"
    ///    },
    ///    "Hide": {
    ///      "type": "integer"
    ///    },
    ///    "IsPassword": {
    ///      "type": "boolean"
    ///    },
    ///    "Name": {
    ///      "type": "string"
    ///    },
    ///    "NoPrefix": {
    ///      "type": "boolean"
    ///    },
    ///    "Required": {
    ///      "type": "boolean"
    ///    },
    ///    "Sensitive": {
    ///      "type": "boolean"
    ///    },
    ///    "ShortOpt": {
    ///      "type": "string"
    ///    },
    ///    "Type": {
    ///      "type": "string"
    ///    },
    ///    "Value": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "anyOf": [
    ///            {
    ///              "type": "boolean"
    ///            },
    ///            {
    ///              "type": "number"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "ValueStr": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsInfoOption {
        #[serde(rename = "Advanced")]
        pub advanced: bool,
        ///Default value for this option.
        #[serde(rename = "Default")]
        pub default: OptionsInfoOptionDefault,
        #[serde(rename = "DefaultStr")]
        pub default_str: ::std::string::String,
        #[serde(
            rename = "Examples",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub examples: ::std::vec::Vec<OptionsInfoOptionExample>,
        #[serde(rename = "Exclusive")]
        pub exclusive: bool,
        #[serde(rename = "FieldName")]
        pub field_name: ::std::string::String,
        #[serde(
            rename = "Groups",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub groups: ::std::option::Option<::std::string::String>,
        #[serde(rename = "Help")]
        pub help: ::std::string::String,
        #[serde(rename = "Hide")]
        pub hide: i64,
        #[serde(rename = "IsPassword")]
        pub is_password: bool,
        #[serde(rename = "Name")]
        pub name: ::std::string::String,
        #[serde(rename = "NoPrefix")]
        pub no_prefix: bool,
        #[serde(rename = "Required")]
        pub required: bool,
        #[serde(rename = "Sensitive")]
        pub sensitive: bool,
        #[serde(
            rename = "ShortOpt",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_opt: ::std::option::Option<::std::string::String>,
        #[serde(rename = "Type")]
        pub type_: ::std::string::String,
        #[serde(rename = "Value")]
        pub value: ::std::option::Option<OptionsInfoOptionValue>,
        #[serde(rename = "ValueStr")]
        pub value_str: ::std::string::String,
    }

    impl ::std::convert::From<&OptionsInfoOption> for OptionsInfoOption {
        fn from(value: &OptionsInfoOption) -> Self {
            value.clone()
        }
    }

    ///Default value for this option.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Default value for this option.",
    ///  "anyOf": [
    ///    {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Valid",
    ///        "Value"
    ///      ],
    ///      "properties": {
    ///        "Valid": {
    ///          "type": "boolean"
    ///        },
    ///        "Value": {
    ///          "type": "boolean"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged, deny_unknown_fields)]
    pub enum OptionsInfoOptionDefault {
        Variant0(::std::vec::Vec<::std::string::String>),
        Variant1(bool),
        Variant2(f64),
        Variant3(::std::string::String),
        Variant4 {
            #[serde(rename = "Valid")]
            valid: bool,
            #[serde(rename = "Value")]
            value: bool,
        },
    }

    impl ::std::convert::From<&Self> for OptionsInfoOptionDefault {
        fn from(value: &OptionsInfoOptionDefault) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for OptionsInfoOptionDefault {
        fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
            Self::Variant0(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsInfoOptionDefault {
        fn from(value: bool) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<f64> for OptionsInfoOptionDefault {
        fn from(value: f64) -> Self {
            Self::Variant2(value)
        }
    }

    ///`OptionsInfoOptionExample`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "Help",
    ///    "Value"
    ///  ],
    ///  "properties": {
    ///    "Help": {
    ///      "type": "string"
    ///    },
    ///    "Value": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsInfoOptionExample {
        #[serde(rename = "Help")]
        pub help: ::std::string::String,
        #[serde(rename = "Value")]
        pub value: ::std::string::String,
    }

    impl ::std::convert::From<&OptionsInfoOptionExample> for OptionsInfoOptionExample {
        fn from(value: &OptionsInfoOptionExample) -> Self {
            value.clone()
        }
    }

    ///`OptionsInfoOptionValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "anyOf": [
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "number"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsInfoOptionValue {
        Variant0(bool),
        Variant1(f64),
    }

    impl ::std::convert::From<&Self> for OptionsInfoOptionValue {
        fn from(value: &OptionsInfoOptionValue) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for OptionsInfoOptionValue {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if let Ok(v) = value.parse() {
                Ok(Self::Variant0(v))
            } else if let Ok(v) = value.parse() {
                Ok(Self::Variant1(v))
            } else {
                Err("string conversion failed for all variants".into())
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OptionsInfoOptionValue {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OptionsInfoOptionValue {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OptionsInfoOptionValue {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for OptionsInfoOptionValue {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Variant0(x) => x.fmt(f),
                Self::Variant1(x) => x.fmt(f),
            }
        }
    }

    impl ::std::convert::From<bool> for OptionsInfoOptionValue {
        fn from(value: bool) -> Self {
            Self::Variant0(value)
        }
    }

    impl ::std::convert::From<f64> for OptionsInfoOptionValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    ///`OptionsInfoResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "dlna",
    ///    "filter",
    ///    "ftp",
    ///    "http",
    ///    "log",
    ///    "main",
    ///    "mount",
    ///    "nfs",
    ///    "proxy",
    ///    "rc",
    ///    "restic",
    ///    "s3",
    ///    "sftp",
    ///    "vfs",
    ///    "webdav"
    ///  ],
    ///  "properties": {
    ///    "dlna": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    },
    ///    "filter": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    },
    ///    "ftp": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    },
    ///    "http": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    },
    ///    "log": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    },
    ///    "main": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    },
    ///    "mount": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    },
    ///    "nfs": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    },
    ///    "proxy": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    },
    ///    "rc": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    },
    ///    "restic": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    },
    ///    "s3": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    },
    ///    "sftp": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    },
    ///    "vfs": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    },
    ///    "webdav": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionsInfoOption"
    ///      }
    ///    }
    ///  },
    ///  "additionalProperties": {
    ///    "type": "array",
    ///    "items": {
    ///      "$ref": "#/components/schemas/OptionsInfoOption"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsInfoResponse {
        pub dlna: ::std::vec::Vec<OptionsInfoOption>,
        pub filter: ::std::vec::Vec<OptionsInfoOption>,
        pub ftp: ::std::vec::Vec<OptionsInfoOption>,
        pub http: ::std::vec::Vec<OptionsInfoOption>,
        pub log: ::std::vec::Vec<OptionsInfoOption>,
        pub main: ::std::vec::Vec<OptionsInfoOption>,
        pub mount: ::std::vec::Vec<OptionsInfoOption>,
        pub nfs: ::std::vec::Vec<OptionsInfoOption>,
        pub proxy: ::std::vec::Vec<OptionsInfoOption>,
        pub rc: ::std::vec::Vec<OptionsInfoOption>,
        pub restic: ::std::vec::Vec<OptionsInfoOption>,
        pub s3: ::std::vec::Vec<OptionsInfoOption>,
        pub sftp: ::std::vec::Vec<OptionsInfoOption>,
        pub vfs: ::std::vec::Vec<OptionsInfoOption>,
        pub webdav: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(flatten)]
        pub extra:
            ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<OptionsInfoOption>>,
    }

    impl ::std::convert::From<&OptionsInfoResponse> for OptionsInfoResponse {
        fn from(value: &OptionsInfoResponse) -> Self {
            value.clone()
        }
    }

    ///`OptionsLocalResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "config",
    ///    "filter"
    ///  ],
    ///  "properties": {
    ///    "config": {
    ///      "type": "object",
    ///      "required": [
    ///        "AskPassword",
    ///        "AutoConfirm",
    ///        "BackupDir",
    ///        "BindAddr",
    ///        "BufferSize",
    ///        "BwLimit",
    ///        "BwLimitFile",
    ///        "CaCert",
    ///        "CheckFirst",
    ///        "CheckSum",
    ///        "Checkers",
    ///        "ClientCert",
    ///        "ClientKey",
    ///        "CompareDest",
    ///        "ConnectTimeout",
    ///        "Cookie",
    ///        "CopyDest",
    ///        "CutoffMode",
    ///        "DataRateUnit",
    ///        "DefaultTime",
    ///        "DeleteMode",
    ///        "DisableFeatures",
    ///        "DisableHTTP2",
    ///        "DisableHTTPKeepAlives",
    ///        "DownloadHeaders",
    ///        "DryRun",
    ///        "Dump",
    ///        "ErrorOnNoTransfer",
    ///        "ExpectContinueTimeout",
    ///        "FixCase",
    ///        "FsCacheExpireDuration",
    ///        "FsCacheExpireInterval",
    ///        "Headers",
    ///        "HumanReadable",
    ///        "IgnoreCaseSync",
    ///        "IgnoreChecksum",
    ///        "IgnoreErrors",
    ///        "IgnoreExisting",
    ///        "IgnoreSize",
    ///        "IgnoreTimes",
    ///        "Immutable",
    ///        "Inplace",
    ///        "InsecureSkipVerify",
    ///        "Interactive",
    ///        "KvLockTime",
    ///        "Links",
    ///        "LogLevel",
    ///        "LowLevelRetries",
    ///        "MaxBacklog",
    ///        "MaxBufferMemory",
    ///        "MaxDelete",
    ///        "MaxDeleteSize",
    ///        "MaxDepth",
    ///        "MaxDuration",
    ///        "MaxStatsGroups",
    ///        "MaxTransfer",
    ///        "Metadata",
    ///        "MetadataMapper",
    ///        "MetadataSet",
    ///        "ModifyWindow",
    ///        "MultiThreadChunkSize",
    ///        "MultiThreadCutoff",
    ///        "MultiThreadSet",
    ///        "MultiThreadStreams",
    ///        "MultiThreadWriteBufferSize",
    ///        "NoCheckDest",
    ///        "NoConsole",
    ///        "NoGzip",
    ///        "NoTraverse",
    ///        "NoUnicodeNormalization",
    ///        "NoUpdateDirModTime",
    ///        "NoUpdateModTime",
    ///        "OrderBy",
    ///        "PartialSuffix",
    ///        "PasswordCommand",
    ///        "Progress",
    ///        "ProgressTerminalTitle",
    ///        "RefreshTimes",
    ///        "Retries",
    ///        "RetriesInterval",
    ///        "ServerSideAcrossConfigs",
    ///        "SizeOnly",
    ///        "StatsFileNameLength",
    ///        "StatsLogLevel",
    ///        "StatsOneLine",
    ///        "StatsOneLineDate",
    ///        "StatsOneLineDateFormat",
    ///        "StreamingUploadCutoff",
    ///        "Suffix",
    ///        "SuffixKeepExtension",
    ///        "TPSLimit",
    ///        "TPSLimitBurst",
    ///        "TerminalColorMode",
    ///        "Timeout",
    ///        "TrackRenames",
    ///        "TrackRenamesStrategy",
    ///        "TrafficClass",
    ///        "Transfers",
    ///        "UpdateOlder",
    ///        "UploadHeaders",
    ///        "UseJSONLog",
    ///        "UseListR",
    ///        "UseMmap",
    ///        "UseServerModTime",
    ///        "UserAgent"
    ///      ],
    ///      "properties": {
    ///        "AskPassword": {
    ///          "type": "boolean"
    ///        },
    ///        "AutoConfirm": {
    ///          "type": "boolean"
    ///        },
    ///        "BackupDir": {
    ///          "type": "string"
    ///        },
    ///        "BindAddr": {
    ///          "type": "string"
    ///        },
    ///        "BufferSize": {
    ///          "type": "number"
    ///        },
    ///        "BwLimit": {
    ///          "type": "string"
    ///        },
    ///        "BwLimitFile": {
    ///          "type": "string"
    ///        },
    ///        "CaCert": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "CheckFirst": {
    ///          "type": "boolean"
    ///        },
    ///        "CheckSum": {
    ///          "type": "boolean"
    ///        },
    ///        "Checkers": {
    ///          "type": "number"
    ///        },
    ///        "ClientCert": {
    ///          "type": "string"
    ///        },
    ///        "ClientKey": {
    ///          "type": "string"
    ///        },
    ///        "CompareDest": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "ConnectTimeout": {
    ///          "type": "number"
    ///        },
    ///        "Cookie": {
    ///          "type": "boolean"
    ///        },
    ///        "CopyDest": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "CutoffMode": {
    ///          "type": "string"
    ///        },
    ///        "DataRateUnit": {
    ///          "type": "string"
    ///        },
    ///        "DefaultTime": {
    ///          "type": "string"
    ///        },
    ///        "DeleteMode": {
    ///          "type": "number"
    ///        },
    ///        "DisableFeatures": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "null"
    ///        },
    ///        "DisableHTTP2": {
    ///          "type": "boolean"
    ///        },
    ///        "DisableHTTPKeepAlives": {
    ///          "type": "boolean"
    ///        },
    ///        "DownloadHeaders": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "null"
    ///        },
    ///        "DryRun": {
    ///          "type": "boolean"
    ///        },
    ///        "Dump": {
    ///          "type": "string"
    ///        },
    ///        "ErrorOnNoTransfer": {
    ///          "type": "boolean"
    ///        },
    ///        "ExpectContinueTimeout": {
    ///          "type": "number"
    ///        },
    ///        "FixCase": {
    ///          "type": "boolean"
    ///        },
    ///        "FsCacheExpireDuration": {
    ///          "type": "number"
    ///        },
    ///        "FsCacheExpireInterval": {
    ///          "type": "number"
    ///        },
    ///        "Headers": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "null"
    ///        },
    ///        "HumanReadable": {
    ///          "type": "boolean"
    ///        },
    ///        "IgnoreCaseSync": {
    ///          "type": "boolean"
    ///        },
    ///        "IgnoreChecksum": {
    ///          "type": "boolean"
    ///        },
    ///        "IgnoreErrors": {
    ///          "type": "boolean"
    ///        },
    ///        "IgnoreExisting": {
    ///          "type": "boolean"
    ///        },
    ///        "IgnoreSize": {
    ///          "type": "boolean"
    ///        },
    ///        "IgnoreTimes": {
    ///          "type": "boolean"
    ///        },
    ///        "Immutable": {
    ///          "type": "boolean"
    ///        },
    ///        "Inplace": {
    ///          "type": "boolean"
    ///        },
    ///        "InsecureSkipVerify": {
    ///          "type": "boolean"
    ///        },
    ///        "Interactive": {
    ///          "type": "boolean"
    ///        },
    ///        "KvLockTime": {
    ///          "type": "number"
    ///        },
    ///        "Links": {
    ///          "type": "boolean"
    ///        },
    ///        "LogLevel": {
    ///          "type": "string"
    ///        },
    ///        "LowLevelRetries": {
    ///          "type": "number"
    ///        },
    ///        "MaxBacklog": {
    ///          "type": "number"
    ///        },
    ///        "MaxBufferMemory": {
    ///          "type": "number"
    ///        },
    ///        "MaxDelete": {
    ///          "type": "number"
    ///        },
    ///        "MaxDeleteSize": {
    ///          "type": "number"
    ///        },
    ///        "MaxDepth": {
    ///          "type": "number"
    ///        },
    ///        "MaxDuration": {
    ///          "type": "number"
    ///        },
    ///        "MaxStatsGroups": {
    ///          "type": "number"
    ///        },
    ///        "MaxTransfer": {
    ///          "type": "number"
    ///        },
    ///        "Metadata": {
    ///          "type": "boolean"
    ///        },
    ///        "MetadataMapper": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "null"
    ///        },
    ///        "MetadataSet": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "null"
    ///        },
    ///        "ModifyWindow": {
    ///          "type": "number"
    ///        },
    ///        "MultiThreadChunkSize": {
    ///          "type": "number"
    ///        },
    ///        "MultiThreadCutoff": {
    ///          "type": "number"
    ///        },
    ///        "MultiThreadSet": {
    ///          "type": "boolean"
    ///        },
    ///        "MultiThreadStreams": {
    ///          "type": "number"
    ///        },
    ///        "MultiThreadWriteBufferSize": {
    ///          "type": "number"
    ///        },
    ///        "NoCheckDest": {
    ///          "type": "boolean"
    ///        },
    ///        "NoConsole": {
    ///          "type": "boolean"
    ///        },
    ///        "NoGzip": {
    ///          "type": "boolean"
    ///        },
    ///        "NoTraverse": {
    ///          "type": "boolean"
    ///        },
    ///        "NoUnicodeNormalization": {
    ///          "type": "boolean"
    ///        },
    ///        "NoUpdateDirModTime": {
    ///          "type": "boolean"
    ///        },
    ///        "NoUpdateModTime": {
    ///          "type": "boolean"
    ///        },
    ///        "OrderBy": {
    ///          "type": "string"
    ///        },
    ///        "PartialSuffix": {
    ///          "type": "string"
    ///        },
    ///        "PasswordCommand": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "null"
    ///        },
    ///        "Progress": {
    ///          "type": "boolean"
    ///        },
    ///        "ProgressTerminalTitle": {
    ///          "type": "boolean"
    ///        },
    ///        "RefreshTimes": {
    ///          "type": "boolean"
    ///        },
    ///        "Retries": {
    ///          "type": "number"
    ///        },
    ///        "RetriesInterval": {
    ///          "type": "number"
    ///        },
    ///        "ServerSideAcrossConfigs": {
    ///          "type": "boolean"
    ///        },
    ///        "SizeOnly": {
    ///          "type": "boolean"
    ///        },
    ///        "StatsFileNameLength": {
    ///          "type": "number"
    ///        },
    ///        "StatsLogLevel": {
    ///          "type": "string"
    ///        },
    ///        "StatsOneLine": {
    ///          "type": "boolean"
    ///        },
    ///        "StatsOneLineDate": {
    ///          "type": "boolean"
    ///        },
    ///        "StatsOneLineDateFormat": {
    ///          "type": "string"
    ///        },
    ///        "StreamingUploadCutoff": {
    ///          "type": "number"
    ///        },
    ///        "Suffix": {
    ///          "type": "string"
    ///        },
    ///        "SuffixKeepExtension": {
    ///          "type": "boolean"
    ///        },
    ///        "TPSLimit": {
    ///          "type": "number"
    ///        },
    ///        "TPSLimitBurst": {
    ///          "type": "number"
    ///        },
    ///        "TerminalColorMode": {
    ///          "type": "string"
    ///        },
    ///        "Timeout": {
    ///          "type": "number"
    ///        },
    ///        "TrackRenames": {
    ///          "type": "boolean"
    ///        },
    ///        "TrackRenamesStrategy": {
    ///          "type": "string"
    ///        },
    ///        "TrafficClass": {
    ///          "type": "number"
    ///        },
    ///        "Transfers": {
    ///          "type": "number"
    ///        },
    ///        "UpdateOlder": {
    ///          "type": "boolean"
    ///        },
    ///        "UploadHeaders": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ],
    ///          "format": "null"
    ///        },
    ///        "UseJSONLog": {
    ///          "type": "boolean"
    ///        },
    ///        "UseListR": {
    ///          "type": "boolean"
    ///        },
    ///        "UseMmap": {
    ///          "type": "boolean"
    ///        },
    ///        "UseServerModTime": {
    ///          "type": "boolean"
    ///        },
    ///        "UserAgent": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "filter": {
    ///      "type": "object",
    ///      "required": [
    ///        "DeleteExcluded",
    ///        "ExcludeFile",
    ///        "ExcludeFrom",
    ///        "ExcludeRule",
    ///        "FilesFrom",
    ///        "FilesFromRaw",
    ///        "FilterFrom",
    ///        "FilterRule",
    ///        "HashFilter",
    ///        "IgnoreCase",
    ///        "IncludeFrom",
    ///        "IncludeRule",
    ///        "MaxAge",
    ///        "MaxSize",
    ///        "MetaRules",
    ///        "MinAge",
    ///        "MinSize"
    ///      ],
    ///      "properties": {
    ///        "DeleteExcluded": {
    ///          "type": "boolean"
    ///        },
    ///        "ExcludeFile": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "ExcludeFrom": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "ExcludeRule": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "FilesFrom": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "FilesFromRaw": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "FilterFrom": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "FilterRule": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "HashFilter": {
    ///          "type": "string"
    ///        },
    ///        "IgnoreCase": {
    ///          "type": "boolean"
    ///        },
    ///        "IncludeFrom": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "IncludeRule": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "MaxAge": {
    ///          "type": "number"
    ///        },
    ///        "MaxSize": {
    ///          "type": "number"
    ///        },
    ///        "MetaRules": {
    ///          "type": "object",
    ///          "required": [
    ///            "ExcludeFrom",
    ///            "ExcludeRule",
    ///            "FilterFrom",
    ///            "FilterRule",
    ///            "IncludeFrom",
    ///            "IncludeRule"
    ///          ],
    ///          "properties": {
    ///            "ExcludeFrom": {
    ///              "type": "array",
    ///              "items": {
    ///                "type": "string"
    ///              }
    ///            },
    ///            "ExcludeRule": {
    ///              "type": "array",
    ///              "items": {
    ///                "type": "string"
    ///              }
    ///            },
    ///            "FilterFrom": {
    ///              "type": "array",
    ///              "items": {
    ///                "type": "string"
    ///              }
    ///            },
    ///            "FilterRule": {
    ///              "type": "array",
    ///              "items": {
    ///                "type": "string"
    ///              }
    ///            },
    ///            "IncludeFrom": {
    ///              "type": "array",
    ///              "items": {
    ///                "type": "string"
    ///              }
    ///            },
    ///            "IncludeRule": {
    ///              "type": "array",
    ///              "items": {
    ///                "type": "string"
    ///              }
    ///            }
    ///          }
    ///        },
    ///        "MinAge": {
    ///          "type": "number"
    ///        },
    ///        "MinSize": {
    ///          "type": "number"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsLocalResponse {
        pub config: OptionsLocalResponseConfig,
        pub filter: OptionsLocalResponseFilter,
    }

    impl ::std::convert::From<&OptionsLocalResponse> for OptionsLocalResponse {
        fn from(value: &OptionsLocalResponse) -> Self {
            value.clone()
        }
    }

    ///`OptionsLocalResponseConfig`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "AskPassword",
    ///    "AutoConfirm",
    ///    "BackupDir",
    ///    "BindAddr",
    ///    "BufferSize",
    ///    "BwLimit",
    ///    "BwLimitFile",
    ///    "CaCert",
    ///    "CheckFirst",
    ///    "CheckSum",
    ///    "Checkers",
    ///    "ClientCert",
    ///    "ClientKey",
    ///    "CompareDest",
    ///    "ConnectTimeout",
    ///    "Cookie",
    ///    "CopyDest",
    ///    "CutoffMode",
    ///    "DataRateUnit",
    ///    "DefaultTime",
    ///    "DeleteMode",
    ///    "DisableFeatures",
    ///    "DisableHTTP2",
    ///    "DisableHTTPKeepAlives",
    ///    "DownloadHeaders",
    ///    "DryRun",
    ///    "Dump",
    ///    "ErrorOnNoTransfer",
    ///    "ExpectContinueTimeout",
    ///    "FixCase",
    ///    "FsCacheExpireDuration",
    ///    "FsCacheExpireInterval",
    ///    "Headers",
    ///    "HumanReadable",
    ///    "IgnoreCaseSync",
    ///    "IgnoreChecksum",
    ///    "IgnoreErrors",
    ///    "IgnoreExisting",
    ///    "IgnoreSize",
    ///    "IgnoreTimes",
    ///    "Immutable",
    ///    "Inplace",
    ///    "InsecureSkipVerify",
    ///    "Interactive",
    ///    "KvLockTime",
    ///    "Links",
    ///    "LogLevel",
    ///    "LowLevelRetries",
    ///    "MaxBacklog",
    ///    "MaxBufferMemory",
    ///    "MaxDelete",
    ///    "MaxDeleteSize",
    ///    "MaxDepth",
    ///    "MaxDuration",
    ///    "MaxStatsGroups",
    ///    "MaxTransfer",
    ///    "Metadata",
    ///    "MetadataMapper",
    ///    "MetadataSet",
    ///    "ModifyWindow",
    ///    "MultiThreadChunkSize",
    ///    "MultiThreadCutoff",
    ///    "MultiThreadSet",
    ///    "MultiThreadStreams",
    ///    "MultiThreadWriteBufferSize",
    ///    "NoCheckDest",
    ///    "NoConsole",
    ///    "NoGzip",
    ///    "NoTraverse",
    ///    "NoUnicodeNormalization",
    ///    "NoUpdateDirModTime",
    ///    "NoUpdateModTime",
    ///    "OrderBy",
    ///    "PartialSuffix",
    ///    "PasswordCommand",
    ///    "Progress",
    ///    "ProgressTerminalTitle",
    ///    "RefreshTimes",
    ///    "Retries",
    ///    "RetriesInterval",
    ///    "ServerSideAcrossConfigs",
    ///    "SizeOnly",
    ///    "StatsFileNameLength",
    ///    "StatsLogLevel",
    ///    "StatsOneLine",
    ///    "StatsOneLineDate",
    ///    "StatsOneLineDateFormat",
    ///    "StreamingUploadCutoff",
    ///    "Suffix",
    ///    "SuffixKeepExtension",
    ///    "TPSLimit",
    ///    "TPSLimitBurst",
    ///    "TerminalColorMode",
    ///    "Timeout",
    ///    "TrackRenames",
    ///    "TrackRenamesStrategy",
    ///    "TrafficClass",
    ///    "Transfers",
    ///    "UpdateOlder",
    ///    "UploadHeaders",
    ///    "UseJSONLog",
    ///    "UseListR",
    ///    "UseMmap",
    ///    "UseServerModTime",
    ///    "UserAgent"
    ///  ],
    ///  "properties": {
    ///    "AskPassword": {
    ///      "type": "boolean"
    ///    },
    ///    "AutoConfirm": {
    ///      "type": "boolean"
    ///    },
    ///    "BackupDir": {
    ///      "type": "string"
    ///    },
    ///    "BindAddr": {
    ///      "type": "string"
    ///    },
    ///    "BufferSize": {
    ///      "type": "number"
    ///    },
    ///    "BwLimit": {
    ///      "type": "string"
    ///    },
    ///    "BwLimitFile": {
    ///      "type": "string"
    ///    },
    ///    "CaCert": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "CheckFirst": {
    ///      "type": "boolean"
    ///    },
    ///    "CheckSum": {
    ///      "type": "boolean"
    ///    },
    ///    "Checkers": {
    ///      "type": "number"
    ///    },
    ///    "ClientCert": {
    ///      "type": "string"
    ///    },
    ///    "ClientKey": {
    ///      "type": "string"
    ///    },
    ///    "CompareDest": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "ConnectTimeout": {
    ///      "type": "number"
    ///    },
    ///    "Cookie": {
    ///      "type": "boolean"
    ///    },
    ///    "CopyDest": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "CutoffMode": {
    ///      "type": "string"
    ///    },
    ///    "DataRateUnit": {
    ///      "type": "string"
    ///    },
    ///    "DefaultTime": {
    ///      "type": "string"
    ///    },
    ///    "DeleteMode": {
    ///      "type": "number"
    ///    },
    ///    "DisableFeatures": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "null"
    ///    },
    ///    "DisableHTTP2": {
    ///      "type": "boolean"
    ///    },
    ///    "DisableHTTPKeepAlives": {
    ///      "type": "boolean"
    ///    },
    ///    "DownloadHeaders": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "null"
    ///    },
    ///    "DryRun": {
    ///      "type": "boolean"
    ///    },
    ///    "Dump": {
    ///      "type": "string"
    ///    },
    ///    "ErrorOnNoTransfer": {
    ///      "type": "boolean"
    ///    },
    ///    "ExpectContinueTimeout": {
    ///      "type": "number"
    ///    },
    ///    "FixCase": {
    ///      "type": "boolean"
    ///    },
    ///    "FsCacheExpireDuration": {
    ///      "type": "number"
    ///    },
    ///    "FsCacheExpireInterval": {
    ///      "type": "number"
    ///    },
    ///    "Headers": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "null"
    ///    },
    ///    "HumanReadable": {
    ///      "type": "boolean"
    ///    },
    ///    "IgnoreCaseSync": {
    ///      "type": "boolean"
    ///    },
    ///    "IgnoreChecksum": {
    ///      "type": "boolean"
    ///    },
    ///    "IgnoreErrors": {
    ///      "type": "boolean"
    ///    },
    ///    "IgnoreExisting": {
    ///      "type": "boolean"
    ///    },
    ///    "IgnoreSize": {
    ///      "type": "boolean"
    ///    },
    ///    "IgnoreTimes": {
    ///      "type": "boolean"
    ///    },
    ///    "Immutable": {
    ///      "type": "boolean"
    ///    },
    ///    "Inplace": {
    ///      "type": "boolean"
    ///    },
    ///    "InsecureSkipVerify": {
    ///      "type": "boolean"
    ///    },
    ///    "Interactive": {
    ///      "type": "boolean"
    ///    },
    ///    "KvLockTime": {
    ///      "type": "number"
    ///    },
    ///    "Links": {
    ///      "type": "boolean"
    ///    },
    ///    "LogLevel": {
    ///      "type": "string"
    ///    },
    ///    "LowLevelRetries": {
    ///      "type": "number"
    ///    },
    ///    "MaxBacklog": {
    ///      "type": "number"
    ///    },
    ///    "MaxBufferMemory": {
    ///      "type": "number"
    ///    },
    ///    "MaxDelete": {
    ///      "type": "number"
    ///    },
    ///    "MaxDeleteSize": {
    ///      "type": "number"
    ///    },
    ///    "MaxDepth": {
    ///      "type": "number"
    ///    },
    ///    "MaxDuration": {
    ///      "type": "number"
    ///    },
    ///    "MaxStatsGroups": {
    ///      "type": "number"
    ///    },
    ///    "MaxTransfer": {
    ///      "type": "number"
    ///    },
    ///    "Metadata": {
    ///      "type": "boolean"
    ///    },
    ///    "MetadataMapper": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "null"
    ///    },
    ///    "MetadataSet": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "null"
    ///    },
    ///    "ModifyWindow": {
    ///      "type": "number"
    ///    },
    ///    "MultiThreadChunkSize": {
    ///      "type": "number"
    ///    },
    ///    "MultiThreadCutoff": {
    ///      "type": "number"
    ///    },
    ///    "MultiThreadSet": {
    ///      "type": "boolean"
    ///    },
    ///    "MultiThreadStreams": {
    ///      "type": "number"
    ///    },
    ///    "MultiThreadWriteBufferSize": {
    ///      "type": "number"
    ///    },
    ///    "NoCheckDest": {
    ///      "type": "boolean"
    ///    },
    ///    "NoConsole": {
    ///      "type": "boolean"
    ///    },
    ///    "NoGzip": {
    ///      "type": "boolean"
    ///    },
    ///    "NoTraverse": {
    ///      "type": "boolean"
    ///    },
    ///    "NoUnicodeNormalization": {
    ///      "type": "boolean"
    ///    },
    ///    "NoUpdateDirModTime": {
    ///      "type": "boolean"
    ///    },
    ///    "NoUpdateModTime": {
    ///      "type": "boolean"
    ///    },
    ///    "OrderBy": {
    ///      "type": "string"
    ///    },
    ///    "PartialSuffix": {
    ///      "type": "string"
    ///    },
    ///    "PasswordCommand": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "null"
    ///    },
    ///    "Progress": {
    ///      "type": "boolean"
    ///    },
    ///    "ProgressTerminalTitle": {
    ///      "type": "boolean"
    ///    },
    ///    "RefreshTimes": {
    ///      "type": "boolean"
    ///    },
    ///    "Retries": {
    ///      "type": "number"
    ///    },
    ///    "RetriesInterval": {
    ///      "type": "number"
    ///    },
    ///    "ServerSideAcrossConfigs": {
    ///      "type": "boolean"
    ///    },
    ///    "SizeOnly": {
    ///      "type": "boolean"
    ///    },
    ///    "StatsFileNameLength": {
    ///      "type": "number"
    ///    },
    ///    "StatsLogLevel": {
    ///      "type": "string"
    ///    },
    ///    "StatsOneLine": {
    ///      "type": "boolean"
    ///    },
    ///    "StatsOneLineDate": {
    ///      "type": "boolean"
    ///    },
    ///    "StatsOneLineDateFormat": {
    ///      "type": "string"
    ///    },
    ///    "StreamingUploadCutoff": {
    ///      "type": "number"
    ///    },
    ///    "Suffix": {
    ///      "type": "string"
    ///    },
    ///    "SuffixKeepExtension": {
    ///      "type": "boolean"
    ///    },
    ///    "TPSLimit": {
    ///      "type": "number"
    ///    },
    ///    "TPSLimitBurst": {
    ///      "type": "number"
    ///    },
    ///    "TerminalColorMode": {
    ///      "type": "string"
    ///    },
    ///    "Timeout": {
    ///      "type": "number"
    ///    },
    ///    "TrackRenames": {
    ///      "type": "boolean"
    ///    },
    ///    "TrackRenamesStrategy": {
    ///      "type": "string"
    ///    },
    ///    "TrafficClass": {
    ///      "type": "number"
    ///    },
    ///    "Transfers": {
    ///      "type": "number"
    ///    },
    ///    "UpdateOlder": {
    ///      "type": "boolean"
    ///    },
    ///    "UploadHeaders": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "null"
    ///    },
    ///    "UseJSONLog": {
    ///      "type": "boolean"
    ///    },
    ///    "UseListR": {
    ///      "type": "boolean"
    ///    },
    ///    "UseMmap": {
    ///      "type": "boolean"
    ///    },
    ///    "UseServerModTime": {
    ///      "type": "boolean"
    ///    },
    ///    "UserAgent": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsLocalResponseConfig {
        #[serde(rename = "AskPassword")]
        pub ask_password: bool,
        #[serde(rename = "AutoConfirm")]
        pub auto_confirm: bool,
        #[serde(rename = "BackupDir")]
        pub backup_dir: ::std::string::String,
        #[serde(rename = "BindAddr")]
        pub bind_addr: ::std::string::String,
        #[serde(rename = "BufferSize")]
        pub buffer_size: f64,
        #[serde(rename = "BwLimit")]
        pub bw_limit: ::std::string::String,
        #[serde(rename = "BwLimitFile")]
        pub bw_limit_file: ::std::string::String,
        #[serde(rename = "CaCert")]
        pub ca_cert: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "CheckFirst")]
        pub check_first: bool,
        #[serde(rename = "CheckSum")]
        pub check_sum: bool,
        #[serde(rename = "Checkers")]
        pub checkers: f64,
        #[serde(rename = "ClientCert")]
        pub client_cert: ::std::string::String,
        #[serde(rename = "ClientKey")]
        pub client_key: ::std::string::String,
        #[serde(rename = "CompareDest")]
        pub compare_dest: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "ConnectTimeout")]
        pub connect_timeout: f64,
        #[serde(rename = "Cookie")]
        pub cookie: bool,
        #[serde(rename = "CopyDest")]
        pub copy_dest: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "CutoffMode")]
        pub cutoff_mode: ::std::string::String,
        #[serde(rename = "DataRateUnit")]
        pub data_rate_unit: ::std::string::String,
        #[serde(rename = "DefaultTime")]
        pub default_time: ::std::string::String,
        #[serde(rename = "DeleteMode")]
        pub delete_mode: f64,
        #[serde(rename = "DisableFeatures")]
        pub disable_features: ::std::option::Option<::std::string::String>,
        #[serde(rename = "DisableHTTP2")]
        pub disable_http2: bool,
        #[serde(rename = "DisableHTTPKeepAlives")]
        pub disable_http_keep_alives: bool,
        #[serde(rename = "DownloadHeaders")]
        pub download_headers: ::std::option::Option<::std::string::String>,
        #[serde(rename = "DryRun")]
        pub dry_run: bool,
        #[serde(rename = "Dump")]
        pub dump: ::std::string::String,
        #[serde(rename = "ErrorOnNoTransfer")]
        pub error_on_no_transfer: bool,
        #[serde(rename = "ExpectContinueTimeout")]
        pub expect_continue_timeout: f64,
        #[serde(rename = "FixCase")]
        pub fix_case: bool,
        #[serde(rename = "FsCacheExpireDuration")]
        pub fs_cache_expire_duration: f64,
        #[serde(rename = "FsCacheExpireInterval")]
        pub fs_cache_expire_interval: f64,
        #[serde(rename = "Headers")]
        pub headers: ::std::option::Option<::std::string::String>,
        #[serde(rename = "HumanReadable")]
        pub human_readable: bool,
        #[serde(rename = "IgnoreCaseSync")]
        pub ignore_case_sync: bool,
        #[serde(rename = "IgnoreChecksum")]
        pub ignore_checksum: bool,
        #[serde(rename = "IgnoreErrors")]
        pub ignore_errors: bool,
        #[serde(rename = "IgnoreExisting")]
        pub ignore_existing: bool,
        #[serde(rename = "IgnoreSize")]
        pub ignore_size: bool,
        #[serde(rename = "IgnoreTimes")]
        pub ignore_times: bool,
        #[serde(rename = "Immutable")]
        pub immutable: bool,
        #[serde(rename = "Inplace")]
        pub inplace: bool,
        #[serde(rename = "InsecureSkipVerify")]
        pub insecure_skip_verify: bool,
        #[serde(rename = "Interactive")]
        pub interactive: bool,
        #[serde(rename = "KvLockTime")]
        pub kv_lock_time: f64,
        #[serde(rename = "Links")]
        pub links: bool,
        #[serde(rename = "LogLevel")]
        pub log_level: ::std::string::String,
        #[serde(rename = "LowLevelRetries")]
        pub low_level_retries: f64,
        #[serde(rename = "MaxBacklog")]
        pub max_backlog: f64,
        #[serde(rename = "MaxBufferMemory")]
        pub max_buffer_memory: f64,
        #[serde(rename = "MaxDelete")]
        pub max_delete: f64,
        #[serde(rename = "MaxDeleteSize")]
        pub max_delete_size: f64,
        #[serde(rename = "MaxDepth")]
        pub max_depth: f64,
        #[serde(rename = "MaxDuration")]
        pub max_duration: f64,
        #[serde(rename = "MaxStatsGroups")]
        pub max_stats_groups: f64,
        #[serde(rename = "MaxTransfer")]
        pub max_transfer: f64,
        #[serde(rename = "Metadata")]
        pub metadata: bool,
        #[serde(rename = "MetadataMapper")]
        pub metadata_mapper: ::std::option::Option<::std::string::String>,
        #[serde(rename = "MetadataSet")]
        pub metadata_set: ::std::option::Option<::std::string::String>,
        #[serde(rename = "ModifyWindow")]
        pub modify_window: f64,
        #[serde(rename = "MultiThreadChunkSize")]
        pub multi_thread_chunk_size: f64,
        #[serde(rename = "MultiThreadCutoff")]
        pub multi_thread_cutoff: f64,
        #[serde(rename = "MultiThreadSet")]
        pub multi_thread_set: bool,
        #[serde(rename = "MultiThreadStreams")]
        pub multi_thread_streams: f64,
        #[serde(rename = "MultiThreadWriteBufferSize")]
        pub multi_thread_write_buffer_size: f64,
        #[serde(rename = "NoCheckDest")]
        pub no_check_dest: bool,
        #[serde(rename = "NoConsole")]
        pub no_console: bool,
        #[serde(rename = "NoGzip")]
        pub no_gzip: bool,
        #[serde(rename = "NoTraverse")]
        pub no_traverse: bool,
        #[serde(rename = "NoUnicodeNormalization")]
        pub no_unicode_normalization: bool,
        #[serde(rename = "NoUpdateDirModTime")]
        pub no_update_dir_mod_time: bool,
        #[serde(rename = "NoUpdateModTime")]
        pub no_update_mod_time: bool,
        #[serde(rename = "OrderBy")]
        pub order_by: ::std::string::String,
        #[serde(rename = "PartialSuffix")]
        pub partial_suffix: ::std::string::String,
        #[serde(rename = "PasswordCommand")]
        pub password_command: ::std::option::Option<::std::string::String>,
        #[serde(rename = "Progress")]
        pub progress: bool,
        #[serde(rename = "ProgressTerminalTitle")]
        pub progress_terminal_title: bool,
        #[serde(rename = "RefreshTimes")]
        pub refresh_times: bool,
        #[serde(rename = "Retries")]
        pub retries: f64,
        #[serde(rename = "RetriesInterval")]
        pub retries_interval: f64,
        #[serde(rename = "ServerSideAcrossConfigs")]
        pub server_side_across_configs: bool,
        #[serde(rename = "SizeOnly")]
        pub size_only: bool,
        #[serde(rename = "StatsFileNameLength")]
        pub stats_file_name_length: f64,
        #[serde(rename = "StatsLogLevel")]
        pub stats_log_level: ::std::string::String,
        #[serde(rename = "StatsOneLine")]
        pub stats_one_line: bool,
        #[serde(rename = "StatsOneLineDate")]
        pub stats_one_line_date: bool,
        #[serde(rename = "StatsOneLineDateFormat")]
        pub stats_one_line_date_format: ::std::string::String,
        #[serde(rename = "StreamingUploadCutoff")]
        pub streaming_upload_cutoff: f64,
        #[serde(rename = "Suffix")]
        pub suffix: ::std::string::String,
        #[serde(rename = "SuffixKeepExtension")]
        pub suffix_keep_extension: bool,
        #[serde(rename = "TerminalColorMode")]
        pub terminal_color_mode: ::std::string::String,
        #[serde(rename = "Timeout")]
        pub timeout: f64,
        #[serde(rename = "TPSLimit")]
        pub tps_limit: f64,
        #[serde(rename = "TPSLimitBurst")]
        pub tps_limit_burst: f64,
        #[serde(rename = "TrackRenames")]
        pub track_renames: bool,
        #[serde(rename = "TrackRenamesStrategy")]
        pub track_renames_strategy: ::std::string::String,
        #[serde(rename = "TrafficClass")]
        pub traffic_class: f64,
        #[serde(rename = "Transfers")]
        pub transfers: f64,
        #[serde(rename = "UpdateOlder")]
        pub update_older: bool,
        #[serde(rename = "UploadHeaders")]
        pub upload_headers: ::std::option::Option<::std::string::String>,
        #[serde(rename = "UseJSONLog")]
        pub use_json_log: bool,
        #[serde(rename = "UseListR")]
        pub use_list_r: bool,
        #[serde(rename = "UseMmap")]
        pub use_mmap: bool,
        #[serde(rename = "UseServerModTime")]
        pub use_server_mod_time: bool,
        #[serde(rename = "UserAgent")]
        pub user_agent: ::std::string::String,
    }

    impl ::std::convert::From<&OptionsLocalResponseConfig> for OptionsLocalResponseConfig {
        fn from(value: &OptionsLocalResponseConfig) -> Self {
            value.clone()
        }
    }

    ///`OptionsLocalResponseFilter`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "DeleteExcluded",
    ///    "ExcludeFile",
    ///    "ExcludeFrom",
    ///    "ExcludeRule",
    ///    "FilesFrom",
    ///    "FilesFromRaw",
    ///    "FilterFrom",
    ///    "FilterRule",
    ///    "HashFilter",
    ///    "IgnoreCase",
    ///    "IncludeFrom",
    ///    "IncludeRule",
    ///    "MaxAge",
    ///    "MaxSize",
    ///    "MetaRules",
    ///    "MinAge",
    ///    "MinSize"
    ///  ],
    ///  "properties": {
    ///    "DeleteExcluded": {
    ///      "type": "boolean"
    ///    },
    ///    "ExcludeFile": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "ExcludeFrom": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "ExcludeRule": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "FilesFrom": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "FilesFromRaw": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "FilterFrom": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "FilterRule": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "HashFilter": {
    ///      "type": "string"
    ///    },
    ///    "IgnoreCase": {
    ///      "type": "boolean"
    ///    },
    ///    "IncludeFrom": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "IncludeRule": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "MaxAge": {
    ///      "type": "number"
    ///    },
    ///    "MaxSize": {
    ///      "type": "number"
    ///    },
    ///    "MetaRules": {
    ///      "type": "object",
    ///      "required": [
    ///        "ExcludeFrom",
    ///        "ExcludeRule",
    ///        "FilterFrom",
    ///        "FilterRule",
    ///        "IncludeFrom",
    ///        "IncludeRule"
    ///      ],
    ///      "properties": {
    ///        "ExcludeFrom": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "ExcludeRule": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "FilterFrom": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "FilterRule": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "IncludeFrom": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "IncludeRule": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "MinAge": {
    ///      "type": "number"
    ///    },
    ///    "MinSize": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsLocalResponseFilter {
        #[serde(rename = "DeleteExcluded")]
        pub delete_excluded: bool,
        #[serde(rename = "ExcludeFile")]
        pub exclude_file: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "ExcludeFrom")]
        pub exclude_from: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "ExcludeRule")]
        pub exclude_rule: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "FilesFrom")]
        pub files_from: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "FilesFromRaw")]
        pub files_from_raw: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "FilterFrom")]
        pub filter_from: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "FilterRule")]
        pub filter_rule: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "HashFilter")]
        pub hash_filter: ::std::string::String,
        #[serde(rename = "IgnoreCase")]
        pub ignore_case: bool,
        #[serde(rename = "IncludeFrom")]
        pub include_from: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "IncludeRule")]
        pub include_rule: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "MaxAge")]
        pub max_age: f64,
        #[serde(rename = "MaxSize")]
        pub max_size: f64,
        #[serde(rename = "MetaRules")]
        pub meta_rules: OptionsLocalResponseFilterMetaRules,
        #[serde(rename = "MinAge")]
        pub min_age: f64,
        #[serde(rename = "MinSize")]
        pub min_size: f64,
    }

    impl ::std::convert::From<&OptionsLocalResponseFilter> for OptionsLocalResponseFilter {
        fn from(value: &OptionsLocalResponseFilter) -> Self {
            value.clone()
        }
    }

    ///`OptionsLocalResponseFilterMetaRules`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "ExcludeFrom",
    ///    "ExcludeRule",
    ///    "FilterFrom",
    ///    "FilterRule",
    ///    "IncludeFrom",
    ///    "IncludeRule"
    ///  ],
    ///  "properties": {
    ///    "ExcludeFrom": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "ExcludeRule": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "FilterFrom": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "FilterRule": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "IncludeFrom": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "IncludeRule": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsLocalResponseFilterMetaRules {
        #[serde(rename = "ExcludeFrom")]
        pub exclude_from: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "ExcludeRule")]
        pub exclude_rule: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "FilterFrom")]
        pub filter_from: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "FilterRule")]
        pub filter_rule: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "IncludeFrom")]
        pub include_from: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "IncludeRule")]
        pub include_rule: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::convert::From<&OptionsLocalResponseFilterMetaRules>
        for OptionsLocalResponseFilterMetaRules
    {
        fn from(value: &OptionsLocalResponseFilterMetaRules) -> Self {
            value.clone()
        }
    }

    ///`OptionsSetDlnaValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetDlnaValue {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetDlnaValue {
        fn from(value: &OptionsSetDlnaValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetDlnaValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetDlnaValue {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetDlnaValue {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetDlnaValue {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetDlnaValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`OptionsSetFilterValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetFilterValue {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetFilterValue {
        fn from(value: &OptionsSetFilterValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetFilterValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetFilterValue {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetFilterValue {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetFilterValue {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetFilterValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`OptionsSetFtpValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetFtpValue {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetFtpValue {
        fn from(value: &OptionsSetFtpValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetFtpValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetFtpValue {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetFtpValue {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetFtpValue {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetFtpValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`OptionsSetHttpValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetHttpValue {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetHttpValue {
        fn from(value: &OptionsSetHttpValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetHttpValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetHttpValue {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetHttpValue {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetHttpValue {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetHttpValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`OptionsSetLogValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetLogValue {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetLogValue {
        fn from(value: &OptionsSetLogValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetLogValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetLogValue {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetLogValue {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetLogValue {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetLogValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`OptionsSetMainValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetMainValue {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetMainValue {
        fn from(value: &OptionsSetMainValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetMainValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetMainValue {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetMainValue {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetMainValue {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetMainValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`OptionsSetMountValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetMountValue {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetMountValue {
        fn from(value: &OptionsSetMountValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetMountValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetMountValue {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetMountValue {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetMountValue {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetMountValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`OptionsSetNfsValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetNfsValue {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetNfsValue {
        fn from(value: &OptionsSetNfsValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetNfsValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetNfsValue {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetNfsValue {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetNfsValue {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetNfsValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`OptionsSetProxyValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetProxyValue {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetProxyValue {
        fn from(value: &OptionsSetProxyValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetProxyValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetProxyValue {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetProxyValue {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetProxyValue {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetProxyValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`OptionsSetRcValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetRcValue {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetRcValue {
        fn from(value: &OptionsSetRcValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetRcValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetRcValue {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetRcValue {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetRcValue {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetRcValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`OptionsSetResticValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetResticValue {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetResticValue {
        fn from(value: &OptionsSetResticValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetResticValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetResticValue {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetResticValue {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetResticValue {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetResticValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`OptionsSetS3Value`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetS3Value {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetS3Value {
        fn from(value: &OptionsSetS3Value) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetS3Value {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetS3Value {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetS3Value {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetS3Value {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetS3Value
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`OptionsSetSftpValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetSftpValue {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetSftpValue {
        fn from(value: &OptionsSetSftpValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetSftpValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetSftpValue {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetSftpValue {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetSftpValue {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetSftpValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`OptionsSetVfsValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetVfsValue {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetVfsValue {
        fn from(value: &OptionsSetVfsValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetVfsValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetVfsValue {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetVfsValue {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetVfsValue {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetVfsValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`OptionsSetWebdavValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "number"
    ///    },
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OptionsSetWebdavValue {
        Variant0(::std::string::String),
        Variant1(f64),
        Variant2(i64),
        Variant3(bool),
        Variant4(::std::vec::Vec<::serde_json::Value>),
        Variant5(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    impl ::std::convert::From<&Self> for OptionsSetWebdavValue {
        fn from(value: &OptionsSetWebdavValue) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<f64> for OptionsSetWebdavValue {
        fn from(value: f64) -> Self {
            Self::Variant1(value)
        }
    }

    impl ::std::convert::From<i64> for OptionsSetWebdavValue {
        fn from(value: i64) -> Self {
            Self::Variant2(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsSetWebdavValue {
        fn from(value: bool) -> Self {
            Self::Variant3(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for OptionsSetWebdavValue {
        fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
            Self::Variant4(value)
        }
    }

    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for OptionsSetWebdavValue
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self::Variant5(value)
        }
    }

    ///`PluginsctlGetPluginsForTypeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "loadedPlugins",
    ///    "loadedTestPlugins"
    ///  ],
    ///  "properties": {
    ///    "loadedPlugins": {
    ///      "description": "Installed plugins keyed by repository name.",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "object",
    ///        "additionalProperties": true
    ///      }
    ///    },
    ///    "loadedTestPlugins": {
    ///      "description": "Installed test plugins keyed by repository name.",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "object",
    ///        "additionalProperties": true
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PluginsctlGetPluginsForTypeResponse {
        ///Installed plugins keyed by repository name.
        #[serde(rename = "loadedPlugins")]
        pub loaded_plugins: ::std::collections::HashMap<
            ::std::string::String,
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        >,
        ///Installed test plugins keyed by repository name.
        #[serde(rename = "loadedTestPlugins")]
        pub loaded_test_plugins: ::std::collections::HashMap<
            ::std::string::String,
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        >,
    }

    impl ::std::convert::From<&PluginsctlGetPluginsForTypeResponse>
        for PluginsctlGetPluginsForTypeResponse
    {
        fn from(value: &PluginsctlGetPluginsForTypeResponse) -> Self {
            value.clone()
        }
    }

    ///`PluginsctlListPluginsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "loadedPlugins",
    ///    "testPlugins"
    ///  ],
    ///  "properties": {
    ///    "loadedPlugins": {
    ///      "description": "Metadata entries for installed plugins.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "additionalProperties": true
    ///      }
    ///    },
    ///    "testPlugins": {
    ///      "description": "Metadata entries for installed test plugins.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "additionalProperties": true
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PluginsctlListPluginsResponse {
        ///Metadata entries for installed plugins.
        #[serde(rename = "loadedPlugins")]
        pub loaded_plugins:
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Metadata entries for installed test plugins.
        #[serde(rename = "testPlugins")]
        pub test_plugins:
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    }

    impl ::std::convert::From<&PluginsctlListPluginsResponse> for PluginsctlListPluginsResponse {
        fn from(value: &PluginsctlListPluginsResponse) -> Self {
            value.clone()
        }
    }

    ///`PluginsctlListTestPluginsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "loadedTestPlugins"
    ///  ],
    ///  "properties": {
    ///    "loadedTestPlugins": {
    ///      "description": "Installed test plugin metadata keyed by
    /// repository.",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "object",
    ///        "additionalProperties": true
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PluginsctlListTestPluginsResponse {
        ///Installed test plugin metadata keyed by repository.
        #[serde(rename = "loadedTestPlugins")]
        pub loaded_test_plugins: ::std::collections::HashMap<
            ::std::string::String,
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        >,
    }

    impl ::std::convert::From<&PluginsctlListTestPluginsResponse>
        for PluginsctlListTestPluginsResponse
    {
        fn from(value: &PluginsctlListTestPluginsResponse) -> Self {
            value.clone()
        }
    }

    ///`RcError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "error",
    ///    "input",
    ///    "path",
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "error": {
    ///      "type": "string"
    ///    },
    ///    "input": {
    ///      "description": "Original request parameters echoed for debugging.",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": {}
    ///    },
    ///    "path": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RcError {
        pub error: ::std::string::String,
        ///Original request parameters echoed for debugging.
        pub input:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        pub path: ::std::string::String,
        pub status: i64,
    }

    impl ::std::convert::From<&RcError> for RcError {
        fn from(value: &RcError) -> Self {
            value.clone()
        }
    }

    ///`RcListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "commands"
    ///  ],
    ///  "properties": {
    ///    "commands": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "properties": {
    ///          "AuthRequired": {
    ///            "type": "boolean"
    ///          },
    ///          "Help": {
    ///            "type": "string"
    ///          },
    ///          "NeedsRequest": {
    ///            "type": "boolean"
    ///          },
    ///          "NeedsResponse": {
    ///            "type": "boolean"
    ///          },
    ///          "Path": {
    ///            "type": "string"
    ///          },
    ///          "Title": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": true
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RcListResponse {
        pub commands: ::std::vec::Vec<RcListResponseCommandsItem>,
    }

    impl ::std::convert::From<&RcListResponse> for RcListResponse {
        fn from(value: &RcListResponse) -> Self {
            value.clone()
        }
    }

    ///`RcListResponseCommandsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "AuthRequired": {
    ///      "type": "boolean"
    ///    },
    ///    "Help": {
    ///      "type": "string"
    ///    },
    ///    "NeedsRequest": {
    ///      "type": "boolean"
    ///    },
    ///    "NeedsResponse": {
    ///      "type": "boolean"
    ///    },
    ///    "Path": {
    ///      "type": "string"
    ///    },
    ///    "Title": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RcListResponseCommandsItem {
        #[serde(
            rename = "AuthRequired",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub auth_required: ::std::option::Option<bool>,
        #[serde(
            rename = "Help",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub help: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "NeedsRequest",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub needs_request: ::std::option::Option<bool>,
        #[serde(
            rename = "NeedsResponse",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub needs_response: ::std::option::Option<bool>,
        #[serde(
            rename = "Path",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Title",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&RcListResponseCommandsItem> for RcListResponseCommandsItem {
        fn from(value: &RcListResponseCommandsItem) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for RcListResponseCommandsItem {
        fn default() -> Self {
            Self {
                auth_required: Default::default(),
                help: Default::default(),
                needs_request: Default::default(),
                needs_response: Default::default(),
                path: Default::default(),
                title: Default::default(),
            }
        }
    }

    ///`ServeListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "list"
    ///  ],
    ///  "properties": {
    ///    "list": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "addr",
    ///          "id"
    ///        ],
    ///        "properties": {
    ///          "addr": {
    ///            "description": "Address and port the server is listening
    /// on.",
    ///            "type": "string"
    ///          },
    ///          "id": {
    ///            "description": "Identifier returned by `serve/start`.",
    ///            "type": "string"
    ///          },
    ///          "params": {
    ///            "description": "Serve configuration parameters supplied at
    /// startup.",
    ///            "type": "object",
    ///            "required": [
    ///              "fs",
    ///              "id",
    ///              "type"
    ///            ],
    ///            "properties": {
    ///              "fs": {
    ///                "type": "string"
    ///              },
    ///              "opt": {
    ///                "type": "object",
    ///                "additionalProperties": true
    ///              },
    ///              "type": {
    ///                "type": "string"
    ///              },
    ///              "vfsOpt": {
    ///                "type": "object",
    ///                "additionalProperties": true
    ///              }
    ///            },
    ///            "additionalProperties": true
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ServeListResponse {
        pub list: ::std::vec::Vec<ServeListResponseListItem>,
    }

    impl ::std::convert::From<&ServeListResponse> for ServeListResponse {
        fn from(value: &ServeListResponse) -> Self {
            value.clone()
        }
    }

    ///`ServeListResponseListItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "addr",
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "addr": {
    ///      "description": "Address and port the server is listening on.",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "Identifier returned by `serve/start`.",
    ///      "type": "string"
    ///    },
    ///    "params": {
    ///      "description": "Serve configuration parameters supplied at
    /// startup.",
    ///      "type": "object",
    ///      "required": [
    ///        "fs",
    ///        "id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "fs": {
    ///          "type": "string"
    ///        },
    ///        "opt": {
    ///          "type": "object",
    ///          "additionalProperties": true
    ///        },
    ///        "type": {
    ///          "type": "string"
    ///        },
    ///        "vfsOpt": {
    ///          "type": "object",
    ///          "additionalProperties": true
    ///        }
    ///      },
    ///      "additionalProperties": true
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ServeListResponseListItem {
        ///Address and port the server is listening on.
        pub addr: ::std::string::String,
        ///Identifier returned by `serve/start`.
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub params: ::std::option::Option<ServeListResponseListItemParams>,
    }

    impl ::std::convert::From<&ServeListResponseListItem> for ServeListResponseListItem {
        fn from(value: &ServeListResponseListItem) -> Self {
            value.clone()
        }
    }

    ///Serve configuration parameters supplied at startup.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Serve configuration parameters supplied at startup.",
    ///  "type": "object",
    ///  "required": [
    ///    "fs",
    ///    "id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "fs": {
    ///      "type": "string"
    ///    },
    ///    "opt": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    },
    ///    "vfsOpt": {
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    }
    ///  },
    ///  "additionalProperties": true
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ServeListResponseListItemParams {
        pub fs: ::std::string::String,
        pub id: ::serde_json::Value,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub opt: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        #[serde(
            rename = "vfsOpt",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub vfs_opt: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    impl ::std::convert::From<&ServeListResponseListItemParams> for ServeListResponseListItemParams {
        fn from(value: &ServeListResponseListItemParams) -> Self {
            value.clone()
        }
    }

    ///`ServeStartResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "addr",
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "addr": {
    ///      "description": "Address and port the server is listening on.",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "Identifier to pass to `serve/stop`.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ServeStartResponse {
        ///Address and port the server is listening on.
        pub addr: ::std::string::String,
        ///Identifier to pass to `serve/stop`.
        pub id: ::std::string::String,
    }

    impl ::std::convert::From<&ServeStartResponse> for ServeStartResponse {
        fn from(value: &ServeStartResponse) -> Self {
            value.clone()
        }
    }

    ///`SyncBisyncResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "jobid": {
    ///      "description": "Job ID of the operation.",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SyncBisyncResponse {
        ///Job ID of the operation.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub jobid: ::std::option::Option<i64>,
    }

    impl ::std::convert::From<&SyncBisyncResponse> for SyncBisyncResponse {
        fn from(value: &SyncBisyncResponse) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for SyncBisyncResponse {
        fn default() -> Self {
            Self {
                jobid: Default::default(),
            }
        }
    }

    ///`SyncCopyResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "jobid": {
    ///      "description": "Job ID of the operation.",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SyncCopyResponse {
        ///Job ID of the operation.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub jobid: ::std::option::Option<i64>,
    }

    impl ::std::convert::From<&SyncCopyResponse> for SyncCopyResponse {
        fn from(value: &SyncCopyResponse) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for SyncCopyResponse {
        fn default() -> Self {
            Self {
                jobid: Default::default(),
            }
        }
    }

    ///`SyncMoveResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "jobid": {
    ///      "description": "Job ID of the operation.",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SyncMoveResponse {
        ///Job ID of the operation.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub jobid: ::std::option::Option<i64>,
    }

    impl ::std::convert::From<&SyncMoveResponse> for SyncMoveResponse {
        fn from(value: &SyncMoveResponse) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for SyncMoveResponse {
        fn default() -> Self {
            Self {
                jobid: Default::default(),
            }
        }
    }

    ///`SyncSyncResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "jobid": {
    ///      "description": "Job ID of the operation.",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SyncSyncResponse {
        ///Job ID of the operation.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub jobid: ::std::option::Option<i64>,
    }

    impl ::std::convert::From<&SyncSyncResponse> for SyncSyncResponse {
        fn from(value: &SyncSyncResponse) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for SyncSyncResponse {
        fn default() -> Self {
            Self {
                jobid: Default::default(),
            }
        }
    }

    ///`VfsForgetResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "forgotten"
    ///  ],
    ///  "properties": {
    ///    "forgotten": {
    ///      "description": "Paths that were successfully forgotten.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsForgetResponse {
        ///Paths that were successfully forgotten.
        pub forgotten: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::convert::From<&VfsForgetResponse> for VfsForgetResponse {
        fn from(value: &VfsForgetResponse) -> Self {
            value.clone()
        }
    }

    ///`VfsListResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "vfses"
    ///  ],
    ///  "properties": {
    ///    "vfses": {
    ///      "description": "VFS name that can be used with other VFS
    /// endpoints.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsListResponse {
        ///VFS name that can be used with other VFS endpoints.
        pub vfses: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::convert::From<&VfsListResponse> for VfsListResponse {
        fn from(value: &VfsListResponse) -> Self {
            value.clone()
        }
    }

    ///`VfsQueueResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "queued": {
    ///      "type": "array",
    ///      "items": {
    ///        "description": "Queued item metadata such as name, size, expiry,
    /// and upload state.",
    ///        "type": "object",
    ///        "additionalProperties": true
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsQueueResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub queued: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    }

    impl ::std::convert::From<&VfsQueueResponse> for VfsQueueResponse {
        fn from(value: &VfsQueueResponse) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for VfsQueueResponse {
        fn default() -> Self {
            Self {
                queued: Default::default(),
            }
        }
    }

    ///`VfsRefreshResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "description": "Map of refreshed directories to status messages.",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsRefreshResponse {
        ///Map of refreshed directories to status messages.
        pub result: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    }

    impl ::std::convert::From<&VfsRefreshResponse> for VfsRefreshResponse {
        fn from(value: &VfsRefreshResponse) -> Self {
            value.clone()
        }
    }

    ///`VfsStatsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "fs",
    ///    "inUse",
    ///    "metadataCache",
    ///    "opt"
    ///  ],
    ///  "properties": {
    ///    "diskCache": {
    ///      "description": "Disk cache metrics when caching is enabled.",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": true
    ///    },
    ///    "fs": {
    ///      "description": "Name of the VFS.",
    ///      "type": "string"
    ///    },
    ///    "inUse": {
    ///      "description": "Number of active references to the VFS.",
    ///      "type": "integer"
    ///    },
    ///    "metadataCache": {
    ///      "description": "In-memory metadata cache counters.",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "opt": {
    ///      "description": "Effective options applied to the VFS.",
    ///      "type": "object",
    ///      "additionalProperties": true
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsStatsResponse {
        ///Disk cache metrics when caching is enabled.
        #[serde(
            rename = "diskCache",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub disk_cache:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Name of the VFS.
        pub fs: ::std::string::String,
        ///Number of active references to the VFS.
        #[serde(rename = "inUse")]
        pub in_use: i64,
        ///In-memory metadata cache counters.
        #[serde(rename = "metadataCache")]
        pub metadata_cache: ::std::collections::HashMap<::std::string::String, i64>,
        ///Effective options applied to the VFS.
        pub opt: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    impl ::std::convert::From<&VfsStatsResponse> for VfsStatsResponse {
        fn from(value: &VfsStatsResponse) -> Self {
            value.clone()
        }
    }
}

#[derive(Clone, Debug)]
///Client for Rclone RC API
///
///Full OpenAPI specification for the Rclone RC API.
///
///Version: 0.1.3
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "0.1.3"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    ///Echo request parameters
    ///
    ///Returns all supplied parameters unchanged so you can verify RC
    /// connectivity.
    ///
    ///Sends a `POST` request to `/rc/noop`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `params`: Additional arbitrary parameters allowed.
    pub async fn rc_noop<'a>(
        &'a self,
        async_: Option<bool>,
        params: Option<&'a ::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/rc/noop", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("params", &params))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "rc_noop",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove trashed files
    ///
    ///Permanently removes trashed objects from the specified remote path.
    ///
    ///Sends a `POST` request to `/operations/cleanup`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote name or path to clean up, for example `drive:`.
    pub async fn operations_cleanup<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/operations/cleanup", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_cleanup",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Copy a single file
    ///
    ///Copies one object from a source remote and path to a destination remote
    /// and path.
    ///
    ///Sends a `POST` request to `/operations/copyfile`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `dst_fs`: Destination remote name or path, such as `drive2:` or `/`
    ///   for local filesystem.
    /// - `dst_remote`: Target path within `dstFs` where the file should be
    ///   written.
    /// - `src_fs`: Source remote name or path, such as `drive:` or `/` for the
    ///   local filesystem.
    /// - `src_remote`: Path to the source object within `srcFs`, for example
    ///   `dir/file.txt`.
    pub async fn operations_copyfile<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        dst_fs: &'a str,
        dst_remote: &'a str,
        src_fs: &'a str,
        src_remote: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/operations/copyfile", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("dstFs", &dst_fs))
            .query(&progenitor_client::QueryParam::new(
                "dstRemote",
                &dst_remote,
            ))
            .query(&progenitor_client::QueryParam::new("srcFs", &src_fs))
            .query(&progenitor_client::QueryParam::new(
                "srcRemote",
                &src_remote,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_copyfile",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Copy from URL
    ///
    ///Downloads a public URL and stores it at the requested remote path.
    ///
    ///Sends a `POST` request to `/operations/copyurl`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `auto_filename`: Set to true to derive the destination filename from
    ///   the URL.
    /// - `fs`: Remote name or path that will receive the downloaded file, e.g.
    ///   `drive:`.
    /// - `remote`: Destination path within `fs` where the fetched object will
    ///   be stored.
    /// - `url`: Source URL to fetch the object from.
    pub async fn operations_copyurl<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        auto_filename: Option<bool>,
        fs: &'a str,
        remote: &'a str,
        url: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let _url = format!("{}/operations/copyurl", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(_url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new(
                "autoFilename",
                &auto_filename,
            ))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("remote", &remote))
            .query(&progenitor_client::QueryParam::new("url", &url))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_copyurl",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete objects in path
    ///
    ///Deletes matching files and directories for the provided remote,
    /// honouring filters and config overrides.
    ///
    ///Sends a `POST` request to `/operations/delete`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `config`: JSON encoded config overrides applied for this call only.
    /// - `filter`: JSON encoded filter overrides applied for this call only.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote name or path whose contents should be removed.
    pub async fn operations_delete<'a>(
        &'a self,
        async_: Option<bool>,
        config: Option<&'a str>,
        filter: Option<&'a str>,
        group: Option<&'a str>,
        fs: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/operations/delete", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_config", &config))
            .query(&progenitor_client::QueryParam::new("_filter", &filter))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_delete",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete single file
    ///
    ///Removes a specific object from the remote.
    ///
    ///Sends a `POST` request to `/operations/deletefile`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote name or path that contains the file to delete.
    /// - `remote`: Exact path to the file within `fs` that should be deleted.
    pub async fn operations_deletefile<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: &'a str,
        remote: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/operations/deletefile", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("remote", &remote))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_deletefile",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Describe remote capabilities
    ///
    ///Returns backend features, hash support, metadata descriptions, and other
    /// info for the remote.
    ///
    ///Sends a `POST` request to `/operations/fsinfo`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote name or path to inspect, e.g. `drive:`.
    pub async fn operations_fsinfo<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: &'a str,
    ) -> Result<ResponseValue<types::OperationsFsinfoResponse>, Error<types::RcError>> {
        let url = format!("{}/operations/fsinfo", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_fsinfo",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Generate hash sums
    ///
    ///Produces a hash sum listing for files under the given path using the
    /// requested hash algorithm.
    ///
    ///Sends a `POST` request to `/operations/hashsum`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `base64`: Set to true to emit hash values in base64 rather than
    ///   hexadecimal.
    /// - `download`: Set to true to force reading the data instead of using
    ///   remote checksums.
    /// - `fs`: Remote name or path to hash, such as `drive:` or `/`.
    /// - `hash_type`: Hash algorithm to use, e.g. `md5`, `sha1`, or another
    ///   supported name.
    pub async fn operations_hashsum<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        base64: Option<bool>,
        download: Option<bool>,
        fs: &'a str,
        hash_type: &'a str,
    ) -> Result<ResponseValue<types::OperationsHashsumResponse>, Error<types::RcError>> {
        let url = format!("{}/operations/hashsum", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("base64", &base64))
            .query(&progenitor_client::QueryParam::new("download", &download))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("hashType", &hash_type))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_hashsum",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Move a single file
    ///
    ///Moves one object from a source remote and path to a destination remote
    /// and path.
    ///
    ///Sends a `POST` request to `/operations/movefile`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `dst_fs`: Destination remote name or path where the file will be
    ///   moved.
    /// - `dst_remote`: Destination path within `dstFs` for the moved object.
    /// - `src_fs`: Source remote name or path containing the file to move.
    /// - `src_remote`: Path to the source object within `srcFs`.
    pub async fn operations_movefile<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        dst_fs: &'a str,
        dst_remote: &'a str,
        src_fs: &'a str,
        src_remote: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/operations/movefile", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("dstFs", &dst_fs))
            .query(&progenitor_client::QueryParam::new(
                "dstRemote",
                &dst_remote,
            ))
            .query(&progenitor_client::QueryParam::new("srcFs", &src_fs))
            .query(&progenitor_client::QueryParam::new(
                "srcRemote",
                &src_remote,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_movefile",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create or remove public link
    ///
    ///Creates a share URL for an object or removes an existing link when
    /// `unlink=true`.
    ///
    ///Sends a `POST` request to `/operations/publiclink`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `expire`: Optional expiration time for the public link, formatted as
    ///   supported by the backend.
    /// - `fs`: Remote name or path hosting the object for which to manage a
    ///   public link.
    /// - `remote`: Path within `fs` to the object for which to create or remove
    ///   a public link.
    /// - `unlink`: Set to true to remove an existing public link instead of
    ///   creating one.
    pub async fn operations_publiclink<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        expire: Option<&'a str>,
        fs: &'a str,
        remote: &'a str,
        unlink: Option<bool>,
    ) -> Result<ResponseValue<types::OperationsPubliclinkResponse>, Error<types::RcError>> {
        let url = format!("{}/operations/publiclink", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("expire", &expire))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("remote", &remote))
            .query(&progenitor_client::QueryParam::new("unlink", &unlink))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_publiclink",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove empty directories
    ///
    ///Deletes empty subdirectories beneath the specified path, optionally
    /// leaving the root.
    ///
    ///Sends a `POST` request to `/operations/rmdirs`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote name or path to scan for empty directories.
    /// - `leave_root`: Set to true to preserve the top-level directory even if
    ///   empty.
    /// - `remote`: Path within `fs` whose empty subdirectories should be
    ///   removed.
    pub async fn operations_rmdirs<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: &'a str,
        leave_root: Option<bool>,
        remote: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/operations/rmdirs", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new(
                "leaveRoot",
                &leave_root,
            ))
            .query(&progenitor_client::QueryParam::new("remote", &remote))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_rmdirs",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Change storage tier
    ///
    ///Updates the storage class or tier for every object in the specified
    /// remote path.
    ///
    ///Sends a `POST` request to `/operations/settier`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote name or path whose storage class tier should be changed.
    pub async fn operations_settier<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/operations/settier", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_settier",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Change file storage tier
    ///
    ///Updates the storage class or tier for a single object.
    ///
    ///Sends a `POST` request to `/operations/settierfile`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote name or path that contains the object whose tier should
    ///   change.
    /// - `remote`: Path within `fs` to the object whose storage class tier
    ///   should be updated.
    pub async fn operations_settierfile<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: &'a str,
        remote: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/operations/settierfile", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("remote", &remote))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_settierfile",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Count remote size
    ///
    ///Reports total size, file count, and number of objects without size
    /// metadata.
    ///
    ///Sends a `POST` request to `/operations/size`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote name or path to measure aggregate size information for.
    pub async fn operations_size<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: &'a str,
    ) -> Result<ResponseValue<types::OperationsSizeResponse>, Error<types::RcError>> {
        let url = format!("{}/operations/size", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_size",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get or update bandwidth limits
    ///
    ///Reads the current bandwidth limit or applies a new schedule string, just
    /// like `rclone rc core/bwlimit`.
    ///
    ///Sends a `POST` request to `/core/bwlimit`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `rate`: Bandwidth limit to apply, for example `off`, `5M`, or a
    ///   schedule string.
    pub async fn core_bwlimit<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        rate: Option<&'a str>,
    ) -> Result<ResponseValue<types::CoreBwlimitResponse>, Error<types::RcError>> {
        let url = format!("{}/core/bwlimit", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("rate", &rate))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_bwlimit",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Run an rclone command
    ///
    ///Executes a standard rclone CLI command remotely and streams or returns
    /// its output.
    ///
    ///Sends a `POST` request to `/core/command`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `arg`: Optional positional arguments for the command. Repeat to supply
    ///   multiple values.
    /// - `command`: Name of the rclone command to execute, for example `ls` or
    ///   `lsf`.
    /// - `opt`: Optional command options encoded as a JSON string.
    /// - `return_type`: Controls how output is returned; accepts
    ///   `COMBINED_OUTPUT`, `STREAM`, `STREAM_ONLY_STDOUT`, or
    ///   `STREAM_ONLY_STDERR`.
    pub async fn core_command<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        arg: Option<&'a ::std::vec::Vec<::std::string::String>>,
        command: &'a str,
        opt: Option<&'a str>,
        return_type: Option<&'a str>,
    ) -> Result<ResponseValue<types::CoreCommandResponse>, Error<types::RcError>> {
        let url = format!("{}/core/command", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("arg", &arg))
            .query(&progenitor_client::QueryParam::new("command", &command))
            .query(&progenitor_client::QueryParam::new("opt", &opt))
            .query(&progenitor_client::QueryParam::new(
                "returnType",
                &return_type,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_command",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Report disk usage
    ///
    ///Returns disk usage statistics for the supplied local directory (defaults
    /// to the cache dir).
    ///
    ///Sends a `POST` request to `/core/du`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `dir`: Local directory path to report disk usage for. Defaults to the
    ///   rclone cache directory when omitted.
    pub async fn core_du<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        dir: Option<&'a str>,
    ) -> Result<ResponseValue<types::CoreDuResponse>, Error<types::RcError>> {
        let url = format!("{}/core/du", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("dir", &dir))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_du",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Force garbage collection
    ///
    ///Triggers Go's garbage collector to release unused memory.
    ///
    ///Sends a `POST` request to `/core/gc`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn core_gc<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/core/gc", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_gc",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List stats groups
    ///
    ///Lists stats groups currently tracked by rclone.
    ///
    ///Sends a `POST` request to `/core/group-list`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn core_group_list<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::CoreGroupListResponse>, Error<types::RcError>> {
        let url = format!("{}/core/group-list", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_group_list",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Fetch runtime memory stats
    ///
    ///Returns Go runtime memory statistics similar to `runtime.ReadMemStats`.
    ///
    ///Sends a `POST` request to `/core/memstats`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn core_memstats<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<
        ResponseValue<::std::collections::HashMap<::std::string::String, f64>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/core/memstats", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_memstats",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Obscure a clear string
    ///
    ///Obscures a plain-text secret for inclusion in `rclone.conf`.
    ///
    ///Sends a `POST` request to `/core/obscure`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `clear`: Plain-text string to obscure for storage in the config file.
    pub async fn core_obscure<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        clear: &'a str,
    ) -> Result<ResponseValue<types::CoreObscureResponse>, Error<types::RcError>> {
        let url = format!("{}/core/obscure", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("clear", &clear))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_obscure",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return rclone PID
    ///
    ///Returns the process ID of the running rclone instance.
    ///
    ///Sends a `POST` request to `/core/pid`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn core_pid<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::CorePidResponse>, Error<types::RcError>> {
        let url = format!("{}/core/pid", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_pid",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Terminate rclone
    ///
    ///Stops the rclone process, optionally supplying an exit code.
    ///
    ///Sends a `POST` request to `/core/quit`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `exit_code`: Optional exit code to use when terminating the rclone
    ///   process.
    pub async fn core_quit<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        exit_code: Option<i64>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/core/quit", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("exitCode", &exit_code))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_quit",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete stats group
    ///
    ///Deletes the counters associated with a specific stats group.
    ///
    ///Sends a `POST` request to `/core/stats-delete`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `group`: Stats group identifier to remove.
    pub async fn core_stats_delete<'a>(
        &'a self,
        async_: Option<bool>,
        _group: Option<&'a str>,
        group: &'a str
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/core/stats-delete", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_stats_delete",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Reset stats counters
    ///
    ///Clears counters, errors, and finished transfers for the provided stats
    /// group or all groups.
    ///
    ///Sends a `POST` request to `/core/stats-reset`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `group`: Stats group identifier whose counters should be reset. Leave
    ///   unset to reset all groups.
    pub async fn core_stats_reset<'a>(
        &'a self,
        async_: Option<bool>,
        _group: Option<&'a str>,
        group: Option<&'a str>
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/core/stats-reset", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_stats_reset",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List completed transfers
    ///
    ///Returns up to 100 recently completed transfers for the requested stats
    /// group.
    ///
    ///Sends a `POST` request to `/core/transferred`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `group`: Stats group identifier to filter the completed transfer list.
    ///   Leave unset for all groups.
    pub async fn core_transferred<'a>(
        &'a self,
        async_: Option<bool>,
        _group: Option<&'a str>,
        group: Option<&'a str>
    ) -> Result<ResponseValue<types::CoreTransferredResponse>, Error<types::RcError>> {
        let url = format!("{}/core/transferred", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_transferred",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/debug/set-block-profile-rate`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `rate`: Sampling interval in nanoseconds for blocking profile
    ///   collection; use 1 to capture all events.
    pub async fn debug_set_block_profile_rate<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        rate: i64,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/debug/set-block-profile-rate", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("rate", &rate))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "debug_set_block_profile_rate",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/debug/set-gc-percent`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `gc_percent`: Target percentage of newly allocated data to trigger
    ///   garbage collection.
    pub async fn debug_set_gc_percent<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        gc_percent: i64,
    ) -> Result<ResponseValue<types::DebugSetGcPercentResponse>, Error<types::RcError>> {
        let url = format!("{}/debug/set-gc-percent", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new(
                "gc-percent",
                &gc_percent,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "debug_set_gc_percent",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/debug/set-mutex-profile-fraction`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `rate`: Sampling fraction for mutex contention profiling; set to 0 to
    ///   disable.
    pub async fn debug_set_mutex_profile_fraction<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        rate: i64,
    ) -> Result<ResponseValue<types::DebugSetMutexProfileFractionResponse>, Error<types::RcError>>
    {
        let url = format!("{}/debug/set-mutex-profile-fraction", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("rate", &rate))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "debug_set_mutex_profile_fraction",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/debug/set-soft-memory-limit`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `mem_limit`: Soft memory limit for the Go runtime in bytes.
    pub async fn debug_set_soft_memory_limit<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        mem_limit: i64,
    ) -> Result<ResponseValue<types::DebugSetSoftMemoryLimitResponse>, Error<types::RcError>> {
        let url = format!("{}/debug/set-soft-memory-limit", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("mem-limit", &mem_limit))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "debug_set_soft_memory_limit",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/fscache/clear`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn fscache_clear<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/fscache/clear", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "fscache_clear",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/fscache/entries`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn fscache_entries<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::FscacheEntriesResponse>, Error<types::RcError>> {
        let url = format!("{}/fscache/entries", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "fscache_entries",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/mount/listmounts`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn mount_listmounts<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::MountListmountsResponse>, Error<types::RcError>> {
        let url = format!("{}/mount/listmounts", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "mount_listmounts",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/mount/mount`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `config`: JSON encoded config overrides applied for this call only.
    /// - `filter`: JSON encoded filter overrides applied for this call only.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote path to mount, such as `drive:` or `remote:subdir`.
    /// - `mount_opt`: Mount options encoded as JSON, matching flags accepted by
    ///   `rclone mount`.
    /// - `mount_point`: Absolute local path where the remote should be mounted.
    /// - `mount_type`: Optional mount implementation to use (`mount`, `cmount`,
    ///   or `mount2`).
    /// - `vfs_opt`: VFS options encoded as JSON, matching flags accepted by
    ///   `rclone mount`.
    pub async fn mount_mount<'a>(
        &'a self,
        async_: Option<bool>,
        config: Option<&'a str>,
        filter: Option<&'a str>,
        group: Option<&'a str>,
        fs: &'a str,
        mount_opt: Option<&'a str>,
        mount_point: &'a str,
        mount_type: Option<&'a str>,
        vfs_opt: Option<&'a str>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/mount/mount", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_config", &config))
            .query(&progenitor_client::QueryParam::new("_filter", &filter))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("mountOpt", &mount_opt))
            .query(&progenitor_client::QueryParam::new(
                "mountPoint",
                &mount_point,
            ))
            .query(&progenitor_client::QueryParam::new(
                "mountType",
                &mount_type,
            ))
            .query(&progenitor_client::QueryParam::new("vfsOpt", &vfs_opt))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "mount_mount",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/mount/types`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn mount_types<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::MountTypesResponse>, Error<types::RcError>> {
        let url = format!("{}/mount/types", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "mount_types",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/mount/unmount`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `mount_point`: Local mount point path to unmount.
    pub async fn mount_unmount<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        mount_point: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/mount/unmount", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new(
                "mountPoint",
                &mount_point,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "mount_unmount",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/mount/unmountall`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn mount_unmountall<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/mount/unmountall", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "mount_unmountall",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Echo parameters (auth required)
    ///
    ///Same as `rc/noop`, but requires authentication to validate access
    /// control.
    ///
    ///Sends a `POST` request to `/rc/noopauth`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `params`: Additional arbitrary parameters allowed.
    pub async fn rc_noop_auth<'a>(
        &'a self,
        async_: Option<bool>,
        params: Option<&'a ::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/rc/noopauth", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("params", &params))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "rc_noop_auth",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return a test error
    ///
    ///Always returns an error response incorporating the supplied parameters,
    /// useful for testing error handling.
    ///
    ///Sends a `POST` request to `/rc/error`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `params`: Additional arbitrary parameters allowed.
    pub async fn rc_error<'a>(
        &'a self,
        async_: Option<bool>,
        params: Option<&'a ::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    ) -> Result<ResponseValue<()>, Error<types::RcError>> {
        let url = format!("{}/rc/error", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("params", &params))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "rc_error",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List RC commands
    ///
    ///Returns metadata about every available RC command, including whether
    /// authentication is required.
    ///
    ///Sends a `POST` request to `/rc/list`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn rc_list<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::RcListResponse>, Error<types::RcError>> {
        let url = format!("{}/rc/list", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "rc_list",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Run backend command
    ///
    ///Invokes a backend-specific management command against an optional
    /// remote.
    ///
    ///Sends a `POST` request to `/backend/command`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `arg`: Optional positional arguments for the backend command.
    /// - `command`: Backend-specific command to invoke.
    /// - `fs`: Remote name or path the backend command should target.
    /// - `opt`: Backend command options encoded as a JSON string.
    pub async fn backend_command<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        arg: Option<&'a ::std::vec::Vec<::std::string::String>>,
        command: &'a str,
        fs: Option<&'a str>,
        opt: Option<&'a str>,
    ) -> Result<ResponseValue<types::BackendCommandResponse>, Error<types::RcError>> {
        let url = format!("{}/backend/command", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("arg", &arg))
            .query(&progenitor_client::QueryParam::new("command", &command))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("opt", &opt))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "backend_command",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Expire cache entries
    ///
    ///Drops cached directory entries, and optionally cached file data, for the
    /// cache backend.
    ///
    ///Sends a `POST` request to `/cache/expire`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `remote`: Remote path to expire from the cache, e.g.
    ///   `remote:path/to/dir`.
    /// - `with_data`: Set to true to drop cached chunk data along with
    ///   directory entries.
    pub async fn cache_expire<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        remote: &'a str,
        with_data: Option<bool>,
    ) -> Result<ResponseValue<()>, Error<types::RcError>> {
        let url = format!("{}/cache/expire", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("remote", &remote))
            .query(&progenitor_client::QueryParam::new("withData", &with_data))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "cache_expire",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Prefetch cache chunks
    ///
    ///Ensures specified file chunks are cached locally for a cache remote.
    ///
    ///Sends a `POST` request to `/cache/fetch`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `chunks`: Comma-separated chunk specifier list (e.g. `0:10,25:30`)
    ///   describing file pieces to prefetch.
    /// - `params`: Additional arbitrary parameters allowed.
    pub async fn cache_fetch<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        chunks: Option<&'a str>,
        params: Option<&'a ::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    ) -> Result<ResponseValue<()>, Error<types::RcError>> {
        let url = format!("{}/cache/fetch", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("chunks", &chunks))
            .query(&progenitor_client::QueryParam::new("params", &params))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "cache_fetch",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Show cache stats
    ///
    ///Returns runtime statistics for the cache backend.
    ///
    ///Sends a `POST` request to `/cache/stats`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn cache_stats<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<types::RcError>> {
        let url = format!("{}/cache/stats", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "cache_stats",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create remote configuration
    ///
    ///Creates a new remote in `rclone.conf`, mirroring `rclone config create`.
    ///
    ///Sends a `POST` request to `/config/create`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `name`: Name of the new remote configuration.
    /// - `opt`: Optional JSON object controlling interactive behaviour (e.g.
    ///   `obscure`, `continue`).
    /// - `parameters`: JSON object of configuration key/value pairs required
    ///   for the remote.
    /// - `type_`: Backend type identifier, such as `drive`, `s3`, or `dropbox`.
    pub async fn config_create<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        name: &'a str,
        opt: Option<&'a str>,
        parameters: &'a str,
        type_: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/config/create", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new("opt", &opt))
            .query(&progenitor_client::QueryParam::new(
                "parameters",
                &parameters,
            ))
            .query(&progenitor_client::QueryParam::new("type", &type_))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "config_create",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete remote configuration
    ///
    ///Removes an existing remote from `rclone.conf`.
    ///
    ///Sends a `POST` request to `/config/delete`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `name`: Name of the remote configuration to delete.
    pub async fn config_delete<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::RcError>> {
        let url = format!("{}/config/delete", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "config_delete",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Dump configuration
    ///
    ///Returns the contents of the config file as a JSON object keyed by remote
    /// name.
    ///
    ///Sends a `POST` request to `/config/dump`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn config_dump<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<
        ResponseValue<
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
        >,
        Error<types::RcError>,
    > {
        let url = format!("{}/config/dump", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "config_dump",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get remote configuration
    ///
    ///Returns the key/value settings for a single remote.
    ///
    ///Sends a `POST` request to `/config/get`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `name`: Name of the remote configuration to fetch.
    pub async fn config_get<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        name: &'a str,
    ) -> Result<ResponseValue<types::ConfigGetResponse>, Error<types::RcError>> {
        let url = format!("{}/config/get", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "config_get",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List configured remotes
    ///
    ///Returns the names of all remotes defined in the config file.
    ///
    ///Sends a `POST` request to `/config/listremotes`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn config_listremotes<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::ConfigListremotesResponse>, Error<types::RcError>> {
        let url = format!("{}/config/listremotes", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "config_listremotes",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update remote secrets
    ///
    ///Sets obscured password fields for a remote configuration.
    ///
    ///Sends a `POST` request to `/config/password`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `name`: Name of the remote whose secrets should be updated.
    /// - `parameters`: JSON object of password answers, typically including
    ///   `pass`.
    pub async fn config_password<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        name: &'a str,
        parameters: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::RcError>> {
        let url = format!("{}/config/password", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new(
                "parameters",
                &parameters,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "config_password",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Show config paths
    ///
    ///Returns the paths to the config file, cache directory, and temporary
    /// directory.
    ///
    ///Sends a `POST` request to `/config/paths`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn config_paths<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::ConfigPathsResponse>, Error<types::RcError>> {
        let url = format!("{}/config/paths", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "config_paths",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List backend providers
    ///
    ///Returns metadata describing each supported storage provider.
    ///
    ///Sends a `POST` request to `/config/providers`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn config_providers<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::ConfigProvidersResponse>, Error<types::RcError>> {
        let url = format!("{}/config/providers", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "config_providers",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Set config path
    ///
    ///Points rclone at a specific `rclone.conf` file.
    ///
    ///Sends a `POST` request to `/config/setpath`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `path`: Absolute path to the `rclone.conf` file that rclone should
    ///   use.
    pub async fn config_setpath<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        path: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::RcError>> {
        let url = format!("{}/config/setpath", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("path", &path))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "config_setpath",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Unlock encrypted config
    ///
    ///Unlocks the configuration file using the provided password.
    ///
    ///Sends a `POST` request to `/config/unlock`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `config_password`: Password used to unlock an encrypted config file.
    pub async fn config_unlock<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        config_password: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::RcError>> {
        let url = format!("{}/config/unlock", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new(
                "configPassword",
                &config_password,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "config_unlock",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update remote configuration
    ///
    ///Updates an existing remote with new parameter values.
    ///
    ///Sends a `POST` request to `/config/update`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `name`: Name of the remote configuration to update.
    /// - `opt`: Optional JSON object controlling update behaviour (e.g.
    ///   `obscure`, `continue`).
    /// - `parameters`: JSON object of configuration key/value pairs to apply to
    ///   the remote.
    pub async fn config_update<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        name: &'a str,
        opt: Option<&'a str>,
        parameters: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/config/update", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .query(&progenitor_client::QueryParam::new("opt", &opt))
            .query(&progenitor_client::QueryParam::new(
                "parameters",
                &parameters,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "config_update",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Report rclone version
    ///
    ///Returns the running rclone version, build metadata, and Go runtime
    /// details.
    ///
    ///Sends a `POST` request to `/core/version`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn core_version<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::CoreVersionResponse>, Error<types::RcError>> {
        let url = format!("{}/core/version", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_version",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Current stats snapshot
    ///
    ///Returns active transfer statistics including bytes transferred, speed,
    /// and error counts.
    ///
    ///Sends a `POST` request to `/core/stats`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `group`: Stats group identifier to return a snapshot for. Leave unset
    ///   to include all groups.
    /// - `short`: When true, omit the `transferring` and `checking` arrays from
    ///   the response.
    pub async fn core_stats<'a>(
        &'a self,
        async_: Option<bool>,
        _group: Option<&'a str>,
        group: Option<&'a str>,
        short: Option<bool>
    ) -> Result<ResponseValue<types::CoreStatsResponse>, Error<types::RcError>> {
        let url = format!("{}/core/stats", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("group", &group))
            .query(&progenitor_client::QueryParam::new("short", &short))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_stats",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Run batch of commands
    ///
    ///Run a batch of rclone rc commands concurrently.
    ///
    ///Sends a `POST` request to `/job/batch`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `concurrency`: Do this many commands concurrently. Defaults to
    ///   --transfers if not set.
    /// - `inputs`: List of inputs to the commands with an extra _path
    ///   parameter.
    /// - `body`
    pub async fn job_batch<'a>(
        &'a self,
        async_: Option<bool>,
        concurrency: Option<i64>,
        inputs: Option<&'a ::std::vec::Vec<types::JobBatchInputsItem>>,
        body: &'a types::JobBatchRequest,
    ) -> Result<ResponseValue<types::JobBatchResponse>, Error<types::RcError>> {
        let url = format!("{}/job/batch", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new(
                "concurrency",
                &concurrency,
            ))
            .query(&progenitor_client::QueryParam::new("inputs", &inputs))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "job_batch",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List jobs
    ///
    ///Returns identifiers of active and recently completed asynchronous jobs.
    ///
    ///Sends a `POST` request to `/job/list`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    pub async fn job_list<'a>(
        &'a self,
        async_: Option<bool>,
    ) -> Result<ResponseValue<types::JobListResponse>, Error<types::RcError>> {
        let url = format!("{}/job/list", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "job_list",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get job status
    ///
    ///Returns timing, success state, output, and progress for a specific job.
    ///
    ///Sends a `POST` request to `/job/status`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `jobid`: Numeric identifier of the job to query, as returned from an
    ///   async call.
    pub async fn job_status<'a>(
        &'a self,
        async_: Option<bool>,
        jobid: f64,
    ) -> Result<ResponseValue<types::JobStatusResponse>, Error<types::RcError>> {
        let url = format!("{}/job/status", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("jobid", &jobid))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "job_status",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Stop job
    ///
    ///Attempts to cancel a running job by ID.
    ///
    ///Sends a `POST` request to `/job/stop`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `jobid`: Numeric identifier of the job to cancel.
    pub async fn job_stop<'a>(
        &'a self,
        async_: Option<bool>,
        jobid: f64,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/job/stop", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("jobid", &jobid))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "job_stop",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Stop jobs in group
    ///
    ///Cancels all active jobs associated with the provided stats group.
    ///
    ///Sends a `POST` request to `/job/stopgroup`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Stats group name whose active jobs should be stopped.
    pub async fn job_stopgroup<'a>(
        &'a self,
        async_: Option<bool>,
        group: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/job/stopgroup", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "job_stopgroup",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List objects
    ///
    ///Lists objects and directories for a remote path, returning the same
    /// fields as `rclone lsjson`.
    ///
    ///Sends a `POST` request to `/operations/list`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `dirs_only`: Set to true to return only directory entries.
    /// - `files_only`: Set to true to return only file entries.
    /// - `fs`: Remote name or path to list, for example `drive:`.
    /// - `hash_types`: Specify one or more hash algorithms to include when
    ///   `showHash` is true (e.g. `md5`).
    /// - `metadata`: Set to true to include backend-provided metadata maps.
    /// - `no_mime_type`: Set to true to omit MIME type detection.
    /// - `no_mod_time`: Set to true to omit modification times for faster
    ///   listings on some backends.
    /// - `opt`: Optional JSON-encoded object of listing flags (e.g. `{
    ///   "recurse": true, "showHash": true }`).
    /// - `recurse`: Set to true to list directories recursively.
    /// - `remote`: Directory path within `fs` to list; leave empty to target
    ///   the root.
    /// - `show_encrypted`: Set to true to include encrypted names when using
    ///   crypt remotes.
    /// - `show_hash`: Set to true to include hash digests for each entry.
    /// - `show_orig_i_ds`: Set to true to include original backend identifiers
    ///   where available.
    pub async fn operations_list<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        dirs_only: Option<bool>,
        files_only: Option<bool>,
        fs: &'a str,
        hash_types: Option<&'a ::std::vec::Vec<::std::string::String>>,
        metadata: Option<bool>,
        no_mime_type: Option<bool>,
        no_mod_time: Option<bool>,
        opt: Option<&'a str>,
        recurse: Option<bool>,
        remote: &'a str,
        show_encrypted: Option<bool>,
        show_hash: Option<bool>,
        show_orig_i_ds: Option<bool>,
    ) -> Result<ResponseValue<types::OperationsListResponse>, Error<types::RcError>> {
        let url = format!("{}/operations/list", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("dirsOnly", &dirs_only))
            .query(&progenitor_client::QueryParam::new(
                "filesOnly",
                &files_only,
            ))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new(
                "hashTypes",
                &hash_types,
            ))
            .query(&progenitor_client::QueryParam::new("metadata", &metadata))
            .query(&progenitor_client::QueryParam::new(
                "noMimeType",
                &no_mime_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "noModTime",
                &no_mod_time,
            ))
            .query(&progenitor_client::QueryParam::new("opt", &opt))
            .query(&progenitor_client::QueryParam::new("recurse", &recurse))
            .query(&progenitor_client::QueryParam::new("remote", &remote))
            .query(&progenitor_client::QueryParam::new(
                "showEncrypted",
                &show_encrypted,
            ))
            .query(&progenitor_client::QueryParam::new("showHash", &show_hash))
            .query(&progenitor_client::QueryParam::new(
                "showOrigIDs",
                &show_orig_i_ds,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_list",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Stat an object
    ///
    ///Returns metadata for a single file or directory, mirroring `rclone
    /// lsjson` on one entry.
    ///
    ///Sends a `POST` request to `/operations/stat`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote name or path that contains the item to inspect.
    /// - `opt`: Optional JSON object of listing flags, matching those accepted
    ///   by `operations/list`.
    /// - `remote`: Path to the file or directory within `fs` to describe.
    pub async fn operations_stat<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: &'a str,
        opt: Option<&'a str>,
        remote: &'a str,
    ) -> Result<ResponseValue<types::OperationsStatResponse>, Error<types::RcError>> {
        let url = format!("{}/operations/stat", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("opt", &opt))
            .query(&progenitor_client::QueryParam::new("remote", &remote))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_stat",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get remote quota
    ///
    ///Returns storage quota and usage details for the remote, equivalent to
    /// `rclone about`.
    ///
    ///Sends a `POST` request to `/operations/about`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote name or path to query for capacity information.
    pub async fn operations_about<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: &'a str,
    ) -> Result<ResponseValue<types::OperationsAboutResponse>, Error<types::RcError>> {
        let url = format!("{}/operations/about", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_about",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Upload files via multipart
    ///
    ///Accepts multipart/form-data payloads and writes the uploaded files to
    /// the specified remote path.
    ///
    ///Sends a `POST` request to `/operations/uploadfile`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote name or path where the uploaded file should be stored.
    /// - `remote`: Destination path within `fs` for the uploaded file.
    /// - `body`: Multipart form payload containing one or more files to upload.
    pub async fn operations_uploadfile<'a, B: Into<reqwest::Body>>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: &'a str,
        remote: &'a str,
        body: B,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/operations/uploadfile", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .header(
                ::reqwest::header::CONTENT_TYPE,
                ::reqwest::header::HeaderValue::from_static("application/octet-stream"),
            )
            .body(body)
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("remote", &remote))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_uploadfile",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Purge directory
    ///
    ///Deletes a directory or container and all of its contents.
    ///
    ///Sends a `POST` request to `/operations/purge`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `config`: JSON encoded config overrides applied for this call only.
    /// - `filter`: JSON encoded filter overrides applied for this call only.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote name or path from which to remove all contents.
    /// - `remote`: Path within `fs` whose contents should be purged.
    pub async fn operations_purge<'a>(
        &'a self,
        async_: Option<bool>,
        config: Option<&'a str>,
        filter: Option<&'a str>,
        group: Option<&'a str>,
        fs: &'a str,
        remote: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/operations/purge", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_config", &config))
            .query(&progenitor_client::QueryParam::new("_filter", &filter))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("remote", &remote))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_purge",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create directory
    ///
    ///Creates the target directory or container if it does not exist.
    ///
    ///Sends a `POST` request to `/operations/mkdir`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote name or path in which to create a directory.
    /// - `remote`: Directory path within `fs` to create.
    pub async fn operations_mkdir<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: &'a str,
        remote: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/operations/mkdir", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("remote", &remote))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_mkdir",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove empty directory
    ///
    ///Deletes an empty directory or container.
    ///
    ///Sends a `POST` request to `/operations/rmdir`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Remote name or path containing the directory to remove.
    /// - `remote`: Directory path within `fs` to delete.
    pub async fn operations_rmdir<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: &'a str,
        remote: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/operations/rmdir", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("remote", &remote))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_rmdir",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Compare source and destination
    ///
    ///Compares source and destination trees, reporting matches, differences,
    /// and missing files.
    ///
    ///Sends a `POST` request to `/operations/check`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `check_file_fs`: Remote containing the checksum SUM file when using
    ///   `checkFileHash`.
    /// - `check_file_hash`: Hash name to expect in the supplied SUM file, such
    ///   as `md5`.
    /// - `check_file_remote`: Path within `checkFileFs` to the checksum SUM
    ///   file.
    /// - `combined`: Set to true to include a combined summary report in the
    ///   response.
    /// - `differ`: Set to true to include differing files in the report.
    /// - `download`: Set to true to read file contents during comparison
    ///   instead of relying on hashes.
    /// - `dst_fs`: Destination remote name or path that should match the
    ///   source.
    /// - `error`: Set to true to include entries that encountered errors.
    /// - `match_`: Set to true to include matching files in the report.
    /// - `missing_on_dst`: Set to true to report files missing from the
    ///   destination.
    /// - `missing_on_src`: Set to true to report files missing from the source.
    /// - `one_way`: Set to true to only ensure that source files exist on the
    ///   destination.
    /// - `src_fs`: Source remote name or path to verify, e.g. `drive:`.
    pub async fn operations_check<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        check_file_fs: Option<&'a str>,
        check_file_hash: Option<&'a str>,
        check_file_remote: Option<&'a str>,
        combined: Option<bool>,
        differ: Option<bool>,
        download: Option<bool>,
        dst_fs: &'a str,
        error: Option<bool>,
        match_: Option<bool>,
        missing_on_dst: Option<bool>,
        missing_on_src: Option<bool>,
        one_way: Option<bool>,
        src_fs: &'a str,
    ) -> Result<ResponseValue<types::OperationsCheckResponse>, Error<types::RcError>> {
        let url = format!("{}/operations/check", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new(
                "checkFileFs",
                &check_file_fs,
            ))
            .query(&progenitor_client::QueryParam::new(
                "checkFileHash",
                &check_file_hash,
            ))
            .query(&progenitor_client::QueryParam::new(
                "checkFileRemote",
                &check_file_remote,
            ))
            .query(&progenitor_client::QueryParam::new("combined", &combined))
            .query(&progenitor_client::QueryParam::new("differ", &differ))
            .query(&progenitor_client::QueryParam::new("download", &download))
            .query(&progenitor_client::QueryParam::new("dstFs", &dst_fs))
            .query(&progenitor_client::QueryParam::new("error", &error))
            .query(&progenitor_client::QueryParam::new("match", &match_))
            .query(&progenitor_client::QueryParam::new(
                "missingOnDst",
                &missing_on_dst,
            ))
            .query(&progenitor_client::QueryParam::new(
                "missingOnSrc",
                &missing_on_src,
            ))
            .query(&progenitor_client::QueryParam::new("oneWay", &one_way))
            .query(&progenitor_client::QueryParam::new("srcFs", &src_fs))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_check",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sync source to destination
    ///
    ///Synchronises a source remote to a destination remote, making the
    /// destination match the source.
    ///
    ///Sends a `POST` request to `/sync/sync`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `config`: JSON encoded config overrides applied for this call only.
    /// - `filter`: JSON encoded filter overrides applied for this call only.
    /// - `group`: Assign the request to a custom stats group.
    /// - `create_empty_src_dirs`: Set to true to create empty source
    ///   directories on the destination.
    /// - `dst_fs`: Destination remote path to sync to, e.g. `drive:dst`.
    /// - `src_fs`: Source remote path to sync from, e.g. `drive:src`.
    pub async fn sync_sync<'a>(
        &'a self,
        async_: Option<bool>,
        config: Option<&'a str>,
        filter: Option<&'a str>,
        group: Option<&'a str>,
        create_empty_src_dirs: Option<bool>,
        dst_fs: &'a str,
        src_fs: &'a str,
    ) -> Result<ResponseValue<types::SyncSyncResponse>, Error<types::RcError>> {
        let url = format!("{}/sync/sync", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_config", &config))
            .query(&progenitor_client::QueryParam::new("_filter", &filter))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new(
                "createEmptySrcDirs",
                &create_empty_src_dirs,
            ))
            .query(&progenitor_client::QueryParam::new("dstFs", &dst_fs))
            .query(&progenitor_client::QueryParam::new("srcFs", &src_fs))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "sync_sync",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Copy source to destination
    ///
    ///Copies objects from a source remote to a destination remote without
    /// deleting destination files.
    ///
    ///Sends a `POST` request to `/sync/copy`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `config`: JSON encoded config overrides applied for this call only.
    /// - `filter`: JSON encoded filter overrides applied for this call only.
    /// - `group`: Assign the request to a custom stats group.
    /// - `create_empty_src_dirs`: Set to true to replicate empty source
    ///   directories on the destination.
    /// - `dst_fs`: Destination remote path to copy to.
    /// - `src_fs`: Source remote path to copy from.
    pub async fn sync_copy<'a>(
        &'a self,
        async_: Option<bool>,
        config: Option<&'a str>,
        filter: Option<&'a str>,
        group: Option<&'a str>,
        create_empty_src_dirs: Option<bool>,
        dst_fs: &'a str,
        src_fs: &'a str,
    ) -> Result<ResponseValue<types::SyncCopyResponse>, Error<types::RcError>> {
        let url = format!("{}/sync/copy", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_config", &config))
            .query(&progenitor_client::QueryParam::new("_filter", &filter))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new(
                "createEmptySrcDirs",
                &create_empty_src_dirs,
            ))
            .query(&progenitor_client::QueryParam::new("dstFs", &dst_fs))
            .query(&progenitor_client::QueryParam::new("srcFs", &src_fs))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "sync_copy",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Move source to destination
    ///
    ///Moves objects from a source remote to a destination remote, optionally
    /// cleaning up empty directories.
    ///
    ///Sends a `POST` request to `/sync/move`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `config`: JSON encoded config overrides applied for this call only.
    /// - `filter`: JSON encoded filter overrides applied for this call only.
    /// - `group`: Assign the request to a custom stats group.
    /// - `create_empty_src_dirs`: Set to true to create empty source
    ///   directories on the destination.
    /// - `delete_empty_src_dirs`: Set to true to delete empty directories from
    ///   the source after the move completes.
    /// - `dst_fs`: Destination remote path that will receive moved files.
    /// - `src_fs`: Source remote path whose contents will be moved.
    pub async fn sync_move<'a>(
        &'a self,
        async_: Option<bool>,
        config: Option<&'a str>,
        filter: Option<&'a str>,
        group: Option<&'a str>,
        create_empty_src_dirs: Option<bool>,
        delete_empty_src_dirs: Option<bool>,
        dst_fs: &'a str,
        src_fs: &'a str,
    ) -> Result<ResponseValue<types::SyncMoveResponse>, Error<types::RcError>> {
        let url = format!("{}/sync/move", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_config", &config))
            .query(&progenitor_client::QueryParam::new("_filter", &filter))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new(
                "createEmptySrcDirs",
                &create_empty_src_dirs,
            ))
            .query(&progenitor_client::QueryParam::new(
                "deleteEmptySrcDirs",
                &delete_empty_src_dirs,
            ))
            .query(&progenitor_client::QueryParam::new("dstFs", &dst_fs))
            .query(&progenitor_client::QueryParam::new("srcFs", &src_fs))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "sync_move",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Bidirectional sync
    ///
    ///Performs a bidirectional synchronisation between two paths, supporting
    /// safety checks and recovery options.
    ///
    ///Sends a `POST` request to `/sync/bisync`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `config`: JSON encoded config overrides applied for this call only.
    /// - `filter`: JSON encoded filter overrides applied for this call only.
    /// - `group`: Assign the request to a custom stats group.
    /// - `backupdir1`: Backup directory on the first remote for changed files.
    /// - `backupdir2`: Backup directory on the second remote for changed files.
    /// - `check_access`: Set to true to abort if `RCLONE_TEST` files are
    ///   missing on either side.
    /// - `check_filename`: Override the access-check sentinel filename;
    ///   defaults to `RCLONE_TEST`.
    /// - `check_sync`: Controls final listing comparison; leave true for normal
    ///   verification or set false to skip.
    /// - `create_empty_src_dirs`: Set to true to mirror empty directories
    ///   between the two paths.
    /// - `dry_run`: Set to true to simulate the bisync run without making
    ///   changes.
    /// - `filters_file`: Path to an rclone filters file applied to both paths.
    /// - `force`: Set to true to bypass the `maxDelete` safety check.
    /// - `ignore_listing_checksum`: Set to true to ignore checksum differences
    ///   when comparing listings.
    /// - `max_delete`: Abort the run if deletions exceed this percentage
    ///   (default 50).
    /// - `no_cleanup`: Set to true to keep bisync working files after
    ///   completion.
    /// - `path1`: First remote directory, e.g. `drive:path1`.
    /// - `path2`: Second remote directory, e.g. `drive:path2`.
    /// - `remove_empty_dirs`: Set to true to remove empty directories during
    ///   cleanup.
    /// - `resilient`: Set to true to allow retrying after certain recoverable
    ///   errors.
    /// - `resync`: Set to true to perform a one-time resync, rebuilding bisync
    ///   history.
    /// - `workdir`: Directory path used to store bisync working files.
    pub async fn sync_bisync<'a>(
        &'a self,
        async_: Option<bool>,
        config: Option<&'a str>,
        filter: Option<&'a str>,
        group: Option<&'a str>,
        backupdir1: Option<&'a str>,
        backupdir2: Option<&'a str>,
        check_access: Option<bool>,
        check_filename: Option<&'a str>,
        check_sync: Option<bool>,
        create_empty_src_dirs: Option<bool>,
        dry_run: Option<bool>,
        filters_file: Option<&'a str>,
        force: Option<bool>,
        ignore_listing_checksum: Option<bool>,
        max_delete: Option<f64>,
        no_cleanup: Option<bool>,
        path1: &'a str,
        path2: &'a str,
        remove_empty_dirs: Option<bool>,
        resilient: Option<bool>,
        resync: Option<bool>,
        workdir: Option<&'a str>,
    ) -> Result<ResponseValue<types::SyncBisyncResponse>, Error<types::RcError>> {
        let url = format!("{}/sync/bisync", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_config", &config))
            .query(&progenitor_client::QueryParam::new("_filter", &filter))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new(
                "backupdir1",
                &backupdir1,
            ))
            .query(&progenitor_client::QueryParam::new(
                "backupdir2",
                &backupdir2,
            ))
            .query(&progenitor_client::QueryParam::new(
                "checkAccess",
                &check_access,
            ))
            .query(&progenitor_client::QueryParam::new(
                "checkFilename",
                &check_filename,
            ))
            .query(&progenitor_client::QueryParam::new(
                "checkSync",
                &check_sync,
            ))
            .query(&progenitor_client::QueryParam::new(
                "createEmptySrcDirs",
                &create_empty_src_dirs,
            ))
            .query(&progenitor_client::QueryParam::new("dryRun", &dry_run))
            .query(&progenitor_client::QueryParam::new(
                "filtersFile",
                &filters_file,
            ))
            .query(&progenitor_client::QueryParam::new("force", &force))
            .query(&progenitor_client::QueryParam::new(
                "ignoreListingChecksum",
                &ignore_listing_checksum,
            ))
            .query(&progenitor_client::QueryParam::new(
                "maxDelete",
                &max_delete,
            ))
            .query(&progenitor_client::QueryParam::new(
                "noCleanup",
                &no_cleanup,
            ))
            .query(&progenitor_client::QueryParam::new("path1", &path1))
            .query(&progenitor_client::QueryParam::new("path2", &path2))
            .query(&progenitor_client::QueryParam::new(
                "removeEmptyDirs",
                &remove_empty_dirs,
            ))
            .query(&progenitor_client::QueryParam::new("resilient", &resilient))
            .query(&progenitor_client::QueryParam::new("resync", &resync))
            .query(&progenitor_client::QueryParam::new("workdir", &workdir))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "sync_bisync",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List option blocks
    ///
    ///Returns the names of option blocks that can be queried or updated.
    ///
    ///Sends a `POST` request to `/options/blocks`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn options_blocks<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::OptionsBlocksResponse>, Error<types::RcError>> {
        let url = format!("{}/options/blocks", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "options_blocks",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get option values
    ///
    ///Returns the current global option values, optionally filtered by block.
    ///
    ///Sends a `POST` request to `/options/get`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `blocks`: Optional comma-separated list of option block names to
    ///   return.
    pub async fn options_get<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        blocks: Option<&'a str>,
    ) -> Result<ResponseValue<types::OptionsGetResponse>, Error<types::RcError>> {
        let url = format!("{}/options/get", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("blocks", &blocks))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "options_get",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Describe options
    ///
    ///Returns metadata for options, including help text and defaults, grouped
    /// by block.
    ///
    ///Sends a `POST` request to `/options/info`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `blocks`: Optional comma-separated list of option block names to
    ///   describe.
    pub async fn options_info<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        blocks: Option<&'a str>,
    ) -> Result<ResponseValue<types::OptionsInfoResponse>, Error<types::RcError>> {
        let url = format!("{}/options/info", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("blocks", &blocks))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "options_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Set option values
    ///
    ///Sets temporary option overrides for the running process by supplying
    /// key/value pairs grouped under option block names. Provide one or more
    /// query parameters whose names match the blocks you want to modify (for
    /// example `main`, `rc`, `http`). Each block parameter carries an object of
    /// option overrides.
    ///
    ///
    ///Sends a `POST` request to `/options/set`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `dlna`: Overrides for the `dlna` option block.
    /// - `filter`: Overrides for the `filter` option block.
    /// - `ftp`: Overrides for the `ftp` option block.
    /// - `http`: Overrides for the `http` option block.
    /// - `log`: Overrides for the `log` option block.
    /// - `main`: Overrides for the `main` option block.
    /// - `mount`: Overrides for the `mount` option block.
    /// - `nfs`: Overrides for the `nfs` option block.
    /// - `proxy`: Overrides for the `proxy` option block.
    /// - `rc`: Overrides for the `rc` option block.
    /// - `restic`: Overrides for the `restic` option block.
    /// - `s3`: Overrides for the `s3` option block.
    /// - `sftp`: Overrides for the `sftp` option block.
    /// - `vfs`: Overrides for the `vfs` option block.
    /// - `webdav`: Overrides for the `webdav` option block.
    pub async fn options_set<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        dlna: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetDlnaValue>,
        >,
        filter: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetFilterValue>,
        >,
        ftp: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetFtpValue>,
        >,
        http: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetHttpValue>,
        >,
        log: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetLogValue>,
        >,
        main: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetMainValue>,
        >,
        mount: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetMountValue>,
        >,
        nfs: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetNfsValue>,
        >,
        proxy: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetProxyValue>,
        >,
        rc: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetRcValue>,
        >,
        restic: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetResticValue>,
        >,
        s3: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetS3Value>,
        >,
        sftp: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetSftpValue>,
        >,
        vfs: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetVfsValue>,
        >,
        webdav: Option<
            &'a ::std::collections::HashMap<::std::string::String, types::OptionsSetWebdavValue>,
        >,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/options/set", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("dlna", &dlna))
            .query(&progenitor_client::QueryParam::new("filter", &filter))
            .query(&progenitor_client::QueryParam::new("ftp", &ftp))
            .query(&progenitor_client::QueryParam::new("http", &http))
            .query(&progenitor_client::QueryParam::new("log", &log))
            .query(&progenitor_client::QueryParam::new("main", &main))
            .query(&progenitor_client::QueryParam::new("mount", &mount))
            .query(&progenitor_client::QueryParam::new("nfs", &nfs))
            .query(&progenitor_client::QueryParam::new("proxy", &proxy))
            .query(&progenitor_client::QueryParam::new("rc", &rc))
            .query(&progenitor_client::QueryParam::new("restic", &restic))
            .query(&progenitor_client::QueryParam::new("s3", &s3))
            .query(&progenitor_client::QueryParam::new("sftp", &sftp))
            .query(&progenitor_client::QueryParam::new("vfs", &vfs))
            .query(&progenitor_client::QueryParam::new("webdav", &webdav))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "options_set",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Show effective options
    ///
    ///Returns the current effective options for this request, including
    /// `_config` and `_filter` overrides.
    ///
    ///Sends a `POST` request to `/options/local`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn options_local<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::OptionsLocalResponse>, Error<types::RcError>> {
        let url = format!("{}/options/local", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "options_local",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List serve instances
    ///
    ///Returns all running `rclone serve` instances with their IDs and options.
    ///
    ///Sends a `POST` request to `/serve/list`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn serve_list<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::ServeListResponse>, Error<types::RcError>> {
        let url = format!("{}/serve/list", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "serve_list",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Start serve instance
    ///
    ///Launches a new `rclone serve` endpoint (http, webdav, ftp, etc.) with
    /// the provided parameters.
    ///
    ///Sends a `POST` request to `/serve/start`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `config`: JSON encoded config overrides applied for this call only.
    /// - `filter`: JSON encoded filter overrides applied for this call only.
    /// - `group`: Assign the request to a custom stats group.
    /// - `addr`: Address and port to bind the server to, such as `:5572` or
    ///   `localhost:8080`.
    /// - `fs`: Remote path that will be served.
    /// - `params`: Additional arbitrary parameters allowed.
    /// - `type_`: Type of server to start (e.g. `http`, `webdav`, `ftp`,
    ///   `sftp`).
    pub async fn serve_start<'a>(
        &'a self,
        async_: Option<bool>,
        config: Option<&'a str>,
        filter: Option<&'a str>,
        group: Option<&'a str>,
        addr: &'a str,
        fs: &'a str,
        params: Option<&'a ::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        type_: &'a str,
    ) -> Result<ResponseValue<types::ServeStartResponse>, Error<types::RcError>> {
        let url = format!("{}/serve/start", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_config", &config))
            .query(&progenitor_client::QueryParam::new("_filter", &filter))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("addr", &addr))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("params", &params))
            .query(&progenitor_client::QueryParam::new("type", &type_))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "serve_start",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Stop serve instance
    ///
    ///Stops a running `serve` instance identified by its ID.
    ///
    ///Sends a `POST` request to `/serve/stop`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `id`: Identifier of the running serve instance returned by
    ///   `serve/start`.
    pub async fn serve_stop<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        id: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/serve/stop", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "serve_stop",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Stop all serve instances
    ///
    ///Stops every active `serve` instance.
    ///
    ///Sends a `POST` request to `/serve/stopall`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn serve_stopall<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/serve/stopall", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "serve_stopall",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List serve types
    ///
    ///Returns the list of supported `rclone serve` protocols.
    ///
    ///Sends a `POST` request to `/serve/types`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn serve_types<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<types::RcError>> {
        let url = format!("{}/serve/types", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "serve_types",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Forget cached paths
    ///
    ///Evicts specific files or directories from the VFS directory cache.
    ///
    ///Sends a `POST` request to `/vfs/forget`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Optional VFS identifier to target; required when more than one
    ///   VFS is active.
    /// - `params`: Additional arbitrary parameters allowed.
    pub async fn vfs_forget<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: Option<&'a str>,
        params: Option<&'a ::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    ) -> Result<ResponseValue<types::VfsForgetResponse>, Error<types::RcError>> {
        let url = format!("{}/vfs/forget", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("params", &params))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vfs_forget",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List VFS instances
    ///
    ///Lists the active VFS instances and their identifiers.
    ///
    ///Sends a `POST` request to `/vfs/list`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Optional VFS identifier; omit to list all active VFS instances.
    pub async fn vfs_list<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: Option<&'a str>,
    ) -> Result<ResponseValue<types::VfsListResponse>, Error<types::RcError>> {
        let url = format!("{}/vfs/list", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vfs_list",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get or set poll interval
    ///
    ///Reads or updates the VFS poll interval duration, optionally waiting for
    /// the change to apply.
    ///
    ///Sends a `POST` request to `/vfs/poll-interval`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Optional VFS identifier whose poll interval should be queried or
    ///   modified.
    /// - `interval`: Duration string (e.g. `5m`) to set as the new poll
    ///   interval.
    /// - `timeout`: Duration to wait for the poll interval change to take
    ///   effect; `0` waits indefinitely.
    pub async fn vfs_poll_interval<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: Option<&'a str>,
        interval: Option<&'a str>,
        timeout: Option<&'a str>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/vfs/poll-interval", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("interval", &interval))
            .query(&progenitor_client::QueryParam::new("timeout", &timeout))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vfs_poll_interval",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Inspect upload queue
    ///
    ///Returns the contents of the VFS upload queue.
    ///
    ///Sends a `POST` request to `/vfs/queue`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Optional VFS identifier whose upload queue should be inspected.
    pub async fn vfs_queue<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: Option<&'a str>,
    ) -> Result<ResponseValue<types::VfsQueueResponse>, Error<types::RcError>> {
        let url = format!("{}/vfs/queue", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vfs_queue",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Adjust queue expiry
    ///
    ///Sets the expiry time of a queued VFS upload item, optionally relative to
    /// its current value.
    ///
    ///Sends a `POST` request to `/vfs/queue-set-expiry`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `expiry`: New eligibility time in seconds (may be negative for
    ///   immediate upload).
    /// - `fs`: Optional VFS identifier for the queued item.
    /// - `id`: Queue item ID as returned by `vfs/queue`.
    /// - `relative`: Set to true to treat `expiry` as relative to the current
    ///   value.
    pub async fn vfs_queue_set_expiry<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        expiry: f64,
        fs: Option<&'a str>,
        id: i64,
        relative: Option<bool>,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/vfs/queue-set-expiry", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("expiry", &expiry))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("id", &id))
            .query(&progenitor_client::QueryParam::new("relative", &relative))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vfs_queue_set_expiry",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Refresh directory cache
    ///
    ///Refreshes one or more directories in the VFS cache, optionally
    /// recursively.
    ///
    ///Sends a `POST` request to `/vfs/refresh`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Optional VFS identifier whose directory cache should be
    ///   refreshed.
    /// - `params`: Additional arbitrary parameters allowed.
    /// - `recursive`: Set to true to refresh entire directory trees.
    pub async fn vfs_refresh<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: Option<&'a str>,
        params: Option<&'a ::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        recursive: Option<bool>,
    ) -> Result<ResponseValue<types::VfsRefreshResponse>, Error<types::RcError>> {
        let url = format!("{}/vfs/refresh", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .query(&progenitor_client::QueryParam::new("params", &params))
            .query(&progenitor_client::QueryParam::new("recursive", &recursive))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vfs_refresh",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Show VFS stats
    ///
    ///Returns VFS statistics including disk cache usage and metadata cache
    /// counters.
    ///
    ///Sends a `POST` request to `/vfs/stats`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `fs`: Optional VFS identifier whose statistics should be returned.
    pub async fn vfs_stats<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        fs: Option<&'a str>,
    ) -> Result<ResponseValue<types::VfsStatsResponse>, Error<types::RcError>> {
        let url = format!("{}/vfs/stats", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("fs", &fs))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "vfs_stats",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Install plugin
    ///
    ///Downloads and installs a plugin into the WebUI from the provided
    /// repository URL.
    ///
    ///Sends a `POST` request to `/pluginsctl/addPlugin`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `url`: Repository URL of the plugin to install.
    pub async fn pluginsctl_add_plugin<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        url: &'a str,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let _url = format!("{}/pluginsctl/addPlugin", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(_url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("url", &url))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "pluginsctl_add_plugin",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Filter plugins by MIME type
    ///
    ///Returns plugins matching the requested MIME type and optional plugin
    /// type.
    ///
    ///Sends a `POST` request to `/pluginsctl/getPluginsForType`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `plugin_type`: Filter results by plugin type (e.g. `test`).
    /// - `type_`: MIME type to match when listing plugins.
    pub async fn pluginsctl_get_plugins_for_type<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        plugin_type: Option<&'a str>,
        type_: Option<&'a str>,
    ) -> Result<ResponseValue<types::PluginsctlGetPluginsForTypeResponse>, Error<types::RcError>>
    {
        let url = format!("{}/pluginsctl/getPluginsForType", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new(
                "pluginType",
                &plugin_type,
            ))
            .query(&progenitor_client::QueryParam::new("type", &type_))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "pluginsctl_get_plugins_for_type",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List installed plugins
    ///
    ///Returns metadata for installed production and test plugins.
    ///
    ///Sends a `POST` request to `/pluginsctl/listPlugins`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn pluginsctl_list_plugins<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::PluginsctlListPluginsResponse>, Error<types::RcError>> {
        let url = format!("{}/pluginsctl/listPlugins", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "pluginsctl_list_plugins",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List installed test plugins
    ///
    ///Returns metadata for installed test plugins.
    ///
    ///Sends a `POST` request to `/pluginsctl/listTestPlugins`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    pub async fn pluginsctl_list_test_plugins<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
    ) -> Result<ResponseValue<types::PluginsctlListTestPluginsResponse>, Error<types::RcError>>
    {
        let url = format!("{}/pluginsctl/listTestPlugins", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "pluginsctl_list_test_plugins",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove plugin
    ///
    ///Uninstalls a plugin from the WebUI.
    ///
    ///Sends a `POST` request to `/pluginsctl/removePlugin`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `name`: Name of the plugin to uninstall.
    pub async fn pluginsctl_remove_plugin<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::RcError>> {
        let url = format!("{}/pluginsctl/removePlugin", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "pluginsctl_remove_plugin",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove test plugin
    ///
    ///Uninstalls a test plugin from the WebUI.
    ///
    ///Sends a `POST` request to `/pluginsctl/removeTestPlugin`
    ///
    ///Arguments:
    /// - `async_`: Run the command asynchronously. Returns a job id
    ///   immediately.
    /// - `group`: Assign the request to a custom stats group.
    /// - `name`: Name of the test plugin to uninstall.
    pub async fn pluginsctl_remove_test_plugin<'a>(
        &'a self,
        async_: Option<bool>,
        group: Option<&'a str>,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::RcError>> {
        let url = format!("{}/pluginsctl/removeTestPlugin", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("_async", &async_))
            .query(&progenitor_client::QueryParam::new("_group", &group))
            .query(&progenitor_client::QueryParam::new("name", &name))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "pluginsctl_remove_test_plugin",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
