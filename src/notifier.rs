use std::error::Error;

pub mod email;
pub mod freedesktop;

pub trait Notifier {
    fn send(&self, summary: String, content: String) -> Result<(), Box<dyn Error>>;
}
