#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    ///`BackendCommandBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BackendCommandBody {
        ///Arguments passed to the backend command.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub arg: ::std::vec::Vec<::std::string::String>,
        ///Backend-specific command to invoke.
        pub command: ::std::string::String,
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path the backend command should target.
        pub fs: BackendCommandBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Options passed to the backend command.
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub opt: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    }

    ///Remote name or path the backend command should target.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum BackendCommandBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for BackendCommandBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`BackendCommandResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BackendCommandResponse {
        ///Backend command result payload
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<::serde_json::Value>,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`CacheExpireBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CacheExpireBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Remote path to expire from the cache, e.g. `remote:path/to/dir`.
        pub remote: ::std::string::String,
        ///Set this to drop cached chunk data along with directory entries.
        /// Rclone only checks whether the key is present, so any value turns it
        /// on, including `false`.
        #[serde(
            rename = "withData",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub with_data: ::std::option::Option<bool>,
    }

    ///`CacheExpireResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CacheExpireResponse {
        ///Describes what was cleared.
        pub message: ::std::string::String,
        ///Always `ok`.
        pub status: ::std::string::String,
    }

    ///`CacheFetchBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CacheFetchBody {
        ///Comma-separated chunk specifier list (e.g. `0:10,25:30`) describing
        /// file pieces to prefetch.
        pub chunks: ::std::string::String,
        ///File to fetch chunks of. Add more with keys that start with `file`,
        /// such as `file2`.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub file: ::std::option::Option<::std::string::String>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        #[serde(flatten)]
        pub extra: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    }

    ///`CacheFetchResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CacheFetchResponse {
        ///Results keyed by file name.
        pub status:
            ::std::collections::HashMap<::std::string::String, CacheFetchResponseStatusValue>,
    }

    ///`CacheFetchResponseStatusValue`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CacheFetchResponseStatusValue {
        ///Error message, or an empty string.
        #[serde(rename = "Error")]
        pub error: ::std::string::String,
        ///Number of chunks fetched.
        #[serde(rename = "FetchedChunks")]
        pub fetched_chunks: i64,
    }

    ///`CacheStatsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CacheStatsResponse {
        ///Cache statistics.
        pub stats: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Always `ok`.
        pub status: ::std::string::String,
    }

    ///`ConfigCreateBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigCreateBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Name of the new remote configuration.
        pub name: ::std::string::String,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub opt: ::std::option::Option<ConfigCreateBodyOpt>,
        ///Backend options for the remote.
        pub parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Backend type identifier, such as `drive`, `s3`, or `dropbox`.
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }

    ///Options that control the configuration.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct ConfigCreateBodyOpt {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub all: ::std::option::Option<bool>,
        #[serde(
            rename = "continue",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub continue_: ::std::option::Option<bool>,
        #[serde(
            rename = "noObscure",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_obscure: ::std::option::Option<bool>,
        #[serde(
            rename = "noOutput",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_output: ::std::option::Option<bool>,
        #[serde(
            rename = "nonInteractive",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub non_interactive: ::std::option::Option<bool>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub obscure: ::std::option::Option<bool>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<::std::string::String>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<::std::string::String>,
    }

    ///`ConfigCreateResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct ConfigCreateResponse {
        ///Error to show to the user.
        #[serde(
            rename = "Error",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Option",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub option: ::std::option::Option<ConfigProviderOption>,
        ///Result to send back to continue.
        #[serde(
            rename = "Result",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub result: ::std::option::Option<::std::string::String>,
        ///State to send back to continue.
        #[serde(
            rename = "State",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<::std::string::String>,
    }

    ///`ConfigDeleteBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigDeleteBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Name of the remote configuration to delete.
        pub name: ::std::string::String,
    }

    ///`ConfigGetBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigGetBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Name of the remote configuration to fetch.
        pub name: ::std::string::String,
    }

    ///`ConfigGetResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigGetResponse {
        #[serde(
            rename = "type",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
        #[serde(flatten)]
        pub extra: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    }

    ///`ConfigListremotesResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigListremotesResponse {
        pub remotes: ::std::vec::Vec<::std::string::String>,
    }

    ///`ConfigOauthstatusResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigOauthstatusResponse {
        ///Authorization URL to open in a browser. Present only when status is
        /// "running".
        #[serde(
            rename = "authUrl",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub auth_url: ::std::option::Option<::std::string::String>,
        ///Whether the OAuth authentication server is currently running.
        pub status: ConfigOauthstatusResponseStatus,
    }

    ///Whether the OAuth authentication server is currently running.
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
    pub enum ConfigOauthstatusResponseStatus {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "stopped")]
        Stopped,
    }

    impl ::std::fmt::Display for ConfigOauthstatusResponseStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Stopped => f.write_str("stopped"),
            }
        }
    }

    impl ::std::str::FromStr for ConfigOauthstatusResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "stopped" => Ok(Self::Stopped),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ConfigOauthstatusResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ConfigOauthstatusResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`ConfigPasswordBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigPasswordBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Name of the remote whose secrets should be updated.
        pub name: ::std::string::String,
        ///Password options for the remote.
        pub parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`ConfigPathsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigPathsResponse {
        pub cache: ::std::string::String,
        pub config: ::std::string::String,
        pub temp: ::std::string::String,
    }

    ///`ConfigProvider`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigProvider {
        #[serde(
            rename = "Aliases",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "CommandHelp",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub command_help: ::std::option::Option<::std::vec::Vec<ConfigProviderCommandHelp>>,
        #[serde(rename = "Description")]
        pub description: ::std::string::String,
        #[serde(
            rename = "Hide",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub hide: ::std::option::Option<bool>,
        #[serde(
            rename = "MetadataInfo",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub metadata_info: ::std::option::Option<ConfigProviderMetadataInfo>,
        #[serde(rename = "Name")]
        pub name: ::std::string::String,
        #[serde(rename = "Options")]
        pub options: ::std::vec::Vec<ConfigProviderOption>,
        #[serde(
            rename = "Overview",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub overview: ::std::option::Option<ConfigProviderOverview>,
        #[serde(rename = "Prefix")]
        pub prefix: ::std::string::String,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`ConfigProviderCommandHelp`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigProviderCommandHelp {
        #[serde(
            rename = "Long",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub long: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Name",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Opts",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub opts: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
        #[serde(
            rename = "Short",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short: ::std::option::Option<::std::string::String>,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`ConfigProviderMetadataInfo`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigProviderMetadataInfo {
        #[serde(
            rename = "Help",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub help: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "System",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub system: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ConfigProviderMetadataSystemEntry>,
        >,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`ConfigProviderMetadataSystemEntry`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigProviderMetadataSystemEntry {
        #[serde(
            rename = "Example",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub example: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Help",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub help: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "ReadOnly",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub read_only: ::std::option::Option<bool>,
        #[serde(
            rename = "Type",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`ConfigProviderOption`
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
        pub hide: i64,
        #[serde(rename = "IsPassword")]
        pub is_password: bool,
        #[serde(rename = "Name")]
        pub name: ::std::string::String,
        #[serde(rename = "NoPrefix")]
        pub no_prefix: bool,
        #[serde(
            rename = "Provider",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub provider: ::std::option::Option<::std::string::String>,
        #[serde(rename = "Required")]
        pub required: bool,
        #[serde(rename = "Sensitive")]
        pub sensitive: bool,
        #[serde(
            rename = "ShortOpt",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_opt: ::std::option::Option<::std::string::String>,
        #[serde(rename = "Type")]
        pub type_: ConfigProviderOptionType,
        #[serde(rename = "Value")]
        pub value: ConfigProviderOptionAny,
        #[serde(rename = "ValueStr")]
        pub value_str: ::std::string::String,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`ConfigProviderOptionAny`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct ConfigProviderOptionAny(pub ::serde_json::Value);
    impl ::std::ops::Deref for ConfigProviderOptionAny {
        type Target = ::serde_json::Value;
        fn deref(&self) -> &::serde_json::Value {
            &self.0
        }
    }

    impl ::std::convert::From<ConfigProviderOptionAny> for ::serde_json::Value {
        fn from(value: ConfigProviderOptionAny) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::serde_json::Value> for ConfigProviderOptionAny {
        fn from(value: ::serde_json::Value) -> Self {
            Self(value)
        }
    }

    ///`ConfigProviderOptionExample`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigProviderOptionExample {
        #[serde(rename = "Help")]
        pub help: ::std::string::String,
        #[serde(
            rename = "Provider",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub provider: ::std::option::Option<::std::string::String>,
        #[serde(rename = "Value")]
        pub value: ::std::string::String,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`ConfigProviderOptionType`
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
        #[serde(rename = "float64")]
        Float64,
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
                Self::Float64 => f.write_str("float64"),
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
                "float64" => Ok(Self::Float64),
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

    impl ::std::convert::TryFrom<::std::string::String> for ConfigProviderOptionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Summary of the backend from rclone's docs. Its fields are empty for
    /// backends without one.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct ConfigProviderOverview {
        #[serde(
            rename = "Adoption",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub adoption: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Backend",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub backend: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "DataIntegrity",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub data_integrity: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Docs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub docs: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Features",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub features: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "FeaturesScore",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub features_score: ::std::option::Option<i64>,
        #[serde(
            rename = "Hashes",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub hashes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "IntegrationTests",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub integration_tests: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Maintainers",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub maintainers: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Name",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Performance",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub performance: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Precision",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub precision: ::std::option::Option<i64>,
        #[serde(
            rename = "Remote",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub remote: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Security",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub security: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Tier",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tier: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Virtual",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub virtual_: ::std::option::Option<bool>,
    }

    ///`ConfigProvidersResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigProvidersResponse {
        pub providers: ::std::vec::Vec<ConfigProvider>,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`ConfigSetpathBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigSetpathBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Absolute path to the `rclone.conf` file that rclone should use.
        pub path: ::std::string::String,
    }

    ///`ConfigUnlockBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigUnlockBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Password used to unlock an encrypted config file.
        #[serde(rename = "configPassword")]
        pub config_password: ::std::string::String,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`ConfigUnsetBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigUnsetBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Key names to remove.
        pub keys: ::std::vec::Vec<::std::string::String>,
        ///Name of the remote.
        pub name: ::std::string::String,
    }

    ///`ConfigUnsetResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigUnsetResponse {
        ///The keys that were removed, or null if none were.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub removed: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }

    ///`ConfigUpdateBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ConfigUpdateBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Name of the remote configuration to update.
        pub name: ::std::string::String,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub opt: ::std::option::Option<ConfigUpdateBodyOpt>,
        ///Backend options for the remote.
        pub parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///Options that control the configuration.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct ConfigUpdateBodyOpt {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub all: ::std::option::Option<bool>,
        #[serde(
            rename = "continue",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub continue_: ::std::option::Option<bool>,
        #[serde(
            rename = "noObscure",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_obscure: ::std::option::Option<bool>,
        #[serde(
            rename = "noOutput",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_output: ::std::option::Option<bool>,
        #[serde(
            rename = "nonInteractive",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub non_interactive: ::std::option::Option<bool>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub obscure: ::std::option::Option<bool>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<::std::string::String>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<::std::string::String>,
    }

    ///`ConfigUpdateResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct ConfigUpdateResponse {
        ///Error to show to the user.
        #[serde(
            rename = "Error",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Option",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub option: ::std::option::Option<ConfigProviderOption>,
        ///Result to send back to continue.
        #[serde(
            rename = "Result",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub result: ::std::option::Option<::std::string::String>,
        ///State to send back to continue.
        #[serde(
            rename = "State",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<::std::string::String>,
    }

    ///`CoreBwlimitBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct CoreBwlimitBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Bandwidth limit to apply, for example `off`, `5M`, or a schedule
        /// string.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub rate: ::std::option::Option<::std::string::String>,
    }

    ///`CoreBwlimitResponse`
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

    ///`CoreCommandBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct CoreCommandBody {
        ///Arguments passed to the command.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub arg: ::std::vec::Vec<::std::string::String>,
        ///Name of the rclone command to execute, for example `ls` or `lsf`.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub command: ::std::option::Option<::std::string::String>,
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Flags passed to the command.
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub opt: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        #[serde(
            rename = "returnType",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub return_type: ::std::option::Option<CoreCommandBodyReturnType>,
    }

    ///`CoreCommandBodyReturnType`
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
    pub enum CoreCommandBodyReturnType {
        #[serde(rename = "COMBINED_OUTPUT")]
        CombinedOutput,
        #[serde(rename = "STREAM")]
        Stream,
        #[serde(rename = "STREAM_ONLY_STDOUT")]
        StreamOnlyStdout,
        #[serde(rename = "STREAM_ONLY_STDERR")]
        StreamOnlyStderr,
    }

    impl ::std::fmt::Display for CoreCommandBodyReturnType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::CombinedOutput => f.write_str("COMBINED_OUTPUT"),
                Self::Stream => f.write_str("STREAM"),
                Self::StreamOnlyStdout => f.write_str("STREAM_ONLY_STDOUT"),
                Self::StreamOnlyStderr => f.write_str("STREAM_ONLY_STDERR"),
            }
        }
    }

    impl ::std::str::FromStr for CoreCommandBodyReturnType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "COMBINED_OUTPUT" => Ok(Self::CombinedOutput),
                "STREAM" => Ok(Self::Stream),
                "STREAM_ONLY_STDOUT" => Ok(Self::StreamOnlyStdout),
                "STREAM_ONLY_STDERR" => Ok(Self::StreamOnlyStderr),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for CoreCommandBodyReturnType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CoreCommandBodyReturnType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`CoreCommandResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreCommandResponse {
        pub error: bool,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub result: ::std::option::Option<::std::string::String>,
    }

    ///`CoreDisksResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreDisksResponse {
        ///Accessible local paths such as disk mount points, user home folders,
        /// and removable volumes.
        pub disks: ::std::vec::Vec<::std::string::String>,
    }

    ///`CoreDuBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct CoreDuBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Local directory path to report disk usage for. Defaults to the
        /// rclone cache directory when omitted.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub dir: ::std::option::Option<::std::string::String>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`CoreDuResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreDuResponse {
        pub dir: ::std::string::String,
        pub info: CoreDuResponseInfo,
    }

    ///`CoreDuResponseInfo`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreDuResponseInfo {
        #[serde(rename = "Available")]
        pub available: i64,
        #[serde(rename = "Free")]
        pub free: i64,
        #[serde(rename = "Total")]
        pub total: i64,
    }

    ///`CoreGroupListResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreGroupListResponse {
        ///Names of the stats groups, or null when there are none.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }

    ///`CoreMemstatsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreMemstatsResponse {
        #[serde(rename = "Alloc")]
        pub alloc: i64,
        #[serde(rename = "BuckHashSys")]
        pub buck_hash_sys: i64,
        #[serde(rename = "Frees")]
        pub frees: i64,
        #[serde(rename = "GCSys")]
        pub gc_sys: i64,
        #[serde(rename = "HeapAlloc")]
        pub heap_alloc: i64,
        #[serde(rename = "HeapIdle")]
        pub heap_idle: i64,
        #[serde(rename = "HeapInuse")]
        pub heap_inuse: i64,
        #[serde(rename = "HeapObjects")]
        pub heap_objects: i64,
        #[serde(rename = "HeapReleased")]
        pub heap_released: i64,
        #[serde(rename = "HeapSys")]
        pub heap_sys: i64,
        #[serde(rename = "MCacheInuse")]
        pub m_cache_inuse: i64,
        #[serde(rename = "MCacheSys")]
        pub m_cache_sys: i64,
        #[serde(rename = "MSpanInuse")]
        pub m_span_inuse: i64,
        #[serde(rename = "MSpanSys")]
        pub m_span_sys: i64,
        #[serde(rename = "Mallocs")]
        pub mallocs: i64,
        #[serde(rename = "OtherSys")]
        pub other_sys: i64,
        #[serde(rename = "StackInuse")]
        pub stack_inuse: i64,
        #[serde(rename = "StackSys")]
        pub stack_sys: i64,
        #[serde(rename = "Sys")]
        pub sys: i64,
        #[serde(rename = "TotalAlloc")]
        pub total_alloc: i64,
    }

    ///`CoreObscureBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreObscureBody {
        ///Plain-text string to obscure for storage in the config file.
        pub clear: ::std::string::String,
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`CoreObscureResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreObscureResponse {
        pub obscured: ::std::string::String,
    }

    ///`CorePidResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CorePidResponse {
        pub pid: i64,
    }

    ///`CoreQuitBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct CoreQuitBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Optional exit code to use when terminating the rclone process.
        #[serde(
            rename = "exitCode",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub exit_code: ::std::option::Option<i64>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`CoreStatsBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct CoreStatsBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group_: ::std::option::Option<::std::string::String>,
        ///Stats group identifier to return a snapshot for. Leave unset to
        /// include all groups.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub group: ::std::option::Option<::std::string::String>,
        ///When true, omit the `transferring` and `checking` arrays from the
        /// response.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub short: ::std::option::Option<bool>,
    }

    ///Metadata for an item currently undergoing verification.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreStatsChecking {
        ///Stats group name associated with this verification.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub group: ::std::option::Option<::std::string::String>,
        ///Remote path of the object being verified.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Total size in bytes of the object.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<i64>,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`CoreStatsDeleteBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct CoreStatsDeleteBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group_: ::std::option::Option<::std::string::String>,
        ///Stats group identifier to remove.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`CoreStatsResetBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct CoreStatsResetBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group_: ::std::option::Option<::std::string::String>,
        ///Stats group identifier whose counters should be reset. Leave unset
        /// to reset all groups.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`CoreStatsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreStatsResponse {
        pub bytes: i64,
        ///Objects currently undergoing verification operations.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub checking: ::std::vec::Vec<CoreStatsChecking>,
        pub checks: i64,
        #[serde(rename = "deletedDirs")]
        pub deleted_dirs: i64,
        pub deletes: i64,
        #[serde(rename = "elapsedTime")]
        pub elapsed_time: f64,
        pub errors: i64,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub eta: ::std::option::Option<f64>,
        #[serde(rename = "fatalError")]
        pub fatal_error: bool,
        #[serde(
            rename = "lastError",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_error: ::std::option::Option<::std::string::String>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub listed: ::std::option::Option<i64>,
        pub renames: i64,
        #[serde(rename = "retryError")]
        pub retry_error: bool,
        #[serde(rename = "serverSideCopies")]
        pub server_side_copies: i64,
        #[serde(rename = "serverSideCopyBytes")]
        pub server_side_copy_bytes: i64,
        #[serde(rename = "serverSideMoveBytes")]
        pub server_side_move_bytes: i64,
        #[serde(rename = "serverSideMoves")]
        pub server_side_moves: i64,
        pub speed: f64,
        #[serde(rename = "totalBytes")]
        pub total_bytes: i64,
        #[serde(rename = "totalChecks")]
        pub total_checks: i64,
        #[serde(rename = "totalTransfers")]
        pub total_transfers: i64,
        #[serde(rename = "transferTime")]
        pub transfer_time: f64,
        ///Active transfers currently in progress grouped by stats group.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub transferring: ::std::vec::Vec<CoreStatsTransfer>,
        pub transfers: i64,
    }

    ///Progress metrics for an in-flight transfer.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreStatsTransfer {
        ///Bytes transferred so far for this object.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub bytes: ::std::option::Option<i64>,
        ///Destination remote or filesystem for this transfer.
        #[serde(
            rename = "dstFs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dst_fs: ::std::option::Option<::std::string::String>,
        ///Destination path within dstFs.
        #[serde(
            rename = "dstRemote",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dst_remote: ::std::option::Option<::std::string::String>,
        ///Estimated seconds remaining, when available.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub eta: ::std::option::Option<f64>,
        ///Stats group name associated with this transfer.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub group: ::std::option::Option<::std::string::String>,
        ///Remote path of the object being transferred.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        ///Completion percentage from 0-100.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub percentage: ::std::option::Option<i64>,
        ///Total size in bytes of the object.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<i64>,
        ///Current transfer speed in bytes per second.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub speed: ::std::option::Option<f64>,
        ///Current speed in bytes per second as an exponentially weighted
        /// moving average.
        #[serde(
            rename = "speedAvg",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub speed_avg: ::std::option::Option<f64>,
        ///Source remote or filesystem for this transfer.
        #[serde(
            rename = "srcFs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub src_fs: ::std::option::Option<::std::string::String>,
        ///Source path within srcFs.
        #[serde(
            rename = "srcRemote",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub src_remote: ::std::option::Option<::std::string::String>,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`CoreTransferredBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct CoreTransferredBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group_: ::std::option::Option<::std::string::String>,
        ///Stats group identifier to filter the completed transfer list. Leave
        /// unset for all groups.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`CoreTransferredResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreTransferredResponse {
        pub transferred: ::std::vec::Vec<CoreTransferredResponseTransferredItem>,
    }

    ///`CoreTransferredResponseTransferredItem`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreTransferredResponseTransferredItem {
        pub bytes: i64,
        pub checked: bool,
        ///ISO8601 timestamp when the transfer completed.
        pub completed_at: ::std::string::String,
        ///Destination remote or filesystem used for the transfer.
        #[serde(
            rename = "dstFs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dst_fs: ::std::option::Option<::std::string::String>,
        ///Destination path within `dstFs`, when provided.
        #[serde(
            rename = "dstRemote",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dst_remote: ::std::option::Option<::std::string::String>,
        pub error: ::std::string::String,
        ///Stats group identifier this transfer belonged to.
        pub group: ::std::string::String,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub jobid: ::std::option::Option<i64>,
        pub name: ::std::string::String,
        pub size: i64,
        ///Source remote or filesystem used for the transfer.
        #[serde(
            rename = "srcFs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub src_fs: ::std::option::Option<::std::string::String>,
        ///Source path within `srcFs`, when provided.
        #[serde(
            rename = "srcRemote",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub src_remote: ::std::option::Option<::std::string::String>,
        ///ISO8601 timestamp when the transfer started.
        pub started_at: ::std::string::String,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub timestamp: ::std::option::Option<i64>,
        ///What the transfer was doing, such as `transferring` or `checking`.
        /// Bisync adds values such as `listing file - Path1`, so this is free
        /// text.
        pub what: ::std::string::String,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`CoreVersionResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CoreVersionResponse {
        ///CPU architecture (e.g. amd64, arm64).
        pub arch: ::std::string::String,
        ///Version number broken into components.
        pub decomposed: ::std::vec::Vec<i64>,
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
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub os_arch: ::std::option::Option<::std::string::String>,
        ///OS Kernel version (e.g. 6.8.0-86-generic (x86_64)).
        #[serde(
            rename = "osKernel",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub os_kernel: ::std::option::Option<::std::string::String>,
        ///OS Version (e.g. ubuntu 24.04 (64 bit)).
        #[serde(
            rename = "osVersion",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub os_version: ::std::option::Option<::std::string::String>,
        ///Full semantic version string (e.g. 1.67.0).
        pub version: ::std::string::String,
    }

    ///`DebugSetBlockProfileRateBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DebugSetBlockProfileRateBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Sampling interval in nanoseconds for blocking profile collection;
        /// use 1 to capture all events.
        pub rate: i64,
    }

    ///`DebugSetGcPercentBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DebugSetGcPercentBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Target percentage of newly allocated data to trigger garbage
        /// collection.
        #[serde(rename = "gc-percent")]
        pub gc_percent: i64,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`DebugSetGcPercentResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DebugSetGcPercentResponse {
        #[serde(rename = "existing-gc-percent")]
        pub existing_gc_percent: i64,
    }

    ///`DebugSetMutexProfileFractionBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DebugSetMutexProfileFractionBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Sampling fraction for mutex contention profiling; set to 0 to
        /// disable.
        pub rate: i64,
    }

    ///`DebugSetMutexProfileFractionResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DebugSetMutexProfileFractionResponse {
        #[serde(rename = "previousRate")]
        pub previous_rate: i64,
    }

    ///`DebugSetSoftMemoryLimitBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DebugSetSoftMemoryLimitBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Memory limit in bytes. Use a string for values above 2^53, such as
        /// the default of 9223372036854775807.
        #[serde(rename = "mem-limit")]
        pub mem_limit: DebugSetSoftMemoryLimitBodyMemLimit,
    }

    ///Memory limit in bytes. Use a string for values above 2^53, such as the
    /// default of 9223372036854775807.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum DebugSetSoftMemoryLimitBodyMemLimit {
        Integer(i64),
        String(::std::string::String),
    }

    impl ::std::fmt::Display for DebugSetSoftMemoryLimitBodyMemLimit {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Integer(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }

    impl ::std::convert::From<i64> for DebugSetSoftMemoryLimitBodyMemLimit {
        fn from(value: i64) -> Self {
            Self::Integer(value)
        }
    }

    ///`DebugSetSoftMemoryLimitResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DebugSetSoftMemoryLimitResponse {
        #[serde(rename = "existing-mem-limit")]
        pub existing_mem_limit: i64,
    }

    ///`FscacheEntriesResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FscacheEntriesResponse {
        pub entries: i64,
    }

    ///`JobBatchBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JobBatchBody {
        ///Do this many commands concurrently. Defaults to --transfers if not
        /// set.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub concurrency: ::std::option::Option<i64>,
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///List of inputs to the commands with an extra _path parameter.
        pub inputs: ::std::vec::Vec<JobBatchBodyInputsItem>,
    }

    ///`JobBatchBodyInputsItem`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JobBatchBodyInputsItem {
        ///Run this item in the background. Its entry in `results` is then a
        /// job ID. Items have no headers, so `Prefer` is not an option here.
        #[serde(
            rename = "_async",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub async_: ::std::option::Option<bool>,
        ///Stats group this input accumulates under.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///rc/path
        #[serde(rename = "_path")]
        pub path: ::std::string::String,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`JobBatchResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JobBatchResponse {
        ///One entry for each input, in order. Each is that command's result,
        /// its error, or a job ID if the input set `_async`.
        pub results: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    }

    ///`JobListResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JobListResponse {
        ///Identifier for this rclone process.
        #[serde(rename = "executeId")]
        pub execute_id: ::std::string::String,
        ///Array of integer job ids that are finished.
        #[serde(rename = "finishedIds")]
        pub finished_ids: ::std::vec::Vec<i64>,
        ///Job IDs suitable for use with `job/status` and `job/stop`.
        pub jobids: ::std::vec::Vec<i64>,
        ///Array of integer job ids that are running.
        #[serde(rename = "runningIds")]
        pub running_ids: ::std::vec::Vec<i64>,
    }

    ///`JobStatusBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JobStatusBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Job id.
        pub jobid: i64,
    }

    ///`JobStatusResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JobStatusResponse {
        ///Execution time in seconds.
        pub duration: f64,
        ///Timestamp when the job finished. (e.g.
        /// '2025-12-26T18:50:20.528746884+01:00')
        #[serde(rename = "endTime")]
        pub end_time: ::std::string::String,
        ///Error message, or empty string on success.
        pub error: ::std::string::String,
        ///Identifier for this rclone process.
        #[serde(rename = "executeId")]
        pub execute_id: ::std::string::String,
        ///True once the job has completed.
        pub finished: bool,
        ///Stats group of the job.
        pub group: ::std::string::String,
        ///Job identifier.
        pub id: i64,
        ///Synchronous-style output payload when available.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub output: ::std::option::Option<::serde_json::Value>,
        ///Progress measurements supplied by the underlying command.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub progress: ::std::option::Option<::serde_json::Value>,
        ///Timestamp when the job started. (e.g.
        /// '2025-12-24T18:50:20.5281314+01:00')
        #[serde(rename = "startTime")]
        pub start_time: ::std::string::String,
        ///True if the job completed successfully.
        pub success: bool,
    }

    ///`JobStopBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JobStopBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Job id.
        pub jobid: i64,
    }

    ///`JobStopgroupBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JobStopgroupBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group_: ::std::option::Option<::std::string::String>,
        ///Stats group name whose active jobs should be stopped.
        pub group: ::std::string::String,
    }

    ///`MountListmountsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MountListmountsResponse {
        #[serde(rename = "mountPoints")]
        pub mount_points: ::std::vec::Vec<MountListmountsResponseMountPointsItem>,
    }

    ///`MountListmountsResponseMountPointsItem`
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

    ///`MountMountBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MountMountBody {
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote path to mount, such as `drive:` or `remote:subdir`.
        pub fs: MountMountBodyFs,
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Mount options, using the option names from `options/info`.
        #[serde(
            rename = "mountOpt",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub mount_opt: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Absolute local path where the remote should be mounted.
        #[serde(rename = "mountPoint")]
        pub mount_point: ::std::string::String,
        ///Optional mount implementation to use (`mount`, `cmount`, or
        /// `mount2`).
        #[serde(
            rename = "mountType",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mount_type: ::std::option::Option<::std::string::String>,
        ///VFS options, using the option names from `options/info`.
        #[serde(
            rename = "vfsOpt",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub vfs_opt: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///Remote path to mount, such as `drive:` or `remote:subdir`.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum MountMountBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for MountMountBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`MountMountResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MountMountResponse {
        ///Actual local path where the remote was mounted. May differ from the
        /// requested path (e.g. when '*' is used on Windows).
        #[serde(rename = "mountPoint")]
        pub mount_point: ::std::string::String,
    }

    ///`MountTypesResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MountTypesResponse {
        #[serde(rename = "mountTypes")]
        pub mount_types: ::std::vec::Vec<::std::string::String>,
    }

    ///`MountUnmountBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MountUnmountBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Local mount point path to unmount.
        #[serde(rename = "mountPoint")]
        pub mount_point: ::std::string::String,
    }

    ///`OperationsAboutBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsAboutBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path to query for capacity information.
        pub fs: OperationsAboutBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///Remote name or path to query for capacity information.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsAboutBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsAboutBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsAboutResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct OperationsAboutResponse {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub free: ::std::option::Option<i64>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub objects: ::std::option::Option<i64>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub other: ::std::option::Option<i64>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub total: ::std::option::Option<i64>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub trashed: ::std::option::Option<i64>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub used: ::std::option::Option<i64>,
    }

    ///`OperationsCheckBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsCheckBody {
        ///Remote containing the checksum SUM file when using `checkFileHash`.
        #[serde(
            rename = "checkFileFs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub check_file_fs: ::std::option::Option<OperationsCheckBodyCheckFileFs>,
        ///Hash name to expect in the supplied SUM file, such as `md5`.
        #[serde(
            rename = "checkFileHash",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub check_file_hash: ::std::option::Option<::std::string::String>,
        ///Path within `checkFileFs` to the checksum SUM file.
        #[serde(
            rename = "checkFileRemote",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub check_file_remote: ::std::option::Option<::std::string::String>,
        ///Set to true to include a combined summary report in the response.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub combined: ::std::option::Option<bool>,
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Set to true to include differing files in the report.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub differ: ::std::option::Option<bool>,
        ///Set to true to read file contents during comparison instead of
        /// relying on hashes.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub download: ::std::option::Option<bool>,
        ///Destination remote name or path that should match the source.
        #[serde(rename = "dstFs")]
        pub dst_fs: OperationsCheckBodyDstFs,
        ///Set to true to include entries that encountered errors.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub error: ::std::option::Option<bool>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Set to true to include matching files in the report.
        #[serde(
            rename = "match",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub match_: ::std::option::Option<bool>,
        ///Set to true to report files missing from the destination.
        #[serde(
            rename = "missingOnDst",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub missing_on_dst: ::std::option::Option<bool>,
        ///Set to true to report files missing from the source.
        #[serde(
            rename = "missingOnSrc",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub missing_on_src: ::std::option::Option<bool>,
        ///Set to true to only ensure that source files exist on the
        /// destination.
        #[serde(
            rename = "oneWay",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub one_way: ::std::option::Option<bool>,
        ///Source remote name or path to verify, e.g. `drive:`. Required unless
        /// `checkFileHash` is set.
        #[serde(
            rename = "srcFs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub src_fs: ::std::option::Option<OperationsCheckBodySrcFs>,
    }

    ///Remote containing the checksum SUM file when using `checkFileHash`.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsCheckBodyCheckFileFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsCheckBodyCheckFileFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///Destination remote name or path that should match the source.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsCheckBodyDstFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsCheckBodyDstFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///Source remote name or path to verify, e.g. `drive:`. Required unless
    /// `checkFileHash` is set.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsCheckBodySrcFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsCheckBodySrcFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsCheckResponse`
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

    ///`OperationsCleanupBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsCleanupBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path to clean up, for example `drive:`.
        pub fs: OperationsCleanupBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///Remote name or path to clean up, for example `drive:`.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsCleanupBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsCleanupBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsCopyfileBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsCopyfileBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Destination remote name or path, such as `drive2:` or `/` for local
        /// filesystem.
        #[serde(rename = "dstFs")]
        pub dst_fs: OperationsCopyfileBodyDstFs,
        ///Target path within `dstFs` where the file should be written.
        #[serde(rename = "dstRemote")]
        pub dst_remote: ::std::string::String,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Source remote name or path, such as `drive:` or `/` for the local
        /// filesystem.
        #[serde(rename = "srcFs")]
        pub src_fs: OperationsCopyfileBodySrcFs,
        ///Path to the source object within `srcFs`, for example
        /// `dir/file.txt`.
        #[serde(rename = "srcRemote")]
        pub src_remote: ::std::string::String,
    }

    ///Destination remote name or path, such as `drive2:` or `/` for local
    /// filesystem.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsCopyfileBodyDstFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsCopyfileBodyDstFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///Source remote name or path, such as `drive:` or `/` for the local
    /// filesystem.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsCopyfileBodySrcFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsCopyfileBodySrcFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsCopyurlBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsCopyurlBody {
        ///Set to true to derive the destination filename from the URL.
        #[serde(
            rename = "autoFilename",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub auto_filename: ::std::option::Option<bool>,
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path that will receive the downloaded file, e.g.
        /// `drive:`.
        pub fs: OperationsCopyurlBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "headerFilename",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub header_filename: ::std::option::Option<bool>,
        #[serde(
            rename = "noClobber",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_clobber: ::std::option::Option<bool>,
        ///Destination path within `fs` where the fetched object will be
        /// stored.
        pub remote: ::std::string::String,
        ///Source URL to fetch the object from.
        pub url: ::std::string::String,
    }

    ///Remote name or path that will receive the downloaded file, e.g.
    /// `drive:`.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsCopyurlBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsCopyurlBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsDeleteBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsDeleteBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path whose contents should be removed.
        pub fs: OperationsDeleteBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///Remote name or path whose contents should be removed.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsDeleteBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsDeleteBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsDeletefileBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsDeletefileBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path that contains the file to delete.
        pub fs: OperationsDeletefileBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Exact path to the file within `fs` that should be deleted.
        pub remote: ::std::string::String,
    }

    ///Remote name or path that contains the file to delete.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsDeletefileBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsDeletefileBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsFsinfoBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsFsinfoBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path to inspect, e.g. `drive:`.
        pub fs: OperationsFsinfoBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///Remote name or path to inspect, e.g. `drive:`.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsFsinfoBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsFsinfoBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsFsinfoResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsFsinfoResponse {
        #[serde(rename = "Features")]
        pub features: ::std::collections::HashMap<::std::string::String, bool>,
        #[serde(rename = "Hashes")]
        pub hashes: ::std::vec::Vec<::std::string::String>,
        #[serde(
            rename = "MetadataInfo",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub metadata_info: ::std::option::Option<ConfigProviderMetadataInfo>,
        #[serde(rename = "Name")]
        pub name: ::std::string::String,
        #[serde(rename = "Precision")]
        pub precision: i64,
        #[serde(rename = "Root")]
        pub root: ::std::string::String,
        #[serde(rename = "String")]
        pub string: ::std::string::String,
    }

    ///`OperationsHashsumBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsHashsumBody {
        ///Set to true to emit hash values in base64 rather than hexadecimal.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub base64: ::std::option::Option<bool>,
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Set to true to force reading the data instead of using remote
        /// checksums.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub download: ::std::option::Option<bool>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path to hash, such as `drive:` or `/`.
        pub fs: OperationsHashsumBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Hash algorithm to use, e.g. `md5`, `sha1`, or another supported
        /// name.
        #[serde(rename = "hashType")]
        pub hash_type: ::std::string::String,
    }

    ///Remote name or path to hash, such as `drive:` or `/`.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsHashsumBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsHashsumBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsHashsumResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsHashsumResponse {
        #[serde(rename = "hashType")]
        pub hash_type: ::std::string::String,
        pub hashsum: ::std::vec::Vec<::std::string::String>,
    }

    ///`OperationsHashsumfileBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsHashsumfileBody {
        ///Set to true to emit the hash value in base64 rather than
        /// hexadecimal.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub base64: ::std::option::Option<bool>,
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Set to true to force reading the data instead of using remote
        /// checksums.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub download: ::std::option::Option<bool>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path containing the file to hash.
        pub fs: OperationsHashsumfileBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Hash algorithm to use, e.g. `md5`, `sha1`, or another supported
        /// name.
        #[serde(rename = "hashType")]
        pub hash_type: ::std::string::String,
        ///Path to the specific file within `fs` to hash.
        pub remote: ::std::string::String,
    }

    ///Remote name or path containing the file to hash.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsHashsumfileBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsHashsumfileBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsHashsumfileResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsHashsumfileResponse {
        ///The hash value of the file.
        pub hash: ::std::string::String,
        ///The hash algorithm that was used.
        #[serde(rename = "hashType")]
        pub hash_type: ::std::string::String,
    }

    ///`OperationsListBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsListBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path to list, for example `drive:`.
        pub fs: OperationsListBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub opt: ::std::option::Option<OperationsListBodyOpt>,
        ///Directory path within `fs` to list; leave empty to target the root.
        pub remote: ::std::string::String,
    }

    ///Remote name or path to list, for example `drive:`.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsListBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsListBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///Options that control the listing.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct OperationsListBodyOpt {
        #[serde(
            rename = "dirsOnly",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dirs_only: ::std::option::Option<bool>,
        #[serde(
            rename = "filesOnly",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub files_only: ::std::option::Option<bool>,
        #[serde(
            rename = "hashTypes",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub hash_types: ::std::vec::Vec<::std::string::String>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub metadata: ::std::option::Option<bool>,
        #[serde(
            rename = "noMimeType",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_mime_type: ::std::option::Option<bool>,
        #[serde(
            rename = "noModTime",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_mod_time: ::std::option::Option<bool>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub recurse: ::std::option::Option<bool>,
        #[serde(
            rename = "showEncrypted",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub show_encrypted: ::std::option::Option<bool>,
        #[serde(
            rename = "showHash",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub show_hash: ::std::option::Option<bool>,
        #[serde(
            rename = "showOrigIDs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub show_orig_i_ds: ::std::option::Option<bool>,
    }

    ///`OperationsListResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsListResponse {
        ///Array of entries equivalent to the items returned by `rclone
        /// lsjson`.
        pub list: ::std::vec::Vec<OperationsListResponseListItem>,
    }

    ///`OperationsListResponseListItem`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsListResponseListItem {
        ///Encrypted entry name when using crypt remotes.
        #[serde(
            rename = "Encrypted",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub encrypted: ::std::option::Option<::std::string::String>,
        ///Encrypted path when using crypt remotes.
        #[serde(
            rename = "EncryptedPath",
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
        #[serde(rename = "ID", skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        ///True for bucket/root entries on bucket-based remotes.
        #[serde(
            rename = "IsBucket",
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
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<::std::string::String>,
        ///Modification timestamp in RFC3339 format.
        #[serde(
            rename = "ModTime",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mod_time: ::std::option::Option<::std::string::String>,
        ///Base name of the entry.
        #[serde(rename = "Name")]
        pub name: ::std::string::String,
        ///Original backend identifier when recorded.
        #[serde(
            rename = "OrigID",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub orig_id: ::std::option::Option<::std::string::String>,
        ///Path relative to the requested remote root.
        #[serde(rename = "Path")]
        pub path: ::std::string::String,
        ///Object size in bytes.
        #[serde(
            rename = "Size",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub size: ::std::option::Option<i64>,
        ///Storage class or tier, if supplied by the backend.
        #[serde(
            rename = "Tier",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tier: ::std::option::Option<::std::string::String>,
    }

    ///`OperationsMkdirBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsMkdirBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path in which to create a directory.
        pub fs: OperationsMkdirBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Directory path within `fs` to create.
        pub remote: ::std::string::String,
    }

    ///Remote name or path in which to create a directory.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsMkdirBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsMkdirBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsMovefileBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsMovefileBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Destination remote name or path where the file will be moved.
        #[serde(rename = "dstFs")]
        pub dst_fs: OperationsMovefileBodyDstFs,
        ///Destination path within `dstFs` for the moved object.
        #[serde(rename = "dstRemote")]
        pub dst_remote: ::std::string::String,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Source remote name or path containing the file to move.
        #[serde(rename = "srcFs")]
        pub src_fs: OperationsMovefileBodySrcFs,
        ///Path to the source object within `srcFs`.
        #[serde(rename = "srcRemote")]
        pub src_remote: ::std::string::String,
    }

    ///Destination remote name or path where the file will be moved.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsMovefileBodyDstFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsMovefileBodyDstFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///Source remote name or path containing the file to move.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsMovefileBodySrcFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsMovefileBodySrcFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsPubliclinkBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsPubliclinkBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Optional expiration time for the public link, formatted as supported
        /// by the backend.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub expire: ::std::option::Option<::std::string::String>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path hosting the object for which to manage a public
        /// link.
        pub fs: OperationsPubliclinkBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Path within `fs` to the object for which to create or remove a
        /// public link.
        pub remote: ::std::string::String,
        ///Set to true to remove an existing public link instead of creating
        /// one.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub unlink: ::std::option::Option<bool>,
    }

    ///Remote name or path hosting the object for which to manage a public
    /// link.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsPubliclinkBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsPubliclinkBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsPubliclinkResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsPubliclinkResponse {
        pub url: ::std::string::String,
    }

    ///`OperationsPurgeBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsPurgeBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path from which to remove all contents.
        pub fs: OperationsPurgeBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Path within `fs` whose contents should be purged.
        pub remote: ::std::string::String,
    }

    ///Remote name or path from which to remove all contents.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsPurgeBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsPurgeBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsRmdirBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsRmdirBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path containing the directory to remove.
        pub fs: OperationsRmdirBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Directory path within `fs` to delete.
        pub remote: ::std::string::String,
    }

    ///Remote name or path containing the directory to remove.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsRmdirBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsRmdirBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsRmdirsBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsRmdirsBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path to scan for empty directories.
        pub fs: OperationsRmdirsBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Set to true to preserve the top-level directory even if empty.
        #[serde(
            rename = "leaveRoot",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub leave_root: ::std::option::Option<bool>,
        ///Path within `fs` whose empty subdirectories should be removed.
        pub remote: ::std::string::String,
    }

    ///Remote name or path to scan for empty directories.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsRmdirsBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsRmdirsBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsSettierBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsSettierBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path whose storage class tier should be changed.
        pub fs: OperationsSettierBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Storage tier or class.
        pub tier: ::std::string::String,
    }

    ///Remote name or path whose storage class tier should be changed.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsSettierBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsSettierBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsSettierfileBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsSettierfileBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path that contains the object whose tier should
        /// change.
        pub fs: OperationsSettierfileBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Path within `fs` to the object whose storage class tier should be
        /// updated.
        pub remote: ::std::string::String,
        ///Storage tier or class.
        pub tier: ::std::string::String,
    }

    ///Remote name or path that contains the object whose tier should change.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsSettierfileBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsSettierfileBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsSizeBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsSizeBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path to measure aggregate size information for.
        pub fs: OperationsSizeBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///Remote name or path to measure aggregate size information for.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsSizeBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsSizeBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`OperationsSizeResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsSizeResponse {
        pub bytes: i64,
        pub count: i64,
        pub sizeless: i64,
    }

    ///`OperationsStatBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsStatBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote name or path that contains the item to inspect.
        pub fs: OperationsStatBodyFs,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub opt: ::std::option::Option<OperationsStatBodyOpt>,
        ///Path to the file or directory within `fs` to describe.
        pub remote: ::std::string::String,
    }

    ///Remote name or path that contains the item to inspect.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum OperationsStatBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for OperationsStatBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///Options that control the listing.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct OperationsStatBodyOpt {
        #[serde(
            rename = "dirsOnly",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dirs_only: ::std::option::Option<bool>,
        #[serde(
            rename = "filesOnly",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub files_only: ::std::option::Option<bool>,
        #[serde(
            rename = "hashTypes",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub hash_types: ::std::vec::Vec<::std::string::String>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub metadata: ::std::option::Option<bool>,
        #[serde(
            rename = "noMimeType",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_mime_type: ::std::option::Option<bool>,
        #[serde(
            rename = "noModTime",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_mod_time: ::std::option::Option<bool>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub recurse: ::std::option::Option<bool>,
        #[serde(
            rename = "showEncrypted",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub show_encrypted: ::std::option::Option<bool>,
        #[serde(
            rename = "showHash",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub show_hash: ::std::option::Option<bool>,
        #[serde(
            rename = "showOrigIDs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub show_orig_i_ds: ::std::option::Option<bool>,
    }

    ///`OperationsStatResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsStatResponse {
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub item: ::std::option::Option<OperationsStatResponseItem>,
    }

    ///`OperationsStatResponseItem`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OperationsStatResponseItem {
        ///Encrypted entry name when using crypt remotes.
        #[serde(
            rename = "Encrypted",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub encrypted: ::std::option::Option<::std::string::String>,
        ///Encrypted path when using crypt remotes.
        #[serde(
            rename = "EncryptedPath",
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
        #[serde(rename = "ID", skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        ///True for bucket/root entries on bucket-based remotes.
        #[serde(
            rename = "IsBucket",
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
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub orig_id: ::std::option::Option<::std::string::String>,
        ///Path relative to the remote root.
        #[serde(rename = "Path")]
        pub path: ::std::string::String,
        ///Object size in bytes.
        #[serde(rename = "Size")]
        pub size: i64,
        ///Storage class or tier, if supplied by the backend.
        #[serde(
            rename = "Tier",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tier: ::std::option::Option<::std::string::String>,
    }

    ///`OptionsBlocksResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsBlocksResponse {
        pub options: ::std::vec::Vec<::std::string::String>,
    }

    ///`OptionsGetBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct OptionsGetBody {
        ///Optional comma-separated list of option block names to return.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub blocks: ::std::option::Option<::std::string::String>,
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`OptionsGetResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsGetResponse {
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub dlna: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub ftp: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub http: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub log: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub main: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub mount: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub nfs: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub proxy: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub rc: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub restic: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub s3: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub sftp: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub vfs: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub webdav: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`OptionsInfoBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct OptionsInfoBody {
        ///Optional comma-separated list of option block names to describe.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub blocks: ::std::option::Option<::std::string::String>,
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`OptionsInfoOption`
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
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_opt: ::std::option::Option<::std::string::String>,
        #[serde(rename = "Type")]
        pub type_: ::std::string::String,
        ///Current value of this option.
        #[serde(
            rename = "Value",
            deserialize_with = "::std::option::Option::deserialize"
        )]
        pub value: ::std::option::Option<OptionsInfoOptionValue>,
        #[serde(rename = "ValueStr")]
        pub value_str: ::std::string::String,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///Default value for this option.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged, deny_unknown_fields)]
    pub enum OptionsInfoOptionDefault {
        Array(::std::vec::Vec<::std::string::String>),
        Boolean(bool),
        Number(f64),
        String(::std::string::String),
        Object {
            #[serde(rename = "Valid")]
            valid: bool,
            #[serde(rename = "Value")]
            value: bool,
        },
    }

    impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for OptionsInfoOptionDefault {
        fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
            Self::Array(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsInfoOptionDefault {
        fn from(value: bool) -> Self {
            Self::Boolean(value)
        }
    }

    impl ::std::convert::From<f64> for OptionsInfoOptionDefault {
        fn from(value: f64) -> Self {
            Self::Number(value)
        }
    }

    ///`OptionsInfoOptionExample`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsInfoOptionExample {
        #[serde(rename = "Help")]
        pub help: ::std::string::String,
        #[serde(rename = "Value")]
        pub value: ::std::string::String,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`OptionsInfoOptionValue`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged, deny_unknown_fields)]
    pub enum OptionsInfoOptionValue {
        Array(::std::vec::Vec<::std::string::String>),
        Boolean(bool),
        Number(f64),
        String(::std::string::String),
        Object {
            #[serde(rename = "Valid")]
            valid: bool,
            #[serde(rename = "Value")]
            value: bool,
        },
    }

    impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for OptionsInfoOptionValue {
        fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
            Self::Array(value)
        }
    }

    impl ::std::convert::From<bool> for OptionsInfoOptionValue {
        fn from(value: bool) -> Self {
            Self::Boolean(value)
        }
    }

    impl ::std::convert::From<f64> for OptionsInfoOptionValue {
        fn from(value: f64) -> Self {
            Self::Number(value)
        }
    }

    ///`OptionsInfoResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsInfoResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub dlna: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub filter: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub ftp: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub http: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub log: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub main: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub mount: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub nfs: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub proxy: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub rc: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub restic: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub s3: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub sftp: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub vfs: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub webdav: ::std::vec::Vec<OptionsInfoOption>,
        #[serde(flatten)]
        pub extra:
            ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<OptionsInfoOption>>,
    }

    ///`OptionsLocalResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsLocalResponse {
        pub config: OptionsLocalResponseConfig,
        pub filter: OptionsLocalResponseFilter,
    }

    ///`OptionsLocalResponseConfig`
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
        pub buffer_size: i64,
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
        pub checkers: i64,
        #[serde(rename = "ClientCert")]
        pub client_cert: ::std::string::String,
        #[serde(rename = "ClientKey")]
        pub client_key: ::std::string::String,
        #[serde(rename = "ClientPass")]
        pub client_pass: ::std::string::String,
        #[serde(rename = "CompareDest")]
        pub compare_dest: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "ConnectTimeout")]
        pub connect_timeout: i64,
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
        pub delete_mode: i64,
        #[serde(
            rename = "DisableFeatures",
            deserialize_with = "::std::option::Option::deserialize"
        )]
        pub disable_features: ::std::option::Option<::std::string::String>,
        #[serde(rename = "DisableHTTP2")]
        pub disable_http2: bool,
        #[serde(rename = "DisableHTTPKeepAlives")]
        pub disable_http_keep_alives: bool,
        #[serde(
            rename = "DownloadHeaders",
            deserialize_with = "::std::option::Option::deserialize"
        )]
        pub download_headers: ::std::option::Option<::std::string::String>,
        #[serde(rename = "DryRun")]
        pub dry_run: bool,
        #[serde(rename = "Dump")]
        pub dump: ::std::string::String,
        #[serde(rename = "ErrorOnNoTransfer")]
        pub error_on_no_transfer: bool,
        #[serde(rename = "ExpectContinueTimeout")]
        pub expect_continue_timeout: i64,
        #[serde(rename = "FixCase")]
        pub fix_case: bool,
        #[serde(rename = "FsCacheExpireDuration")]
        pub fs_cache_expire_duration: i64,
        #[serde(rename = "FsCacheExpireInterval")]
        pub fs_cache_expire_interval: i64,
        #[serde(
            rename = "Headers",
            deserialize_with = "::std::option::Option::deserialize"
        )]
        pub headers: ::std::option::Option<::std::string::String>,
        #[serde(rename = "HTTPProxy")]
        pub http_proxy: ::std::string::String,
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
        pub kv_lock_time: i64,
        #[serde(rename = "Links")]
        pub links: bool,
        #[serde(rename = "ListCutoff")]
        pub list_cutoff: i64,
        #[serde(rename = "LogLevel")]
        pub log_level: ::std::string::String,
        #[serde(rename = "LowLevelRetries")]
        pub low_level_retries: i64,
        #[serde(rename = "MaxBacklog")]
        pub max_backlog: i64,
        #[serde(rename = "MaxBufferMemory")]
        pub max_buffer_memory: i64,
        #[serde(rename = "MaxConnections")]
        pub max_connections: i64,
        #[serde(rename = "MaxDelete")]
        pub max_delete: i64,
        #[serde(rename = "MaxDeleteSize")]
        pub max_delete_size: i64,
        #[serde(rename = "MaxDepth")]
        pub max_depth: i64,
        #[serde(rename = "MaxDuration")]
        pub max_duration: i64,
        #[serde(rename = "MaxStatsGroups")]
        pub max_stats_groups: i64,
        #[serde(rename = "MaxTransfer")]
        pub max_transfer: i64,
        #[serde(rename = "Metadata")]
        pub metadata: bool,
        #[serde(
            rename = "MetadataMapper",
            deserialize_with = "::std::option::Option::deserialize"
        )]
        pub metadata_mapper: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "MetadataSet",
            deserialize_with = "::std::option::Option::deserialize"
        )]
        pub metadata_set: ::std::option::Option<::std::string::String>,
        #[serde(rename = "ModifyWindow")]
        pub modify_window: i64,
        #[serde(rename = "MultiThreadChunkSize")]
        pub multi_thread_chunk_size: i64,
        #[serde(rename = "MultiThreadCutoff")]
        pub multi_thread_cutoff: i64,
        #[serde(rename = "MultiThreadSet")]
        pub multi_thread_set: bool,
        #[serde(rename = "MultiThreadStreams")]
        pub multi_thread_streams: i64,
        #[serde(rename = "MultiThreadWriteBufferSize")]
        pub multi_thread_write_buffer_size: i64,
        #[serde(
            rename = "NameTransform",
            deserialize_with = "::std::option::Option::deserialize"
        )]
        pub name_transform: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
        #[serde(
            rename = "PasswordCommand",
            deserialize_with = "::std::option::Option::deserialize"
        )]
        pub password_command: ::std::option::Option<::std::string::String>,
        #[serde(rename = "Progress")]
        pub progress: bool,
        #[serde(rename = "ProgressTerminalTitle")]
        pub progress_terminal_title: bool,
        #[serde(rename = "RefreshTimes")]
        pub refresh_times: bool,
        #[serde(rename = "Retries")]
        pub retries: i64,
        #[serde(rename = "RetriesInterval")]
        pub retries_interval: i64,
        #[serde(rename = "ServerSideAcrossConfigs")]
        pub server_side_across_configs: bool,
        #[serde(rename = "SizeOnly")]
        pub size_only: bool,
        #[serde(rename = "StatsFileNameLength")]
        pub stats_file_name_length: i64,
        #[serde(rename = "StatsLogLevel")]
        pub stats_log_level: ::std::string::String,
        #[serde(rename = "StatsOneLine")]
        pub stats_one_line: bool,
        #[serde(rename = "StatsOneLineDate")]
        pub stats_one_line_date: bool,
        #[serde(rename = "StatsOneLineDateFormat")]
        pub stats_one_line_date_format: ::std::string::String,
        #[serde(rename = "StreamingUploadCutoff")]
        pub streaming_upload_cutoff: i64,
        #[serde(rename = "Suffix")]
        pub suffix: ::std::string::String,
        #[serde(rename = "SuffixKeepExtension")]
        pub suffix_keep_extension: bool,
        #[serde(rename = "TerminalColorMode")]
        pub terminal_color_mode: ::std::string::String,
        #[serde(rename = "Timeout")]
        pub timeout: i64,
        #[serde(rename = "TPSLimit")]
        pub tps_limit: f64,
        #[serde(rename = "TPSLimitBurst")]
        pub tps_limit_burst: i64,
        #[serde(rename = "TrackRenames")]
        pub track_renames: bool,
        #[serde(rename = "TrackRenamesStrategy")]
        pub track_renames_strategy: ::std::string::String,
        #[serde(rename = "TrafficClass")]
        pub traffic_class: i64,
        #[serde(rename = "Transfers")]
        pub transfers: i64,
        #[serde(rename = "UpdateOlder")]
        pub update_older: bool,
        #[serde(
            rename = "UploadHeaders",
            deserialize_with = "::std::option::Option::deserialize"
        )]
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

    ///`OptionsLocalResponseFilter`
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
        #[serde(
            rename = "FilesFrom0",
            deserialize_with = "::std::option::Option::deserialize"
        )]
        pub files_from0: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
        pub max_age: i64,
        #[serde(rename = "MaxSize")]
        pub max_size: i64,
        #[serde(rename = "MetaRules")]
        pub meta_rules: OptionsLocalResponseFilterMetaRules,
        #[serde(rename = "MinAge")]
        pub min_age: i64,
        #[serde(rename = "MinSize")]
        pub min_size: i64,
    }

    ///`OptionsLocalResponseFilterMetaRules`
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

    ///`OptionsSetBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionsSetBody {
        ///Overrides for the `dlna` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub dlna: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Overrides for the `filter` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Overrides for the `ftp` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub ftp: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Overrides for the `http` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub http: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Overrides for the `log` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub log: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Overrides for the `main` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub main: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Overrides for the `mount` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub mount: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Overrides for the `nfs` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub nfs: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Overrides for the `proxy` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub proxy: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Overrides for the `rc` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub rc: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Overrides for the `restic` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub restic: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Overrides for the `s3` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub s3: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Overrides for the `sftp` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub sftp: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Overrides for the `vfs` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub vfs: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Overrides for the `webdav` option block.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub webdav: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`PluginsctlAddPluginBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PluginsctlAddPluginBody {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub branch: ::std::option::Option<::std::string::String>,
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Repository URL of the plugin to install.
        pub url: ::std::string::String,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<::std::string::String>,
    }

    ///`PluginsctlGetPluginsForTypeBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct PluginsctlGetPluginsForTypeBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Filter results by plugin type (e.g. `test`).
        #[serde(
            rename = "pluginType",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub plugin_type: ::std::option::Option<::std::string::String>,
        ///MIME type to match when listing plugins.
        #[serde(
            rename = "type",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }

    ///`PluginsctlGetPluginsForTypeResponse`
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

    ///`PluginsctlListPluginsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PluginsctlListPluginsResponse {
        ///Metadata entries for installed plugins.
        #[serde(rename = "loadedPlugins")]
        pub loaded_plugins:
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Metadata entries for installed test plugins.
        #[serde(rename = "loadedTestPlugins")]
        pub loaded_test_plugins:
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    }

    ///`PluginsctlListTestPluginsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PluginsctlListTestPluginsResponse {
        ///Installed test plugin metadata keyed by repository.
        #[serde(rename = "loadedTestPlugins")]
        pub loaded_test_plugins: ::std::collections::HashMap<
            ::std::string::String,
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        >,
    }

    ///`PluginsctlRemovePluginBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PluginsctlRemovePluginBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Name of the plugin to uninstall.
        pub name: ::std::string::String,
    }

    ///`PluginsctlRemoveTestPluginBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PluginsctlRemoveTestPluginBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Name of the test plugin to uninstall.
        pub name: ::std::string::String,
    }

    ///`RcError`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RcError {
        pub error: ::std::string::String,
        ///Original request parameters echoed for debugging.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub input:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        pub path: ::std::string::String,
        pub status: i64,
    }

    ///`RcErrorBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct RcErrorBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`RcGlobalOptions`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct RcGlobalOptions {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`RcListResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RcListResponse {
        pub commands: ::std::vec::Vec<RcListResponseCommandsItem>,
    }

    ///`RcListResponseCommandsItem`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RcListResponseCommandsItem {
        #[serde(
            rename = "Help",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub help: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "NeedsRequest",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub needs_request: ::std::option::Option<bool>,
        #[serde(
            rename = "NeedsResponse",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub needs_response: ::std::option::Option<bool>,
        ///True if the call works without authentication.
        #[serde(
            rename = "NoAuth",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_auth: ::std::option::Option<bool>,
        #[serde(
            rename = "Path",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "Title",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<::std::string::String>,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`RcNoopAuthBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct RcNoopAuthBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`RcNoopBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct RcNoopBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///A remote defined inline. Needs `type` or `_name`. Other keys are backend
    /// options and must be strings.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RemoteDefinition {
        ///Name of a configured remote.
        #[serde(
            rename = "_name",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<::std::string::String>,
        ///Root path within the remote.
        #[serde(
            rename = "_root",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub root: ::std::option::Option<::std::string::String>,
        ///Backend type, such as `s3`.
        #[serde(
            rename = "type",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
        #[serde(flatten)]
        pub extra: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    }

    ///`ServeListResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ServeListResponse {
        pub list: ::std::vec::Vec<ServeListResponseListItem>,
    }

    ///`ServeListResponseListItem`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ServeListResponseListItem {
        ///Address and port the server is listening on.
        pub addr: ::std::string::String,
        ///Identifier returned by `serve/start`.
        pub id: ::std::string::String,
        pub params: ServeListResponseListItemParams,
    }

    ///Serve configuration parameters supplied at startup.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ServeListResponseListItemParams {
        pub fs: ServeListResponseListItemParamsFs,
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
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///`ServeListResponseListItemParamsFs`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum ServeListResponseListItemParamsFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for ServeListResponseListItemParamsFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`ServeStartBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ServeStartBody {
        ///Address and port to bind the server to, such as `:5572` or
        /// `localhost:8080`.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub addr: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Remote path that will be served.
        pub fs: ServeStartBodyFs,
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Type of server to start (e.g. `http`, `webdav`, `ftp`, `sftp`).
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///Remote path that will be served.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum ServeStartBodyFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for ServeStartBodyFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`ServeStartResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ServeStartResponse {
        ///Address and port the server is listening on.
        pub addr: ::std::string::String,
        ///Identifier to pass to `serve/stop`.
        pub id: ::std::string::String,
    }

    ///`ServeStopBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ServeStopBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Identifier of the running serve instance returned by `serve/start`.
        pub id: ::std::string::String,
    }

    ///`ServeTypesResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ServeTypesResponse {
        pub types: ::std::vec::Vec<::std::string::String>,
    }

    ///`SyncBisyncBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SyncBisyncBody {
        #[serde(
            rename = "backupDir1",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub backup_dir1: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "backupDir2",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub backup_dir2: ::std::option::Option<::std::string::String>,
        ///Set to true to abort if `RCLONE_TEST` files are missing on either
        /// side.
        #[serde(
            rename = "checkAccess",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub check_access: ::std::option::Option<bool>,
        ///Override the access-check sentinel filename; defaults to
        /// `RCLONE_TEST`.
        #[serde(
            rename = "checkFilename",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub check_filename: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "checkSync",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub check_sync: ::std::option::Option<SyncBisyncBodyCheckSync>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub compare: ::std::option::Option<::std::string::String>,
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(
            rename = "conflictLoser",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub conflict_loser: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "conflictResolve",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub conflict_resolve: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "conflictSuffix",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub conflict_suffix: ::std::option::Option<::std::string::String>,
        ///Set to true to mirror empty directories between the two paths.
        #[serde(
            rename = "createEmptySrcDirs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub create_empty_src_dirs: ::std::option::Option<bool>,
        #[serde(
            rename = "downloadHash",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub download_hash: ::std::option::Option<bool>,
        ///Set to true to simulate the bisync run without making changes.
        #[serde(
            rename = "dryRun",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dry_run: ::std::option::Option<bool>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Path to an rclone filters file applied to both paths.
        #[serde(
            rename = "filtersFile",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub filters_file: ::std::option::Option<::std::string::String>,
        ///Set to true to bypass the `maxDelete` safety check.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub force: ::std::option::Option<bool>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Set to true to ignore checksum differences when comparing listings.
        #[serde(
            rename = "ignoreListingChecksum",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ignore_listing_checksum: ::std::option::Option<bool>,
        #[serde(
            rename = "maxDelete",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub max_delete: ::std::option::Option<i64>,
        #[serde(
            rename = "maxLock",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub max_lock: ::std::option::Option<::std::string::String>,
        ///Set to true to keep bisync working files after completion.
        #[serde(
            rename = "noCleanup",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_cleanup: ::std::option::Option<bool>,
        #[serde(
            rename = "noSlowHash",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_slow_hash: ::std::option::Option<bool>,
        ///First remote directory, e.g. `drive:path1`.
        pub path1: SyncBisyncBodyPath1,
        ///Second remote directory, e.g. `drive:path2`.
        pub path2: SyncBisyncBodyPath2,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub recover: ::std::option::Option<bool>,
        ///Set to true to remove empty directories during cleanup.
        #[serde(
            rename = "removeEmptyDirs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub remove_empty_dirs: ::std::option::Option<bool>,
        ///Set to true to allow retrying after certain recoverable errors.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resilient: ::std::option::Option<bool>,
        ///Set to true to perform a one-time resync, rebuilding bisync history.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resync: ::std::option::Option<bool>,
        #[serde(
            rename = "resyncMode",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub resync_mode: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "slowHashSyncOnly",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub slow_hash_sync_only: ::std::option::Option<bool>,
        ///Directory path used to store bisync working files.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub workdir: ::std::option::Option<::std::string::String>,
    }

    ///`SyncBisyncBodyCheckSync`
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
    pub enum SyncBisyncBodyCheckSync {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
        #[serde(rename = "only")]
        Only,
    }

    impl ::std::fmt::Display for SyncBisyncBodyCheckSync {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::True => f.write_str("true"),
                Self::False => f.write_str("false"),
                Self::Only => f.write_str("only"),
            }
        }
    }

    impl ::std::str::FromStr for SyncBisyncBodyCheckSync {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "true" => Ok(Self::True),
                "false" => Ok(Self::False),
                "only" => Ok(Self::Only),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SyncBisyncBodyCheckSync {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SyncBisyncBodyCheckSync {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///First remote directory, e.g. `drive:path1`.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum SyncBisyncBodyPath1 {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for SyncBisyncBodyPath1 {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///Second remote directory, e.g. `drive:path2`.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum SyncBisyncBodyPath2 {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for SyncBisyncBodyPath2 {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`SyncBisyncResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SyncBisyncResponse {
        ///Base path for listing files.
        #[serde(rename = "basePath")]
        pub base_path: ::std::string::String,
        ///Path to the Path1 listing file.
        pub listing1: ::std::string::String,
        ///Path to the Path2 listing file.
        pub listing2: ::std::string::String,
        ///Path to the log file.
        #[serde(rename = "logFile")]
        pub log_file: ::std::string::String,
        ///Captured output from the bisync operation.
        pub output: ::std::string::String,
        ///Session name derived from the two filesystems.
        pub session: ::std::string::String,
        ///Absolute path to the bisync working directory.
        #[serde(rename = "workDir")]
        pub work_dir: ::std::string::String,
    }

    ///`SyncCopyBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SyncCopyBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Set to true to replicate empty source directories on the
        /// destination.
        #[serde(
            rename = "createEmptySrcDirs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub create_empty_src_dirs: ::std::option::Option<bool>,
        ///Destination remote path to copy to.
        #[serde(rename = "dstFs")]
        pub dst_fs: SyncCopyBodyDstFs,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Source remote path to copy from.
        #[serde(rename = "srcFs")]
        pub src_fs: SyncCopyBodySrcFs,
    }

    ///Destination remote path to copy to.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum SyncCopyBodyDstFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for SyncCopyBodyDstFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///Source remote path to copy from.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum SyncCopyBodySrcFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for SyncCopyBodySrcFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`SyncMoveBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SyncMoveBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Set to true to create empty source directories on the destination.
        #[serde(
            rename = "createEmptySrcDirs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub create_empty_src_dirs: ::std::option::Option<bool>,
        ///Set to true to delete empty directories from the source after the
        /// move completes.
        #[serde(
            rename = "deleteEmptySrcDirs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub delete_empty_src_dirs: ::std::option::Option<bool>,
        ///Destination remote path that will receive moved files.
        #[serde(rename = "dstFs")]
        pub dst_fs: SyncMoveBodyDstFs,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Source remote path whose contents will be moved.
        #[serde(rename = "srcFs")]
        pub src_fs: SyncMoveBodySrcFs,
    }

    ///Destination remote path that will receive moved files.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum SyncMoveBodyDstFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for SyncMoveBodyDstFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///Source remote path whose contents will be moved.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum SyncMoveBodySrcFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for SyncMoveBodySrcFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`SyncSyncBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SyncSyncBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Set to true to create empty source directories on the destination.
        #[serde(
            rename = "createEmptySrcDirs",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub create_empty_src_dirs: ::std::option::Option<bool>,
        ///Destination remote path to sync to, e.g. `drive:dst`.
        #[serde(rename = "dstFs")]
        pub dst_fs: SyncSyncBodyDstFs,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Source remote path to sync from, e.g. `drive:src`.
        #[serde(rename = "srcFs")]
        pub src_fs: SyncSyncBodySrcFs,
    }

    ///Destination remote path to sync to, e.g. `drive:dst`.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum SyncSyncBodyDstFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for SyncSyncBodyDstFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///Source remote path to sync from, e.g. `drive:src`.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum SyncSyncBodySrcFs {
        String(::std::string::String),
        RemoteDefinition(RemoteDefinition),
    }

    impl ::std::convert::From<RemoteDefinition> for SyncSyncBodySrcFs {
        fn from(value: RemoteDefinition) -> Self {
            Self::RemoteDefinition(value)
        }
    }

    ///`VfsForgetBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsForgetBody {
        ///Directory to forget. Add more with keys that start with `dir`, such
        /// as `dir2`.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub dir: ::std::option::Option<::std::string::String>,
        ///File to forget. Add more with keys that start with `file`, such as
        /// `file2`.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub file: ::std::option::Option<::std::string::String>,
        ///Optional VFS identifier to target; required when more than one VFS
        /// is active.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub fs: ::std::option::Option<::std::string::String>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        #[serde(flatten)]
        pub extra: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    }

    ///`VfsForgetResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsForgetResponse {
        ///Paths that were successfully forgotten.
        pub forgotten: ::std::vec::Vec<::std::string::String>,
    }

    ///`VfsListResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsListResponse {
        ///VFS name that can be used with other VFS endpoints.
        pub vfses: ::std::vec::Vec<::std::string::String>,
    }

    ///`VfsPollIntervalBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct VfsPollIntervalBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Optional VFS identifier whose poll interval should be queried or
        /// modified.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub fs: ::std::option::Option<::std::string::String>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Duration string (e.g. `5m`) to set as the new poll interval.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub interval: ::std::option::Option<::std::string::String>,
        ///Duration to wait for the poll interval change to take effect; `0`
        /// waits indefinitely.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub timeout: ::std::option::Option<::std::string::String>,
    }

    ///`VfsPollIntervalResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsPollIntervalResponse {
        ///True if polling is on.
        pub enabled: bool,
        pub interval: VfsPollIntervalResponseInterval,
        ///True if the remote supports polling.
        pub supported: bool,
        ///True if setting the interval timed out. Only present when setting
        /// it.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub timeout: ::std::option::Option<bool>,
    }

    ///The polling interval.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsPollIntervalResponseInterval {
        ///Interval in nanoseconds.
        pub raw: i64,
        ///Interval in whole seconds.
        pub seconds: i64,
        ///Interval as text, such as `1m0s`.
        pub string: ::std::string::String,
    }

    ///`VfsQueueBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct VfsQueueBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Optional VFS identifier whose upload queue should be inspected.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub fs: ::std::option::Option<::std::string::String>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`VfsQueueResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct VfsQueueResponse {
        ///Files waiting to upload. Absent when the VFS has no cache.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub queue: ::std::vec::Vec<VfsQueueResponseQueueItem>,
    }

    ///`VfsQueueResponseQueueItem`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsQueueResponseQueueItem {
        ///Delay between upload attempts in seconds.
        pub delay: f64,
        ///Seconds until the file is eligible for transfer.
        pub expiry: f64,
        ///Queue item ID, for `vfs/queue-set-expiry`.
        pub id: i64,
        ///Full path of the file.
        pub name: ::std::string::String,
        ///Size in bytes.
        pub size: i64,
        ///Upload attempts so far.
        pub tries: i64,
        ///True if the file is uploading now.
        pub uploading: bool,
    }

    ///`VfsQueueSetExpiryBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsQueueSetExpiryBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///New eligibility time in seconds (may be negative for immediate
        /// upload).
        pub expiry: f64,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Optional VFS identifier for the queued item.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub fs: ::std::option::Option<::std::string::String>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Queue item ID as returned by `vfs/queue`.
        pub id: i64,
        ///Set to true to treat `expiry` as relative to the current value.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub relative: ::std::option::Option<bool>,
    }

    ///`VfsRefreshBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsRefreshBody {
        ///Directory to refresh. Add more with keys that start with `dir`, such
        /// as `dir2`.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub dir: ::std::option::Option<::std::string::String>,
        ///Optional VFS identifier whose directory cache should be refreshed.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub fs: ::std::option::Option<::std::string::String>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
        ///Must be the string `true` or `false`, not a JSON boolean.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub recursive: ::std::option::Option<VfsRefreshBodyRecursive>,
        #[serde(flatten)]
        pub extra: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    }

    ///Must be the string `true` or `false`, not a JSON boolean.
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
    pub enum VfsRefreshBodyRecursive {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
    }

    impl ::std::fmt::Display for VfsRefreshBodyRecursive {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::True => f.write_str("true"),
                Self::False => f.write_str("false"),
            }
        }
    }

    impl ::std::str::FromStr for VfsRefreshBodyRecursive {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "true" => Ok(Self::True),
                "false" => Ok(Self::False),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for VfsRefreshBodyRecursive {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for VfsRefreshBodyRecursive {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`VfsRefreshResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsRefreshResponse {
        ///Map of refreshed directories to status messages.
        pub result: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    }

    ///`VfsStatsBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct VfsStatsBody {
        ///Config overrides for this call only.
        #[serde(
            rename = "_config",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub config: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Filter overrides for this call only.
        #[serde(
            rename = "_filter",
            default,
            skip_serializing_if = "::serde_json::Map::is_empty"
        )]
        pub filter: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Optional VFS identifier whose statistics should be returned.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub fs: ::std::option::Option<::std::string::String>,
        ///Stats group for this call.
        #[serde(
            rename = "_group",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<::std::string::String>,
    }

    ///`VfsStatsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VfsStatsResponse {
        ///Disk cache metrics when caching is enabled.
        #[serde(
            rename = "diskCache",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub disk_cache: ::std::option::Option<VfsStatsResponseDiskCache>,
        ///Name of the VFS.
        pub fs: ::std::string::String,
        ///Number of active references to the VFS.
        #[serde(rename = "inUse")]
        pub in_use: i64,
        #[serde(rename = "metadataCache")]
        pub metadata_cache: VfsStatsResponseMetadataCache,
        pub opt: VfsStatsResponseOpt,
    }

    ///Disk cache metrics when caching is enabled.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct VfsStatsResponseDiskCache {
        ///Bytes used by the cache.
        #[serde(
            rename = "bytesUsed",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bytes_used: ::std::option::Option<i64>,
        ///Files with errors.
        #[serde(
            rename = "erroredFiles",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub errored_files: ::std::option::Option<i64>,
        ///Files in the cache.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub files: ::std::option::Option<i64>,
        ///Internal number of the hash the cache uses.
        #[serde(
            rename = "hashType",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub hash_type: ::std::option::Option<i64>,
        ///True if the cache has run out of space.
        #[serde(
            rename = "outOfSpace",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub out_of_space: ::std::option::Option<bool>,
        ///Directory of the cached data.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub path: ::std::option::Option<::std::string::String>,
        ///Directory of the cached metadata.
        #[serde(
            rename = "pathMeta",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub path_meta: ::std::option::Option<::std::string::String>,
        ///Uploads running now.
        #[serde(
            rename = "uploadsInProgress",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub uploads_in_progress: ::std::option::Option<i64>,
        ///Uploads waiting to run.
        #[serde(
            rename = "uploadsQueued",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub uploads_queued: ::std::option::Option<i64>,
    }

    ///In-memory metadata cache counters.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct VfsStatsResponseMetadataCache {
        ///Directories cached.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub dirs: ::std::option::Option<i64>,
        ///Files cached.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub files: ::std::option::Option<i64>,
    }

    ///Effective options of the VFS. Durations are in nanoseconds, and sizes
    /// are in bytes.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    pub struct VfsStatsResponseOpt {
        #[serde(
            rename = "BlockNormDupes",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub block_norm_dupes: ::std::option::Option<bool>,
        #[serde(
            rename = "CacheMaxAge",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cache_max_age: ::std::option::Option<i64>,
        #[serde(
            rename = "CacheMaxSize",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cache_max_size: ::std::option::Option<i64>,
        #[serde(
            rename = "CacheMinFreeSpace",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cache_min_free_space: ::std::option::Option<i64>,
        #[serde(
            rename = "CacheMode",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cache_mode: ::std::option::Option<i64>,
        #[serde(
            rename = "CachePollInterval",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cache_poll_interval: ::std::option::Option<i64>,
        #[serde(
            rename = "CaseInsensitive",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub case_insensitive: ::std::option::Option<bool>,
        #[serde(
            rename = "ChunkSize",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub chunk_size: ::std::option::Option<i64>,
        #[serde(
            rename = "ChunkSizeLimit",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub chunk_size_limit: ::std::option::Option<i64>,
        #[serde(
            rename = "ChunkStreams",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub chunk_streams: ::std::option::Option<i64>,
        #[serde(
            rename = "DirCacheTime",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dir_cache_time: ::std::option::Option<i64>,
        #[serde(
            rename = "DirPerms",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dir_perms: ::std::option::Option<i64>,
        #[serde(
            rename = "DiskSpaceTotalSize",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub disk_space_total_size: ::std::option::Option<i64>,
        #[serde(
            rename = "FastFingerprint",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fast_fingerprint: ::std::option::Option<bool>,
        #[serde(
            rename = "FilePerms",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub file_perms: ::std::option::Option<i64>,
        #[serde(rename = "GID", skip_serializing_if = "::std::option::Option::is_none")]
        pub gid: ::std::option::Option<i64>,
        #[serde(
            rename = "HandleCaching",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub handle_caching: ::std::option::Option<i64>,
        #[serde(
            rename = "LinkPerms",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub link_perms: ::std::option::Option<i64>,
        #[serde(
            rename = "Links",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub links: ::std::option::Option<bool>,
        #[serde(
            rename = "MetadataExtension",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub metadata_extension: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "NoChecksum",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_checksum: ::std::option::Option<bool>,
        #[serde(
            rename = "NoModTime",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_mod_time: ::std::option::Option<bool>,
        #[serde(
            rename = "NoSeek",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub no_seek: ::std::option::Option<bool>,
        #[serde(
            rename = "PollInterval",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub poll_interval: ::std::option::Option<i64>,
        #[serde(
            rename = "ReadAhead",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub read_ahead: ::std::option::Option<i64>,
        #[serde(
            rename = "ReadOnly",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub read_only: ::std::option::Option<bool>,
        #[serde(
            rename = "ReadWait",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub read_wait: ::std::option::Option<i64>,
        #[serde(
            rename = "Refresh",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub refresh: ::std::option::Option<bool>,
        #[serde(rename = "UID", skip_serializing_if = "::std::option::Option::is_none")]
        pub uid: ::std::option::Option<i64>,
        #[serde(
            rename = "Umask",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub umask: ::std::option::Option<i64>,
        #[serde(
            rename = "UsedIsSize",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub used_is_size: ::std::option::Option<bool>,
        #[serde(
            rename = "WriteBack",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub write_back: ::std::option::Option<i64>,
        #[serde(
            rename = "WriteWait",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub write_wait: ::std::option::Option<i64>,
    }

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
}

#[derive(Clone, Debug)]
///Client for Rclone RC API
///
///OpenAPI specification for the rclone RC API. Each release describes exactly
/// one rclone version, shown in `info.version`.
///
///## Scope
///
///This covers the `POST` RC commands with JSON bodies. It doesn't cover
/// rclone's file serving, `/metrics` or the web GUI.
///
///Rclone also accepts these params in the query string or a form-encoded body,
/// and an `_async` param. This spec leaves those out on purpose, so that each
/// field has one documented form and `required` can be enforced. The exception
/// is `operations/uploadfile`, whose params can only go in the query string.
///
///A request with `Content-Type: application/json` and an empty body returns a
/// 400. Send `{}`, or leave the header out. JSON bodies always cause a browser
/// preflight, which rclone answers when `--rc-allow-origin` is set.
///
///## Authentication
///
///Routes use HTTP basic auth, with credentials from `--rc-user` and
/// `--rc-pass`, or from `--rc-htpasswd`. With `--rc-user-from-header`, rclone
/// takes the user name from an HTTP header instead. A missing or wrong login
/// gets a plain text 401.
///
///A server with no credentials set up answers these routes with a JSON 403,
/// unless it runs with `--rc-no-auth`. Routes with an empty `security` list
/// never need authentication.
///
///## Running calls in the background
///
///Send `Prefer: respond-async` to run a call in the background. Rclone returns
/// a 202 with a job ID right away, and `job/status` reports progress. Rclone's
/// `_async` param has the same effect, but on its own it returns a 200 with the
/// job ID rather than the route's normal body. That can't be described here, so
/// routes don't list it. The exception is `job/batch` inputs, which have no
/// headers and use `_async` instead.
///
///## What depends on the server
///
/// - `cache/*` only exists after a cache remote is created.
/// - `pluginsctl/*` only works with the web GUI.
/// - `mount/*` and the option blocks depend on how rclone was built.
/// - `backend/command` results depend on the backend.
///
///
///Version: 1.75.2
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
        "1.75.2"
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
    pub async fn rc_noop<'a>(
        &'a self,
        body: &'a types::RcNoopBody,
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
            .json(&body)
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
    pub async fn operations_cleanup<'a>(
        &'a self,
        body: &'a types::OperationsCleanupBody,
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
            .json(&body)
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
    pub async fn operations_copyfile<'a>(
        &'a self,
        body: &'a types::OperationsCopyfileBody,
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
            .json(&body)
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
    pub async fn operations_copyurl<'a>(
        &'a self,
        body: &'a types::OperationsCopyurlBody,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/operations/copyurl", self.baseurl,);
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
    pub async fn operations_delete<'a>(
        &'a self,
        body: &'a types::OperationsDeleteBody,
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
            .json(&body)
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
    pub async fn operations_deletefile<'a>(
        &'a self,
        body: &'a types::OperationsDeletefileBody,
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
            .json(&body)
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
    pub async fn operations_fsinfo<'a>(
        &'a self,
        body: &'a types::OperationsFsinfoBody,
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
            .json(&body)
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
    pub async fn operations_hashsum<'a>(
        &'a self,
        body: &'a types::OperationsHashsumBody,
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
            .json(&body)
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

    ///Hash a single file
    ///
    ///Returns the hash of a single file using the specified hash algorithm.
    ///
    ///Sends a `POST` request to `/operations/hashsumfile`
    pub async fn operations_hashsumfile<'a>(
        &'a self,
        body: &'a types::OperationsHashsumfileBody,
    ) -> Result<ResponseValue<types::OperationsHashsumfileResponse>, Error<types::RcError>> {
        let url = format!("{}/operations/hashsumfile", self.baseurl,);
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "operations_hashsumfile",
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
    pub async fn operations_movefile<'a>(
        &'a self,
        body: &'a types::OperationsMovefileBody,
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
            .json(&body)
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
    pub async fn operations_publiclink<'a>(
        &'a self,
        body: &'a types::OperationsPubliclinkBody,
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
            .json(&body)
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
    pub async fn operations_rmdirs<'a>(
        &'a self,
        body: &'a types::OperationsRmdirsBody,
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
            .json(&body)
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
    pub async fn operations_settier<'a>(
        &'a self,
        body: &'a types::OperationsSettierBody,
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
            .json(&body)
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
    pub async fn operations_settierfile<'a>(
        &'a self,
        body: &'a types::OperationsSettierfileBody,
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
            .json(&body)
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
    pub async fn operations_size<'a>(
        &'a self,
        body: &'a types::OperationsSizeBody,
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
            .json(&body)
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
    pub async fn core_bwlimit<'a>(
        &'a self,
        body: &'a types::CoreBwlimitBody,
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
            .json(&body)
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
    /// its output. Don't combine the `STREAM` return types with `Prefer:
    /// respond-async`, because the 202 is sent before the command runs.
    ///
    ///Sends a `POST` request to `/core/command`
    pub async fn core_command<'a>(
        &'a self,
        body: &'a types::CoreCommandBody,
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
            .json(&body)
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

    ///List locally accessible paths
    ///
    ///Returns a list of locally accessible paths including mount points, user
    /// directories, and removable volumes.
    ///
    ///Sends a `POST` request to `/core/disks`
    pub async fn core_disks<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
    ) -> Result<ResponseValue<types::CoreDisksResponse>, Error<types::RcError>> {
        let url = format!("{}/core/disks", self.baseurl,);
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "core_disks",
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
    pub async fn core_du<'a>(
        &'a self,
        body: &'a types::CoreDuBody,
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
            .json(&body)
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
    pub async fn core_gc<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn core_group_list<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn core_memstats<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
    ) -> Result<ResponseValue<types::CoreMemstatsResponse>, Error<types::RcError>> {
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
            .json(&body)
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
    pub async fn core_obscure<'a>(
        &'a self,
        body: &'a types::CoreObscureBody,
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
            .json(&body)
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
    pub async fn core_pid<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn core_quit<'a>(
        &'a self,
        body: &'a types::CoreQuitBody,
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
            .json(&body)
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
    pub async fn core_stats_delete<'a>(
        &'a self,
        body: &'a types::CoreStatsDeleteBody,
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
            .json(&body)
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
    pub async fn core_stats_reset<'a>(
        &'a self,
        body: &'a types::CoreStatsResetBody,
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
            .json(&body)
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
    pub async fn core_transferred<'a>(
        &'a self,
        body: &'a types::CoreTransferredBody,
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
            .json(&body)
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

    ///Set the block profile rate
    ///
    ///Controls the fraction of goroutine blocking events reported in the
    /// blocking profile. Use a rate of 1 to include every event, or 0 or less
    /// to turn profiling off.
    ///
    ///Sends a `POST` request to `/debug/set-block-profile-rate`
    pub async fn debug_set_block_profile_rate<'a>(
        &'a self,
        body: &'a types::DebugSetBlockProfileRateBody,
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
            .json(&body)
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

    ///Set the GC target percentage
    ///
    ///Sets the garbage collection target percentage and returns the previous
    /// value. A negative percentage effectively turns garbage collection off,
    /// unless the memory limit is reached.
    ///
    ///Sends a `POST` request to `/debug/set-gc-percent`
    pub async fn debug_set_gc_percent<'a>(
        &'a self,
        body: &'a types::DebugSetGcPercentBody,
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
            .json(&body)
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

    ///Set the mutex profile fraction
    ///
    ///Controls the fraction of mutex contention events reported in the mutex
    /// profile, and returns the previous rate. Use 0 to turn profiling off, or
    /// a negative rate to only read the current value.
    ///
    ///Sends a `POST` request to `/debug/set-mutex-profile-fraction`
    pub async fn debug_set_mutex_profile_fraction<'a>(
        &'a self,
        body: &'a types::DebugSetMutexProfileFractionBody,
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
            .json(&body)
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

    ///Set the soft memory limit
    ///
    ///Sets a soft memory limit for the Go runtime, in bytes, and returns the
    /// previous limit.
    ///
    ///Sends a `POST` request to `/debug/set-soft-memory-limit`
    pub async fn debug_set_soft_memory_limit<'a>(
        &'a self,
        body: &'a types::DebugSetSoftMemoryLimitBody,
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
            .json(&body)
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

    ///Clear the remote cache
    ///
    ///Clears the cache of remotes created from backends. Call this after
    /// changing a backend's parameters, so that the next call creates the
    /// remote again.
    ///
    ///Sends a `POST` request to `/fscache/clear`
    pub async fn fscache_clear<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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

    ///Count cached remotes
    ///
    ///Returns the number of entries in the cache of remotes.
    ///
    ///Sends a `POST` request to `/fscache/entries`
    pub async fn fscache_entries<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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

    ///List mount points
    ///
    ///Returns the current mount points, which can be passed to
    /// `mount/unmount`.
    ///
    ///Sends a `POST` request to `/mount/listmounts`
    pub async fn mount_listmounts<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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

    ///Create a mount point
    ///
    ///Mounts a remote as a file system with FUSE. If `mountType` isn't set,
    /// rclone tries `mount`, then `cmount`, then `mount2`. Mount and VFS
    /// options can also be given as top-level keys. Rclone rejects any other
    /// key.
    ///
    ///Sends a `POST` request to `/mount/mount`
    pub async fn mount_mount<'a>(
        &'a self,
        body: &'a types::MountMountBody,
    ) -> Result<ResponseValue<types::MountMountResponse>, Error<types::RcError>> {
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
            .json(&body)
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

    ///List mount types
    ///
    ///Returns the possible mount types, which can be passed to `mount/mount`
    /// as `mountType`.
    ///
    ///Sends a `POST` request to `/mount/types`
    pub async fn mount_types<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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

    ///Unmount a mount point
    ///
    ///Unmounts the mount at `mountPoint`.
    ///
    ///Sends a `POST` request to `/mount/unmount`
    pub async fn mount_unmount<'a>(
        &'a self,
        body: &'a types::MountUnmountBody,
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
            .json(&body)
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

    ///Unmount everything
    ///
    ///Unmounts all active mounts, and returns an error if an unmount fails.
    ///
    ///Sends a `POST` request to `/mount/unmountall`
    pub async fn mount_unmountall<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn rc_noop_auth<'a>(
        &'a self,
        body: &'a types::RcNoopAuthBody,
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
            .json(&body)
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
    pub async fn rc_error<'a>(
        &'a self,
        body: &'a types::RcErrorBody,
    ) -> Result<ResponseValue<ByteStream>, Error<types::RcError>> {
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
            .json(&body)
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
            200..=299 => Ok(ResponseValue::stream(response)),
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
    pub async fn rc_list<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn backend_command<'a>(
        &'a self,
        body: &'a types::BackendCommandBody,
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
            .json(&body)
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
    pub async fn cache_expire<'a>(
        &'a self,
        body: &'a types::CacheExpireBody,
    ) -> Result<ResponseValue<types::CacheExpireResponse>, Error<types::RcError>> {
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
            .json(&body)
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

    ///Prefetch cache chunks
    ///
    ///Ensures specified file chunks are cached locally for a cache remote.
    /// Rclone also accepts `_config` and `_filter` objects here, but this
    /// schema leaves them out, because it can't allow objects for only those
    /// two keys.
    ///
    ///Sends a `POST` request to `/cache/fetch`
    pub async fn cache_fetch<'a>(
        &'a self,
        body: &'a types::CacheFetchBody,
    ) -> Result<ResponseValue<types::CacheFetchResponse>, Error<types::RcError>> {
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
            .json(&body)
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

    ///Show cache stats
    ///
    ///Returns runtime statistics for the cache backend.
    ///
    ///Sends a `POST` request to `/cache/stats`
    pub async fn cache_stats<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
    ) -> Result<ResponseValue<types::CacheStatsResponse>, Error<types::RcError>> {
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
            .json(&body)
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

    ///Create remote configuration
    ///
    ///Creates a new remote in `rclone.conf`, mirroring `rclone config create`.
    ///
    ///Sends a `POST` request to `/config/create`
    pub async fn config_create<'a>(
        &'a self,
        body: &'a types::ConfigCreateBody,
    ) -> Result<ResponseValue<types::ConfigCreateResponse>, Error<types::RcError>> {
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
            .json(&body)
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
    pub async fn config_delete<'a>(
        &'a self,
        body: &'a types::ConfigDeleteBody,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
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
            .json(&body)
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

    ///Dump configuration
    ///
    ///Returns the contents of the config file as a JSON object keyed by remote
    /// name.
    ///
    ///Sends a `POST` request to `/config/dump`
    pub async fn config_dump<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn config_get<'a>(
        &'a self,
        body: &'a types::ConfigGetBody,
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
            .json(&body)
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
    pub async fn config_listremotes<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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

    ///Get OAuth server status
    ///
    ///Returns the status of the interactive OAuth authentication server,
    /// including the authorization URL when it is running.
    ///
    ///Sends a `POST` request to `/config/oauthstatus`
    pub async fn config_oauthstatus<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
    ) -> Result<ResponseValue<types::ConfigOauthstatusResponse>, Error<types::RcError>> {
        let url = format!("{}/config/oauthstatus", self.baseurl,);
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "config_oauthstatus",
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

    ///Stop OAuth server
    ///
    ///Stops the interactive OAuth authentication server if one is running.
    /// Returns an error if no OAuth flow is in progress.
    ///
    ///Sends a `POST` request to `/config/oauthstop`
    pub async fn config_oauthstop<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/config/oauthstop", self.baseurl,);
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "config_oauthstop",
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
    pub async fn config_password<'a>(
        &'a self,
        body: &'a types::ConfigPasswordBody,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
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
            .json(&body)
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

    ///Show config paths
    ///
    ///Returns the paths to the config file, cache directory, and temporary
    /// directory.
    ///
    ///Sends a `POST` request to `/config/paths`
    pub async fn config_paths<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn config_providers<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn config_setpath<'a>(
        &'a self,
        body: &'a types::ConfigSetpathBody,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
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
            .json(&body)
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

    ///Unlock encrypted config
    ///
    ///Unlocks the configuration file using the provided password.
    ///
    ///Sends a `POST` request to `/config/unlock`
    pub async fn config_unlock<'a>(
        &'a self,
        body: &'a types::ConfigUnlockBody,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
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
            .json(&body)
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

    ///Update remote configuration
    ///
    ///Updates an existing remote with new parameter values.
    ///
    ///Sends a `POST` request to `/config/update`
    pub async fn config_update<'a>(
        &'a self,
        body: &'a types::ConfigUpdateBody,
    ) -> Result<ResponseValue<types::ConfigUpdateResponse>, Error<types::RcError>> {
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
            .json(&body)
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
    pub async fn core_version<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn core_stats<'a>(
        &'a self,
        body: &'a types::CoreStatsBody,
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
            .json(&body)
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
    ///Run a batch of rclone rc commands concurrently. An input can set
    /// `_async` to run in the background, and its entry in `results` is then a
    /// job ID. Inputs have no headers, so they can't use `Prefer`.
    ///
    ///Sends a `POST` request to `/job/batch`
    pub async fn job_batch<'a>(
        &'a self,
        body: &'a types::JobBatchBody,
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
    /// Since rclone 1.75.2 this call requires authentication, so a server with
    /// no credentials set up returns 403 unless it runs with `--rc-no-auth`.
    ///
    ///Sends a `POST` request to `/job/list`
    pub async fn job_list<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    /// Since rclone 1.75.2 this call requires authentication, so a server with
    /// no credentials set up returns 403 unless it runs with `--rc-no-auth`.
    ///
    ///Sends a `POST` request to `/job/status`
    pub async fn job_status<'a>(
        &'a self,
        body: &'a types::JobStatusBody,
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
            .json(&body)
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
    pub async fn job_stop<'a>(
        &'a self,
        body: &'a types::JobStopBody,
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
            .json(&body)
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
    pub async fn job_stopgroup<'a>(
        &'a self,
        body: &'a types::JobStopgroupBody,
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
            .json(&body)
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
    pub async fn operations_list<'a>(
        &'a self,
        body: &'a types::OperationsListBody,
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
            .json(&body)
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
    pub async fn operations_stat<'a>(
        &'a self,
        body: &'a types::OperationsStatBody,
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
            .json(&body)
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
    pub async fn operations_about<'a>(
        &'a self,
        body: &'a types::OperationsAboutBody,
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
            .json(&body)
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

    ///Purge directory
    ///
    ///Deletes a directory or container and all of its contents. This can take
    /// a long time, so run it in the background with `Prefer: respond-async` to
    /// avoid HTTP timeouts.
    ///
    ///Sends a `POST` request to `/operations/purge`
    pub async fn operations_purge<'a>(
        &'a self,
        body: &'a types::OperationsPurgeBody,
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
            .json(&body)
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
    pub async fn operations_mkdir<'a>(
        &'a self,
        body: &'a types::OperationsMkdirBody,
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
            .json(&body)
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
    pub async fn operations_rmdir<'a>(
        &'a self,
        body: &'a types::OperationsRmdirBody,
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
            .json(&body)
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
    pub async fn operations_check<'a>(
        &'a self,
        body: &'a types::OperationsCheckBody,
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
            .json(&body)
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
    /// destination match the source. This can take a long time, so run it in
    /// the background with `Prefer: respond-async` to avoid HTTP timeouts.
    ///
    ///Sends a `POST` request to `/sync/sync`
    pub async fn sync_sync<'a>(
        &'a self,
        body: &'a types::SyncSyncBody,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
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
            .json(&body)
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
    /// deleting destination files. This can take a long time, so run it in the
    /// background with `Prefer: respond-async` to avoid HTTP timeouts.
    ///
    ///Sends a `POST` request to `/sync/copy`
    pub async fn sync_copy<'a>(
        &'a self,
        body: &'a types::SyncCopyBody,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
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
            .json(&body)
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
    /// cleaning up empty directories. This can take a long time, so run it in
    /// the background with `Prefer: respond-async` to avoid HTTP timeouts.
    ///
    ///Sends a `POST` request to `/sync/move`
    pub async fn sync_move<'a>(
        &'a self,
        body: &'a types::SyncMoveBody,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
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
            .json(&body)
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
    pub async fn sync_bisync<'a>(
        &'a self,
        body: &'a types::SyncBisyncBody,
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
            .json(&body)
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
    pub async fn options_blocks<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn options_get<'a>(
        &'a self,
        body: &'a types::OptionsGetBody,
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
            .json(&body)
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
    pub async fn options_info<'a>(
        &'a self,
        body: &'a types::OptionsInfoBody,
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
            .json(&body)
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
    pub async fn options_set<'a>(
        &'a self,
        body: &'a types::OptionsSetBody,
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
            .json(&body)
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
    pub async fn options_local<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn serve_list<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    /// the provided parameters. It also takes the options of the chosen server
    /// type, using the option names from `options/info`. Rclone rejects any
    /// other key.
    ///
    ///Sends a `POST` request to `/serve/start`
    pub async fn serve_start<'a>(
        &'a self,
        body: &'a types::ServeStartBody,
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
            .json(&body)
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
    pub async fn serve_stop<'a>(
        &'a self,
        body: &'a types::ServeStopBody,
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
            .json(&body)
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
    pub async fn serve_stopall<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn serve_types<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
    ) -> Result<ResponseValue<types::ServeTypesResponse>, Error<types::RcError>> {
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
            .json(&body)
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

    ///Forget cached paths
    ///
    ///Evicts specific files or directories from the VFS directory cache. All
    /// values must be strings. Rclone also accepts `_config` and `_filter`
    /// objects here, but this schema leaves them out, because it can't allow
    /// objects for only those two keys.
    ///
    ///Sends a `POST` request to `/vfs/forget`
    pub async fn vfs_forget<'a>(
        &'a self,
        body: &'a types::VfsForgetBody,
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
            .json(&body)
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
    pub async fn vfs_list<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn vfs_poll_interval<'a>(
        &'a self,
        body: &'a types::VfsPollIntervalBody,
    ) -> Result<ResponseValue<types::VfsPollIntervalResponse>, Error<types::RcError>> {
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
            .json(&body)
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
    pub async fn vfs_queue<'a>(
        &'a self,
        body: &'a types::VfsQueueBody,
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
            .json(&body)
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
    pub async fn vfs_queue_set_expiry<'a>(
        &'a self,
        body: &'a types::VfsQueueSetExpiryBody,
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
            .json(&body)
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
    /// recursively. All values must be strings. Rclone also accepts `_config`
    /// and `_filter` objects here, but this schema leaves them out, because it
    /// can't allow objects for only those two keys.
    ///
    ///Sends a `POST` request to `/vfs/refresh`
    pub async fn vfs_refresh<'a>(
        &'a self,
        body: &'a types::VfsRefreshBody,
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
            .json(&body)
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
    pub async fn vfs_stats<'a>(
        &'a self,
        body: &'a types::VfsStatsBody,
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
            .json(&body)
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
    pub async fn pluginsctl_add_plugin<'a>(
        &'a self,
        body: &'a types::PluginsctlAddPluginBody,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
        let url = format!("{}/pluginsctl/addPlugin", self.baseurl,);
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
    pub async fn pluginsctl_get_plugins_for_type<'a>(
        &'a self,
        body: &'a types::PluginsctlGetPluginsForTypeBody,
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
            .json(&body)
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
    pub async fn pluginsctl_list_plugins<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn pluginsctl_list_test_plugins<'a>(
        &'a self,
        body: &'a types::RcGlobalOptions,
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
            .json(&body)
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
    pub async fn pluginsctl_remove_plugin<'a>(
        &'a self,
        body: &'a types::PluginsctlRemovePluginBody,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
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
            .json(&body)
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

    ///Remove test plugin
    ///
    ///Uninstalls a test plugin from the WebUI.
    ///
    ///Sends a `POST` request to `/pluginsctl/removeTestPlugin`
    pub async fn pluginsctl_remove_test_plugin<'a>(
        &'a self,
        body: &'a types::PluginsctlRemoveTestPluginBody,
    ) -> Result<
        ResponseValue<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        Error<types::RcError>,
    > {
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
            .json(&body)
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

    ///Unset keys in a remote
    ///
    ///Removes keys from a remote in the config file.
    ///
    ///Sends a `POST` request to `/config/unset`
    pub async fn config_unset<'a>(
        &'a self,
        body: &'a types::ConfigUnsetBody,
    ) -> Result<ResponseValue<types::ConfigUnsetResponse>, Error<types::RcError>> {
        let url = format!("{}/config/unset", self.baseurl,);
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "config_unset",
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
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}

impl From<&str> for types::BackendCommandBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::BackendCommandBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::MountMountBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::MountMountBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsAboutBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsAboutBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsCheckBodyCheckFileFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsCheckBodyCheckFileFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsCheckBodyDstFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsCheckBodyDstFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsCheckBodySrcFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsCheckBodySrcFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsCleanupBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsCleanupBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsCopyfileBodyDstFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsCopyfileBodyDstFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsCopyfileBodySrcFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsCopyfileBodySrcFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsCopyurlBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsCopyurlBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsDeleteBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsDeleteBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsDeletefileBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsDeletefileBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsFsinfoBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsFsinfoBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsHashsumBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsHashsumBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsHashsumfileBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsHashsumfileBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsListBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsListBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsMkdirBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsMkdirBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsMovefileBodyDstFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsMovefileBodyDstFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsMovefileBodySrcFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsMovefileBodySrcFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsPubliclinkBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsPubliclinkBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsPurgeBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsPurgeBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsRmdirBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsRmdirBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsRmdirsBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsRmdirsBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsSettierBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsSettierBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsSettierfileBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsSettierfileBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsSizeBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsSizeBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::OperationsStatBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::OperationsStatBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::ServeListResponseListItemParamsFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::ServeListResponseListItemParamsFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::ServeStartBodyFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::ServeStartBodyFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::SyncBisyncBodyPath1 {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::SyncBisyncBodyPath1 {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::SyncBisyncBodyPath2 {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::SyncBisyncBodyPath2 {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::SyncCopyBodyDstFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::SyncCopyBodyDstFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::SyncCopyBodySrcFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::SyncCopyBodySrcFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::SyncMoveBodyDstFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::SyncMoveBodyDstFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::SyncMoveBodySrcFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::SyncMoveBodySrcFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::SyncSyncBodyDstFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::SyncSyncBodyDstFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for types::SyncSyncBodySrcFs {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for types::SyncSyncBodySrcFs {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}


// ---- rclone-sdk hand-written overrides (see src/overrides.rs) ----
pub mod overrides;
pub use overrides::*;
