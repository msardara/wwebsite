//! Shared response-parsing and RPC helper functions.
//!
//! All status-checking and text-extraction flows through [`response_to_text`].
//! Higher-level helpers compose on top of it so error-handling logic is never
//! duplicated.

use super::error::{SupabaseError, SupabaseResult};
use postgrest::Postgrest;

// ============================================================================
// CORE RESPONSE HELPERS
// ============================================================================

/// The single point that checks HTTP status and extracts the response body.
/// Every other helper delegates here so error-handling logic is never duplicated.
pub async fn response_to_text(response: reqwest::Response) -> SupabaseResult<String> {
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(SupabaseError::ServerError(format!(
            "Status: {}, Body: {}",
            status, error_text
        )));
    }

    response
        .text()
        .await
        .map_err(|e| SupabaseError::ParseError(e.to_string()))
}

/// Parse a successful response body as `T`.
pub async fn execute_and_parse<T>(response: reqwest::Response) -> SupabaseResult<T>
where
    T: serde::de::DeserializeOwned,
{
    let text = response_to_text(response).await?;
    serde_json::from_str(&text).map_err(|e| SupabaseError::ParseError(e.to_string()))
}

/// Parse a successful response body as `Vec<T>`, treating 404 / empty as `[]`.
#[allow(dead_code)]
pub async fn execute_and_parse_vec<T>(response: reqwest::Response) -> SupabaseResult<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    if response.status() == 404 {
        return Ok(Vec::new());
    }

    let text = response_to_text(response).await?;

    if text.is_empty() || text == "[]" {
        return Ok(Vec::new());
    }

    serde_json::from_str(&text).map_err(|e| SupabaseError::ParseError(e.to_string()))
}

/// Parse a successful response body as `Option<T>`, treating 404 / empty as `None`.
pub async fn execute_and_parse_option<T>(response: reqwest::Response) -> SupabaseResult<Option<T>>
where
    T: serde::de::DeserializeOwned,
{
    if response.status() == 404 {
        return Ok(None);
    }

    let text = response_to_text(response).await?;

    if text.is_empty() || text == "[]" {
        return Ok(None);
    }

    let items: Vec<T> =
        serde_json::from_str(&text).map_err(|e| SupabaseError::ParseError(e.to_string()))?;

    Ok(items.into_iter().next())
}

/// Parse a successful response as a single `T`, trying object then array.
pub async fn execute_and_parse_single<T>(response: reqwest::Response) -> SupabaseResult<T>
where
    T: serde::de::DeserializeOwned,
{
    let text = response_to_text(response).await?;

    // Try parsing as single object first
    if let Ok(item) = serde_json::from_str::<T>(&text) {
        return Ok(item);
    }

    // Try parsing as array
    let items: Vec<T> =
        serde_json::from_str(&text).map_err(|e| SupabaseError::ParseError(e.to_string()))?;

    items
        .into_iter()
        .next()
        .ok_or(SupabaseError::ServerError("Empty response".to_string()))
}

/// Check a response for success, discarding the body.
pub async fn execute_delete(response: reqwest::Response) -> SupabaseResult<()> {
    response_to_text(response).await?;
    Ok(())
}

// ============================================================================
// RPC HELPERS
//
// Thin wrappers that call `.rpc()`, send the request, then delegate to the
// appropriate parse helper above.
// ============================================================================

/// Fire an RPC call and parse the response as `T`.
pub async fn execute_rpc<T>(
    client: &Postgrest,
    function_name: &str,
    params: String,
) -> SupabaseResult<T>
where
    T: serde::de::DeserializeOwned,
{
    let response = client
        .rpc(function_name, params)
        .execute()
        .await
        .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

    execute_and_parse(response).await
}

/// Fire an RPC call and parse the response as `Option<T>`.
pub async fn execute_rpc_option<T>(
    client: &Postgrest,
    function_name: &str,
    params: String,
) -> SupabaseResult<Option<T>>
where
    T: serde::de::DeserializeOwned,
{
    let response = client
        .rpc(function_name, params)
        .execute()
        .await
        .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

    execute_and_parse_option(response).await
}
