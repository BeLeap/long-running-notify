use std::{error::Error, fmt::Display, time::Duration};

use dbus::{arg::PropMap, blocking::Connection};

use super::Notifier;

#[derive(Debug)]
pub enum FreedesktopError {
    FailedCreateDBusSession,
    FailedToSendNotification,
}

impl Display for FreedesktopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for FreedesktopError {}

#[derive(Debug)]
pub struct FreedesktopNotifier {}

impl FreedesktopNotifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl Notifier for FreedesktopNotifier {
    fn send(&self, summary: String, content: String) -> Result<(), Box<dyn Error>> {
        let conn = match Connection::new_session() {
            Ok(v) => v,
            Err(_) => return Err(Box::new(FreedesktopError::FailedCreateDBusSession)),
        };

        let proxy = conn.with_proxy(
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            Duration::from_millis(5000),
        );

        let app_name = "lrn";
        let replaces_id: u32 = 0;
        let app_icon = "";
        let actions: Vec<String> = vec![];
        let hints = PropMap::new();

        let _: (u32,) = match proxy.method_call(
            "org.freedesktop.Notifications",
            "Notify",
            (
                app_name,
                replaces_id,
                app_icon,
                summary,
                content,
                actions,
                hints,
                0,
            ),
        ) {
            Ok(v) => v,
            Err(_) => return Err(Box::new(FreedesktopError::FailedToSendNotification)),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::notifier::Notifier;

    use super::FreedesktopNotifier;

    #[test]
    fn test_freedesktop_notifier() {
        let notifier = FreedesktopNotifier::new();

        notifier
            .send("summary".to_string(), "testcontent".to_string())
            .unwrap();
    }
}
