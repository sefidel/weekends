use std::env;

use askama::Template;
use axum::{
    extract::{Multipart, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router, body::StreamBody,
};
use tracing::info;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index() -> IndexTemplate {
    IndexTemplate
}

async fn stream_forwarder(mut multipart: Multipart) -> impl IntoResponse {
    if let Some(field) = multipart.next_field().await.unwrap() {
        if field.name().unwrap() != "origin" {
            return Err((
                StatusCode::BAD_REQUEST,
                "Invalid multipart field was provided".to_string(),
            ));
        }

        let origin_url = &field
            .text()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        info!("Attempting to fetch content from {}", origin_url);

        let byte_stream = reqwest::get(origin_url)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .bytes_stream();

        info!("Inbound stream created");

        let stream_body = StreamBody::new(byte_stream);

        Ok((StatusCode::OK, stream_body))
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            "No multipart field was provided".to_string(),
        ))
    }
}

async fn stream_forwarder_get(Path(origin_url): Path<String>) -> impl IntoResponse {
    if origin_url.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Empty URL provided".to_string()))
    }

    info!("Attempting to fetch content from {}", origin_url);

    let byte_stream = reqwest::get(origin_url)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .bytes_stream();

    info!("Inbound stream created");

    let stream_body = StreamBody::new(byte_stream);

    Ok((StatusCode::OK, stream_body))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let host = env::var("BEAMY_HOST").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    let app = Router::new()
        .route("/", get(index))
        .route("/*url", get(stream_forwarder_get))
        .route("/", post(stream_forwarder));

    info!("Binding to {host}");

    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
