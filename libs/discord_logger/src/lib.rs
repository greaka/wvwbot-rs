extern crate log;
extern crate serenity;

use log::{Level, Metadata, Record};
use serenity::{http, model::webhook::Webhook};

pub struct DiscordLogger {
    webhook: Webhook,
    levels: Vec<Level>,
}

impl DiscordLogger {
    pub fn new(webhook_id: u64, webhook_token: &str, log_levels: Vec<Level>) -> DiscordLogger {
        DiscordLogger {
            webhook: http::get_webhook_with_token(webhook_id, webhook_token)
                .expect("error initializing discord logger"),
            levels: log_levels,
        }
    }
}

impl log::Log for &DiscordLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.levels.contains(&metadata.level())
    }

    fn log(&self, record: &Record) {
        let _ = self
            .webhook
            .execute(false, |w| w.content(&format!("`{}`", record.args())));
    }

    fn flush(&self) {}
}
