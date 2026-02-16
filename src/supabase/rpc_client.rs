//! RPC client for guest/anonymous users.
//!
//! Only has access to secure RPC functions that validate invitation codes.
//! This keeps the attack surface minimal for unauthenticated visitors.

use super::error::SupabaseResult;
use super::helpers::{execute_rpc, execute_rpc_option};

use crate::constants::SUPABASE_REST_PATH;
use crate::types::{Guest, GuestGroup};
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

    /// Bulk save the entire RSVP in a single RPC call.
    ///
    /// Creates new guests (those with temp IDs), updates existing ones,
    /// and updates party_size + additional_notes on the guest group —
    /// all in one transactional round-trip.
    pub async fn save_rsvp(
        &self,
        guest_group_id: &str,
        invitation_code: &str,
        guests: &[Guest],
        guest_location_map: &std::collections::HashMap<String, std::collections::HashSet<String>>,
        additional_notes: &str,
    ) -> SupabaseResult<Vec<Guest>> {
        // Build the JSON guest array expected by the DB function
        let guests_json: Vec<serde_json::Value> = guests
            .iter()
            .map(|g| {
                let attending_locs: Vec<String> = guest_location_map
                    .get(&g.id)
                    .map(|locs| locs.iter().cloned().collect())
                    .unwrap_or_default();

                serde_json::json!({
                    "id": g.id,
                    "name": g.name,
                    "attending_locations": attending_locs,
                    "dietary_preferences": serde_json::to_value(&g.dietary_preferences).unwrap_or_default(),
                    "age_category": g.age_category.as_str()
                })
            })
            .collect();

        let params = serde_json::json!({
            "p_guest_group_id": guest_group_id,
            "p_invitation_code": invitation_code,
            "p_guests": guests_json,
            "p_additional_notes": additional_notes
        });

        execute_rpc(&self.client, "save_rsvp", params.to_string()).await
    }
}

impl Default for SupabaseRpcClient {
    fn default() -> Self {
        Self::new()
    }
}
