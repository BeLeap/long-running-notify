use clap::{Parser, ValueEnum};
use notifier::{email::EmailNotifier, freedesktop::FreedesktopNotifier, Notifier};
use std::process::Command;

mod notifier;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(value_enum)]
    mode: Mode,

    #[clap(short, long)]
    target_email: Option<String>,

    #[clap()]
    command: Vec<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Email,
    Notification,
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

    let output = command.status().unwrap();

    match cli.mode {
        Mode::Email => {
            let target_email = cli
                .target_email
                .expect("target_email is required for email mode");
            let notifier = EmailNotifier::new(target_email);
            notifier.send(real_command, output.to_string()).unwrap();
        }
        Mode::Notification => {
            let notifier = FreedesktopNotifier::new();
            notifier.send(real_command, output.to_string()).unwrap();
        }
    }
}
