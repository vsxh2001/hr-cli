use clap::{Parser, Subcommand};
use crate::{models::Human, storage};


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

pub fn run(storage: &storage::Storage) {
    let command = Cli::parse();
    match command.command {
        Command::Add { human } => {
            storage.save(&human);
            println!("Adding {}", human.name);
        }
        Command::Remove { name } => {
            storage.remove(&name).unwrap();
            println!("Removing {}", name);
        }
        Command::List => {
            let humans = storage.load_all().unwrap();
            for human in humans {
                println!("Found human: {}", human.name);
            }
        }
    }
}