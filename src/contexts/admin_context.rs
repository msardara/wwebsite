use crate::constants::ADMIN_SESSION_KEY;
use crate::contexts::supabase_context::use_supabase_context;
use crate::contexts::use_supabase_admin;
use crate::supabase::SupabaseAdminClient;
use crate::types::{AuthSession, LoginCredentials};
use gloo_storage::{LocalStorage, Storage};
use leptos::*;

/// Admin context for managing admin-specific state and authentication
#[derive(Clone, Copy)]
pub struct AdminContext {
    /// Whether the admin is authenticated
    pub is_authenticated: RwSignal<bool>,
    /// Admin user email (if authenticated)
    pub admin_email: RwSignal<Option<String>>,
    /// Access token for authenticated requests
    pub access_token: RwSignal<Option<String>>,
}

impl AdminContext {
    /// Create a new admin context
    pub fn new() -> Self {
        let context = Self {
            is_authenticated: create_rw_signal(false),
            admin_email: create_rw_signal(None),
            access_token: create_rw_signal(None),
        };

        // Try to restore session from localStorage on init
        context.try_restore_session();

        context
    }

    /// Check if admin is authenticated
    pub fn is_admin_authenticated(&self) -> bool {
        self.is_authenticated.get()
    }

    /// Get the authenticated Supabase client
    pub fn authenticated_client(&self) -> SupabaseAdminClient {
        use_supabase_admin()
    }

    /// Get admin email
    pub fn get_email(&self) -> Option<String> {
        self.admin_email.get()
    }

    /// Get access token
    #[allow(dead_code)]
    pub fn get_access_token(&self) -> Option<String> {
        self.access_token.get()
    }

    /// Sign in with email and password
    pub async fn sign_in(&self, email: String, password: String) -> Result<(), String> {
        let credentials = LoginCredentials { email, password };
        let base_client = use_supabase_admin();

        match base_client.sign_in(credentials).await {
            Ok(session) => {
                // Store session in localStorage
                if let Err(e) = LocalStorage::set(ADMIN_SESSION_KEY, &session) {
                    return Err(format!("Failed to save session: {}", e));
                }

                // Update context state
                self.is_authenticated.set(true);
                self.admin_email.set(Some(session.user.email.clone()));
                self.access_token.set(Some(session.access_token.clone()));

                // Update authenticated client in Supabase context with new token
                use_supabase_context().set_admin_token(&session.access_token);

                Ok(())
            }
            Err(_) => Err("Invalid email or password. Please try again.".to_string()),
        }
    }

    /// Try to restore session from localStorage
    fn try_restore_session(&self) {
        match LocalStorage::get::<AuthSession>(ADMIN_SESSION_KEY) {
            Ok(session) => {
                web_sys::console::log_1(&"🔄 Restoring admin session from localStorage...".into());

                // Verify the session is still valid (you could add expiry check here)
                self.is_authenticated.set(true);
                self.admin_email.set(Some(session.user.email.clone()));
                self.access_token.set(Some(session.access_token.clone()));

                // Restore authenticated client in Supabase context with token
                let supabase_ctx = use_supabase_context();
                supabase_ctx.set_admin_token(&session.access_token);

                web_sys::console::log_1(&"✅ Admin session restored successfully".into());
            }
            Err(e) => {
                web_sys::console::warn_1(&format!("⚠️ Could not restore session: {:?}", e).into());
                web_sys::console::log_1(&"💡 Clearing any corrupted session data...".into());
                // Clear any corrupted session data
                LocalStorage::delete(ADMIN_SESSION_KEY);
            }
        }
    }

    /// Verify current session is valid
    pub async fn verify_session(&self) -> bool {
        if let Some(token) = self.access_token.get() {
            web_sys::console::log_1(&"🔍 Verifying admin session...".into());
            let base_client = use_supabase_admin();
            match base_client.get_user(&token).await {
                Ok(user) => {
                    self.admin_email.set(Some(user.email));
                    web_sys::console::log_1(&"✅ Session is valid".into());
                    true
                }
                Err(e) => {
                    // Session is invalid, clear it
                    web_sys::console::warn_1(
                        &format!("❌ Session verification failed: {}", e).into(),
                    );
                    web_sys::console::log_1(&"🔄 Clearing invalid session...".into());
                    self.logout();
                    false
                }
            }
        } else {
            web_sys::console::log_1(&"ℹ️ No admin session to verify".into());
            false
        }
    }

    /// Log out admin
    pub fn logout(&self) {
        web_sys::console::log_1(&"🚪 Logging out admin...".into());

        // Clear localStorage
        LocalStorage::delete(ADMIN_SESSION_KEY);

        // Clear context state
        self.is_authenticated.set(false);
        self.admin_email.set(None);
        self.access_token.set(None);

        // Reset to base unauthenticated client in Supabase context
        let supabase_ctx = use_supabase_context();
        supabase_ctx.reset_admin_client();

        web_sys::console::log_1(&"✅ Admin logged out successfully".into());
    }

    /// Sign out with server notification
    pub async fn sign_out(&self) {
        if let Some(token) = self.access_token.get() {
            // Attempt to sign out on server (best effort)
            let base_client = use_supabase_admin();
            let _ = base_client.sign_out(&token).await;
        }

        self.logout();
    }
}

impl Default for AdminContext {
    fn default() -> Self {
        Self::new()
    }
}
