
use clap::{Parser, Subcommand};
use debug::DebugLevel;

mod db;
mod debug;
mod tui;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "CLI for interacting with a PiMesh System"
)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    User,
}

fn main() {
    let cli = Cli::parse();
    let mut _debuglevel: DebugLevel = DebugLevel::Prod;

    match cli.debug {
        1 => _debuglevel = DebugLevel::Low,
        2 => _debuglevel = DebugLevel::High,
        _ => {}
    }

    match &cli.command {
        Some(Commands::User) => {
            manage_user_command(&cli);
            return;
        }
        None => {}
    }


    let res = tui::run_tui();
    if let Err(e) = res {
        println!("Error running TUI: {e}");
    }


    // Should be cli ( ratatui )

}

fn manage_user_command(_cli: &Cli) {
    println!("In user command");
}
