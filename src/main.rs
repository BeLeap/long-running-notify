use clap::Parser;
use clap::{Subcommand, ValueHint};
use notifier::{email::EmailNotifier, freedesktop::FreedesktopNotifier, Notifier};
use std::process::{Command, ExitStatus};

mod notifier;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    mode: Mode,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Mode {
    Email(EmailArgs),
    Notification(NotificationArgs),
}

#[derive(Parser, Debug, PartialEq)]
struct EmailArgs {
    #[clap(short, long)]
    target_email: Option<String>,

    #[clap(value_hint = ValueHint::CommandWithArguments)]
    command: Vec<String>,
}

#[derive(Parser, Debug, PartialEq)]
struct NotificationArgs {
    #[clap()]
    command: Vec<String>,
}

fn run_command(cmd: &str) -> ExitStatus {
    let shell = match std::env::var("SHELL") {
        Ok(shell) => shell,
        Err(_) => "sh".to_string(),
    };

    let mut command = Command::new(shell);
    command.args(["-c", cmd]);

    command.status().unwrap()
}

fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Email(ref args) => {
            let target_email = args
                .target_email
                .clone()
                .expect("target_email is required for email mode");
            let command = args.command.join(" ");
            let output = run_command(&command);

            let notifier = EmailNotifier::new(target_email.to_string());
            notifier.send(command, output.to_string()).unwrap();
        }
        Mode::Notification(ref args) => {
            let command = args.command.join(" ");
            let output = run_command(&command);

            let notifier = FreedesktopNotifier::new();
            notifier.send(command, output.to_string()).unwrap();
        }
    }
}
