//! Supabase authentication types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub user: AuthUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub role: Option<String>,
    pub user_metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub user: AuthUser,
}
