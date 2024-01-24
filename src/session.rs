use crate::constants;

pub struct Session {
    token: String,
}

impl Session {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }

    pub fn set_token(&mut self, jwt: String) {
        self.token = jwt;
    }

    pub fn get_token(&self) -> String {
        self.token.clone()
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
    // let jwt = jwt.trim_matches('"').to_string();

    Ok(Session::new(&jwt))
}
