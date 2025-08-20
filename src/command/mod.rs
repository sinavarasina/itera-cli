use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "itera-cli", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    #[command(subcommand)]
    Account(AccountCommands),
    Exit,
}

#[derive(Debug, Subcommand, Clone)]
pub enum AccountCommands {
    Login {
        #[arg(long)]
        email: Option<String>,

        #[clap(skip)]
        password: Option<String>,
    },
    Logout,
    Status,
}

pub mod account;
pub mod presence;
pub mod subject;
