use crate::api_itera::IteraAPI;
use crate::utils;
use owo_colors::OwoColorize;

pub async fn handle_presence_list(
    itera_api_client: &IteraAPI,
    by_id: Option<String>,
    by_name: Option<String>,
    style: &str,
) {
    if let Some(id) = by_id {
        println!("Search by ID: {}", id.as_str().green().underline());
        match itera_api_client.get_presensi_kelas(&id).await {
            Ok(res) => utils::presence_helper::print_presence_data(&res.data, style, None),
            Err(e) => eprintln!("{} {:?}", "[ERROR]".black().on_red(), e),
        }
    } else if let Some(name) = by_name {
        println!("Search by Name: {}", name.green().underline());

        let course_list = match itera_api_client.get_mahasiswa_kelas().await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{} {:?}", "[ERROR]".black().on_red(), e);
                return;
            }
        };

        let matches: Vec<_> = course_list
            .data
            .iter()
            .filter(|course| course.nama_mk.to_lowercase().contains(&name.to_lowercase()))
            .collect();

        if matches.is_empty() {
            eprintln!("{} {}", "[ERROR]".black().on_red(), "No class name matched");
            return;
        }

        for (index, found_courses) in matches.iter().enumerate() {
            if index > 0 {
                println!();
            }

            match itera_api_client
                .get_presensi_kelas(&found_courses.kode_kelas)
                .await
            {
                Ok(res) => {
                    utils::presence_helper::print_presence_data(
                        &res.data,
                        style,
                        Some(&found_courses.nama_mk),
                    );
                }
                Err(e) => {
                    eprintln!(
                        "{} Could not fetch presence for {}: {:?}",
                        "[ERROR]".black().on_red(),
                        found_courses.nama_mk.yellow(),
                        e
                    );
                }
            }
        }
    } else {
        eprintln!(
            "{} {}",
            "[ERROR]".black().on_red(),
            "No search criteria provided"
        );
    }
}

pub async fn handle_submit(
    itera_api_client: &IteraAPI,
    nim: Option<String>,
    token: Option<String>,
) {
    println!("Trying to submit presence token");

    let Some(token) = token else {
        println!("{} {}", "[Error]".black().on_red(), "Token is required");
        return;
    };

    let Some(nim_to_use) = nim.or_else(|| itera_api_client.get_logged_nim().map(|s| s.to_string()))
    else {
        println!(
            "{} {}",
            "[Error]".red(),
            "Not logged in and no NIM provided"
        );
        return;
    };

    println!("{} {}", "Using NIM :".green(), nim_to_use.yellow());
    println!("{} {}", "Token :".green(), token.yellow());

    match itera_api_client
        .send_token_presensi(&nim_to_use, &token)
        .await
    {
        Ok(msg) => {
            if msg.contains("Tidak Sesuai") {
                println!("{} {}", "[Error]".black().on_red(), "Token not valid");
            } else {
                println!("{} {}", "[Success]".black().on_green(), "Presence Success");
            }
        }
        Err(e) => println!("{} {:?}", "[Error]".black().on_red(), e),
    }
}
