//! Supabase client service.
//!
//! This module handles all interactions with Supabase using postgrest-rs.
//! It is split into submodules for better maintainability:
//!
//! - [`error`] — shared error and result types
//! - [`helpers`] — response-parsing and RPC helper functions
//! - [`rpc_client`] — `SupabaseRpcClient` for anonymous/guest users (RPC only)
//! - [`admin_client`] — `SupabaseAdminClient` for authenticated admins (full table access)

mod admin_client;
mod error;
mod helpers;
mod rpc_client;

// Re-export the public API so that existing `use crate::supabase::*` paths
// continue to work without any changes in the rest of the codebase.
pub use admin_client::SupabaseAdminClient;
pub use error::SupabaseError;
#[allow(unused_imports)]
pub use error::SupabaseResult;
pub use rpc_client::SupabaseRpcClient;
