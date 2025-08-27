use owo_colors::OwoColorize;
use serde::Serialize;
use tabled::{Table, settings::Style};

pub fn print_presence_data<T: tabled::Tabled + Serialize>(
    data: &[T],
    style: &str,
    title: Option<&str>,
) {
    if let Some(t) = title {
        println!("\n{}", t.bold().cyan());
    }

    match style {
        "table" => {
            let mut table = Table::new(data);
            table.with(Style::modern());
            println!("{}", table);
        }
        "table_rounded" => {
            let mut table = Table::new(data);
            table.with(Style::rounded());
            println!("{}", table);
        }
        _ => match serde_json::to_string_pretty(data) {
            Ok(json_string) => println!("{}", json_string),
            Err(_) => eprintln!(
                "{} {}",
                "[ERROR]".black().on_red(),
                "failed to Serialize.".red().blink()
            ),
        },
    }
}
