pub mod api_itera;
pub mod app;
pub mod command;
pub mod config;
pub mod utils;

use crate::api_itera::IteraAPI;
use crate::utils::device_info::DeviceInfo;

// just make it to try/test the functionalities
#[tokio::main]
async fn main() {
    let device = DeviceInfo::new();
    let device_name = device.name();
    let device_id = device.id();
    println!("Device Name\t: {}", device_name);
    println!("Device ID \t: {}", device_id);

    let mut itera_api = IteraAPI::new(&device_name, &device_id);

    let email = " ";
    let password = " ";
    let login_response = itera_api.login(email, password).await;
    match login_response {
        Ok(_) => {
            println!("Login Successful!");

            if let Some(user_data) = itera_api.get_user_data() {
                match serde_json::to_string_pretty(&user_data) {
                    Ok(json_string) => {
                        println!("{}", json_string);
                    }
                    Err(_) => {
                        eprintln!("Failed to Serialize.");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Login Fail: {:?}", e);
        }
    }
}
