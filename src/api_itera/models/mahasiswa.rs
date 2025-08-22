use crate::api_itera::models::Meta;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize, Tabled)]
#[serde(rename_all = "snake_case")]
pub struct Kelas {
    pub nomor_mk: String,
    pub kode_mk: String,
    pub kode_kelas: String,
    pub nama_kelas: String,
    #[tabled(display = "null_handler")]
    pub mode: Option<String>,
    pub nama_mk: String,
    pub sks_mk: String,
    pub nama_dosen_list: String,
    pub jadwal_hari: String,
}

#[derive(Debug, Deserialize)]
pub struct KelasListResponse {
    pub meta: Meta,
    pub data: Vec<Kelas>,
}

fn null_handler(opt: &Option<String>) -> String {
    opt.clone().unwrap_or_else(|| "-".to_string())
}
