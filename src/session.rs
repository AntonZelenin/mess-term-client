use std::fs;
use std::path::Path;
use crate::constants;

pub struct Session {
    pub token: String,
    pub refresh_token: String,
}

impl Session {
    pub fn new(token: &str, refresh_token: &str) -> Self {
        Self {
            token: token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }
}

pub fn authenticate(username: &str, password: &str) -> Result<Session, String> {
    let client = reqwest::blocking::Client::new();
    let form_params = [
        ("username", username),
        ("password", password),
    ];
    let res = client
        .post(constants::AUTH_SERVER_API_URL)
        .form(&form_params)
        .send()
        .map_err(|e| e.to_string())?;

    let status = res.status();
    let data = res.json::<serde_json::Value>()
        .map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(data["message"].to_string());
    }

    let jwt = data["token"].to_string();
    let refresh_token = data["refresh_token"].to_string();
    // let jwt = jwt.trim_matches('"').to_string();

    Ok(Session::new(&jwt, &refresh_token))
}

pub fn load_session() -> Option<Session> {
    // todo it's temporary, I think I'll store it in a more secure way
    let home_dir = dirs::home_dir().expect("Home directory not found");
    let token_file_path = home_dir.join("token.txt");
    let refresh_token_file_path = home_dir.join("refresh_token.txt");
    
    if !Path::new(&token_file_path).exists() || !Path::new(&refresh_token_file_path).exists() {
        // todo logger.warn("kinda Session file not found");
        return None;
    }

    let token = fs::read_to_string(token_file_path)
        .expect("Failed to read the token file");
    let refresh_token = fs::read_to_string(refresh_token_file_path)
        .expect("Failed to read the refresh token file");
    
    Some(Session::new(&token, &refresh_token))
}
