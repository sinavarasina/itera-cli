use crate::api_itera::models::Meta;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize, Tabled)]
#[serde(rename_all = "snake_case")]
pub struct Presensi {
    pub no_pertemuan: u8,
    pub pertemuan: String,
    pub waktu_mulai: String,
    pub mhs_masuk: String,
    pub mhs_tdkmasuk: String,
    pub mhs_jumlah: String,
    pub absen_mahasiswa: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PresensiDataResponse {
    pub meta: Meta,
    pub data: Vec<Presensi>,
}

#[derive(Debug, Deserialize)]
pub struct PresensiResponse {
    pub msg: String,
}
