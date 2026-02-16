//! RPC client for guest/anonymous users.
//!
//! Only has access to secure RPC functions that validate invitation codes.
//! This keeps the attack surface minimal for unauthenticated visitors.

use super::error::{SupabaseError, SupabaseResult};
use super::helpers::{execute_rpc, execute_rpc_flexible, execute_rpc_option, execute_rpc_void};
use crate::constants::SUPABASE_REST_PATH;
use crate::types::{DietaryPreferences, Guest, GuestGroup};
use postgrest::Postgrest;

/// Supabase RPC client for guest/anonymous users.
///
/// Only has access to secure RPC functions that validate invitation codes.
#[derive(Clone)]
pub struct SupabaseRpcClient {
    client: Postgrest,
    #[allow(dead_code)]
    publishable_key: String,
    #[allow(dead_code)]
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
        let result: Option<GuestGroup> = execute_rpc_option(
            &self.client,
            "authenticate_guest_group",
            serde_json::json!({"code": code}).to_string(),
        )
        .await?;

        // Set invitation_code from caller input — the DB no longer returns it
        Ok(result.map(|mut g| {
            g.invitation_code = code.to_string();
            g
        }))
    }

    /// Update party_size for a guest group with invitation code validation (RPC)
    pub async fn update_guest_group_party_size(
        &self,
        guest_group_id: &str,
        invitation_code: &str,
        new_party_size: i32,
    ) -> SupabaseResult<()> {
        execute_rpc_void(
            &self.client,
            "update_guest_group_party_size",
            serde_json::json!({
                "p_guest_group_id": guest_group_id,
                "p_invitation_code": invitation_code,
                "p_new_party_size": new_party_size
            })
            .to_string(),
        )
        .await
    }

    /// Update additional_notes for a guest group with invitation code validation (RPC)
    pub async fn update_guest_group_notes(
        &self,
        guest_group_id: &str,
        invitation_code: &str,
        additional_notes: &str,
    ) -> SupabaseResult<()> {
        execute_rpc_void(
            &self.client,
            "update_guest_group_notes",
            serde_json::json!({
                "p_guest_group_id": guest_group_id,
                "p_invitation_code": invitation_code,
                "p_additional_notes": additional_notes
            })
            .to_string(),
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
    #[allow(clippy::too_many_arguments)]
    pub async fn create_guest_secure(
        &self,
        guest_group_id: &str,
        invitation_code: &str,
        name: &str,
        attending_locations: &[String],
        dietary_preferences: &DietaryPreferences,
        age_category: &crate::types::AgeCategory,
    ) -> SupabaseResult<Guest> {
        let params = serde_json::json!({
            "p_guest_group_id": guest_group_id,
            "p_invitation_code": invitation_code,
            "p_name": name,
            "p_attending_locations": attending_locations,
            "p_dietary_preferences": serde_json::to_value(dietary_preferences)
                .map_err(|e| SupabaseError::ParseError(e.to_string()))?,
            "p_age_category": age_category.as_str()
        });

        execute_rpc_flexible(&self.client, "create_guest_for_group", params.to_string()).await
    }

    /// Update a guest with invitation code validation (RPC)
    #[allow(clippy::too_many_arguments)]
    pub async fn update_guest_secure(
        &self,
        guest_id: &str,
        guest_group_id: &str,
        invitation_code: &str,
        name: &str,
        attending_locations: &[String],
        dietary_preferences: &DietaryPreferences,
        age_category: &crate::types::AgeCategory,
    ) -> SupabaseResult<Guest> {
        let params = serde_json::json!({
            "p_guest_id": guest_id,
            "p_guest_group_id": guest_group_id,
            "p_invitation_code": invitation_code,
            "p_name": name,
            "p_attending_locations": attending_locations,
            "p_dietary_preferences": serde_json::to_value(dietary_preferences)
                .map_err(|e| SupabaseError::ParseError(e.to_string()))?,
            "p_age_category": age_category.as_str()
        });

        execute_rpc_flexible(&self.client, "update_guest_for_group", params.to_string()).await
    }

    /// Delete a guest with invitation code validation (RPC)
    pub async fn delete_guest_secure(
        &self,
        guest_id: &str,
        guest_group_id: &str,
        invitation_code: &str,
    ) -> SupabaseResult<()> {
        execute_rpc_void(
            &self.client,
            "delete_guest_for_group",
            serde_json::json!({
                "p_guest_id": guest_id,
                "p_guest_group_id": guest_group_id,
                "p_invitation_code": invitation_code
            })
            .to_string(),
        )
        .await
    }
}

impl Default for SupabaseRpcClient {
    fn default() -> Self {
        Self::new()
    }
}
