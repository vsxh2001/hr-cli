use clap::{Parser, Subcommand};
use crate::models::Human;


#[derive(Parser)]
#[command(name = "hr", about = "Human Resource CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a human
    Add {
        #[command(flatten)]
        human: Human,
    },
    /// Remove a human
    Remove {
        /// Name of the human to remove
        name: String,
    },
    /// List all humans
    List,
}

pub fn run() {
    let command = Cli::parse();
    match command.command {
        Command::Add { human } => println!("Adding {}", human.name),
        Command::Remove { name } => println!("Removing {}", name),
        Command::List => println!("Listing"),
    }
}