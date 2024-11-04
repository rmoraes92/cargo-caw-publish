use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn init_logger(debug_mode: bool) {
    let level_filter: LevelFilter = if debug_mode {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    let encoder = Box::new(
        PatternEncoder::new(
            //"{d} || {l} || {t} || {m} || {n}"
            "[{l}] {m}{n}"
        )
    );
    let stdout = ConsoleAppender::builder()
        .encoder(encoder)
        .build();
    log4rs::init_config(
        Config::builder()
        .appender(
            Appender::builder().build(
                "stdout",
                Box::new(stdout),
            )
        )
        .build(
            Root::builder()
                .appender("stdout")
                .build(level_filter)
        )
        .unwrap()
    ).unwrap();
}