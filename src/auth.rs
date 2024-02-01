use std::fs;
use std::path::{Path, PathBuf};

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

pub fn load_auth_tokens() -> Option<AuthTokens> {
    let token_file_path = get_token_file_path();
    let refresh_token_file_path = get_refresh_token_file_path();

    if !Path::new(&token_file_path).exists() || !Path::new(&refresh_token_file_path).exists() {
        return None;
    }

    let token = fs::read_to_string(token_file_path)
        .expect("Failed to read the token file");
    let refresh_token = fs::read_to_string(refresh_token_file_path)
        .expect("Failed to read the refresh token file");

    Some(AuthTokens::new(&token, &refresh_token))
}

pub fn store_auth_tokens(session: &AuthTokens) {
    let token_file_path = get_token_file_path();
    let refresh_token_file_path = get_refresh_token_file_path();

    fs::write(token_file_path, &session.token)
        .expect("Failed to write the token file");
    fs::write(refresh_token_file_path, &session.refresh_token)
        .expect("Failed to write the refresh token file");
}

fn get_token_file_path() -> PathBuf {
    // todo it's temporary, I think I'll store it in a more secure way
    get_credentials_dir().join("mess_jwt.txt")
}

fn get_refresh_token_file_path() -> PathBuf {
    // todo it's temporary, I think I'll store it in a more secure way
    get_credentials_dir().join("mess_refresh_token.txt")
}

fn get_credentials_dir() -> PathBuf {
    dirs::home_dir().expect("Home directory not found").join(".credentials")
}
