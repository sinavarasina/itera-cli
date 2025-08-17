use crate::api_itera::models::PresensiDataResponse;
use reqwest::{Client, Response};
use std::collections::HashMap;

const BASE_URL: &str = "https://api.itera.ac.id/v2/presensi";

pub async fn get_data_mahasiswa(
    client: &Client,
    token: &str,
    nim: &str,
    kelas_id: &str,
    device_id: &str,
) -> Result<PresensiDataResponse, reqwest::Error> {
    let mut form = HashMap::new();
    form.insert("nim", nim);
    form.insert("kelas", kelas_id);

    client
        .post(format!("{}/data_mahasiswa", BASE_URL))
        .header("Authorization", token)
        .header("X-Device-Id", device_id)
        .form(&form)
        .send()
        .await?
        .json::<PresensiDataResponse>()
        .await
}

pub async fn send_presence_token(
    client: &Client,
    nim: &str,
    classmeet_token: &str,
) -> Result<Response, reqwest::Error> {
    let mut form = HashMap::new();
    form.insert("nim", nim);
    form.insert("token", classmeet_token);
    client
        .post(format!("{}/kelas", BASE_URL))
        .form(&form)
        .send()
        .await
}
