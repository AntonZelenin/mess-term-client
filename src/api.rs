use std::fmt;
use futures::{SinkExt, StreamExt};
use futures::stream::{SplitSink, SplitStream};
use reqwest::{Response, StatusCode};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::http::header::AUTHORIZATION;
use tokio_tungstenite::tungstenite::http::Request;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, tungstenite, WebSocketStream};
use tokio_tungstenite::tungstenite::Error;
use url::Url;
use crate::{helpers, schemas, storage};
use crate::schemas::*;
use crate::auth::AuthTokens;
use crate::helpers::types::{ChatId, UserId};

pub const HOST: &str = "185.191.177.247:55800";
// todo https
pub const AUTH_SERVICE_API_URL: &str = "185.191.177.247:55800/api/auth/v1";
pub const USER_SERVICE_API_URL: &str = "185.191.177.247:55800/api/user/v1";
pub const MESSAGE_SERVICE_API_URL: &str = "185.191.177.247:55800/api/message/v1";
pub const MESSAGE_WEBSOCKET_URL: &str = "ws://185.191.177.247:55800/ws/message/v1/messages";

type WriteMessageWs = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type ReadMessageWs = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

struct RequestParams {
    uri: String,
    query_params: Vec<(String, String)>,
    body: Option<serde_json::Value>,
    can_reauthenticate: bool,
}

impl RequestParams {
    pub fn set_cant_reauthenticate(&mut self) {
        self.can_reauthenticate = false;
    }
}

impl Default for RequestParams {
    fn default() -> Self {
        Self {
            uri: "".to_string(),
            query_params: vec![],
            body: None,
            can_reauthenticate: true,
        }
    }
}

pub struct Client {
    client: reqwest::Client,
    auth_tokens: Option<AuthTokens>,
    store_auth_tokens_callback: Box<dyn Fn(&AuthTokens)>,
    delete_auth_tokens_callback: Box<dyn Fn()>,
    write_message_ws: Option<WriteMessageWs>,
    read_message_ws: Option<ReadMessageWs>,
}

impl Client {
    pub async fn new(auth_tokens: Option<AuthTokens>) -> Self {
        let mut obj = Self {
            client: reqwest::Client::new(),
            auth_tokens,
            store_auth_tokens_callback: Box::new(storage::store_auth_tokens),
            delete_auth_tokens_callback: Box::new(storage::delete_auth_tokens),
            write_message_ws: None,
            read_message_ws: None,
        };

        if obj.auth_tokens.is_some() {
            obj.connect_to_message_ws(RequestParams::default()).await;
        }

        obj
    }

    pub fn is_authenticated(&self) -> bool {
        self.auth_tokens.is_some()
    }

    pub async fn login(&mut self, username: &str, password: &str) -> ApiResult<String> {
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
            .map_err(|e| ApiError::RequestError(e.to_string()))?;

        let status = res.status();
        let data = res.json::<serde_json::Value>()
            .await
            .map_err(|e| ApiError::DataError(e.to_string()))?;
        if !status.is_success() {
            return Err(ApiError::RequestError(data["detail"].as_str().unwrap().to_string()));
        }

        let jwt = data["access_token"].to_string();
        let refresh_token = data["refresh_token"].to_string();
        if jwt.is_empty() || refresh_token.is_empty() {
            panic!("JWT or refresh token is empty");
        }
        let user_id = data["user_id"].to_string().trim_matches('"').to_string();

        let jwt = jwt.trim_matches('"').to_string();
        let refresh_token = refresh_token.trim_matches('"').to_string();

        self.set_auth_tokens(AuthTokens::new(&jwt, &refresh_token));

        self.connect_to_message_ws(RequestParams::default()).await;

        Ok(user_id)
    }

    pub async fn register(&mut self, username: &str, password: &str) -> ApiResult<String> {
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
            .map_err(|e| ApiError::RequestError(e.to_string()))?;

        let status = res.status();
        let data = res.json::<serde_json::Value>()
            .await
            .map_err(|e| ApiError::DataError(e.to_string()))?;
        if !status.is_success() {
            let errors = data["errors"].as_object().unwrap();
            return Err(
                ApiError::RequestError(
                    errors
                        .values()
                        .filter_map(|v| v.as_str())
                        .map(|v| v.to_owned())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            );
        }

        let jwt = data["access_token"].to_string();
        let refresh_token = data["refresh_token"].to_string();
        if jwt.is_empty() || refresh_token.is_empty() {
            panic!("JWT or refresh token is empty");
        }
        let user_id = data["user_id"].to_string().trim_matches('"').to_string();

        let jwt = jwt.trim_matches('"').to_string();
        let refresh_token = refresh_token.trim_matches('"').to_string();

        self.set_auth_tokens(AuthTokens::new(&jwt, &refresh_token));

        self.connect_to_message_ws(RequestParams::default()).await;

        Ok(user_id)
    }

    pub async fn get_users_by_ids(&mut self, user_ids: Vec<UserId>) -> ApiResult<UserSearchResults> {
        let rp = RequestParams {
            uri: format!("http://{}/users/batch-query", USER_SERVICE_API_URL),
            body: Some(serde_json::to_value(&GetUsersByIdsRequest { user_ids }).unwrap()),
            ..Default::default()
        };
        let res = self.post(rp).await?;
        let data = res.json::<serde_json::Value>()
            .await
            .map_err(|e| ApiError::DataError(e.to_string()))?;
       
        Ok(serde_json::from_str(&data.to_string()).unwrap())
    }

    pub async fn get_chats(&mut self) -> ApiResult<ChatSearchResults> {
        let rp = RequestParams {
            uri: format!("http://{}/chats", MESSAGE_SERVICE_API_URL),
            ..Default::default()
        };
        match self.get(rp).await {
            Ok(res) => {
                let data = res.json::<serde_json::Value>()
                    .await
                    .map_err(|e| ApiError::DataError(e.to_string()))?;
                let chats: ChatSearchResults = serde_json::from_str(&data.to_string()).unwrap();
                Ok(chats)
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub async fn get_chat(&mut self, chat_id: ChatId) -> ApiResult<ChatModel> {
        let rp = RequestParams {
            uri: format!("http://{}/chats/{}", MESSAGE_SERVICE_API_URL, chat_id),
            ..Default::default()
        };
        let res = self.get(rp).await?;
        let data = res.json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())
            .expect("Failed to parse response");

        Ok(serde_json::from_str(&data.to_string()).unwrap())
    }

    pub async fn mark_chat_as_read(&mut self, chat_id: ChatId) {
        let rp = RequestParams {
            uri: format!("http://{}/chats/{}/read", MESSAGE_SERVICE_API_URL, chat_id),
            ..Default::default()
        };
        match self.post(rp).await {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    pub async fn search_users(&mut self, username: String) -> ApiResult<UserSearchResults> {
        let rp = RequestParams {
            uri: format!("http://{}/users", USER_SERVICE_API_URL),
            query_params: vec![("username".parse().unwrap(), username)],
            ..Default::default()
        };
        let res = self.get(rp).await?;
        let data = res.json::<serde_json::Value>()
            .await
            .map_err(|e| ApiError::DataError(e.to_string()))?;
        let users: UserSearchResults = serde_json::from_str(&data.to_string()).unwrap();

        Ok(users)
    }

    pub async fn send_message(&mut self, message: NewMessage) {
        let send_message = self
            .write_message_ws
            .as_mut()
            .expect("Unauthenticated")
            .send(Message::Text(serde_json::to_string(&message).unwrap()));
        send_message
            .await
            .expect("Failed to send message");
    }

    pub async fn create_chat(&mut self, chat: NewChatModel) -> ApiResult<ChatModel> {
        let rp = RequestParams {
            uri: format!("http://{}/chats", MESSAGE_SERVICE_API_URL),
            body: Some(serde_json::to_value(&chat).unwrap()),
            ..Default::default()
        };
        let res = self.post(rp).await?;
        let status = res.status();
        let data = res.json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())
            .expect("Failed to parse response");
        if !status.is_success() {
            return Err(ApiError::RequestError(data["detail"].to_string()));
        }

        Ok(serde_json::from_str(&data.to_string()).unwrap())
    }

    pub async fn receive_message(&mut self) -> Option<schemas::MessageModel> {
        if let Some(message) = self.read_message_ws.as_mut().expect("Unauthenticated").next().await {
            match message {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str::<schemas::MessageModel>(&text) {
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

    async fn post(&mut self, mut rp: RequestParams) -> ApiResult<Response> {
        loop {
            let url = Url::parse_with_params(&rp.uri, rp.query_params.clone()).unwrap();
            let res = self.
                client
                .post(url)
                .header("Authorization", self.get_authorization_header())
                .json(&rp.body)
                .send()
                .await
                .map_err(|e| ApiError::RequestError(e.to_string()))?;

            if self.should_refresh_tokens(&mut rp, &res) {
                match self.refresh_tokens(&mut rp).await {
                    Ok(_) => continue,
                    Err(e) => {
                        self.unauthenticate();
                        return Err(e);
                    }
                }
            }
            if !res.status().is_success() {
                let data = res.json::<serde_json::Value>()
                    .await
                    .map_err(|e| ApiError::DataError(e.to_string()))?;
                return Err(ApiError::RequestError(data["detail"].to_string()));
            }

            return Ok(res);
        }
    }

    fn should_refresh_tokens(&mut self, rp: &mut RequestParams, res: &Response) -> bool {
        res.status() == StatusCode::UNAUTHORIZED && rp.can_reauthenticate && self.auth_tokens.is_some()
    }

    async fn get(&mut self, mut rp: RequestParams) -> ApiResult<Response> {
        loop {
            let url = Url::parse_with_params(&rp.uri, rp.query_params.clone()).unwrap();
            let res = self.
                client
                .get(url)
                .header("Authorization", self.get_authorization_header())
                .send()
                .await
                .map_err(|e| ApiError::RequestError(e.to_string()))?;

            if self.should_refresh_tokens(&mut rp, &res) {
                match self.refresh_tokens(&mut rp).await {
                    Ok(_) => continue,
                    Err(_) => {
                        self.unauthenticate();
                        return Err(ApiError::Unauthenticated);
                    }
                }
            }
            if !res.status().is_success() {
                let data = res.json::<serde_json::Value>()
                    .await
                    .map_err(|e| ApiError::DataError(e.to_string()))?;
                return Err(ApiError::RequestError(data["detail"].to_string()));
            }

            return Ok(res);
        }
    }

    async fn refresh_tokens(&mut self, rp: &mut RequestParams) -> ApiResult<()> {
        if !rp.can_reauthenticate {
            panic!("Cannot reauthenticate");
        }
        rp.set_cant_reauthenticate();

        let refresh_token_data = RefreshTokenData {
            refresh_token: self.auth_tokens.as_ref().expect("Unauthenticated").refresh_token.clone(),
        };
        let res = self.
            client
            .post(&format!("http://{}/refresh-token", AUTH_SERVICE_API_URL))
            .json(&refresh_token_data)
            .send()
            .await
            .map_err(|e| ApiError::RequestError(e.to_string()))?;

        let status = res.status();
        let data = res.json::<serde_json::Value>()
            .await
            .map_err(|e| ApiError::DataError(e.to_string()))?;
        if !status.is_success() {
            if status == StatusCode::UNAUTHORIZED {
                return Err(ApiError::Unauthenticated);
            }
            return Err(ApiError::RequestError(data["detail"].to_string()));
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

    async fn connect_to_message_ws(&mut self, mut rp: RequestParams) {
        loop {
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

            match connect_async(request).await {
                Ok((ws_stream, _)) => {
                    let (write_ws, read_ws) = ws_stream.split();
                    self.write_message_ws = Some(write_ws);
                    self.read_message_ws = Some(read_ws);
                    break;
                }
                Err(Error::Http(response)) => {
                    if response.status() == tungstenite::http::StatusCode::UNAUTHORIZED && rp.can_reauthenticate {
                        if self.auth_tokens.is_some() {
                            match self.refresh_tokens(&mut rp).await {
                                Ok(_) => continue,
                                Err(_) => {
                                    self.unauthenticate();
                                    return;
                                }
                            }
                        } else {
                            panic!("Can't connect to ws: Unauthenticated");
                        }
                    }
                    panic!("Failed to connect to message websocket: {}", response.status());
                }
                Err(e) => {
                    panic!("Failed to connect to message websocket: {}", e);
                }
            }
        }
    }

    fn set_auth_tokens(&mut self, tokens: AuthTokens) {
        self.auth_tokens = Some(tokens.clone());
        (self.store_auth_tokens_callback)(&tokens);
    }

    fn unauthenticate(&mut self) {
        self.auth_tokens = None;
        (self.delete_auth_tokens_callback)();
    }

    fn get_authorization_header(&mut self) -> String {
        format!("Bearer {}", self.auth_tokens.as_ref().expect("Unauthenticated").token)
    }
}

type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Clone)]
pub enum ApiError {
    Unauthenticated,
    RequestError(String),
    DataError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::Unauthenticated => write!(f, "Unauthenticated"),
            ApiError::RequestError(e) => write!(f, "Request error: {}", e),
            ApiError::DataError(e) => write!(f, "Data error: {}", e),
        }
    }
}
