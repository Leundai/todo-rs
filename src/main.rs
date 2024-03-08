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
}

fn main() {
    let cli = Cli::parse();
    let mut todo = Todos::new();

    todo.add(String::from("Hello there"));
    todo.add(String::from("Hello there"));
    todo.add(String::from("Hello there"));
    todo.add(String::from("Hello there"));

    todo.mark_done(1);
    todo.remove(1);
    match &cli.command.unwrap() {
        Commands::Add { content } => {
            todo.add(content.clone());
        }
        Commands::Remove { index } => {
            todo.remove(index.clone());
        }
        Commands::Done { index } => {
            todo.mark_done(index.clone());
        }
    }

    println!("{}", todo)
}
