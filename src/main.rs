use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo
    Add {
        name: String
    },
    /// Mark as done
    Done {
        name: String
    },
    /// Remove a todo
    Remove {
        name: String
    },
}

fn main() {
    println!("Hello, world!");
    let cli = Cli::parse();
}
