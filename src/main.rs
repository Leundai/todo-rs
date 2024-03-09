use clap::{Parser, Subcommand};

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
    List,
}

fn main() {
    let cli = Cli::parse();
    let mut todos = Todos::new();

    // todos.add(String::from("Hello there"));
    // todos.add(String::from("Hello there"));
    // todos.add(String::from("Hello there"));
    // todos.add(String::from("Hello there"));

    // todos.mark_done(1);
    // todos.remove(1);

    if let Some(command) = &cli.command {
        match command {
            Commands::Add { content } => {
                todos.add(content.clone());
            }
            Commands::Remove { index } => {
                todos.remove(index.clone());
            }
            Commands::Done { index } => {
                todos.mark_done(index.clone());
            }
            Commands::List {} => {
                println!("{}", todos)
            }
        }
    }
}
