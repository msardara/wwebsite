use crate::supabase::{SupabaseAdminClient, SupabaseRpcClient};
use leptos::*;

/// Context for managing Supabase clients across the application
/// Provides singleton instances to avoid creating new clients on every request
#[derive(Clone, Copy)]
pub struct SupabaseContext {
    /// RPC client for guest/anonymous users (read-only, RPC-based operations)
    pub rpc_client: ReadSignal<SupabaseRpcClient>,
    /// Admin client for authenticated admin users (full CRUD access)
    pub admin_client: RwSignal<SupabaseAdminClient>,
}

impl SupabaseContext {
    /// Create a new Supabase context with singleton clients
    pub fn new() -> Self {
        // Create singleton RPC client for guests
        let rpc_client = create_signal(SupabaseRpcClient::new()).0;

        // Create singleton Admin client (will be updated with auth token when admin logs in)
        let admin_client = create_rw_signal(SupabaseAdminClient::new());

        Self {
            rpc_client,
            admin_client,
        }
    }

    /// Get the RPC client for guest operations
    pub fn rpc(&self) -> SupabaseRpcClient {
        self.rpc_client.get()
    }

    /// Get the admin client for admin operations
    pub fn admin(&self) -> SupabaseAdminClient {
        self.admin_client.get()
    }

    /// Update the admin client with an authenticated token
    pub fn set_admin_token(&self, access_token: &str) {
        self.admin_client
            .set(SupabaseAdminClient::with_auth_token(access_token));
    }

    /// Reset the admin client to unauthenticated state
    pub fn reset_admin_client(&self) {
        self.admin_client.set(SupabaseAdminClient::new());
    }
}

impl Default for SupabaseContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Hook to get the Supabase RPC client from context
pub fn use_supabase_rpc() -> SupabaseRpcClient {
    let context = use_context::<SupabaseContext>()
        .expect("SupabaseContext not found. Make sure it's provided at the app level.");
    context.rpc()
}

/// Hook to get the Supabase Admin client from context
pub fn use_supabase_admin() -> SupabaseAdminClient {
    let context = use_context::<SupabaseContext>()
        .expect("SupabaseContext not found. Make sure it's provided at the app level.");
    context.admin()
}

/// Hook to get the full Supabase context
pub fn use_supabase_context() -> SupabaseContext {
    use_context::<SupabaseContext>()
        .expect("SupabaseContext not found. Make sure it's provided at the app level.")
}
