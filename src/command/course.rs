use crate::api_itera::IteraAPI;
use owo_colors::OwoColorize;
use tabled::{Table, settings::Style};

pub async fn handle_list_courses(itera_api_client: &IteraAPI, style: &str) {
    println!("fetching courses list...");
    match itera_api_client.get_mahasiswa_kelas().await {
        Ok(response) => {
            let courses = response.data;

            if courses.is_empty() {
                println!("{}", "No courses found".yellow().blink());
            }

            match style {
                "table" => {
                    let style = Style::modern();
                    println!("{}", Table::new(&courses).with(style));
                }
                "table_rounded" => {
                    let style = Style::rounded();
                    println!("{}", Table::new(&courses).with(style));
                }
                "json" | _ => match serde_json::to_string_pretty(&courses) {
                    Ok(json_string) => {
                        println!("{}", json_string);
                    }
                    Err(_) => {
                        eprintln!(
                            "{} {}",
                            "[ERROR]".black().on_red(),
                            "failed to Serialize.".red().blink()
                        )
                    }
                },
            }
        }
        Err(err) => {
            eprintln!(
                "{} Failed to fetch courses : {:?}",
                "[ERROR]".black().on_red(),
                err.red().blink()
            );
        }
    }
}
