use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long, value_parser)]
    target_email: String,

    #[clap(value_parser)]
    command: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli.command);
}
