use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Json},
    routing::{delete, get, post, put},
    Router,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::oneshot;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
};

use crate::args::Args;
use crate::command::Command;
use crate::frame::Frame;
use crate::store::db::DatabaseMessage;
use crate::store::db_manager::DatabaseManager;

// ==================== JWT 认证模块 ====================

/// JWT Token claims
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    pub sub: String, // 用户名
    pub iat: i64,    // 签发时间
    pub exp: i64,    // 过期时间
}

/// JWT 认证配置
pub struct JwtAuth {
    secret: String,
    expiration_hours: i64,
}

impl JwtAuth {
    /// 创建新的 JWT 认证实例
    fn new(secret: String, expiration_hours: i64) -> Self {
        Self {
            secret,
            expiration_hours,
        }
    }

    /// 从用户名密码创建（使用 webuser:webpass 作为密钥）
    fn from_credentials(webuser: &str, webpass: &str) -> Self {
        let secret = format!("{}:{}", webuser, webpass);
        Self::new(secret, 24) // 默认24小时过期
    }

    /// 签发 Token
    fn generate_token(&self, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let claims = Claims {
            sub: username.to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::hours(self.expiration_hours)).timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    /// 验证 Token
    fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )?;
        Ok(token_data.claims)
    }
}

/// 认证中间件
async fn auth_middleware<B>(
    State(state): State<Arc<WebState>>,
    req: axum::http::Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    // 从请求头中获取 Authorization
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "error": "缺少认证 Token，请在请求头中提供 Authorization: Bearer <token>"
                })),
            )
                .into_response();
        }
    };

    // 验证 Token
    match state.jwt_auth.verify_token(token) {
        Ok(claims) => {
            // Token 有效，检查用户名是否匹配
            if claims.sub == state.webuser {
                next.run(req).await
            } else {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "success": false,
                        "error": "Token 用户无效"
                    })),
                )
                    .into_response()
            }
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "error": "无效的 Token 或 Token 已过期"
            })),
        )
            .into_response(),
    }
}

/// Web服务器
pub struct WebServer {
    args: Arc<Args>,
    db_manager: Arc<DatabaseManager>,
}

impl WebServer {
    
    pub fn new(args: Arc<Args>, db_manager: Arc<DatabaseManager>) -> Self {
        WebServer {
            args,
            db_manager,
        }
    }

    pub async fn start(self, port: u16) {
        let max_databases = self.args.databases;
        let bind_addr = format!("{}:{}", self.args.bind, port);
        let aof_path = PathBuf::from(&self.args.appendfilename);
        let jwt_auth = JwtAuth::from_credentials(&self.args.webuser, &self.args.webpass);
        let web_state = Arc::new(WebState {
            db_manager: self.db_manager,
            max_databases,
            webuser: self.args.webuser.clone(),
            webpass: self.args.webpass.clone(),
            aof_path,
            jwt_auth,
        });

        let web_router = create_router(web_state);
        axum::Server::bind(&bind_addr.parse().unwrap())
            .serve(web_router.into_make_service())
            .await
            .expect("Web server failed to start");
    }
}

/// Web服务状态
pub struct WebState {
    pub db_manager: Arc<DatabaseManager>,
    pub max_databases: usize,
    pub webuser: String,
    pub webpass: String,
    pub aof_path: PathBuf,
    pub jwt_auth: JwtAuth,
}

/// 数据库信息
#[derive(Serialize)]
struct DatabaseInfo {
    id: usize,
    name: String,
    key_count: usize,
}

/// 键信息
#[derive(Serialize)]
struct KeyInfo {
    key: String,
    #[serde(rename = "type")]
    key_type: String,
    ttl: i64,
}

/// 键值响应
#[derive(Serialize)]
struct KeyValueResponse {
    key: String,
    #[serde(rename = "type")]
    key_type: String,
    value: serde_json::Value,
    ttl: i64,
}

/// 设置键值请求
#[derive(Deserialize)]
struct SetKeyRequest {
    #[serde(default)]
    value: String,                           // 用于string类型
    #[serde(default)]
    fields: Vec<(String, String)>,          // 用于hash类型，包含(field, value)对
    #[serde(default)]
    values: Vec<String>,                     // 用于list/set类型
    #[serde(default)]
    members_with_scores: Vec<(f64, String)>,  // 用于zset类型，包含(score, member)对
    ttl: Option<u64>,
}

/// 更新TTL请求
#[derive(Deserialize)]
struct UpdateTtlRequest {
    ttl: u64,
}

/// 登录请求
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

/// 登录响应
#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_in: Option<i64>,
}

/// CLI命令请求
#[derive(Deserialize)]
struct CliRequest {
    command: String,
    db: Option<usize>,
}

/// 查询参数
#[derive(Deserialize)]
struct ListKeysQuery {
    pattern: Option<String>,
    db: Option<usize>,
    #[serde(rename = "type")]
    key_type: Option<String>,
}

/// AOF日志查询参数
#[derive(Deserialize)]
struct AofLogsQuery {
    page: Option<usize>,
    size: Option<usize>,
}

/// 创建Web路由
fn create_router(state: Arc<WebState>) -> Router {
    // 需要认证的路由
    let protected_routes = Router::new()
        .route("/api/stats", get(get_stats))
        .route("/api/databases", get(list_databases))
        .route("/api/keys", get(list_keys))
        .route("/api/keys/:key", get(get_key_value))
        .route("/api/keys/:key", delete(delete_key))
        .route("/api/keys/:key/ttl", put(update_key_ttl))
        .route("/api/keys/:key", post(set_key_value))
        .route("/api/cli", post(execute_cli))
        .route("/api/aof-logs", get(get_aof_logs))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    // 公开路由（不需要认证）
    let public_routes = Router::new().route("/api/login", post(login));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        // 静态文件服务
        .nest_service("/", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// 登录验证 - 签发 JWT Token
async fn login(
    State(state): State<Arc<WebState>>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    if req.username == state.webuser && req.password == state.webpass {
        match state.jwt_auth.generate_token(&req.username) {
            Ok(token) => {
                let response = LoginResponse {
                    success: true,
                    message: "登录成功".to_string(),
                    token: Some(token),
                    expires_in: Some(24 * 3600), // 24小时，单位秒
                };
                (StatusCode::OK, Json(response))
            }
            Err(e) => {
                let response = LoginResponse {
                    success: false,
                    message: format!("Token 生成失败: {}", e),
                    token: None,
                    expires_in: None,
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
            }
        }
    } else {
        let response = LoginResponse {
            success: false,
            message: "用户名或密码错误".to_string(),
            token: None,
            expires_in: None,
        };
        (StatusCode::UNAUTHORIZED, Json(response))
    }
}

/// 执行CLI命令
async fn execute_cli(
    State(state): State<Arc<WebState>>,
    Json(req): Json<CliRequest>,
) -> impl IntoResponse {
    let db_id = req.db.unwrap_or(0);
    if db_id >= state.max_databases {
        return Json(json!({
            "success": false,
            "result": "数据库索引超出范围"
        }));
    }

    // 解析命令
    let parts: Vec<&str> = req.command.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Json(json!({
            "success": false,
            "result": "命令不能为空"
        }));
    }

    let frame = Frame::Array(
        parts.iter().map(|s| Frame::BulkString(s.to_string())).collect()
    );

    let command = match Command::parse_from_frame(frame) {
        Ok(cmd) => cmd,
        Err(e) => {
            return Json(json!({
                "success": false,
                "result": format!("(error) {}", e)
            }));
        }
    };

    let sender = state.db_manager.get_sender(db_id);
    let (tx, rx) = oneshot::channel();
    let message = DatabaseMessage::Command { sender: tx, command };

    if sender.send(message).await.is_err() {
        return Json(json!({
            "success": false,
            "result": "(error) 发送命令失败"
        }));
    }

    match rx.await {
        Ok(result_frame) => {
            let result = format_frame_result(&result_frame);
            Json(json!({
                "success": true,
                "result": result
            }))
        }
        Err(_) => Json(json!({
            "success": false,
            "result": "(error) 接收响应失败"
        }))
    }
}

/// 格式化Frame结果
fn format_frame_result(frame: &Frame) -> String {
    match frame {
        Frame::SimpleString(s) => s.clone(),
        Frame::BulkString(s) => format!("\"{}\"", s),
        Frame::Integer(i) => format!("(integer) {}", i),
        Frame::Null => "(nil)".to_string(),
        Frame::Error(e) => format!("(error) {}", e),
        Frame::Ok => "OK".to_string(),
        Frame::RDBFile(_) => "(rdb file)".to_string(),
        Frame::Array(arr) => {
            if arr.is_empty() {
                "(empty array)".to_string()
            } else {
                arr.iter()
                    .enumerate()
                    .map(|(i, f)| format!("{}) {}", i + 1, format_frame_result(f)))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }
    }
}

/// 获取AOF日志
async fn get_aof_logs(
    State(state): State<Arc<WebState>>,
    Query(params): Query<AofLogsQuery>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(1000).min(1000);
    let file_path = state.aof_path.to_string_lossy().to_string();
    
    if !state.aof_path.exists() {
        return Json(json!({
            "success": true,
            "data": [],
            "total": 0,
            "page": page,
            "size": size,
            "file_path": file_path,
            "file_size": "0B"
        }));
    }

    let file_size = match tokio::fs::metadata(&state.aof_path).await {
        Ok(meta) => format_memory(meta.len() as usize),
        Err(_) => "0B".to_string(),
    };

    match tokio::fs::read(&state.aof_path).await {
        Ok(content) => {
            let mut commands = Vec::new();
            let separator = b"\r\n\r\n";
            let mut start = 0;
            
            while let Some(pos) = content[start..].windows(separator.len()).position(|w| w == separator) {
                let end = start + pos;
                let frame_data = &content[start..end + 2];
                if !frame_data.is_empty() {
                    if let Ok(frame) = Frame::parse_from_bytes(frame_data) {
                        commands.push(format_frame_as_command(&frame));
                    }
                }
                start = end + separator.len();
            }
            
            let total = commands.len();
            // 从后往前取，返回最新的命令
            let start_idx = if total > page * size { total - page * size } else { 0 };
            let end_idx = if total > (page - 1) * size { total - (page - 1) * size } else { 0 };
            let page_data: Vec<_> = commands[start_idx..end_idx].iter().rev().cloned().collect();
            
            Json(json!({
                "success": true,
                "data": page_data,
                "total": total,
                "page": page,
                "size": size,
                "file_path": file_path,
                "file_size": file_size
            }))
        }
        Err(e) => Json(json!({
            "success": false,
            "error": format!("读取AOF文件失败: {}", e)
        }))
    }
}

/// 将Frame格式化为命令字符串
fn format_frame_as_command(frame: &Frame) -> String {
    match frame {
        Frame::Array(arr) => {
            arr.iter()
                .map(|f| match f {
                    Frame::BulkString(s) => s.clone(),
                    Frame::SimpleString(s) => s.clone(),
                    Frame::Integer(i) => i.to_string(),
                    _ => String::new(),
                })
                .collect::<Vec<_>>()
                .join(" ")
        }
        _ => String::new(),
    }
}

/// 获取服务器统计信息
async fn get_stats(State(state): State<Arc<WebState>>) -> impl IntoResponse {
    let mut total_keys = 0;
    let mut total_memory = 0;
    
    // 统计所有数据库的键数量和内存
    for db_id in 0..state.max_databases {
        let sender = state.db_manager.get_sender(db_id);
        
        // 获取DBSIZE
        let frame = Frame::Array(vec![Frame::BulkString("DBSIZE".to_string())]);
        if let Ok(command) = Command::parse_from_frame(frame) {
            let (tx, rx) = oneshot::channel();
            let message = DatabaseMessage::Command { sender: tx, command };
            if sender.send(message).await.is_ok() {
                if let Ok(Frame::Integer(count)) = rx.await {
                    total_keys += count as usize;
                    total_memory += count as usize * 100; // 估算内存
                }
            }
        }
    }
    
    Json(json!({
        "success": true,
        "data": {
            "connected_clients": 1,
            "total_keys": total_keys,
            "used_memory": total_memory,
            "used_memory_human": format_memory(total_memory),
            "databases": state.max_databases
        }
    }))
}

/// 格式化内存大小
fn format_memory(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2}KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2}MB", bytes as f64 / 1024.0 / 1024.0)
    } else {
        format!("{:.2}GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0)
    }
}

/// 列出所有数据库
async fn list_databases(State(state): State<Arc<WebState>>) -> impl IntoResponse {
    let mut databases = Vec::new();
    
    for db_id in 0..state.max_databases {
        let sender = state.db_manager.get_sender(db_id);
        
        // 获取DBSIZE
        let frame = Frame::Array(vec![Frame::BulkString("DBSIZE".to_string())]);
        let command = match Command::parse_from_frame(frame) {
            Ok(cmd) => cmd,
            Err(_) => continue,
        };
        
        let (tx, rx) = oneshot::channel();
        let message = DatabaseMessage::Command { sender: tx, command };
        
        if sender.send(message).await.is_ok() {
            if let Ok(result_frame) = rx.await {
                let key_count = match result_frame {
                    Frame::Integer(count) => count as usize,
                    _ => 0,
                };
                
                databases.push(DatabaseInfo {
                    id: db_id,
                    name: format!("db{}", db_id),
                    key_count,
                });
            }
        }
    }
    
    Json(json!({
        "success": true,
        "data": databases
    }))
}

/// 列出指定数据库的所有键
async fn list_keys(
    State(state): State<Arc<WebState>>,
    Query(params): Query<ListKeysQuery>,
) -> impl IntoResponse {
    let db_id = params.db.unwrap_or(0);
    let pattern = params.pattern.unwrap_or_else(|| "*".to_string());
    
    if db_id >= state.max_databases {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "数据库索引超出范围"
            }))
        );
    }
    
    let sender = state.db_manager.get_sender(db_id);
    
    // 执行KEYS命令
    let frame = Frame::Array(vec![
        Frame::BulkString("KEYS".to_string()),
        Frame::BulkString(pattern),
    ]);
    
    let command = match Command::parse_from_frame(frame) {
        Ok(cmd) => cmd,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("解析命令失败: {}", e)
                }))
            );
        }
    };
    
    let (tx, rx) = oneshot::channel();
    let message = DatabaseMessage::Command { sender: tx, command };
    
    if sender.send(message).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "发送命令失败"
            }))
        );
    }
    
    match rx.await {
        Ok(result_frame) => {
            let keys = match result_frame {
                Frame::Array(frames) => {
                    frames.into_iter().filter_map(|f| {
                        if let Frame::BulkString(key) = f {
                            Some(key)
                        } else {
                            None
                        }
                    }).collect::<Vec<_>>()
                },
                _ => vec![],
            };
            
            // 获取每个键的详细信息
            let mut key_infos = Vec::new();
            for key in keys {
                if let Some(info) = get_key_info(&state, db_id, &key).await {
                    key_infos.push(info);
                }
            }
            
            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "data": key_infos
                }))
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": format!("接收响应失败: {}", e)
            }))
        ),
    }
}

/// 获取键的详细信息（类型和TTL）
async fn get_key_info(state: &Arc<WebState>, db_id: usize, key: &str) -> Option<KeyInfo> {
    let sender = state.db_manager.get_sender(db_id);
    
    // 获取键类型
    let type_frame = Frame::Array(vec![
        Frame::BulkString("TYPE".to_string()),
        Frame::BulkString(key.to_string()),
    ]);
    
    let type_command = Command::parse_from_frame(type_frame).ok()?;
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command: type_command }).await.ok()?;
    
    let key_type = match rx.await.ok()? {
        Frame::SimpleString(t) => t,
        _ => "unknown".to_string(),
    };
    
    // 获取TTL
    let ttl_frame = Frame::Array(vec![
        Frame::BulkString("TTL".to_string()),
        Frame::BulkString(key.to_string()),
    ]);
    
    let ttl_command = Command::parse_from_frame(ttl_frame).ok()?;
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command: ttl_command }).await.ok()?;
    
    let ttl = match rx.await.ok()? {
        Frame::Integer(t) => t,
        _ => -1,
    };
    
    Some(KeyInfo {
        key: key.to_string(),
        key_type,
        ttl,
    })
}

/// 获取键值
async fn get_key_value(
    State(state): State<Arc<WebState>>,
    Path(key): Path<String>,
    Query(params): Query<ListKeysQuery>,
) -> impl IntoResponse {
    let db_id = params.db.unwrap_or(0);
    
    if db_id >= state.max_databases {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "数据库索引超出范围"
            }))
        );
    }
    
    let sender = state.db_manager.get_sender(db_id);
    
    // 先获取键类型
    let type_frame = Frame::Array(vec![
        Frame::BulkString("TYPE".to_string()),
        Frame::BulkString(key.clone()),
    ]);
    
    let type_command = match Command::parse_from_frame(type_frame) {
        Ok(cmd) => cmd,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("解析TYPE命令失败: {}", e)
                }))
            );
        }
    };
    
    let (tx, rx) = oneshot::channel();
    if sender.send(DatabaseMessage::Command { sender: tx, command: type_command }).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "发送TYPE命令失败"
            }))
        );
    }
    
    let key_type = match rx.await {
        Ok(Frame::SimpleString(t)) => t,
        _ => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "error": "键不存在"
                }))
            );
        }
    };
    
    // 根据类型获取值
    let value = match key_type.as_str() {
        "string" => get_string_value(&state, db_id, &key).await,
        "hash" => get_hash_value(&state, db_id, &key).await,
        "list" => get_list_value(&state, db_id, &key).await,
        "set" => get_set_value(&state, db_id, &key).await,
        "zset" => get_zset_value(&state, db_id, &key).await,
        _ => None,
    };
    
    if value.is_none() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "获取键值失败"
            }))
        );
    }
    
    // 获取TTL
    let ttl_frame = Frame::Array(vec![
        Frame::BulkString("TTL".to_string()),
        Frame::BulkString(key.clone()),
    ]);
    
    let ttl_command = Command::parse_from_frame(ttl_frame).ok().unwrap();
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command: ttl_command }).await.ok();
    
    let ttl = match rx.await.ok() {
        Some(Frame::Integer(t)) => t,
        _ => -1,
    };
    
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": KeyValueResponse {
                key: key.clone(),
                key_type,
                value: value.unwrap(),
                ttl,
            }
        }))
    )
}

/// 获取字符串类型的值
async fn get_string_value(state: &Arc<WebState>, db_id: usize, key: &str) -> Option<serde_json::Value> {
    let sender = state.db_manager.get_sender(db_id);
    let frame = Frame::Array(vec![
        Frame::BulkString("GET".to_string()),
        Frame::BulkString(key.to_string()),
    ]);
    
    let command = Command::parse_from_frame(frame).ok()?;
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command }).await.ok()?;
    
    match rx.await.ok()? {
        Frame::BulkString(s) => Some(json!(s)),
        Frame::Null => None,
        _ => None,
    }
}

/// 获取哈希类型的值
async fn get_hash_value(state: &Arc<WebState>, db_id: usize, key: &str) -> Option<serde_json::Value> {
    let sender = state.db_manager.get_sender(db_id);
    let frame = Frame::Array(vec![
        Frame::BulkString("HGETALL".to_string()),
        Frame::BulkString(key.to_string()),
    ]);
    
    let command = Command::parse_from_frame(frame).ok()?;
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command }).await.ok()?;
    
    match rx.await.ok()? {
        Frame::Array(frames) => {
            let mut map = serde_json::Map::new();
            let mut iter = frames.into_iter();
            while let (Some(Frame::BulkString(k)), Some(Frame::BulkString(v))) = (iter.next(), iter.next()) {
                map.insert(k, json!(v));
            }
            Some(json!(map))
        }
        _ => None,
    }
}

/// 获取列表类型的值
async fn get_list_value(state: &Arc<WebState>, db_id: usize, key: &str) -> Option<serde_json::Value> {
    let sender = state.db_manager.get_sender(db_id);
    let frame = Frame::Array(vec![
        Frame::BulkString("LRANGE".to_string()),
        Frame::BulkString(key.to_string()),
        Frame::BulkString("0".to_string()),
        Frame::BulkString("-1".to_string()),
    ]);
    
    let command = Command::parse_from_frame(frame).ok()?;
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command }).await.ok()?;
    
    match rx.await.ok()? {
        Frame::Array(frames) => {
            let values: Vec<String> = frames.into_iter().filter_map(|f| {
                if let Frame::BulkString(s) = f {
                    Some(s)
                } else {
                    None
                }
            }).collect();
            Some(json!(values))
        }
        _ => None,
    }
}

/// 获取集合类型的值
async fn get_set_value(state: &Arc<WebState>, db_id: usize, key: &str) -> Option<serde_json::Value> {
    let sender = state.db_manager.get_sender(db_id);
    let frame = Frame::Array(vec![
        Frame::BulkString("SMEMBERS".to_string()),
        Frame::BulkString(key.to_string()),
    ]);
    
    let command = Command::parse_from_frame(frame).ok()?;
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command }).await.ok()?;
    
    match rx.await.ok()? {
        Frame::Array(frames) => {
            let values: Vec<String> = frames.into_iter().filter_map(|f| {
                if let Frame::BulkString(s) = f {
                    Some(s)
                } else {
                    None
                }
            }).collect();
            Some(json!(values))
        }
        _ => None,
    }
}

/// 获取有序集合类型的值
async fn get_zset_value(state: &Arc<WebState>, db_id: usize, key: &str) -> Option<serde_json::Value> {
    let sender = state.db_manager.get_sender(db_id);
    // 使用ZRANGE获取所有成员和分数
    let frame = Frame::Array(vec![
        Frame::BulkString("ZRANGE".to_string()),
        Frame::BulkString(key.to_string()),
        Frame::BulkString("0".to_string()),
        Frame::BulkString("-1".to_string()),
    ]);
    
    let command = Command::parse_from_frame(frame).ok()?;
    let (tx, rx) = oneshot::channel();
    sender.send(DatabaseMessage::Command { sender: tx, command }).await.ok()?;
    
    match rx.await.ok()? {
        Frame::Array(frames) => {
            let values: Vec<String> = frames.into_iter().filter_map(|f| {
                if let Frame::BulkString(s) = f {
                    Some(s)
                } else {
                    None
                }
            }).collect();
            Some(json!(values))
        }
        _ => None,
    }
}

/// 设置键值
async fn set_key_value(
    State(state): State<Arc<WebState>>,
    Path(key): Path<String>,
    Query(params): Query<ListKeysQuery>,
    Json(payload): Json<SetKeyRequest>,
) -> impl IntoResponse {
    let db_id = params.db.unwrap_or(0);
    
    if db_id >= state.max_databases {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!( {
                "success": false,
                "error": "数据库索引超出范围"
            }))
        );
    }
    
    let sender = state.db_manager.get_sender(db_id);
    
    // 根据payload内容决定使用哪种命令
    let command_result = if !payload.fields.is_empty() {
        // 创建hash类型
        let mut args = vec![Frame::BulkString("HMSET".to_string()), Frame::BulkString(key.clone())];
        for (field, value) in payload.fields {
            args.push(Frame::BulkString(field));
            args.push(Frame::BulkString(value));
        }
        let frame = Frame::Array(args);
        Command::parse_from_frame(frame)
    } else if !payload.members_with_scores.is_empty() {
        // 创建zset类型
        let mut args = vec![Frame::BulkString("ZADD".to_string()), Frame::BulkString(key.clone())];
        for (score, member) in payload.members_with_scores {
            args.push(Frame::BulkString(score.to_string()));
            args.push(Frame::BulkString(member));
        }
        let frame = Frame::Array(args);
        Command::parse_from_frame(frame)
    } else if !payload.values.is_empty() {
        // 从查询参数获取类型，默认为list
        let key_type = params.key_type.clone().unwrap_or_else(|| "list".to_string()).to_lowercase();
        let (cmd_name, _) = match key_type.as_str() {
            "set" => ("SADD", "集合"),
            "list" | _ => ("LPUSH", "列表"),
        };
        
        let mut args = vec![Frame::BulkString(cmd_name.to_string()), Frame::BulkString(key.clone())];
        for value in payload.values {
            args.push(Frame::BulkString(value));
        }
        let frame = Frame::Array(args);
        Command::parse_from_frame(frame)
    } else {
        // 创建string类型
        let frame = Frame::Array(vec![
            Frame::BulkString("SET".to_string()),
            Frame::BulkString(key.clone()),
            Frame::BulkString(payload.value),
        ]);
        Command::parse_from_frame(frame)
    };
    
    let command = match command_result {
        Ok(cmd) => cmd,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!( {
                    "success": false,
                    "error": format!("解析命令失败: {}", e)
                }))
            );
        }
    };
    
    let (tx, rx) = oneshot::channel();
    if sender.send(DatabaseMessage::Command { sender: tx, command }).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!( {
                "success": false,
                "error": "发送命令失败"
            }))
        );
    }
    
    if rx.await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!( {
                "success": false,
                "error": "设置键值失败"
            }))
        );
    }
    
    // 如果指定了TTL，设置过期时间
    if let Some(ttl) = payload.ttl {
        let expire_frame = Frame::Array(vec![
            Frame::BulkString("EXPIRE".to_string()),
            Frame::BulkString(key.clone()),
            Frame::BulkString(ttl.to_string()),
        ]);
        
        if let Ok(expire_command) = Command::parse_from_frame(expire_frame) {
            let (tx, _rx) = oneshot::channel();
            let _ = sender.send(DatabaseMessage::Command { sender: tx, command: expire_command }).await;
        }
    }
    
    (
        StatusCode::OK,
        Json(json!( {
            "success": true,
            "message": "键值设置成功"
        }))
    )
}

/// 删除键
async fn delete_key(
    State(state): State<Arc<WebState>>,
    Path(key): Path<String>,
    Query(params): Query<ListKeysQuery>,
) -> impl IntoResponse {
    let db_id = params.db.unwrap_or(0);
    
    if db_id >= state.max_databases {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "数据库索引超出范围"
            }))
        );
    }
    
    let sender = state.db_manager.get_sender(db_id);
    
    // 执行DEL命令
    let frame = Frame::Array(vec![
        Frame::BulkString("DEL".to_string()),
        Frame::BulkString(key),
    ]);
    
    let command = match Command::parse_from_frame(frame) {
        Ok(cmd) => cmd,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("解析DEL命令失败: {}", e)
                }))
            );
        }
    };
    
    let (tx, rx) = oneshot::channel();
    if sender.send(DatabaseMessage::Command { sender: tx, command }).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "发送DEL命令失败"
            }))
        );
    }
    
    match rx.await {
        Ok(Frame::Integer(count)) if count > 0 => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "键删除成功"
            }))
        ),
        _ => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "error": "键不存在或删除失败"
            }))
        ),
    }
}

/// 更新键的TTL
async fn update_key_ttl(
    State(state): State<Arc<WebState>>,
    Path(key): Path<String>,
    Query(params): Query<ListKeysQuery>,
    Json(payload): Json<UpdateTtlRequest>,
) -> impl IntoResponse {
    let db_id = params.db.unwrap_or(0);
    
    if db_id >= state.max_databases {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "数据库索引超出范围"
            }))
        );
    }
    
    let sender = state.db_manager.get_sender(db_id);
    
    // 执行EXPIRE命令
    let frame = Frame::Array(vec![
        Frame::BulkString("EXPIRE".to_string()),
        Frame::BulkString(key),
        Frame::BulkString(payload.ttl.to_string()),
    ]);
    
    let command = match Command::parse_from_frame(frame) {
        Ok(cmd) => cmd,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("解析EXPIRE命令失败: {}", e)
                }))
            );
        }
    };
    
    let (tx, rx) = oneshot::channel();
    if sender.send(DatabaseMessage::Command { sender: tx, command }).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": "发送EXPIRE命令失败"
            }))
        );
    }
    
    match rx.await {
        Ok(Frame::Integer(1)) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "TTL更新成功"
            }))
        ),
        _ => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "error": "键不存在或TTL更新失败"
            }))
        ),
    }
}
