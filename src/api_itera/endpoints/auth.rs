use crate::api_itera::models::LoginResponse;
use reqwest::Client;
use std::collections::HashMap;

const BASE_URL: &str = "https://api.itera.ac.id/v2/auth";

pub async fn login(
    client: &Client,
    username: &str,
    password: &str,
    device: &str,
    device_id: &str,
) -> Result<LoginResponse, reqwest::Error> {
    let mut form = HashMap::new();
    form.insert("username", username);
    form.insert("password", password);
    form.insert("device", device);
    form.insert("device_id", device_id);
    client
        .post(format!("{}/login", BASE_URL))
        .form(&form)
        .send()
        .await?
        .json::<LoginResponse>()
        .await
}
