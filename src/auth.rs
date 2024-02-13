#[derive(Debug, Clone)]
pub struct AuthTokens {
    pub token: String,
    pub refresh_token: String,
}

impl AuthTokens {
    pub fn new(token: &str, refresh_token: &str) -> Self {
        Self {
            token: token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }
}
