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
