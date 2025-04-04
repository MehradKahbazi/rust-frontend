use gloo_net::Error;
use gloo_net::http::Request;
use serde::Deserialize;
use serde_json::json;

use super::APP_HOST;

#[derive(Deserialize)]
pub struct LoginResponse{
    pub token: String
}

#[derive(Deserialize)]
pub struct MeResponse{
    pub id: i32,
    pub username: String,
    pub created_at: String
}

pub async fn api_login(username: String, password: String) -> Result<LoginResponse, Error>{
    let response = Request::post(&format!("{}/login", APP_HOST))
    .json(&json!(
        {
            "username": username,
            "password": password
        }
    ))?.send().await?;

    response.json::<LoginResponse>().await

}

pub async fn api_me(token: &String) -> Result<MeResponse, Error>{
    let response = Request::get(&format!("{}/me", APP_HOST))
    .header("Authorization", &format!("Bearer {}", token))
    .send().await?;

    response.json::<MeResponse>().await
}
