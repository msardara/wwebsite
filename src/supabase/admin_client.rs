//! Admin client for authenticated admin users.
//!
//! Has full access to all tables and operations. The auth token is set after
//! a successful sign-in and stored in the Supabase context.

use super::error::{SupabaseError, SupabaseResult};
use super::helpers::{
    execute_and_parse, execute_and_parse_option, execute_and_parse_single, execute_delete,
    response_to_text,
};
use crate::constants::{SUPABASE_AUTH_PATH, SUPABASE_REST_PATH, TABLE_GUESTS, TABLE_GUEST_GROUPS};
use crate::types::{
    AdminStats, AuthResponse, AuthSession, Guest, GuestGroup, GuestGroupInput, GuestGroupUpdate,
    GuestGroupWithCount, GuestInput, GuestUpdate, LoginCredentials,
};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::Deserialize;

/// Supabase admin client for authenticated users.
///
/// Has full access to all tables and operations.
#[derive(Clone)]
pub struct SupabaseAdminClient {
    client: Postgrest,
    #[allow(dead_code)]
    publishable_key: String,
    pub base_url: String,
    #[allow(dead_code)]
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
        let response = self
            .client
            .from(TABLE_GUEST_GROUPS)
            .select("*")
            .order("name")
            .execute()
            .await
            .map_err(|e| {
                SupabaseError::NetworkError(format!("Network error calling guest_groups: {}", e))
            })?;

        execute_and_parse(response).await
    }

    /// Get all guest groups with actual guest count (admin only)
    pub async fn get_all_guest_groups_with_count(
        &self,
    ) -> SupabaseResult<Vec<GuestGroupWithCount>> {
        // Query guest_groups with a count of guests
        let response = self
            .client
            .from(TABLE_GUEST_GROUPS)
            .select("*, guests!guest_group_id(count)")
            .order("name")
            .execute()
            .await
            .map_err(|e| {
                SupabaseError::NetworkError(format!("Network error calling guest_groups: {}", e))
            })?;

        let text = response_to_text(response).await?;

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
            default_language: String,
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
                        additional_notes: None,
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
    // GUEST LIST OPERATIONS (Admin)
    // ========================================================================

    /// Get all guests attending a specific location (admin only)
    pub async fn get_all_guests_for_location(&self, location: &str) -> SupabaseResult<Vec<Guest>> {
        let response = self
            .client
            .from(TABLE_GUESTS)
            .select("*")
            .cs("attending_locations", format!("{{{}}}", location))
            .execute()
            .await
            .map_err(|e| SupabaseError::NetworkError(e.to_string()))?;

        execute_and_parse(response).await
    }

    // ========================================================================
    // ADMIN STATISTICS
    // ========================================================================

    /// Get comprehensive admin statistics
    pub async fn get_admin_stats(&self) -> SupabaseResult<AdminStats> {
        let guest_groups = self.get_all_guest_groups().await?;

        // Get all guests
        let sardinia_guests_list = self.get_all_guests_for_location("sardinia").await?;
        let tunisia_guests_list = self.get_all_guests_for_location("tunisia").await?;

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

        // Count guests who have selected at least one location (confirmed attending)
        let mut guests_with_locations = std::collections::HashSet::new();
        for guest_group in guest_groups.iter() {
            match self.get_guests_admin(&guest_group.id).await {
                Ok(guests) => {
                    for guest in guests {
                        if !guest.attending_locations.is_empty() {
                            guests_with_locations.insert(guest.id);
                        }
                    }
                }
                Err(e) => {
                    web_sys::console::warn_1(
                        &format!("Failed to fetch guests for group {}: {}", guest_group.id, e)
                            .into(),
                    );
                }
            }
        }
        let total_confirmed = guests_with_locations.len() as i32;

        // Count guests by location
        let sardinia_guests = sardinia_guests_list.len() as i32;
        let tunisia_guests = tunisia_guests_list.len() as i32;

        // Calculate guests invited to both locations
        let both_locations_guests = guest_groups
            .iter()
            .filter(|g| {
                g.locations.contains(&"sardinia".to_string())
                    && g.locations.contains(&"tunisia".to_string())
            })
            .map(|g| g.party_size)
            .sum::<i32>();

        // Count pending guests (guests with no location selections)
        let pending_guests = total_guests - total_confirmed;

        // Count dietary preferences (from all guests with location selections)
        let mut vegetarian_count = 0i32;
        let mut vegan_count = 0i32;
        let mut halal_count = 0i32;
        let mut no_pork_count = 0i32;
        let mut gluten_free_count = 0i32;
        let mut other_dietary_count = 0i32;

        for guest_group in guest_groups.iter() {
            match self.get_guests_admin(&guest_group.id).await {
                Ok(guests) => {
                    for guest in guests {
                        if !guest.attending_locations.is_empty() {
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
                }
                Err(e) => {
                    web_sys::console::warn_1(
                        &format!(
                            "Failed to fetch dietary info for group {}: {}",
                            guest_group.id, e
                        )
                        .into(),
                    );
                }
            }
        }

        Ok(AdminStats {
            total_guests,
            total_confirmed,
            pending_rsvps: pending_guests,
            sardinia_guests,
            tunisia_guests,
            both_locations_guests,
            vegetarian_count,
            vegan_count,
            halal_count,
            no_pork_count,
            gluten_free_count,
            other_dietary_count,
        })
    }
}

impl Default for SupabaseAdminClient {
    fn default() -> Self {
        Self::new()
    }
}
