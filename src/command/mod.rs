use clap::{Parser, Subcommand};

pub mod account;
pub mod course;
pub mod presence;

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
    #[command(subcommand, about = "Courses related commands")]
    Course(CourseCommand),
    #[command(subcommand, about = "presence related commands")]
    Presence(PresenceCommands),
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
    #[command(about = "List presence status for a class")]
    List {
        #[arg(
            long,
            help = "Search by course ID",
            conflicts_with = "by_name",
            required = true
        )]
        by_id: Option<String>,
        #[arg(
            long,
            help = "Search by course name",
            conflicts_with = "by_id",
            required = true
        )]
        by_name: Option<String>,
        #[arg(
            short,
            long,
            help = "Output format: table, table_rounded, or json (default)"
        )]
        style: Option<String>,
    },
    #[command(about = "submit token from QR code to get presence status in class")]
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
    List {
        #[arg(
            short,
            long,
            help = "to format the output be the same as you desire (table, table_rounded, json <default>))"
        )]
        style: Option<String>,
    },
}
