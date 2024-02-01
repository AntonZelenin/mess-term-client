use std::collections::HashMap;
use crate::chat::Chat;
use crate::constants::HTTP_LOGIN_EXPIRED_STATUS_CODE;
use crate::contact::Contact;
use crate::auth::AuthTokens;

// todo https
pub const AUTH_SERVICE_API_URL: &str = "localhost:8000/api/auth/v1";
pub const USER_SERVICE_API_URL: &str = "localhost:8800/api/user/v1";
pub const MESSAGE_SERVICE_API_URL: &str = "localhost:8800/api/message/v1";

pub struct Client {
    client: reqwest::blocking::Client,
    auth_tokens: Option<AuthTokens>,
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
        let form_params = [
            ("username", username),
            ("password", password),
        ];
        let res = self.
            client
            .post(&format!("http://{}/login", AUTH_SERVICE_API_URL))
            .form(&form_params)
            .send()
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let data = res.json::<serde_json::Value>()
            .map_err(|e| e.to_string())?;
        if !status.is_success() {
            return Err(data["message"].to_string());
        }

        let jwt = data["access_token"].to_string();
        let refresh_token = data["refresh_token"].to_string();
        if jwt.is_empty() || refresh_token.is_empty() {
            panic!("JWT or refresh token is empty");
        }

        let jwt = jwt.trim_matches('"').to_string();
        let refresh_token = refresh_token.trim_matches('"').to_string();

        self.auth_tokens = Some(AuthTokens::new(&jwt, &refresh_token));

        Ok(())
    }

    pub fn is_authenticated(&self) -> bool {
        self.auth_tokens.is_some()
    }

    pub fn get_chats(&mut self) -> Result<Vec<Chat>, String> {
        match self.post(&format!("http://{}/chats", USER_SERVICE_API_URL), vec![]) {
            Ok(res) => {
                let data = res.json::<serde_json::Value>()
                    .map_err(|e| e.to_string())?;
                let chats: Vec<Chat> = serde_json::from_str(&data["chats"].to_string()).unwrap();
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

    pub fn search_chats(&mut self, username: String) -> Result<Vec<Chat>, String> {
        match self.get(&format!("{}/chats", USER_SERVICE_API_URL), vec![("username".parse().unwrap(), username)]) {
            Ok(res) => {
                let data = res.json::<serde_json::Value>()
                    .map_err(|e| e.to_string())?;
                let chats: Vec<Chat> = serde_json::from_str(&data["chats"].to_string()).unwrap();
                Ok(chats)
            }
            Err(e) => {
                // todo logger.error(e);
                Err(e)
            }
        }
    }

    fn post(&mut self, base_url: &str, query_params: Vec<(String, String)>) -> Result<reqwest::blocking::Response, String> {
        let url = reqwest::Url::parse_with_params(base_url, query_params).map_err(|e| e.to_string())?;
        let res = self.
            client
            .post(url)
            .header("Authorization", self.get_authorization_header())
            .send()
            .map_err(|e| e.to_string())?;

        if res.status() == reqwest::StatusCode::from_u16(HTTP_LOGIN_EXPIRED_STATUS_CODE).unwrap() {
            return self.refresh_token().and_then(|_| self.post(base_url, vec![]));
        }
        if !res.status().is_success() {
            let data = res.json::<serde_json::Value>()
                .map_err(|e| e.to_string())?;
            return Err(data["message"].to_string());
        }

        Ok(res)
    }

    fn get(&mut self, base_url: &str, query_params: Vec<(String, String)>) -> Result<reqwest::blocking::Response, String> {
        let url = reqwest::Url::parse_with_params(base_url, query_params).map_err(|e| e.to_string())?;
        let res = self.
            client
            .get(url)
            .header("Authorization", self.get_authorization_header())
            .send()
            .map_err(|e| e.to_string())?;

        if res.status() == reqwest::StatusCode::from_u16(HTTP_LOGIN_EXPIRED_STATUS_CODE).unwrap() {
            return self.refresh_token().and_then(|_| self.post(base_url, vec![]));
        }
        if !res.status().is_success() {
            let data = res.json::<serde_json::Value>()
                .map_err(|e| e.to_string())?;
            return Err(data["message"].to_string());
        }

        Ok(res)
    }

    pub fn refresh_token(&mut self) -> Result<(), String> {
        let form_params = [
            ("refresh_token", &self.auth_tokens.as_ref().expect("Unauthenticated").refresh_token),
        ];
        let res = self.
            client
            .post(&format!("{}/refresh-token", AUTH_SERVICE_API_URL))
            .form(&form_params)
            .send()
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let data = res.json::<serde_json::Value>()
            .map_err(|e| e.to_string())?;
        if !status.is_success() {
            return Err(data["message"].to_string());
        }

        // todo duplicate code, same thing in login
        let jwt = data["access_token"].to_string();
        let refresh_token = data["refresh_token"].to_string();
        if jwt.is_empty() || refresh_token.is_empty() {
            panic!("JWT or refresh token is empty");
        }
        let jwt = jwt.trim_matches('"').to_string();
        let refresh_token = refresh_token.trim_matches('"').to_string();

        self.auth_tokens = Some(AuthTokens::new(&jwt, &refresh_token));
        Ok(())
    }

    fn get_authorization_header(&mut self) -> String {
        format!("Bearer {}", self.auth_tokens.as_ref().expect("Unauthenticated").token)
    }
}
