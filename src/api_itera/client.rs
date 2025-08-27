use crate::api_itera::endpoints;
use crate::api_itera::error::ApiError;
use crate::api_itera::models::{
    KelasListResponse, PresensiDataResponse, PresensiResponse, UserData,
};
use reqwest::Client;

// it may confusing, because the OG API are using indonesian languages
// but i dont feel right to use full indonesian in programming
// so i made it hybrid with english as task name that explain the function
// and indonesia for the things.
// some function may have weird names, due i follow the api endpoints name.
pub struct IteraAPI {
    client: Client,
    device_info: (String, String),
    auth: Option<UserData>,
}

impl IteraAPI {
    pub fn new(device: &str, device_id: &str) -> Self {
        Self {
            client: Client::new(),
            device_info: (device.to_string(), device_id.to_string()),
            auth: None,
        }
    }

    pub async fn login(&mut self, email: &str, password: &str) -> Result<(), ApiError> {
        let (device, device_id) = (self.device_info.0.as_str(), self.device_info.1.as_str());
        let response =
            endpoints::auth::login(&self.client, email, password, device, device_id).await?;

        if !response.meta.status {
            return Err(ApiError::ApiMessage(response.meta.message));
        }

        self.auth = Some(response.data);
        Ok(())
    }

    pub fn logout(&mut self) {
        self.auth = None;
    }

    pub fn is_logged_in(&self) -> bool {
        self.auth.is_some()
    }

    pub fn get_user_data(&self) -> Option<&UserData> {
        self.auth.as_ref()
    }

    pub fn get_logged_name(&self) -> Option<&str> {
        self.auth.as_ref().map(|user_data| user_data.nama.as_str())
    }

    pub fn get_logged_nim(&self) -> Option<&str> {
        self.auth
            .as_ref()
            .map(|user_data| user_data.nimnrk.as_str())
    }

    pub fn get_logged_prodi(&self) -> Option<&str> {
        self.auth.as_ref().map(|user_data| user_data.unit.as_str())
    }

    pub async fn get_mahasiswa_kelas(&self) -> Result<KelasListResponse, ApiError> {
        if let Some(auth_data) = &self.auth {
            let response = endpoints::mahasiswa::get_kelas(
                &self.client,
                &auth_data.token,
                &auth_data.nimnrk,
                &self.device_info.1,
            )
            .await?;

            if !response.meta.status {
                return Err(ApiError::ApiMessage(response.meta.message));
            }

            Ok(response)
        } else {
            Err(ApiError::AuthenticationRequired)
        }
    }

    pub async fn get_presensi_kelas(
        &self,
        kelas_id: &str,
    ) -> Result<PresensiDataResponse, ApiError> {
        if let Some(auth_data) = &self.auth {
            let response = endpoints::presensi::get_data_mahasiswa(
                &self.client,
                &auth_data.token,
                &auth_data.nimnrk,
                kelas_id,
                &self.device_info.1,
            )
            .await?;

            if !response.meta.status {
                return Err(ApiError::ApiMessage(response.meta.message));
            }

            Ok(response)
        } else {
            Err(ApiError::AuthenticationRequired)
        }
    }

    pub async fn send_token_presensi(
        &self,
        nim: &str,
        presensi_token: &str,
    ) -> Result<String, ApiError> {
        let response =
            endpoints::presensi::send_presence_token(&self.client, nim, presensi_token).await?;

        match response.json::<PresensiResponse>().await {
            Ok(json_response) => Ok(json_response.msg),

            Err(_) => Err(ApiError::ApiMessage(
                "Failed to parse API response message".to_string(),
            )),
        }
    }
}
