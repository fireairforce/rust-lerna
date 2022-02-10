use anyhow::Result;
use axum::{
    extract::{ Path, Extension },
    handler::get,
    http::{ StatusCode, HeaderValue, StatusCode },
    Router,
    AddExtensionLayer,
};
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use std::convert::TryInto;

/// 引入 protobuf 生成的代码
mod pb;

use pb::*;

// 参数使用 serde 做 Deserialize, axum 会自动识别并解析
#[derive(Deserialize)]
struct Params {
  spec: String,
  url: String,
}

#[tokio::main]
async fn main() {
    // 初始化 tracing
    tracing_subscriber::fmt::init();

    // 构建路由 
    let app = Router::new()
    // `GET /image` 会执行 generate 函数把 spec 和 url 参数传进去
        .route("/image/:spec/:url", get(generate));
    
    // 运行 web 服务器
    let addr = "127.0.0.1:3000".parse().unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 把参数解析出来
async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {}\n spec: {:#?}", url, spec))
}