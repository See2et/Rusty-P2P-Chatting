use clap::Parser;

/// CLI options for the Rusty P2P chat application.
#[derive(Parser, Debug)]
struct Cli {
    /// Username used for the session.
    #[arg(long)]
    username: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Username: {}", cli.username);
}
