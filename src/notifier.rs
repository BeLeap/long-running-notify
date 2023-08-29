use self::email::EmailNotifierError;

pub mod email;

#[derive(Debug)]
pub enum NotifierError {
    EmailNotifierError(EmailNotifierError),
}

pub trait Notifier {
    fn send(&self, summary: String, content: String) -> Result<(), NotifierError>;
}
