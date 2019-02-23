extern crate log;
extern crate serenity;

use log::{Level, Metadata, Record};
use serenity::{http, model::webhook::Webhook};
use std;

pub struct DiscordLogger {
    webhook: Webhook,
    level: Level,
}

impl DiscordLogger {
    fn new(webhook_id: u64, webhook_token: &str, log_level: Level) -> DiscordLogger {
        DiscordLogger {
            webhook: http::get_webhook_with_token(webhook_id, webhook_token)
                .expect("error initializing discord logger"),
            level: log_level,
        }
    }
}

impl log::Log for DiscordLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() == self.level
    }

    fn log(&self, record: &Record) {
        let _ = self
            .webhook
            .execute(false, |w| w.content(&format!("`{}`", record.args())));
    }

    fn flush(&self) {}
}
