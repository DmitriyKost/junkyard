use axum::{
    http::{header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue},
    routing::get,
    Router,
};

use reqwest::Client;
use tower_http::set_header::response::SetResponseHeaderLayer;

mod app_state;
mod home_page;
use home_page::{home_page, proxy_suggestions};

#[tokio::main]
async fn main() {
    let state = app_state::AppState {
        client: Client::new(),
    };
    let router = Router::new()
        .route("/", get(home_page))
        .route("/suggestions", get(proxy_suggestions))
        .with_state(state)
        .layer(SetResponseHeaderLayer::if_not_present(
            ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        ));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
