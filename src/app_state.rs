use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
}
