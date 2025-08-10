use clap::{Parser, Subcommand};
use crate::{models::Human, storage};

pub mod search;
// export feature removed


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
    /// Search humans by regex name, must-have labels, and minimal metrics
    Search {
        #[command(flatten)]
        human: Human,
    },
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
        Command::Search { human } => {
            match search::run(storage, &human) {
                Ok(results) => {
                    for h in results {
                        println!("Found human: {}", h.name);
                    }
                }
                Err(e) => eprintln!("Search failed: {}", e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_add_minimal() {
        let cli = Cli::try_parse_from(["hr", "add", "--name", "Alice"]).expect("parse");
        match cli.command {
            Command::Add { human } => {
                assert_eq!(human.name, "Alice");
                assert!(human.id.is_none());
                assert!(human.phone.is_none());
                assert!(human.description.is_none());
                assert!(human.label.is_none());
                assert!(human.metric.is_none());
            }
            _ => panic!("expected Add"),
        }
    }

    #[test]
    fn parse_add_with_labels_and_metrics() {
        let cli = Cli::try_parse_from([
            "hr", "add",
            "--name", "Bob",
            "--label", "eng",
            "--label", "oncall",
            "--metric", "speed:10",
            "--metric", "height:20",
        ]).expect("parse");

        match cli.command {
            Command::Add { human } => {
                let labels = human.label.expect("labels");
                assert_eq!(labels, vec!["eng".to_string(), "oncall".to_string()]);
                let metrics = human.metric.expect("metrics");
                assert_eq!(metrics.len(), 2);
                assert_eq!(metrics[0].name, "speed");
                assert_eq!(metrics[0].value, 10);
                assert_eq!(metrics[1].name, "height");
                assert_eq!(metrics[1].value, 20);
            }
            _ => panic!("expected Add"),
        }
    }
}