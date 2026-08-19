//! Hand-written overrides for the generated client. Survives regeneration; the
//! gen script wires it in via `pub mod overrides;`.

use crate::types;
use crate::Client;
use progenitor_client::{Error, ResponseValue};

/// Interactive step from `config/create` / `config/update` (with `opt.nonInteractive`).
#[derive(::serde::Serialize, ::serde::Deserialize, Debug, Clone, Default)]
pub struct ConfigStep {
    #[serde(rename = "State", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Option", default, skip_serializing_if = "Option::is_none")]
    pub option: Option<::serde_json::Value>,
    #[serde(rename = "Error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "Result", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<String, ::serde_json::Value>,
}

/// Job handle from an async submission (poll `job/status` with `jobid`).
#[derive(::serde::Serialize, ::serde::Deserialize, Debug, Clone, Default)]
pub struct AsyncJob {
    pub jobid: i64,
}

impl Client {
    /// Typed `config/create` (use with `opt.nonInteractive=true`); preserves the
    /// interactive step fields the generated method drops.
    pub async fn config_create_step<'a>(
        &'a self,
        name: Option<&'a str>,
        type_: Option<&'a str>,
        opt: Option<&'a str>,
        parameters: Option<&'a str>,
    ) -> Result<ResponseValue<ConfigStep>, Error<types::RcError>> {
        self.config_step("config/create", name, type_, opt, parameters)
            .await
    }

    /// Typed `config/update` counterpart of [`Client::config_create_step`].
    pub async fn config_update_step<'a>(
        &'a self,
        name: Option<&'a str>,
        opt: Option<&'a str>,
        parameters: Option<&'a str>,
    ) -> Result<ResponseValue<ConfigStep>, Error<types::RcError>> {
        self.config_step("config/update", name, None, opt, parameters)
            .await
    }

    async fn config_step<'a>(
        &'a self,
        path: &'a str,
        name: Option<&'a str>,
        type_: Option<&'a str>,
        opt: Option<&'a str>,
        parameters: Option<&'a str>,
    ) -> Result<ResponseValue<ConfigStep>, Error<types::RcError>> {
        let url = format!("{}/{}", self.baseurl, path);
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(v) = name {
            query.push(("name", v));
        }
        if let Some(v) = type_ {
            query.push(("type", v));
        }
        if let Some(v) = opt {
            query.push(("opt", v));
        }
        if let Some(v) = parameters {
            query.push(("parameters", v));
        }
        let response = self
            .client
            .post(url)
            .header(::reqwest::header::ACCEPT, "application/json")
            .json(&::serde_json::json!({}))
            .query(&query)
            .send()
            .await?;
        match response.status().as_u16() {
            200 => ResponseValue::from_response(response).await,
            400..=599 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Submit any RC operation asynchronously (sets `_async=true` +
    /// `Prefer: respond-async`). Rust analog of the TS `ASYNC`.
    pub async fn post_async<'a>(
        &'a self,
        path: &'a str,
        query: &'a [(&'a str, &'a str)],
        body: &'a ::serde_json::Value,
    ) -> Result<ResponseValue<AsyncJob>, Error<types::RcError>> {
        let url = format!("{}/{}", self.baseurl, path.trim_start_matches('/'));
        let mut q: Vec<(&str, &str)> = query.to_vec();
        q.push(("_async", "true"));
        let response = self
            .client
            .post(url)
            .header(::reqwest::header::ACCEPT, "application/json")
            .header("Prefer", "respond-async")
            .json(body)
            .query(&q)
            .send()
            .await?;
        match response.status().as_u16() {
            200 | 202 => ResponseValue::from_response(response).await,
            400..=599 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
