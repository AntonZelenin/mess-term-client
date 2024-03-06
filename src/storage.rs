use std::fs;
use std::path::{Path, PathBuf};
use crate::auth::AuthTokens;
use crate::schemas::User;

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

pub fn store_auth_tokens(tokens: &AuthTokens) {
    let token_file_path = get_token_file_path();
    let refresh_token_file_path = get_refresh_token_file_path();
    let creds_dir = get_credentials_dir();

    if !get_credentials_dir().exists() {
        fs::create_dir(creds_dir)
            .expect("Failed to create the credentials directory");
    }

    fs::write(token_file_path, &tokens.token)
        .expect("Failed to write the token file");
    fs::write(refresh_token_file_path, &tokens.refresh_token)
        .expect("Failed to write the refresh token file");
}

pub fn store_user(user: &User) {
    store_username(&user.username);
    store_user_id(&user.id);
}

pub fn load_user() -> Option<User> {
    let username = load_username()?;
    let user_id = load_user_id()?;

    Some(User {
        username,
        id: user_id,
    })
}

fn store_username(username: &str) {
    let username_file_path = get_username_file_path();
    let creds_dir = get_credentials_dir();

    if !get_credentials_dir().exists() {
        fs::create_dir(creds_dir)
            .expect("Failed to create the credentials directory");
    }

    fs::write(username_file_path, username)
        .expect("Failed to write the username file");
}

fn load_username() -> Option<String> {
    let username_file_path = get_username_file_path();

    if !Path::new(&username_file_path).exists() {
        return None;
    }

    let username = fs::read_to_string(username_file_path)
        .expect("Failed to read the username file");

    Some(username)
}

fn store_user_id(user_id: &str) {
    let user_id_file_path = get_user_id_file_path();
    let creds_dir = get_credentials_dir();

    if !get_credentials_dir().exists() {
        fs::create_dir(creds_dir)
            .expect("Failed to create the credentials directory");
    }

    fs::write(user_id_file_path, user_id)
        .expect("Failed to write the username file");
}

fn load_user_id() -> Option<String> {
    let user_id_file_path = get_user_id_file_path();

    if !Path::new(&user_id_file_path).exists() {
        return None;
    }

    let username = fs::read_to_string(user_id_file_path)
        .expect("Failed to read the username file");

    Some(username)
}

pub fn delete_auth_tokens() {
    let token_file_path = get_token_file_path();
    let refresh_token_file_path = get_refresh_token_file_path();

    if Path::new(&token_file_path).exists() {
        fs::remove_file(token_file_path)
            .expect("Failed to delete the token file");
    }

    if Path::new(&refresh_token_file_path).exists() {
        fs::remove_file(refresh_token_file_path)
            .expect("Failed to delete the refresh token file");
    }
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

fn get_user_id_file_path() -> PathBuf {
    get_credentials_dir().join("mess_user_id.txt")
}

fn get_username_file_path() -> PathBuf {
    get_credentials_dir().join("mess_username.txt")
}
