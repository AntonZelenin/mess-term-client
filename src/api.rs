use std::collections::HashMap;
use crate::chat::Chat;
use crate::constants::HTTP_LOGIN_EXPIRED_STATUS_CODE;
use crate::contact::Contact;
use crate::auth::AuthTokens;

// todo https
pub const AUTH_SERVER_API_URL: &str = "http://localhost:8000/api/v1";
pub const APP_SERVER_API_URL: &str = "ws://localhost:8800/api/v1";

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

    pub fn login(&mut self, username: &str, password: &str) -> Result<(), String> {
        let form_params = [
            ("username", username),
            ("password", password),
        ];
        let res = self.
            client
            .post(&format!("{}/login", AUTH_SERVER_API_URL))
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
        return match self.post(&format!("{}/chats", APP_SERVER_API_URL)) {
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
        };
    }

    pub fn get_contacts(&mut self) -> Result<HashMap<String, Contact>, String> {
        return match self.post(&format!("{}/contacts", APP_SERVER_API_URL)) {
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
        };
    }

    pub fn refresh_token(&mut self) -> Result<(), String> {
        let form_params = [
            ("refresh_token", &self.auth_tokens.as_ref().expect("Unauthenticated").refresh_token),
        ];
        let res = self.
            client
            .post(&format!("{}/refresh-token", AUTH_SERVER_API_URL))
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

    pub fn get_auth_tokens(&self) -> Option<AuthTokens> {
        self.auth_tokens.clone()
    }

    fn post(&mut self, url: &str) -> Result<reqwest::blocking::Response, String> {
        let res = self.
            client
            .post(url)
            .header("Authorization", format!("Token {}", self.auth_tokens.as_ref().expect("Unauthenticated").token))
            .send()
            .map_err(|e| e.to_string())?;

        if res.status() == reqwest::StatusCode::from_u16(HTTP_LOGIN_EXPIRED_STATUS_CODE).unwrap() {
            return self.refresh_token().and_then(|_| self.post(url));
        }
        if !res.status().is_success() {
            let data = res.json::<serde_json::Value>()
                .map_err(|e| e.to_string())?;
            return Err(data["message"].to_string());
        }

        Ok(res)
    }
}
