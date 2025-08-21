use crate::api_itera::IteraAPI;
use crate::command::{AccountCommands, Cli, Commands, account};
use clap::Parser;
use owo_colors::OwoColorize;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc, oneshot};

pub async fn run(itera_api_client: IteraAPI) {
    println!("{}",
        "itera-cli - Yet an unofficial Command-Line Interface (CLI) app for managing academic activities at ITERA.".yellow()
    );
    println!(
        "type '{}' to see Command List, '{}' to close the app",
        "help".green(),
        "exit".red()
    );

    let (tx, mut rx) = mpsc::channel::<(Commands, oneshot::Sender<()>)>(32);
    let shared_client = Arc::new(Mutex::new(itera_api_client));

    let ui_client = Arc::clone(&shared_client);

    let ui_handle = tokio::spawn(async move {
        run_ui_task(ui_client, tx).await;
    });

    while let Some((command, response_tx)) = rx.recv().await {
        {
            let mut client_guard = shared_client.lock().await;
            match command {
                Commands::Account(account_cmd) => match account_cmd {
                    AccountCommands::Login { email, password } => {
                        if let (Some(e), Some(p)) = (email, password) {
                            account::handle_login(&mut *client_guard, e, p).await;
                        } else {
                            eprintln!("[ERROR] Email or password was not provided to handler.");
                        }
                    }
                    AccountCommands::Logout => {
                        account::handle_logout(&mut *client_guard);
                    }
                    AccountCommands::Status => {
                        account::handle_status(&*client_guard);
                    }
                },
                Commands::Exit => {
                    println!("{}", "Close the app...".red());
                    ui_handle.abort();
                }
            }
        }

        let _ = response_tx.send(());
    }
}

async fn run_ui_task(
    itera_api_client: Arc<Mutex<IteraAPI>>,
    tx: mpsc::Sender<(Commands, oneshot::Sender<()>)>,
) {
    let rl = Arc::new(Mutex::new(
        DefaultEditor::new().expect("[ERROR] failed to initialize rustyline"),
    ));

    loop {
        let prompt = {
            let client_guard = itera_api_client.lock().await;
            match client_guard.get_logged_nim() {
                Some(nim) => format!("{} ({})> ", "itera-cli".blue(), nim.green()),
                None => format!("{} ()> ", "itera-cli".blue()),
            }
        };

        let rl_clone = Arc::clone(&rl);
        let line_result = tokio::task::spawn_blocking(move || {
            let mut rl_guard = rl_clone.blocking_lock();
            rl_guard.readline(&prompt)
        })
        .await;

        let line = match line_result {
            Ok(Ok(line)) => line,
            Ok(Err(ReadlineError::Interrupted | ReadlineError::Eof)) => {
                let _ = tx.send((Commands::Exit, oneshot::channel().0)).await;
                println!("\n[INFO] UI Task stopped");
                break;
            }
            _ => {
                eprintln!("\n[ERROR] Failed to read input or UI task panicked.");
                break;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        {
            let mut rl_guard = rl.lock().await;
            let _ = rl_guard.add_history_entry(line.as_str());
        }

        let args = std::iter::once("itera-cli".to_string())
            .chain(line.split_whitespace().map(String::from));

        match Cli::try_parse_from(args) {
            Ok(cli) => {
                let command_to_process = match cli.command {
                    Commands::Account(AccountCommands::Login { email, password: _ }) => {
                        let (final_email, final_password) =
                            tokio::task::spawn_blocking(move || {
                                let email_to_use = email.unwrap_or_else(|| {
                                    print!("Email: ");
                                    std::io::stdout().flush().unwrap();
                                    let mut buffer = String::new();
                                    std::io::stdin().read_line(&mut buffer).unwrap();
                                    buffer.trim().to_string()
                                });
                                println!("Password for {}:", email_to_use.green());
                                let password_to_use = rpassword::prompt_password("").unwrap();
                                (email_to_use, password_to_use)
                            })
                            .await
                            .expect("Task blocking for login failed");
                        Commands::Account(AccountCommands::Login {
                            email: Some(final_email),
                            password: Some(final_password),
                        })
                    }
                    other_command => other_command,
                };

                let (response_tx, response_rx) = oneshot::channel();

                if tx.send((command_to_process, response_tx)).await.is_err() {
                    break;
                }

                if response_rx.await.is_err() {
                    break;
                }
            }
            Err(e) => {
                e.print().unwrap();
            }
        }
    }
}
