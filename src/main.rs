use clap::Parser;
use std::process::Command;

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

    let shell = match std::env::var("SHELL") {
        Ok(shell) => shell,
        Err(_) => "sh".to_string(),
    };
    let real_command = cli.command.join(" ");

    let mut command = Command::new(shell);
    command.args(["-c", &real_command]);

    let output = command.output().unwrap();
    if output.status.success() {
        println!("{:?}", output);
    } else {
        println!("{:?}", output);
    }

}
