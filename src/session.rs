use std::fs;
use std::path::{Path, PathBuf};

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

pub fn load_session() -> Option<Session> {
    // todo it's temporary, I think I'll store it in a more secure way
    let (token_file_path, refresh_token_file_path) = get_token_paths();
    
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

pub fn save_session(session: &Session) {
    // todo it's temporary, I think I'll store it in a more secure way
   let (token_file_path, refresh_token_file_path) = get_token_paths();

    fs::write(token_file_path, session.token.as_bytes())
        .expect("Failed to write the token file");
    fs::write(refresh_token_file_path, session.refresh_token.as_bytes())
        .expect("Failed to write the refresh token file");
}

fn get_token_paths() -> (PathBuf, PathBuf) {
    let home_dir = dirs::home_dir().expect("Home directory not found");
    let token_file_path = home_dir.join("token.txt");
    let refresh_token_file_path = home_dir.join("refresh_token.txt");
    (token_file_path, refresh_token_file_path)
}
