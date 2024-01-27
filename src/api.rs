use crate::chat::Chat;
use crate::session::Session;

// todo it should be secure
pub const AUTH_SERVER_API_URL: &str = "http://localhost:8000/api/v1/";
pub const APP_SERVER_API_URL: &str = "ws://localhost:8800";

pub struct Client {
    client: reqwest::blocking::Client,
    session: Session,
}

impl Client {
    pub fn new(session: Session) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            session,
        }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<Session, String> {
        let form_params = [
            ("username", username),
            ("password", password),
        ];
        let res = self.
            client
            .post(AUTH_SERVER_API_URL)
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

    pub fn get_chats(&self, session: &Session) -> Result<Vec<Chat>, String> {
        return match self.post(&format!("{}/chats", APP_SERVER_API_URL)) {
            Ok(res) => {
                let data = res.json::<serde_json::Value>()
                    .map_err(|e| e.to_string())?;
                let chats = data["chats"].to_string();
                let chats = serde_json::from_str(&chats)
                    .map_err(|e| e.to_string())?;
                // chats
                Ok(vec![])
            }
            Err(e) => {
                // todo logger.error(e);
                Err(e)
            }
        }
    }

    pub fn get_contacts(self, session: &Session) -> Vec<Chat> {

    }

    fn post(&self, url: &str) -> Result<reqwest::blocking::Response, String> {
        let res = self.
            client
            .post(url)
            .header("Authorization", format!("Token {}", self.session.token))
            .send()
            .map_err(|e| e.to_string())?;

        let data = res.json::<serde_json::Value>()
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Err(data["message"].to_string());
        }

        Ok(res)
    }
}
