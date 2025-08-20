pub mod account;
pub mod course;
pub mod presence;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "itera-cli", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    #[command(subcommand, about = "Account related commands")]
    Account(AccountCommands),
    //    Heavily WIP
    //    #[command(subcommand, about = "Courses related commands")]
    //    Course(CourseCommand),
    //    #[command(subcommand, about = "presence related commands")]
    //    Presence(PresenceCommands),
    Exit,
}

#[derive(Debug, Subcommand, Clone)]
pub enum AccountCommands {
    Login {
        #[arg(
            short,
            long,
            help = "input email for login in directly command args (optional)"
        )]
        email: Option<String>,

        #[clap(skip)]
        password: Option<String>,
    },
    Logout,
    Status,
}

#[derive(Debug, Subcommand, Clone)]
pub enum PresenceCommands {
    List,
    Submit {
        #[arg(
            short,
            long,
            help = "specify nim to submit the presence token (optional for logged user)"
        )]
        nim: Option<String>,
        #[arg(
            short,
            long,
            help = "input token for class you want to get presence/attend"
        )]
        token: Option<String>,
    },
}

#[derive(Debug, Subcommand, Clone)]
pub enum CourseCommand {
    List,
}
