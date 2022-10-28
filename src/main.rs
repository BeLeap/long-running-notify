use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long)]
    target_email: String,

    #[clap()]
    command: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli.command);
}
