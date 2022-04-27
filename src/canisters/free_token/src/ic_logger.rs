use ic_cdk::api;
use log::{Level, LevelFilter, Metadata, Record};
use std::panic;
use yansi::Paint;
pub struct ICLogger;

impl log::Log for ICLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();
            let message = format!(
                "{}, {}: {} - {}",
                "free_token",
                record.target(),
                level,
                record.args()
            );

            let str = match level {
                Level::Error => Paint::red(message),
                Level::Warn => Paint::yellow(message),
                Level::Info => Paint::blue(message),
                Level::Debug => Paint::green(message),
                Level::Trace => Paint::magenta(message),
            };
            api::print(str.to_string());
        }
    }

    fn flush(&self) {}
}
