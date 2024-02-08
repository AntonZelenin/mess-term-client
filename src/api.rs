use futures::{SinkExt, StreamExt};
use futures::stream::{SplitSink, SplitStream};
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::http::header::AUTHORIZATION;
use tokio_tungstenite::tungstenite::http::Request;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;
use crate::{auth, helpers};
use crate::chat::{ChatModel, ChatSearchResults, NewChatModel, UserSearchResults};
use crate::auth::AuthTokens;
use crate::chat::message::NewMessage;

pub const HOST: &str = "localhost:55800";
// todo https
pub const AUTH_SERVICE_API_URL: &str = "localhost:55800/api/auth/v1";
pub const USER_SERVICE_API_URL: &str = "localhost:55800/api/user/v1";
pub const MESSAGE_SERVICE_API_URL: &str = "localhost:55800/api/message/v1";
pub const MESSAGE_WEBSOCKET_URL: &str = "ws://localhost:55800/ws/message/v1/messages";

type WriteMessageWs = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type ReadMessageWs = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

#[derive(Serialize)]
struct RegisterData {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct RefreshTokenData {
    refresh_token: String,
}

pub struct Client {
    client: reqwest::Client,
    auth_tokens: Option<AuthTokens>,
    auth_tokens_store_callback: Box<dyn Fn(&AuthTokens)>,
    write_ws: Option<WriteMessageWs>,
    read_ws: Option<ReadMessageWs>,
}

impl Client {
    pub async fn new(auth_tokens: Option<AuthTokens>) -> Self {
        let mut obj = Self {
            client: reqwest::Client::new(),
            auth_tokens,
            auth_tokens_store_callback: Box::new(auth::store_auth_tokens),
            write_ws: None,
            read_ws: None,
        };

        if obj.auth_tokens.is_some() {
            obj.connect_to_message_ws().await;
        }

        obj
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), String> {
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
            .await
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let data = res.json::<serde_json::Value>()
            .await
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

        self.set_auth_tokens(AuthTokens::new(&jwt, &refresh_token));

        self.connect_to_message_ws().await;

        Ok(())
    }

    pub async fn register(&mut self, username: &str, password: &str) -> Result<(), String> {
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
            .await
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let data = res.json::<serde_json::Value>()
            .await
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

        self.set_auth_tokens(AuthTokens::new(&jwt, &refresh_token));

        self.connect_to_message_ws().await;

        Ok(())
    }

    pub fn is_authenticated(&self) -> bool {
        self.auth_tokens.is_some()
    }

    pub async fn get_chats(&mut self) -> Result<ChatSearchResults, String> {
        match self.get(&format!("http://{}/chats", MESSAGE_SERVICE_API_URL), vec![]).await {
            Ok(res) => {
                let data = res.json::<serde_json::Value>()
                    .await
                    .map_err(|e| e.to_string())?;
                let chats: ChatSearchResults = serde_json::from_str(&data.to_string()).unwrap();
                Ok(chats)
            }
            Err(e) => {
                // todo logger.error(e);
                Err(e)
            }
        }
    }

    pub async fn search_chats(&mut self, username: String) -> Result<ChatSearchResults, String> {
        match self.get(&format!("http://{}/chats", MESSAGE_SERVICE_API_URL), vec![("username".parse().unwrap(), username)]).await {
            Ok(res) => {
                let data = res.json::<serde_json::Value>()
                    .await
                    .map_err(|e| e.to_string())?;
                // todo all these things must be done using derive serialize
                let users: ChatSearchResults = serde_json::from_str(&data.to_string()).unwrap();
                Ok(users)
            }
            Err(e) => {
                // todo logger.error(e);
                Err(e)
            }
        }
    }

    pub async fn search_users(&mut self, username: String) -> Result<UserSearchResults, String> {
        match self.get(&format!("http://{}/users", USER_SERVICE_API_URL), vec![("username".parse().unwrap(), username)]).await {
            Ok(res) => {
                let data = res.json::<serde_json::Value>()
                    .await
                    .map_err(|e| e.to_string())?;
                // todo all these things must be done using derive serialize
                let users: UserSearchResults = serde_json::from_str(&data.to_string()).unwrap();
                Ok(users)
            }
            Err(e) => {
                // todo logger.error(e);
                Err(e)
            }
        }
    }

    pub async fn send_message(&mut self, message: NewMessage) {
        let send_message = self
            .write_ws
            .as_mut()
            .expect("Unauthenticated")
            .send(Message::Text(serde_json::to_string(&message).unwrap()));
        let _ = send_message
            .await
            .expect("Failed to send message");
    }

    pub async fn create_chat(&mut self, chat: NewChatModel) -> Result<ChatModel, String> {
        let res = self
            .client
            .post(&format!("http://{}/chats", MESSAGE_SERVICE_API_URL))
            .header("Authorization", self.get_authorization_header())
            .json(&chat)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let data = res.json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())
            .expect("Failed to parse response");
        if !status.is_success() {
            return Err(data["detail"].to_string());
        }

        serde_json::from_str(&data.to_string()).unwrap()
    }

    pub async fn receive_message(&mut self) -> Option<crate::chat::message::Message> {
        if let Some(message) = self.read_ws.as_mut().expect("Unauthenticated").next().await {
            match message {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str::<crate::chat::message::Message>(&text) {
                        Ok(parsed_message) => {
                            return Some(parsed_message);
                        }
                        Err(e) => {
                            panic!("Failed to parse message: {}", e);
                        }
                    }
                }
                Ok(Message::Binary(_)) => {
                    panic!("Received a binary message");
                }
                Ok(Message::Ping(_)) | Ok(Message::Pong(_)) => {}
                Ok(Message::Close(_)) => {
                    unimplemented!("Handle close messages, possibly clean up or reconnect");
                }
                Err(e) => {
                    panic!("Failed to receive message: {}", e);
                }
                _ => {
                    panic!("Received unexpected message type");
                }
            }
        };

        None
    }

    async fn post(&mut self, base_url: &str, query_params: Vec<(String, String)>) -> Result<reqwest::Response, String> {
        loop {
            let url = Url::parse_with_params(base_url, query_params.clone()).map_err(|e| e.to_string())?;
            let res = self.
                client
                .post(url)
                .header("Authorization", self.get_authorization_header())
                .send()
                .await
                .map_err(|e| e.to_string())?;

            if res.status() == reqwest::StatusCode::from_u16(401).unwrap() {
                self.refresh_token().await.expect("Failed to refresh token");
                continue;
            }
            if !res.status().is_success() {
                let data = res.json::<serde_json::Value>()
                    .await
                    .map_err(|e| e.to_string())?;
                return Err(data["detail"].to_string());
            }

            return Ok(res);
        }
    }

    async fn get(&mut self, base_url: &str, query_params: Vec<(String, String)>) -> Result<reqwest::Response, String> {
        let url = Url::parse_with_params(base_url, query_params.clone()).map_err(|e| e.to_string())?;
        let res = self.
            client
            .get(url)
            .header("Authorization", self.get_authorization_header())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status() == reqwest::StatusCode::from_u16(401).unwrap() && self.auth_tokens.is_some() {
            self.refresh_token().await.expect("Failed to refresh token");
            return self.post(base_url, query_params).await;
        }
        if !res.status().is_success() {
            let data = res.json::<serde_json::Value>()
                .await
                .map_err(|e| e.to_string())?;
            return Err(data["detail"].to_string());
        }

        Ok(res)
    }

    // todo I need to store refreshed tokens right away, I need a new mechanism
    async fn refresh_token(&mut self) -> Result<(), String> {
        let refresh_token_data = RefreshTokenData {
            refresh_token: self.auth_tokens.as_ref().expect("Unauthenticated").refresh_token.clone(),
        };
        let res = self.
            client
            .post(&format!("http://{}/refresh-token", AUTH_SERVICE_API_URL))
            .json(&refresh_token_data)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let status = res.status();
        let data = res.json::<serde_json::Value>()
            .await
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

        self.set_auth_tokens(AuthTokens::new(&jwt, &refresh_token));
        Ok(())
    }

    async fn connect_to_message_ws(&mut self) {
        // todo it cannot handle refresh token and unauthenticated requests
        let request = Request::builder()
            .uri(MESSAGE_WEBSOCKET_URL)
            .header(AUTHORIZATION, self.get_authorization_header())
            .header("sec-websocket-key", helpers::generate_sec_websocket_key())
            .header("host", HOST)
            .header("upgrade", "websocket")
            .header("connection", "upgrade")
            .header("sec-websocket-version", 13)
            .body(())
            .expect("Failed to build request.");

        let (ws_stream, _) = connect_async(request)
            .await
            .expect("Failed to connect to WebSocket");

        let (write_ws, read_ws) = ws_stream.split();
        self.write_ws = Some(write_ws);
        self.read_ws = Some(read_ws);
    }

    fn set_auth_tokens(&mut self, tokens: AuthTokens) {
        self.auth_tokens = Some(tokens.clone());
        (self.auth_tokens_store_callback)(&tokens);
    }

    fn get_authorization_header(&mut self) -> String {
        format!("Bearer {}", self.auth_tokens.as_ref().expect("Unauthenticated").token)
    }
}
