use crate::api_itera::models::Meta;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Kelas {
    pub nomor_mk: String,
    pub kode_mk: String,
    pub nama_mk: String,
    pub sks_mk: String,
    pub nama_dosen_list: String,
}

#[derive(Debug, Deserialize)]
pub struct KelasListResponse {
    pub meta: Meta,
    pub data: Vec<Kelas>,
}
