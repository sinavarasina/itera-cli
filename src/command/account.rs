use crate::api_itera::client::IteraAPI;

pub async fn handle_login(itera_api_client: &mut IteraAPI, email: String, password: String) {
    println!("Trying to loggin ...");
    match itera_api_client.login(&email, &password).await {
        Ok(_) => println!("Login successfully!"),
        Err(e) => eprintln!("[ERROR] Login Failed: {}", e),
    }
}

pub fn handle_logout(itera_api_client: &mut IteraAPI) {
    if itera_api_client.is_logged_in() {
        itera_api_client.logout();
        println!("Logout Success.");
    } else {
        println!("Not even login yet..");
    }
}

pub fn handle_status(itera_api_client: &IteraAPI) {
    if let (Some(name), Some(nim)) = (
        itera_api_client.get_logged_name(),
        itera_api_client.get_logged_nim(),
    ) {
        println!("Status Login: true");
        println!("  Nama: {}", name);
        println!("  NIM : {}", nim);
    } else {
        println!("Status Login: false");
    }
}
