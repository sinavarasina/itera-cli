use crate::api_itera::models::KelasListResponse;
use reqwest::Client;
use std::collections::HashMap;

const BASE_URL: &str = "https://api.itera.ac.id/v2/mahasiswa";

pub async fn get_kelas(
    client: &Client,
    token: &str,
    nim: &str,
    device_id: &str,
) -> Result<KelasListResponse, reqwest::Error> {
    let mut form = HashMap::new();
    form.insert("nim", nim);

    client
        .post(format!("{}/kelas", BASE_URL))
        .header("Authorization", token)
        .header("X-Device-Id", device_id)
        .form(&form)
        .send()
        .await?
        .json::<KelasListResponse>()
        .await
}
