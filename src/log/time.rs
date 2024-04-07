use crate::log::style::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{DateTime, Local};
use env_logger::fmt::Formatter;
use std::io::{Result, Write};
use std::sync::atomic::{AtomicI32, AtomicI64, Ordering};

static ORDERING: Ordering = Ordering::Relaxed;
static UNINIT_OFFSET: i32 = 1; // Unlikely to clash.
static LAST_TIMESTAMP: AtomicI64 = AtomicI64::new(0);
static LAST_OFFSET: AtomicI32 = AtomicI32::new(UNINIT_OFFSET);

pub fn write_now(f: &mut Formatter) -> Result<()> {
    write_timestamp(f, Local::now())
}

pub fn write_timestamp(f: &mut Formatter, ts: DateTime<Local>) -> Result<()> {
    const FORMAT: &str = "%Y%m%d%H%M%S%f";
    let last_ts = format!(
        "{}",
        ts.timezone()
            .timestamp_nanos(LAST_TIMESTAMP.load(ORDERING))
            .format(FORMAT)
    );
    let cur_ts = format!("{}", ts.format(FORMAT));
    let mut sep = f.style();
    deemph_style(&mut sep);
    let mut status = None; // Whether a timestamp diff has been found; style used after.

    let mut chars = last_ts.bytes().zip(cur_ts.bytes()).enumerate();
    while let Some((i, (l, c))) = chars.next() {
        // Separator chars to read "YYYY-mm-DDTHH:MM:SS.sss_sss_sss".
        match i {
            4 | 6 => write!(f, "{}", sep.value("-")),
            8 => write!(f, "{}", invisible_style(&mut f.style()).value("T")),
            10 | 12 => write!(f, "{}", sep.value(":")),
            14 => write!(f, "{}", sep.value(".")),
            17 | 20 => write!(f, "{}", sep.value("_")),
            _ => Ok(()),
        }?;

        let ch = c as char;
        match &status {
            None => {
                use std::cmp::Ordering::*;
                match c.cmp(&l) {
                    // Print timestamp before diff deemphasized.
                    Equal => write!(f, "{}", sep.value(ch)),
                    Greater => {
                        // Print the rest after diff brighter.
                        sep = f.style();
                        info_style(&mut sep);
                        status = Some(f.style());
                        match status {
                            Some(ref mut style) => write!(f, "{}", normal_style(style).value(ch)),
                            _ => unreachable!(),
                        }
                    }
                    Less => {
                        // Time regression: print the rest as error.
                        sep = f.style();
                        error_style(&mut sep);
                        status = Some(f.style());
                        match status {
                            Some(ref mut style) => write!(f, "{}", error_style(style).value(ch)),
                            _ => unreachable!(),
                        }
                    }
                }
            }
            Some(style) => write!(f, "{}", style.value(ch)),
        }?
    }

    let last_off = LAST_OFFSET.load(ORDERING);
    let cur_off = ts.offset().fix().local_minus_utc();
    let mut tzf = f.style();
    if cur_off == LAST_OFFSET.load(ORDERING) {
        deemph_style(&mut tzf);
    } else if last_off == UNINIT_OFFSET {
        normal_style(&mut tzf);
    } else {
        error_style(&mut tzf); // Treat any live timezone change as error.
    }
    write!(f, "{}", tzf.value(ts.format("%:z")))?;

    LAST_TIMESTAMP.store(ts.timestamp_nanos_opt().unwrap(), ORDERING);
    LAST_OFFSET.store(cur_off, ORDERING);
    Ok(())
}
