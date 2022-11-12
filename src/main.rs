use clap::Parser;
use std::process::Command;
use lettre::{Message, SmtpTransport, Transport};
use ulid::Ulid;
use trust_dns_resolver::{Resolver, config::{ResolverConfig, ResolverOpts}, proto::rr::{RecordType, rdata::MX, RData}};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long)]
    target_email: String,

    #[clap()]
    command: Vec<String>,
}

struct EmailAddr {
    id: String,
    domain: String,
}

impl EmailAddr {
    pub fn new(raw_email_addr: &str) -> EmailAddr {
        let splitted_email: Vec<&str> = raw_email_addr.split("@").collect();

        EmailAddr {
            id: splitted_email[0].to_string(),
            domain: splitted_email[1].to_string(),
        }
    }
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

    let email_addr = EmailAddr::new(&cli.target_email);

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let smtp_server = resolver.lookup(email_addr.domain, RecordType::MX).unwrap();
    let smtp_record = smtp_server.iter()
        .fold(None, |acc: Option<&MX>, e| match e {
            RData::MX(mx_record) => {
                let original_pref = match acc {
                    Some(original_mx_record) => original_mx_record.preference(),
                    None => u16::MAX,
                };

                if mx_record.preference() < original_pref {
                    Some(mx_record)
                } else {
                    acc
                }
            },
            _ => acc,
        })
        .expect("No MX Record exists");

    let email = Message::builder()
        .from("Long Running Notifier <notify@beleap.dev>".parse().unwrap())
        .to(format!("You <{}>", cli.target_email).parse().unwrap())
        .message_id(Some(format!("<{}@beleap.dev>", Ulid::new().to_string())))
        .subject(format!("`{}` finished!", real_command))
        .body(format!("`{}` finished with status {}", real_command, output))
        .unwrap();

    let mailer = SmtpTransport::builder_dangerous(smtp_record.exchange().to_string()).build();

    match mailer.send(&email) {
        Ok(_) => {},
        Err(e) => panic!("Failed to send email: {:?}", e),
    }
}
