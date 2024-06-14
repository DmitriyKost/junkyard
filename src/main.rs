use askama_axum::IntoResponse;
use axum::{http::{header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue}, routing::get, Router};
use tower_http::set_header::response::SetResponseHeaderLayer;
use askama::Template;


#[tokio::main()]
async fn main() {
    let router = Router::new().
        route("/", get(home_page))
        .layer(SetResponseHeaderLayer::if_not_present(
            ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        ));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, router).await.unwrap()
}

async fn home_page() -> impl IntoResponse {
    HomePage{}.into_response()
}

#[derive(Template)]
#[template(path = "home_page.html")]
struct HomePage {}
