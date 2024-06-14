use askama::Template;
use askama_axum::IntoResponse;
use axum::Json;
use reqwest::{Client, StatusCode};

#[derive(Template)]
#[template(path = "home_page.html")]
struct HomePage {}

pub async fn home_page() -> impl IntoResponse {
    HomePage {}.into_response()
}


pub async fn proxy_suggestions(query: axum::extract::Query<std::collections::HashMap<String, String>>) -> impl IntoResponse {
    if let Some(q) = query.get("q") {
        let client = Client::new();
        let url = format!("https://suggestqueries.google.com/complete/search?client=firefox&q={}", q);
        match client.get(&url).send().await {
            Ok(response) => {
                if let Ok(body) = response.text().await {
                    return (StatusCode::OK, Json(body)).into_response();
                }
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
    StatusCode::BAD_REQUEST.into_response()
}
