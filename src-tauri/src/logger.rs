use std::{fs, path::PathBuf};

use anyhow::Result;
use chrono::Local;
use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

/// initialize this instance's log file
pub fn init(log_dir: &PathBuf) -> Result<()> {
    if !log_dir.exists() {
        fs::create_dir_all(&log_dir)?;
    }

    let local_time = Local::now().format("%Y-%m-%d-%H%M").to_string();
    let log_file_name = format!("{}.log", local_time);
    let log_file = log_dir.join(log_file_name);

    let log_format = "{d(%Y-%m-%d %H:%M:%S)} {l} - {m}{n}";

    let encode = Box::new(PatternEncoder::new(log_format));

    let stdout = ConsoleAppender::builder().encoder(encode.clone()).build();
    let tofile = FileAppender::builder().encoder(encode).build(log_file)?;

    let level = LevelFilter::Debug;

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(tofile)))
        .build(
            Root::builder()
                .appenders(["file", "stdout"])
                .build(LevelFilter::Info),
        )?;

    log4rs::init_config(config)?;

    Ok(())
}
