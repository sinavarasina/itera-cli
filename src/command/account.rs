use crate::api_itera::client::IteraAPI;
use owo_colors::OwoColorize;

pub async fn handle_login(itera_api_client: &mut IteraAPI, email: String, password: String) {
    println!("Trying to loggin ...");
    match itera_api_client.login(&email, &password).await {
        Ok(_) => println!("{}", "Login successfully!".green()),
        Err(e) => eprintln!(
            "{} Login Failed: {}",
            "[ERROR]".black().on_red(),
            e.red().blink()
        ),
    }
}

pub fn handle_logout(itera_api_client: &mut IteraAPI) {
    if itera_api_client.is_logged_in() {
        itera_api_client.logout();
        println!("{}", "Logout Success.".green());
    } else {
        println!("{}", "Not even login yet..".red());
    }
}

pub fn handle_status(itera_api_client: &IteraAPI) {
    if let (Some(name), Some(nim), Some(prodi)) = (
        itera_api_client.get_logged_name(),
        itera_api_client.get_logged_nim(),
        itera_api_client.get_logged_prodi(),
    ) {
        println!("{}", "[Status]".black().on_green());
        println!("{}\t: {}", "Login".yellow(), "true".green().blink());
        println!("{}\t: {}", "Nama".yellow(), name.blue());
        println!("{}\t: {}", "NIM".yellow(), nim.blue());
        println!("{}\t: {}", "Prodi".yellow(), prodi.blue());
    } else {
        println!("{}", "[Status]".black().on_red());
        println!("{} : {}", "Login".yellow(), "false".red().blink());
    }
}
