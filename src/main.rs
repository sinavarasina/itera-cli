pub mod api_itera;
pub mod app;
pub mod command;
pub mod config;
pub mod utils;

use crate::api_itera::IteraAPI;
use crate::utils::device_info::DeviceInfo;

#[tokio::main]
async fn main() {
    let device = DeviceInfo::new();

    let itera_api_client = IteraAPI::new(&device.name(), &device.id());

    app::run(itera_api_client).await;
}
