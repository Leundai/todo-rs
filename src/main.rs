use clap::{Parser, Subcommand};

use colored::Colorize;
use todo_rs::Todos;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo
    Add { content: String },
    /// Mark as done
    Done { index: u16 },
    /// Remove a todo
    Remove { index: u16 },
    /// Show todos
    List {
        #[arg(short, long)]
        all: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut todos = Todos::new().expect("Failed to load todos!");

    if let Some(command) = &cli.command {
        match command {
            Commands::Add { content } => {
                todos
                    .add(content.clone())
                    .unwrap_or_else(|e| println!("Failed to add todo: {}", e));
            }
            Commands::Remove { index } => {
                todos
                    .remove(index.clone())
                    .unwrap_or_else(|e| println!("Failed to remove todo: {}", e));
            }
            Commands::Done { index } => {
                todos
                    .mark_done(index.clone())
                    .unwrap_or_else(|e| println!("Failed to mark as done: {}", e));
            }
            Commands::List { all } => {
                todos.print(all.clone());
            }
        }
    } else {
        println!(
            "Please provide a command. Run {} for more info",
            "--help".italic()
        );
    }
}
