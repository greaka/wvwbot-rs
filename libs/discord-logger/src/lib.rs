extern crate log;
extern crate serenity;

use log::{Level, Metadata, Record};
use serenity::http;
use std;

pub struct DiscordLogger {
    id: u64,
    token: &str,
    level: Level,
}

impl DiscordLogger {
    fn new(webhook_id: u64, webhook_token: &str, log_level: Level) -> DiscordLogger {
        DiscordLogger {
            id: webhook_id,
            token: webhook_token,
            level: log_level
        }
    }
}

impl log::Log for DiscordLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() == self.level
    }

    fn log(&self, record: &Record) {
        let mut webhook = http::get_webhook_with_token(&self.id, &self.token)?;

        let _ = webhook
            .execute(false, |w| w.content(concat!("`", record.args(), "`")));
    }

    fn flush(&self) {}
}
