use crate::chat::Chat;
use crate::constants::HTTP_LOGIN_EXPIRED_STATUS_CODE;
use crate::contact::Contact;
use crate::session::Session;

// todo it should be secure
pub const AUTH_SERVER_API_URL: &str = "http://localhost:8000/api/v1/";
pub const APP_SERVER_API_URL: &str = "ws://localhost:8800";

pub struct Client {
    client: reqwest::blocking::Client,
    session: Option<Session>,
}

impl Client {
    pub fn new(session: Option<Session>) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            session,
        }
    }

    pub fn login(&self, username: &str, password: &str) -> Result<Session, String> {
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

        let jwt = data["token"].to_string();
        let refresh_token = data["refresh_token"].to_string();
        // let jwt = jwt.trim_matches('"').to_string();

        Ok(Session::new(&jwt, &refresh_token))
    }

    pub fn is_authenticated(&self) -> bool {
        self.session.is_some()
    }

    pub fn get_chats(&mut self) -> Result<Vec<Chat>, String> {
        return match self.post(&format!("{}/chats", APP_SERVER_API_URL), vec![]) {
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

    pub fn get_contacts(&mut self) -> Result<Vec<Contact>, String> {
        return match self.get(&format!("{}/contacts", APP_SERVER_API_URL), vec![]) {
            Ok(res) => {
                let data = res.json::<serde_json::Value>()
                    .map_err(|e| e.to_string())?;
                let contacts: Vec<Contact> = serde_json::from_str(&data["contacts"].to_string()).unwrap();
                Ok(contacts)
            }
            Err(e) => {
                // todo logger.error(e);
                Err(e)
            }
        };
    }
    
    pub fn search_chats(&mut self, username: String) -> Result<Vec<Chat>, String> {
        return match self.get(&format!("{}/chats", APP_SERVER_API_URL), vec![("username".parse().unwrap(), username)]) {
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

    fn post(&mut self, base_url: &str, query_params: Vec<(String, String)>) -> Result<reqwest::blocking::Response, String> {
        let url = reqwest::Url::parse_with_params(base_url, &query_params).map_err(|e| e.to_string())?;
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
        let url = reqwest::Url::parse_with_params(base_url, &query_params).map_err(|e| e.to_string())?;
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

    fn get_authorization_header(&mut self) -> String {
        format!("Bearer {}", self.session.as_ref().expect("Unauthenticated").token)
    }

    fn refresh_token(&mut self) -> Result<(), String> {
        let form_params = [
            ("refresh_token", &self.session.as_ref().expect("Unauthenticated").refresh_token),
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

        let jwt = data["token"].to_string();
        let refresh_token = data["refresh_token"].to_string();
        // let jwt = jwt.trim_matches('"').to_string();

        self.session = Some(Session::new(&jwt, &refresh_token));
        Ok(())
    }
}
