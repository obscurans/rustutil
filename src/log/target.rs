use crate::log::style::*;
use env_logger::fmt::{Formatter, Style};
use std::io::{Result, Write};
use std::sync::atomic::{AtomicUsize, Ordering};

static ORDERING: Ordering = Ordering::Relaxed;
static MAX_TARGET_WIDTH: AtomicUsize = AtomicUsize::new(0);
static TARGET_SEP: &str = ":";
static SEP_LEN: usize = TARGET_SEP.len();

struct TargetStyles {
    pub deemph: Style,
    pub info: Style,
    pub abbrev: Style,
    pub last: Style,
}

impl TargetStyles {
    fn get(f: &mut Formatter) -> TargetStyles {
        let mut deemph = f.style();
        deemph_style(&mut deemph);
        let mut info = f.style();
        info_style(&mut info);
        let mut abbrev = f.style();
        abbrev_style(&mut abbrev);
        let mut last = f.style();
        target_last_style(&mut last);
        TargetStyles {
            deemph,
            info,
            abbrev,
            last,
        }
    }
}

pub fn write_target(f: &mut Formatter, target: &str) -> Result<()> {
    let last_width = MAX_TARGET_WIDTH.load(ORDERING);
    let s = TargetStyles::get(f);
    let mut len;

    let (first, rest) = target.split_once("::").unwrap_or((target, ""));
    match first {
        "rustutil" => {
            len = 1;
            write!(f, "{}", s.abbrev.value("U"))?;
            if !rest.is_empty() {
                write_rustutil(f, rest, &mut len, &s)?;
            }
        }
        _ => {
            len = first.len();
            if !rest.is_empty() {
                write!(f, "{}", s.info.value(first))?;
                write_other(f, rest, &mut len, &s)?;
            } else {
                write!(f, "{}", s.last.value(first))?;
            }
        }
    }

    if last_width < len {
        MAX_TARGET_WIDTH.store(len, ORDERING);
    } else {
        // write!(f, "{: <1$}", "", last_width - len)?;
    }
    Ok(())
}

fn write_rustutil(f: &mut Formatter, rest: &str, len: &mut usize, s: &TargetStyles) -> Result<()> {
    let (next, rest) = rest.split_once("::").unwrap_or((rest, ""));
    match next {
        "log" => {
            *len += 1;
            write!(f, "{}", s.abbrev.value("L"))?;
            if !rest.is_empty() {
                write_other(f, rest, len, s)?;
            }
        }
        _ => {
            *len += SEP_LEN + next.len();
            write!(f, "{}{}", s.deemph.value(TARGET_SEP), s.info.value(next))?;
            if !rest.is_empty() {
                write_other(f, rest, len, s)?;
            }
        }
    }
    Ok(())
}

fn write_other<'a>(f: &mut Formatter, rest: &str, len: &mut usize, s: &TargetStyles) -> Result<()> {
    let (rest, last) = rest.rsplit_once("::").unwrap_or(("", rest));
    if !rest.is_empty() {
        for chunk in rest.split("::") {
            *len += SEP_LEN + chunk.len();
            write!(f, "{}{}", s.deemph.value(TARGET_SEP), s.info.value(chunk))?
        }
    }
    *len += SEP_LEN + last.len();
    write!(f, "{}{}", s.deemph.value(TARGET_SEP), s.last.value(last))
}
