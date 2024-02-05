use std::collections::HashMap;
use serde::Serialize;
use crate::chat::{Chat, SearchUserResult};
use crate::contact::Contact;
use crate::auth::AuthTokens;

// todo https
pub const AUTH_SERVICE_API_URL: &str = "localhost:55800/api/auth/v1";
pub const USER_SERVICE_API_URL: &str = "localhost:55800/api/user/v1";
pub const MESSAGE_SERVICE_API_URL: &str = "localhost:55800/api/message/v1";

pub struct Client {
    client: reqwest::blocking::Client,
    auth_tokens: Option<AuthTokens>,
}

#[derive(Serialize)]
struct RegisterData {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct RefreshTokenData {
    refresh_token: String,
}

impl Client {
    pub fn new(auth_tokens: Option<AuthTokens>) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            auth_tokens,
        }
    }

    pub fn get_auth_tokens(&self) -> Option<AuthTokens> {
        self.auth_tokens.clone()
    }

    pub fn login(&mut self, username: &str, password: &str) -> Result<(), String> {
        let url = &format!("http://{}/login", AUTH_SERVICE_API_URL);
        let form_params = [
            ("username", username),
            ("password", password),
        ];
        let res = self.
            client
            .post(url)
            .form(&form_params)
            .send()
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let data = res.json::<serde_json::Value>()
            .map_err(|e| e.to_string())?;
        if !status.is_success() {
            return Err(data["detail"].as_str().unwrap().to_string());
        }

        let jwt = data["access_token"].to_string();
        let refresh_token = data["refresh_token"].to_string();
        if jwt.is_empty() || refresh_token.is_empty() {
            panic!("JWT or refresh token is empty");
        }

        let jwt = jwt.trim_matches('"').to_string();
        let refresh_token = refresh_token.trim_matches('"').to_string();

        self.auth_tokens = Some(AuthTokens::new(&jwt, &refresh_token, true));

        Ok(())
    }

    pub fn register(&mut self, username: &str, password: &str) -> Result<(), String> {
        let url = &format!("http://{}/users", USER_SERVICE_API_URL);
        let register_data = RegisterData {
            username: username.to_string(),
            password: password.to_string(),
        };
        let res = self.
            client
            .post(url)
            .json(&register_data)
            .send()
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let data = res.json::<serde_json::Value>()
            .map_err(|e| e.to_string())?;
        if !status.is_success() {
            let errors = data["errors"].as_object().unwrap();
            return Err(
                errors
                    .values()
                    .filter_map(|v| v.as_str())
                    .map(|v| v.to_owned())
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        }

        let jwt = data["access_token"].to_string();
        let refresh_token = data["refresh_token"].to_string();
        if jwt.is_empty() || refresh_token.is_empty() {
            panic!("JWT or refresh token is empty");
        }

        let jwt = jwt.trim_matches('"').to_string();
        let refresh_token = refresh_token.trim_matches('"').to_string();

        self.auth_tokens = Some(AuthTokens::new(&jwt, &refresh_token, true));

        Ok(())
    }

    pub fn is_authenticated(&self) -> bool {
        self.auth_tokens.is_some()
    }

    pub fn get_chats(&mut self) -> Result<Vec<Chat>, String> {
        match self.get(&format!("http://{}/chats", MESSAGE_SERVICE_API_URL), vec![]) {
            Ok(res) => {
                let data = res.json::<serde_json::Value>()
                    .map_err(|e| e.to_string())?;
                let chats: Vec<Chat> = serde_json::from_str(&data.to_string()).unwrap();
                Ok(chats)
            }
            Err(e) => {
                // todo logger.error(e);
                Err(e)
            }
        }
    }

    pub fn get_contacts(&mut self) -> Result<HashMap<String, Contact>, String> {
        match self.get(&format!("http://{}/contacts", USER_SERVICE_API_URL), vec![]) {
            Ok(res) => {
                let data = res.json::<serde_json::Value>()
                    .map_err(|e| e.to_string())?;
                let contacts = serde_json::from_str(&data["contacts"].to_string()).unwrap();
                Ok(contacts)
            }
            Err(e) => {
                // todo logger.error(e);
                Err(e)
            }
        }
    }

    pub fn search_users(&mut self, username: String) -> Result<SearchUserResult, String> {
        match self.get(&format!("http://{}/users", USER_SERVICE_API_URL), vec![("username".parse().unwrap(), username)]) {
            Ok(res) => {
                let data = res.json::<serde_json::Value>()
                    .map_err(|e| e.to_string())?;
                // todo all these things must be done using derive serialize
                let users: SearchUserResult = serde_json::from_str(&data.to_string()).unwrap();
                Ok(users)
            }
            Err(e) => {
                // todo logger.error(e);
                Err(e)
            }
        }
    }

    pub fn mark_tokens_not_dirty(&mut self) {
        self.auth_tokens.as_mut().expect("Unauthenticated").dirty = false;
    }

    fn post(&mut self, base_url: &str, query_params: Vec<(String, String)>) -> Result<reqwest::blocking::Response, String> {
        let url = reqwest::Url::parse_with_params(base_url, query_params).map_err(|e| e.to_string())?;
        let res = self.
            client
            .post(url)
            .header("Authorization", self.get_authorization_header())
            .send()
            .map_err(|e| e.to_string())?;

        if res.status() == reqwest::StatusCode::from_u16(401).unwrap() {
            return self.refresh_token().and_then(|_| self.post(base_url, vec![]));
        }
        if !res.status().is_success() {
            let data = res.json::<serde_json::Value>()
                .map_err(|e| e.to_string())?;
            return Err(data["detail"].to_string());
        }

        Ok(res)
    }

    fn get(&mut self, base_url: &str, query_params: Vec<(String, String)>) -> Result<reqwest::blocking::Response, String> {
        let url = reqwest::Url::parse_with_params(base_url, query_params.clone()).map_err(|e| e.to_string())?;
        let res = self.
            client
            .get(url)
            .header("Authorization", self.get_authorization_header())
            .send()
            .map_err(|e| e.to_string())?;

        if res.status() == reqwest::StatusCode::from_u16(401).unwrap() && self.auth_tokens.is_some() {
            self.refresh_token().expect("Failed to refresh token");
            return self.post(base_url, query_params);
        }
        if !res.status().is_success() {
            let data = res.json::<serde_json::Value>()
                .map_err(|e| e.to_string())?;
            return Err(data["detail"].to_string());
        }

        Ok(res)
    }

    // todo I need to store refreshed tokens right away, I need a new mechanism
    fn refresh_token(&mut self) -> Result<(), String> {
        let refresh_token_data = RefreshTokenData {
            refresh_token: self.auth_tokens.as_ref().expect("Unauthenticated").refresh_token.clone(),
        };
        let res = self.
            client
            .post(&format!("http://{}/refresh-token", AUTH_SERVICE_API_URL))
            .json(&refresh_token_data)
            .send()
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let data = res.json::<serde_json::Value>()
            .map_err(|e| e.to_string())?;
        if !status.is_success() {
            return Err(data["detail"].to_string());
        }

        // todo duplicate code, same thing in login
        let jwt = data["access_token"].to_string();
        let refresh_token = data["refresh_token"].to_string();
        if jwt.is_empty() || refresh_token.is_empty() {
            panic!("JWT or refresh token is empty");
        }
        let jwt = jwt.trim_matches('"').to_string();
        let refresh_token = refresh_token.trim_matches('"').to_string();

        self.auth_tokens = Some(AuthTokens::new(&jwt, &refresh_token, true));
        Ok(())
    }

    fn get_authorization_header(&mut self) -> String {
        format!("Bearer {}", self.auth_tokens.as_ref().expect("Unauthenticated").token)
    }
}
