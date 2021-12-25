//! An opinionated [`env_logger`] [`format`](Builder::format).

use env_logger::fmt::Formatter;
use env_logger::Builder;
use log::info;
use log::Record;
use std::io::{Result, Write};

mod style;
mod target;
mod time;
use style::*;

#[doc(no_inline)]
pub use log::Level;

/// Initializes the preset [`Logger`](env_logger::Logger) as global.
pub fn init_logformat() {
    logformat_builder().init();
    info!("Logger initialized");
}

/// Generates a preset [`Builder`].
pub fn logformat_builder() -> Builder {
    let mut builder = Builder::new();
    builder.format(logformat).parse_default_env();
    builder
}

fn logformat(f: &mut Formatter, record: &Record) -> Result<()> {
    use Level::*;
    let mut deemph = f.style();
    deemph_style(&mut deemph);

    time::write_now(f)?;

    let mut style = f.style();
    let level = record.level();
    let level_str = level_style(&mut style, level).value(match level {
        Trace => "TRACE",
        Debug => "DEBUG",
        Info => "INFO ",
        Warn => "WARN ",
        Error => "ERROR",
    });
    write!(f, "{}{}{}", deemph.value("["), level_str, deemph.value(" "))?;

    target::write_target(f, record.target())?;

    let mut style = f.style();
    (match level {
        Info => normal_style,
        Warn => warn_style,
        Error => error_style,
        _ => info_style,
    })(&mut style);
    writeln!(f, "{}{}", deemph.value("] "), style.value(record.args()))
}
