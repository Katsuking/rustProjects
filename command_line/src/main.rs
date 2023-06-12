use clap::Parser;

#[derive(Parser)]
#[command(name = "Test CLI")]
#[command(author = "Tay Keith <Taykeith@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    #[arg(long, action = clap::ArgAction::Count)]
    two: String,
    #[arg(long)]
    one: String,
}

fn main() {
    let cli = Cli::parse();

    println!("two: {:?}", cli.two);
    println!("one: {:?}", cli.one);
}