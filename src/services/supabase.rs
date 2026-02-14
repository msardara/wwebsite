// Supabase client service
// This module handles all interactions with Supabase using postgrest-rs
// Split into two clients for better security:
// - SupabaseRpcClient: For anonymous/guest users, only RPC access
// - SupabaseAdminClient: For authenticated admins, full table access

use crate::constants::{
    SUPABASE_AUTH_PATH, SUPABASE_REST_PATH, SUPABASE_STORAGE_PATH, TABLE_GUESTS,
    TABLE_GUEST_GROUPS, TABLE_PHOTOS, TABLE_RSVPS, WEDDING_PHOTOS_BUCKET,
};
use crate::types::{
    AdminStats, AuthResponse, AuthSession, DietaryPreferences, Guest, GuestGroup, GuestGroupInput,
    GuestGroupUpdate, GuestGroupWithCount, GuestInput, GuestUpdate, Location, LoginCredentials,
    Photo, PhotoInput, Rsvp, RsvpWithDietaryCounts,
};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use reqwest;
use serde::{Deserialize, Serialize};

/// Result type for Supabase operations
pub type SupabaseResult<T> = Result<T, SupabaseError>;

/// Error types for Supabase operations
#[derive(Debug, Clone)]
pub enum SupabaseError {
    #[allow(dead_code)]
    NotFound,
    NetworkError(String),
    ParseError(String),
    #[allow(dead_code)]
    ValidationError(String),
    #[allow(dead_code)]
    Unauthorized,
    ServerError(String),
}

impl std::fmt::Display for SupabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupabaseError::NotFound => write!(f, "Resource not found"),
            SupabaseError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            SupabaseError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            SupabaseError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            SupabaseError::Unauthorized => write!(f, "Unauthorized access"),
            SupabaseError::ServerError(msg) => write!(f, "Server error: {}", msg),
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Execute a request and handle the response, parsing as type T
async fn execute_and_parse<T>(response: reqwest::Response) -> SupabaseResult<T>
where
    T: serde::de::DeserializeOwned,
{
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(SupabaseError::ServerError(format!(
            "Status: {}, Body: {}",
            status, error_text
        )));
    }

    let text = response
        .text()
        .await
        .map_err(|e| SupabaseError::ParseError(e.to_string()))?;

    serde_json::from_str(&text).map_err(|e| SupabaseError::ParseError(e.to_string()))
}

/// Execute a request and parse response as a Vec, handling empty/404 responses
async fn execute_and_parse_vec<T>(response: reqwest::Response) -> SupabaseResult<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    if response.status() == 404 {
        return Ok(Vec::new());
    }

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(SupabaseError::ServerError(format!(
            "Status: {}, Body: {}",
            status, error_text
        )));
    }

    let text = response
        .text()
        .await
        .map_err(|e| SupabaseError::ParseError(e.to_string()))?;

    if text.is_empty() || text == "[]" {
        return Ok(Vec::new());
    }

    serde_json::from_str(&text).map_err(|e| SupabaseError::ParseError(e.to_string()))
}

/// Execute a request and parse response as Option<T>, handling 404/empty responses
async fn execute_and_parse_option<T>(response: reqwest::Response) -> SupabaseResult<Option<T>>
where
    T: serde::de::DeserializeOwned,
{
    if response.status() == 404 {
        return Ok(None);
    }

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(SupabaseError::ServerError(format!(
            "Status: {}, Body: {}",
            status, error_text
        )));
    }

    let text = response
        .text()
        .await
        .map_err(|e| SupabaseError::ParseError(e.to_string()))?;

    if text.is_empty() || text == "[]" {
        return Ok(None);
    }

    let items: Vec<T> =
        serde_json::from_str(&text).map_err(|e| SupabaseError::ParseError(e.to_string()))?;

    Ok(items.into_iter().next())
}

/// Execute a request and parse as single item from array response
async fn execute_and_parse_single<T>(response: reqwest::Response) -> SupabaseResult<T>
where
    T: serde::de::DeserializeOwned,
{
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(SupabaseError::ServerError(format!(
            "Status: {}, Body: {}",
            status, error_text
        )));
    }

    let text = response
        .text()
        .await
        .map_err(|e| SupabaseError::ParseError(e.to_string()))?;

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

/// Execute a delete request
async fn execute_delete(response: reqwest::Response) -> SupabaseResult<()> {
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(SupabaseError::ServerError(format!(
            "Status: {}, Body: {}",
            status, error_text
        )));
    }
    Ok(())
}

/// Execute an RPC call with JSON params
async fn execute_rpc<T>(
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

/// Execute an RPC call and parse as optional result
async fn execute_rpc_option<T>(
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

/// Execute an RPC call with logging
async fn execute_rpc_with_logging<T>(
    client: &Postgrest,
    function_name: &str,
    params: String,
    operation: &str,
) -> SupabaseResult<T>
where
    T: serde::de::DeserializeOwned,
{
    web_sys::console::log_1(
        &format!("🔧 Calling {} with params: {}", function_name, params).into(),
    );

    let response = client
        .rpc(function_name, params)
        .execute()
        .await
        .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        web_sys::console::error_1(
            &format!("❌ RPC Error - Status: {}, Body: {}", status, error_text).into(),
        );
        return Err(SupabaseError::ServerError(format!(
            "Status: {}, Body: {}",
            status, error_text
        )));
    }

    let body = response
        .text()
        .await
        .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

    web_sys::console::log_1(&format!("✅ {} response: {}", operation, body).into());

    serde_json::from_str(&body).map_err(|e| SupabaseError::ParseError(e.to_string()))
}

/// Execute an RPC call with logging and flexible parsing (single object or array)
async fn execute_rpc_with_flexible_parsing<T>(
    client: &Postgrest,
    function_name: &str,
    params: String,
) -> SupabaseResult<T>
where
    T: serde::de::DeserializeOwned,
{
    web_sys::console::log_1(
        &format!("🔧 Calling {} with params: {}", function_name, params).into(),
    );

    let response = client
        .rpc(function_name, params)
        .execute()
        .await
        .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        web_sys::console::error_1(
            &format!("❌ RPC Error - Status: {}, Body: {}", status, error_text).into(),
        );
        return Err(SupabaseError::ServerError(format!(
            "Status: {}, Body: {}",
            status, error_text
        )));
    }

    let body = response
        .text()
        .await
        .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

    web_sys::console::log_1(&format!("🔍 RPC Response Body: {}", body).into());

    // Try parsing as single object first (RETURNS record type)
    if let Ok(item) = serde_json::from_str::<T>(&body) {
        web_sys::console::log_1(&"✅ Parsed as single object".into());
        return Ok(item);
    }

    // Try parsing as array (some RPC functions return arrays)
    if let Ok(items) = serde_json::from_str::<Vec<T>>(&body) {
        web_sys::console::log_1(&format!("✅ Parsed as array with {} items", items.len()).into());
        return items.into_iter().next().ok_or(SupabaseError::ServerError(
            "No item returned from array".to_string(),
        ));
    }

    web_sys::console::error_1(&format!("❌ Failed to parse response: {}", body).into());
    Err(SupabaseError::ParseError(format!(
        "Could not parse response. Body: {}",
        body
    )))
}

/// Execute an RPC delete call with logging
async fn execute_rpc_delete_with_logging(
    client: &Postgrest,
    function_name: &str,
    params: String,
    success_message: &str,
) -> SupabaseResult<()> {
    web_sys::console::log_1(
        &format!("🔧 Calling {} with params: {}", function_name, params).into(),
    );

    let response = client
        .rpc(function_name, params)
        .execute()
        .await
        .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        web_sys::console::error_1(
            &format!("❌ RPC Error - Status: {}, Body: {}", status, error_text).into(),
        );
        return Err(SupabaseError::ServerError(format!(
            "Status: {}, Body: {}",
            status, error_text
        )));
    }

    web_sys::console::log_1(&success_message.into());
    Ok(())
}

// ============================================================================
// RPC CLIENT - FOR GUEST USERS (ANONYMOUS ACCESS)
// ============================================================================

/// Supabase RPC client for guest/anonymous users
/// Only has access to secure RPC functions that validate invitation codes
#[derive(Clone)]
pub struct SupabaseRpcClient {
    client: Postgrest,
    #[allow(dead_code)]
    publishable_key: String,
    pub base_url: String,
}

impl SupabaseRpcClient {
    /// Create a new RPC-only Supabase client
    pub fn new() -> Self {
        let url = env!("SUPABASE_URL");
        let publishable_key = env!("SUPABASE_PUBLISHABLE_KEY");

        let client = Postgrest::new(format!("{}{}", url, SUPABASE_REST_PATH))
            .insert_header("apikey", publishable_key)
            .insert_header("Authorization", format!("Bearer {}", publishable_key));

        Self {
            client,
            publishable_key: publishable_key.to_string(),
            base_url: url.to_string(),
        }
    }

    // ========================================================================
    // GUEST GROUP OPERATIONS (RPC)
    // ========================================================================

    /// Find a guest group by their invitation code using secure RPC function
    pub async fn find_guest_by_code(&self, code: &str) -> SupabaseResult<Option<GuestGroup>> {
        execute_rpc_option(
            &self.client,
            "authenticate_guest_group",
            format!(r#"{{"code":"{}"}}"#, code),
        )
        .await
    }

    /// Update party_size for a guest group with invitation code validation (RPC)
    pub async fn update_guest_group_party_size(
        &self,
        guest_group_id: &str,
        invitation_code: &str,
        new_party_size: i32,
    ) -> SupabaseResult<GuestGroup> {
        let params = serde_json::json!({
            "p_guest_group_id": guest_group_id,
            "p_invitation_code": invitation_code,
            "p_new_party_size": new_party_size
        });

        execute_rpc(
            &self.client,
            "update_guest_group_party_size",
            params.to_string(),
        )
        .await
    }

    // ========================================================================
    // GUEST OPERATIONS (RPC)
    // ========================================================================

    /// Get all guests for a guest group with invitation code validation (RPC)
    pub async fn get_guests(
        &self,
        guest_group_id: &str,
        invitation_code: &str,
    ) -> SupabaseResult<Vec<Guest>> {
        execute_rpc(
            &self.client,
            "get_guests_for_group",
            serde_json::json!({
                "p_guest_group_id": guest_group_id,
                "p_invitation_code": invitation_code
            })
            .to_string(),
        )
        .await
    }

    /// Create a new guest with invitation code validation (RPC)
    pub async fn create_guest_secure(
        &self,
        guest_group_id: &str,
        invitation_code: &str,
        name: &str,
        dietary_preferences: &DietaryPreferences,
    ) -> SupabaseResult<Guest> {
        let params = serde_json::json!({
            "p_guest_group_id": guest_group_id,
            "p_invitation_code": invitation_code,
            "p_name": name,
            "p_dietary_preferences": serde_json::to_value(dietary_preferences)
                .map_err(|e| SupabaseError::ParseError(e.to_string()))?
        });

        execute_rpc_with_logging(
            &self.client,
            "create_guest_for_group",
            params.to_string(),
            "create_guest_for_group",
        )
        .await
    }

    /// Update a guest with invitation code validation (RPC)
    pub async fn update_guest_secure(
        &self,
        guest_id: &str,
        guest_group_id: &str,
        invitation_code: &str,
        name: &str,
        dietary_preferences: &DietaryPreferences,
    ) -> SupabaseResult<Guest> {
        let params = serde_json::json!({
            "p_guest_id": guest_id,
            "p_guest_group_id": guest_group_id,
            "p_invitation_code": invitation_code,
            "p_name": name,
            "p_dietary_preferences": serde_json::to_value(dietary_preferences)
                .map_err(|e| SupabaseError::ParseError(e.to_string()))?
        });

        execute_rpc_with_flexible_parsing(
            &self.client,
            "update_guest_for_group",
            params.to_string(),
        )
        .await
    }

    /// Delete a guest with invitation code validation (RPC)
    pub async fn delete_guest_secure(
        &self,
        guest_id: &str,
        guest_group_id: &str,
        invitation_code: &str,
    ) -> SupabaseResult<()> {
        let params = serde_json::json!({
            "p_guest_id": guest_id,
            "p_guest_group_id": guest_group_id,
            "p_invitation_code": invitation_code
        });

        execute_rpc_delete_with_logging(
            &self.client,
            "delete_guest_for_group",
            params.to_string(),
            "✅ Guest deleted successfully",
        )
        .await
    }

    // ========================================================================
    // RSVP OPERATIONS (Direct - needs RPC for security)
    // ========================================================================

    /// Get all RSVPs for a specific guest group
    pub async fn get_rsvps_by_guest(&self, guest_group_id: &str) -> SupabaseResult<Vec<Rsvp>> {
        let response = self
            .client
            .from(TABLE_RSVPS)
            .select("*")
            .eq("guest_group_id", guest_group_id)
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse_vec(response).await
    }

    /// Create or update RSVP with invitation code validation (RPC)
    /// This replaces both create_rsvp and update_rsvp for guest users
    /// Note: Dietary preferences are tracked per guest in the guests table, not in RSVP
    pub async fn upsert_rsvp_secure(
        &self,
        guest_group_id: &str,
        invitation_code: &str,
        location: &str,
        attending: bool,
        number_of_guests: i32,
        additional_notes: Option<String>,
    ) -> SupabaseResult<Rsvp> {
        let params = serde_json::json!({
            "p_guest_group_id": guest_group_id,
            "p_invitation_code": invitation_code,
            "p_location": location,
            "p_attending": attending,
            "p_number_of_guests": number_of_guests,
            "p_additional_notes": additional_notes
        });

        execute_rpc_with_logging(
            &self.client,
            "upsert_rsvp_for_group",
            params.to_string(),
            "upsert_rsvp_for_group",
        )
        .await
    }

    // ========================================================================
    // GUEST LOCATION ATTENDANCE OPERATIONS
    // ========================================================================

    /// Get attending guests for a specific location
    pub async fn get_attending_guests_for_location(
        &self,
        guest_group_id: &str,
        invitation_code: &str,
        location: &str,
    ) -> SupabaseResult<Vec<Guest>> {
        let params = serde_json::json!({
            "p_guest_group_id": guest_group_id,
            "p_invitation_code": invitation_code,
            "p_location": location
        });

        // The RPC returns a table with guest_id, guest_name, dietary_preferences
        // We need to map it to Guest objects
        let response = execute_rpc_raw(
            &self.client,
            "get_attending_guests_for_location",
            params.to_string(),
        )
        .await?;

        #[derive(serde::Deserialize)]
        struct AttendingGuestRow {
            guest_id: String,
            guest_name: String,
            dietary_preferences: serde_json::Value,
        }

        let rows: Vec<AttendingGuestRow> = serde_json::from_str(&response)
            .map_err(|e| SupabaseError::ParseError(e.to_string()))?;

        // Convert to Guest objects
        let guests = rows
            .into_iter()
            .map(|row| Guest {
                id: row.guest_id,
                guest_group_id: guest_group_id.to_string(),
                name: row.guest_name,
                dietary_preferences: serde_json::from_value(row.dietary_preferences)
                    .unwrap_or_default(),
                created_at: None,
                updated_at: None,
            })
            .collect();

        Ok(guests)
    }

    /// Bulk update guest attendance for a location
    pub async fn bulk_update_guest_location_attendance(
        &self,
        guest_group_id: &str,
        invitation_code: &str,
        location: &str,
        guest_ids: &[String],
    ) -> SupabaseResult<()> {
        let params = serde_json::json!({
            "p_guest_group_id": guest_group_id,
            "p_invitation_code": invitation_code,
            "p_location": location,
            "p_guest_ids": guest_ids
        });

        execute_rpc_raw(
            &self.client,
            "bulk_update_guest_location_attendance",
            params.to_string(),
        )
        .await?;

        Ok(())
    }

    // ========================================================================
    // PHOTO OPERATIONS (Read-only for guests)
    // ========================================================================

    /// Get all photos (ordered by display_order)
    pub async fn get_all_photos(&self) -> SupabaseResult<Vec<Photo>> {
        let response = self
            .client
            .from(TABLE_PHOTOS)
            .select("*")
            .order("display_order")
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse_vec(response).await
    }
}

impl Default for SupabaseRpcClient {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ADMIN CLIENT - FOR AUTHENTICATED ADMIN USERS
// ============================================================================

/// Supabase admin client for authenticated users
/// Has full access to all tables and operations
#[derive(Clone)]
pub struct SupabaseAdminClient {
    client: Postgrest,
    #[allow(dead_code)]
    publishable_key: String,
    pub base_url: String,
    auth_token: Option<String>,
}

impl SupabaseAdminClient {
    /// Create a new admin client from environment variables (uses publishable key)
    pub fn new() -> Self {
        let url = env!("SUPABASE_URL");
        let publishable_key = env!("SUPABASE_PUBLISHABLE_KEY");

        let client = Postgrest::new(format!("{}{}", url, SUPABASE_REST_PATH))
            .insert_header("apikey", publishable_key)
            .insert_header("Authorization", format!("Bearer {}", publishable_key));

        Self {
            client,
            publishable_key: publishable_key.to_string(),
            base_url: url.to_string(),
            auth_token: None,
        }
    }

    /// Create a client with an access token for authenticated requests
    pub fn with_auth_token(access_token: &str) -> Self {
        let url = env!("SUPABASE_URL");
        let publishable_key = env!("SUPABASE_PUBLISHABLE_KEY");

        let client = Postgrest::new(format!("{}{}", url, SUPABASE_REST_PATH))
            .insert_header("apikey", publishable_key)
            .insert_header("Authorization", format!("Bearer {}", access_token));

        Self {
            client,
            publishable_key: publishable_key.to_string(),
            base_url: url.to_string(),
            auth_token: Some(access_token.to_string()),
        }
    }

    // ========================================================================
    // AUTHENTICATION OPERATIONS
    // ========================================================================

    /// Sign in with email and password
    pub async fn sign_in(&self, credentials: LoginCredentials) -> SupabaseResult<AuthSession> {
        let client = reqwest::Client::new();

        let response = client
            .post(format!(
                "{}{}/token?grant_type=password",
                self.base_url, SUPABASE_AUTH_PATH
            ))
            .header("apikey", &self.publishable_key)
            .header("Content-Type", "application/json")
            .json(&credentials)
            .send()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(SupabaseError::ServerError(format!(
                "Authentication failed: {} - {}",
                status, error_text
            )));
        }

        let auth_response: AuthResponse = response
            .json()
            .await
            .map_err(|e| SupabaseError::ParseError(e.to_string()))?;

        Ok(AuthSession {
            access_token: auth_response.access_token,
            refresh_token: auth_response.refresh_token,
            expires_in: auth_response.expires_in,
            token_type: auth_response.token_type,
            user: auth_response.user,
        })
    }

    /// Get current user from access token
    pub async fn get_user(&self, access_token: &str) -> SupabaseResult<crate::types::AuthUser> {
        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}{}/user", self.base_url, SUPABASE_AUTH_PATH))
            .header("apikey", &self.publishable_key)
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(SupabaseError::ServerError(format!(
                "Failed to get user: {} - {}",
                status, error_text
            )));
        }

        let user = response
            .json()
            .await
            .map_err(|e| SupabaseError::ParseError(e.to_string()))?;

        Ok(user)
    }

    /// Sign out (invalidate refresh token)
    pub async fn sign_out(&self, access_token: &str) -> SupabaseResult<()> {
        let client = reqwest::Client::new();

        let response = client
            .post(format!("{}{}/logout", self.base_url, SUPABASE_AUTH_PATH))
            .header("apikey", &self.publishable_key)
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(SupabaseError::ServerError(format!(
                "Sign out failed: {}",
                error_text
            )));
        }

        Ok(())
    }

    // ========================================================================
    // GUEST GROUP OPERATIONS (Admin - Direct Table Access)
    // ========================================================================

    /// Get all guest groups (admin only)
    pub async fn get_all_guest_groups(&self) -> SupabaseResult<Vec<GuestGroup>> {
        web_sys::console::log_1(&format!("🔍 GET {}/rest/v1/guest_groups", self.base_url).into());

        let response = self
            .client
            .from(TABLE_GUEST_GROUPS)
            .select("*")
            .order("name")
            .execute()
            .await
            .map_err(|e| {
                let err_msg = format!("Network error calling guest_groups: {}", e);
                web_sys::console::error_1(&err_msg.clone().into());
                SupabaseError::NetworkError(err_msg)
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            let err_msg = format!("Status: {}, Body: {}", status, error_text);
            web_sys::console::error_1(&format!("❌ guest_groups API error: {}", err_msg).into());
            return Err(SupabaseError::ServerError(err_msg));
        }

        let text = response
            .text()
            .await
            .map_err(|e| SupabaseError::ParseError(e.to_string()))?;

        let guest_groups: Vec<GuestGroup> =
            serde_json::from_str(&text).map_err(|e| SupabaseError::ParseError(e.to_string()))?;

        web_sys::console::log_1(&format!("✅ Loaded {} guest groups", guest_groups.len()).into());
        Ok(guest_groups)
    }

    /// Get all guest groups with actual guest count (admin only)
    pub async fn get_all_guest_groups_with_count(
        &self,
    ) -> SupabaseResult<Vec<GuestGroupWithCount>> {
        web_sys::console::log_1(
            &format!("🔍 GET {}/rest/v1/guest_groups with counts", self.base_url).into(),
        );

        // Query guest_groups with a count of guests
        let response = self
            .client
            .from(TABLE_GUEST_GROUPS)
            .select("*, guests!guest_group_id(count)")
            .order("name")
            .execute()
            .await
            .map_err(|e| {
                let err_msg = format!("Network error calling guest_groups: {}", e);
                web_sys::console::error_1(&err_msg.clone().into());
                SupabaseError::NetworkError(err_msg)
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            let err_msg = format!("Status: {}, Body: {}", status, error_text);
            web_sys::console::error_1(&format!("❌ guest_groups API error: {}", err_msg).into());
            return Err(SupabaseError::ServerError(err_msg));
        }

        let text = response
            .text()
            .await
            .map_err(|e| SupabaseError::ParseError(e.to_string()))?;

        web_sys::console::log_1(&format!("📦 Raw response: {}", text).into());

        // Parse the response - Supabase returns count as an array with one object
        #[derive(Deserialize)]
        struct GuestCountInfo {
            count: i64,
        }

        #[derive(Deserialize)]
        struct RawGuestGroup {
            id: String,
            name: String,
            email: Option<String>,
            invitation_code: String,
            party_size: i32,
            locations: Vec<String>,
            created_at: Option<String>,
            updated_at: Option<String>,
            guests: Vec<GuestCountInfo>,
        }

        let raw_groups: Vec<RawGuestGroup> =
            serde_json::from_str(&text).map_err(|e| SupabaseError::ParseError(e.to_string()))?;

        let guest_groups: Vec<GuestGroupWithCount> = raw_groups
            .into_iter()
            .map(|raw| {
                // Extract count from the array
                let count = raw.guests.first().map(|g| g.count).unwrap_or(0) as i32;

                GuestGroupWithCount {
                    guest_group: GuestGroup {
                        id: raw.id,
                        name: raw.name,
                        email: raw.email,
                        invitation_code: raw.invitation_code,
                        party_size: raw.party_size,
                        locations: raw.locations,
                        default_language: raw.default_language,
                        created_at: raw.created_at.and_then(|s| {
                            DateTime::parse_from_rfc3339(&s)
                                .ok()
                                .map(|dt| dt.with_timezone(&Utc))
                        }),
                        updated_at: raw.updated_at.and_then(|s| {
                            DateTime::parse_from_rfc3339(&s)
                                .ok()
                                .map(|dt| dt.with_timezone(&Utc))
                        }),
                    },
                    guest_count: count,
                }
            })
            .collect();

        web_sys::console::log_1(
            &format!("✅ Loaded {} guest groups with counts", guest_groups.len()).into(),
        );
        Ok(guest_groups)
    }

    /// Find guest groups by name (case-insensitive search)
    #[allow(dead_code)]
    pub async fn find_guest_by_name(&self, name: &str) -> SupabaseResult<Vec<GuestGroup>> {
        let response = self
            .client
            .from(TABLE_GUEST_GROUPS)
            .select("*")
            .ilike("name", format!("%{}%", name))
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse(response).await
    }

    /// Find a guest group by name and email
    #[allow(dead_code)]
    pub async fn find_guest_by_name_and_email(
        &self,
        name: &str,
        email: &str,
    ) -> SupabaseResult<Option<GuestGroup>> {
        let response = self
            .client
            .from(TABLE_GUEST_GROUPS)
            .select("*")
            .ilike("name", format!("%{}%", name))
            .eq("email", email)
            .single()
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse_option(response).await
    }

    /// Create a new guest group (admin only)
    pub async fn create_guest_group(
        &self,
        guest_group: &GuestGroupInput,
    ) -> SupabaseResult<GuestGroup> {
        let json_body = serde_json::to_string(guest_group)
            .map_err(|e| SupabaseError::ParseError(e.to_string()))?;

        let response = self
            .client
            .from(TABLE_GUEST_GROUPS)
            .insert(json_body)
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse_single(response).await
    }

    /// Update a guest group (admin only)
    pub async fn update_guest_group(
        &self,
        id: &str,
        update: &GuestGroupUpdate,
    ) -> SupabaseResult<GuestGroup> {
        let json_body =
            serde_json::to_string(update).map_err(|e| SupabaseError::ParseError(e.to_string()))?;

        let response = self
            .client
            .from(TABLE_GUEST_GROUPS)
            .eq("id", id)
            .update(json_body)
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse_single(response).await
    }

    /// Delete a guest group (admin only)
    pub async fn delete_guest_group(&self, guest_id: &str) -> SupabaseResult<()> {
        let response = self
            .client
            .from(TABLE_GUEST_GROUPS)
            .eq("id", guest_id)
            .delete()
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_delete(response).await
    }

    // ========================================================================
    // GUEST OPERATIONS (Admin - Direct Table Access)
    // ========================================================================

    /// Get all guests for a guest group (admin only)
    pub async fn get_guests_admin(&self, guest_group_id: &str) -> SupabaseResult<Vec<Guest>> {
        let response = self
            .client
            .from(TABLE_GUESTS)
            .eq("guest_group_id", guest_group_id)
            .select("*")
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse(response).await
    }

    /// Create a new guest (admin only)
    pub async fn create_guest(&self, guest: &GuestInput) -> SupabaseResult<Guest> {
        let response = self
            .client
            .from(TABLE_GUESTS)
            .insert(serde_json::to_string(guest).unwrap())
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse_single(response).await
    }

    /// Update a guest (admin only)
    #[allow(dead_code)]
    pub async fn update_guest(&self, id: &str, update: &GuestUpdate) -> SupabaseResult<Guest> {
        let response = self
            .client
            .from(TABLE_GUESTS)
            .eq("id", id)
            .update(serde_json::to_string(&update).unwrap())
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse_single(response).await
    }

    /// Delete a guest (admin only)
    pub async fn delete_guest(&self, guest_id: &str) -> SupabaseResult<()> {
        let response = self
            .client
            .from(TABLE_GUESTS)
            .eq("id", guest_id)
            .delete()
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_delete(response).await
    }

    // ========================================================================
    // RSVP OPERATIONS (Admin - Direct Table Access)
    // ========================================================================

    /// Get all RSVPs (admin only)
    pub async fn get_all_rsvps(&self) -> SupabaseResult<Vec<Rsvp>> {
        let response = self
            .client
            .from(TABLE_RSVPS)
            .select("*")
            .order("created_at")
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse(response).await
    }

    /// Get all RSVPs with dietary counts from guests table (admin only)
    pub async fn get_all_rsvps_with_dietary_counts(
        &self,
    ) -> SupabaseResult<Vec<RsvpWithDietaryCounts>> {
        web_sys::console::log_1(&"🔍 Fetching RSVPs with dietary counts...".into());

        // First, get all RSVPs
        let rsvps = self.get_all_rsvps().await?;

        let mut rsvps_with_counts = Vec::new();

        // For each RSVP, get the guests and count dietary preferences
        for rsvp in rsvps {
            let guests = self
                .get_guests_admin(&rsvp.guest_group_id)
                .await
                .unwrap_or_default();

            let mut vegetarian_count = 0;
            let mut vegan_count = 0;
            let mut halal_count = 0;
            let mut no_pork_count = 0;
            let mut gluten_free_count = 0;
            let mut other_list = Vec::new();

            for guest in guests {
                if guest.dietary_preferences.vegetarian {
                    vegetarian_count += 1;
                }
                if guest.dietary_preferences.vegan {
                    vegan_count += 1;
                }
                if guest.dietary_preferences.halal {
                    halal_count += 1;
                }
                if guest.dietary_preferences.no_pork {
                    no_pork_count += 1;
                }
                if guest.dietary_preferences.gluten_free {
                    gluten_free_count += 1;
                }
                if !guest.dietary_preferences.other.is_empty() {
                    other_list.push(guest.dietary_preferences.other);
                }
            }

            rsvps_with_counts.push(RsvpWithDietaryCounts {
                rsvp,
                dietary_vegetarian: vegetarian_count,
                dietary_vegan: vegan_count,
                dietary_halal: halal_count,
                dietary_no_pork: no_pork_count,
                dietary_gluten_free: gluten_free_count,
                dietary_other: other_list,
            });
        }

        web_sys::console::log_1(
            &format!(
                "✅ Loaded {} RSVPs with dietary counts",
                rsvps_with_counts.len()
            )
            .into(),
        );
        Ok(rsvps_with_counts)
    }

    /// Get RSVP for a specific guest group and location (admin only)
    #[allow(dead_code)]
    pub async fn get_rsvp_by_guest_and_location(
        &self,
        guest_group_id: &str,
        location: &str,
    ) -> SupabaseResult<Option<Rsvp>> {
        let response = self
            .client
            .from(TABLE_RSVPS)
            .select("*")
            .eq("guest_group_id", guest_group_id)
            .eq("location", location)
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse_option(response).await
    }

    /// Delete an RSVP (admin only)
    pub async fn delete_rsvp(&self, rsvp_id: &str) -> SupabaseResult<()> {
        let response = self
            .client
            .from(TABLE_RSVPS)
            .eq("id", rsvp_id)
            .delete()
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_delete(response).await
    }

    // ========================================================================
    // ADMIN STATISTICS
    // ========================================================================

    /// Get comprehensive admin statistics
    pub async fn get_admin_stats(&self) -> SupabaseResult<AdminStats> {
        web_sys::console::log_1(
            &format!("📊 Fetching admin stats... Base URL: {}", self.base_url).into(),
        );

        let guest_groups = self.get_all_guest_groups().await?;
        let rsvps = self.get_all_rsvps().await?;
        web_sys::console::log_1(&format!("✅ Loaded {} RSVPs", rsvps.len()).into());

        // Count actual individual guests in the guests table (total invited guests)
        let mut total_guests = 0i32;
        for guest_group in guest_groups.iter() {
            match self.get_guests_admin(&guest_group.id).await {
                Ok(guests) => total_guests += guests.len() as i32,
                Err(e) => {
                    web_sys::console::warn_1(
                        &format!("Failed to fetch guests for group {}: {}", guest_group.id, e)
                            .into(),
                    );
                }
            }
        }

        // Count guests who confirmed (actual guests from groups that RSVP'd YES)
        let rsvp_groups_attending: std::collections::HashSet<_> = rsvps
            .iter()
            .filter(|r| r.attending)
            .map(|r| r.guest_group_id.clone())
            .collect();

        let mut total_confirmed = 0i32;
        for guest_group in guest_groups.iter() {
            if rsvp_groups_attending.contains(&guest_group.id) {
                match self.get_guests_admin(&guest_group.id).await {
                    Ok(guests) => total_confirmed += guests.len() as i32,
                    Err(e) => {
                        web_sys::console::warn_1(
                            &format!(
                                "Failed to fetch confirmed guests for group {}: {}",
                                guest_group.id, e
                            )
                            .into(),
                        );
                    }
                }
            }
        }

        // Count guests by location (using actual guests from guests table)
        let mut sardinia_guests = 0i32;
        let mut tunisia_guests = 0i32;

        for guest_group in guest_groups.iter() {
            match self.get_guests_admin(&guest_group.id).await {
                Ok(guests) => {
                    let guest_count = guests.len() as i32;
                    // Count guests for each location they're invited to
                    for loc_str in &guest_group.locations {
                        match loc_str.as_str() {
                            "sardinia" => sardinia_guests += guest_count,
                            "tunisia" => tunisia_guests += guest_count,
                            _ => {}
                        }
                    }
                }
                Err(e) => {
                    web_sys::console::warn_1(
                        &format!(
                            "Failed to fetch guests for location count for group {}: {}",
                            guest_group.id, e
                        )
                        .into(),
                    );
                }
            }
        }

        // Guest groups who haven't RSVP'd yet (pending invitations)
        let guests_with_rsvp: std::collections::HashSet<_> =
            rsvps.iter().map(|r| r.guest_group_id.clone()).collect();
        let pending_guest_group_invitations = guest_groups
            .iter()
            .filter(|g| !guests_with_rsvp.contains(&g.id))
            .count() as i32;

        // Pending guests = count of actual guests for groups that haven't RSVP'd
        let mut pending_guests = 0i32;
        for guest_group in guest_groups.iter() {
            if !guests_with_rsvp.contains(&guest_group.id) {
                match self.get_guests_admin(&guest_group.id).await {
                    Ok(guests) => pending_guests += guests.len() as i32,
                    Err(e) => {
                        web_sys::console::warn_1(
                            &format!(
                                "Failed to fetch pending guests for group {}: {}",
                                guest_group.id, e
                            )
                            .into(),
                        );
                    }
                }
            }
        }

        let total_attending = total_confirmed;

        // Dietary restrictions - count from actual guests who RSVP'd attending
        let mut vegetarian_count = 0i32;
        let mut vegan_count = 0i32;
        let mut halal_count = 0i32;
        let mut no_pork_count = 0i32;
        let mut gluten_free_count = 0i32;
        let mut other_dietary_count = 0i32;

        for guest_group in guest_groups.iter() {
            if rsvp_groups_attending.contains(&guest_group.id) {
                match self.get_guests_admin(&guest_group.id).await {
                    Ok(guests) => {
                        for guest in guests {
                            if guest.dietary_preferences.vegetarian {
                                vegetarian_count += 1;
                            }
                            if guest.dietary_preferences.vegan {
                                vegan_count += 1;
                            }
                            if guest.dietary_preferences.halal {
                                halal_count += 1;
                            }
                            if guest.dietary_preferences.no_pork {
                                no_pork_count += 1;
                            }
                            if guest.dietary_preferences.gluten_free {
                                gluten_free_count += 1;
                            }
                            if !guest.dietary_preferences.other.is_empty() {
                                other_dietary_count += 1;
                            }
                        }
                    }
                    Err(e) => {
                        web_sys::console::warn_1(
                            &format!(
                                "Failed to fetch guests for dietary count for group {}: {}",
                                guest_group.id, e
                            )
                            .into(),
                        );
                    }
                }
            }
        }

        Ok(AdminStats {
            total_guests,
            total_confirmed,
            pending_rsvps: pending_guests,
            sardinia_guests,
            tunisia_guests,
            both_locations_guests: pending_guest_group_invitations,
            vegetarian_count,
            vegan_count,
            halal_count,
            no_pork_count,
            gluten_free_count,
            other_dietary_count,
            total_attending,
        })
    }

    // ========================================================================
    // PHOTO OPERATIONS (Admin - Full CRUD)
    // ========================================================================

    /// Get all photos (ordered by display_order)
    pub async fn get_all_photos(&self) -> SupabaseResult<Vec<Photo>> {
        let response = self
            .client
            .from(TABLE_PHOTOS)
            .select("*")
            .order("display_order")
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse_vec(response).await
    }

    /// Create a new photo entry
    pub async fn create_photo(&self, photo: &PhotoInput) -> SupabaseResult<Photo> {
        let response = self
            .client
            .from(TABLE_PHOTOS)
            .insert(serde_json::to_string(photo).unwrap())
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse_single(response).await
    }

    /// Update a photo's caption or display order
    #[allow(dead_code)]
    pub async fn update_photo(
        &self,
        id: &str,
        caption: Option<String>,
        display_order: Option<i32>,
    ) -> SupabaseResult<Photo> {
        #[derive(Serialize)]
        #[allow(dead_code)]
        struct PhotoUpdate {
            #[serde(skip_serializing_if = "Option::is_none")]
            caption: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            display_order: Option<i32>,
        }

        let update = PhotoUpdate {
            caption,
            display_order,
        };
        let json_body =
            serde_json::to_string(&update).map_err(|e| SupabaseError::ParseError(e.to_string()))?;

        let response = self
            .client
            .from(TABLE_PHOTOS)
            .eq("id", id)
            .update(json_body)
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse_single(response).await
    }

    /// Delete a photo
    pub async fn delete_photo(&self, photo_id: &str) -> SupabaseResult<()> {
        let response = self
            .client
            .from(TABLE_PHOTOS)
            .eq("id", photo_id)
            .delete()
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_delete(response).await
    }

    // ========================================================================
    // STORAGE OPERATIONS (Admin)
    // ========================================================================

    /// Upload a photo file to Supabase storage
    pub async fn upload_photo_to_storage(
        &self,
        filename: &str,
        file_data: Vec<u8>,
    ) -> SupabaseResult<String> {
        let storage_url = format!(
            "{}{}/object/{}/{}",
            self.base_url, SUPABASE_STORAGE_PATH, WEDDING_PHOTOS_BUCKET, filename
        );

        let content_type = if filename.to_lowercase().ends_with(".png") {
            "image/png"
        } else if filename.to_lowercase().ends_with(".gif") {
            "image/gif"
        } else if filename.to_lowercase().ends_with(".webp") {
            "image/webp"
        } else {
            "image/jpeg"
        };

        let auth_header = if let Some(ref token) = self.auth_token {
            format!("Bearer {}", token)
        } else {
            format!("Bearer {}", &self.publishable_key)
        };

        let response = reqwest::Client::new()
            .post(&storage_url)
            .header("apikey", &self.publishable_key)
            .header("Authorization", &auth_header)
            .header("Content-Type", content_type)
            .header("x-upsert", "false")
            .body(file_data)
            .send()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(SupabaseError::ServerError(format!(
                "Upload failed - Status: {}, Body: {}",
                status, error_text
            )));
        }

        let authenticated_url = format!(
            "{}{}/object/{}/{}",
            self.base_url, SUPABASE_STORAGE_PATH, WEDDING_PHOTOS_BUCKET, filename
        );
        Ok(authenticated_url)
    }

    /// Delete a photo file from Supabase storage
    pub async fn delete_photo_from_storage(&self, filename: &str) -> SupabaseResult<()> {
        let storage_url = format!(
            "{}{}/object/{}/{}",
            self.base_url, SUPABASE_STORAGE_PATH, WEDDING_PHOTOS_BUCKET, filename
        );

        let auth_header = if let Some(ref token) = self.auth_token {
            format!("Bearer {}", token)
        } else {
            format!("Bearer {}", &self.publishable_key)
        };

        let response = reqwest::Client::new()
            .delete(&storage_url)
            .header("apikey", &self.publishable_key)
            .header("Authorization", &auth_header)
            .send()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(SupabaseError::ServerError(format!(
                "Delete failed - Status: {}, Body: {}",
                status, error_text
            )));
        }

        Ok(())
    }
}

impl Default for SupabaseAdminClient {
    fn default() -> Self {
        Self::new()
    }
}
