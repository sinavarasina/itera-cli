use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Meta {
    pub message: String,
    pub status: bool,
    pub code: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub user_id: String,
    pub nama: String,
    pub level: String,
    pub nomor_id: String,
    pub unit: String,
    pub photo: String,
    pub email: String,
    pub nimnrk: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub meta: Meta,
    pub data: UserData,
}
