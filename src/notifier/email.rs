use std::error::Error;

use super::{Notifier, NotifierError};
use lettre::{Message, SmtpTransport, Transport};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    proto::rr::{rdata::MX, RData, RecordType},
    Resolver,
};
use ulid::Ulid;

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

    pub fn get_raw(&self) -> String {
        format!("{}@{}", self.id, self.domain)
    }
}

#[derive(Debug)]
pub enum EmailNotifierError {
    FailedNameResolverCreation,
    FailedNameResolution,
    EmptyMXRecord,
    InvalidMessage,
    FailedToSendMessage,
}

pub struct EmailNotifier {
    email_addr: String,
}

impl EmailNotifier {
    pub fn new(email_addr: String) -> Self {
        Self { email_addr }
    }
}

impl Notifier for EmailNotifier {
    fn send(&self, summary: String, content: String) -> Result<(), super::NotifierError> {
        let email_addr = EmailAddr::new(&self.email_addr);

        let resolver = match Resolver::new(ResolverConfig::default(), ResolverOpts::default()) {
            Ok(v) => v,
            Err(_) => {
                return Err(NotifierError::EmailNotifierError(
                    EmailNotifierError::FailedNameResolverCreation,
                ))
            }
        };
        let smtp_server = match resolver.lookup(&email_addr.domain, RecordType::MX) {
            Ok(v) => v,
            Err(_) => {
                return Err(NotifierError::EmailNotifierError(
                    EmailNotifierError::FailedNameResolution,
                ));
            }
        };
        let smtp_record = match smtp_server
            .iter()
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
                }
                _ => acc,
            }) {
            Some(v) => v,
            None => {
                return Err(NotifierError::EmailNotifierError(
                    EmailNotifierError::EmptyMXRecord,
                ));
            }
        };

        let email = match Message::builder()
            .from("Long Running Notifier <notify@beleap.dev>".parse().unwrap())
            .to(format!("You <{}>", email_addr.get_raw()).parse().unwrap())
            .message_id(Some(format!("<{}@beleap.dev>", Ulid::new().to_string())))
            .subject(format!("`{}` finished!", summary))
            .body(format!("`{}` finished with status {}", summary, content))
        {
            Ok(v) => v,
            Err(_) => {
                return Err(NotifierError::EmailNotifierError(
                    EmailNotifierError::InvalidMessage,
                ))
            }
        };

        let mailer = SmtpTransport::builder_dangerous(smtp_record.exchange().to_string()).build();

        match mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(_) => Err(NotifierError::EmailNotifierError(
                EmailNotifierError::FailedToSendMessage,
            )),
        }
    }
}
