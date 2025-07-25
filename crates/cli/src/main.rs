use clap::Parser;

#[derive(Parser)]
#[command(about = "Rush CLI", version, author)]
struct Cli {
    /// Optional name to greet
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let greet = rush::greet();
    match cli.name {
        Some(name) => println!("{greet} {name}"),
        None => println!("{greet}"),
    }
}
