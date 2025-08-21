pub mod account;
pub mod course;
pub mod presence;
use clap::{ArgGroup, Parser, Subcommand};

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
    #[command(about = "Close the app")]
    Exit,
}

#[derive(Debug, Subcommand, Clone)]
pub enum AccountCommands {
    #[command(about = "login with SSO account to access some command that require Auth/Token")]
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
    #[command(about = "delete login information")]
    Logout,
    #[command(about = "check your account status")]
    Status,
}

#[derive(Debug, Subcommand, Clone)]
pub enum PresenceCommands {
    #[command(
        group(
            ArgGroup::new("search")
            .args(
                &["by_code", "by_name"]
                ).required(true).multiple(false)
            )
        )]
    List {
        #[arg(
            long,
            help = "using class code (you can find out by using Course List command)"
        )]
        by_code: Option<String>,
        #[arg(
            long,
            help = "search by class name (the name must exact same as in the database"
        )]
        by_name: Option<String>,
    },
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
    #[command(about = "get class list")]
    List,
}
