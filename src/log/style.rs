use env_logger::fmt::Color::*;
use env_logger::fmt::Style;
use log::Level;

type S<'a> = &'a mut Style;

pub fn invisible_style(s: S) -> S {
    s.set_color(Black)
}

pub fn deemph_style(s: S) -> S {
    s.set_intense(true).set_color(Black)
}

pub fn normal_style(s: S) -> S {
    s.set_intense(true).set_color(White)
}

pub fn abbrev_style(s: S) -> S {
    s.set_intense(true).set_color(Cyan)
}

pub fn target_last_style(s: S) -> S {
    s.set_intense(true).set_color(Green)
}

pub fn level_style(s: S, level: Level) -> S {
    use Level::*;
    (match level {
        Trace => trace_style,
        Debug => debug_style,
        Info => info_style,
        Warn => warn_style,
        Error => error_style,
    })(s)
}

pub fn trace_style(s: S) -> S {
    s.set_color(Magenta)
}

pub fn debug_style(s: S) -> S {
    s.set_color(Blue)
}

pub fn info_style(s: S) -> S {
    s.set_color(White)
}

pub fn warn_style(s: S) -> S {
    s.set_intense(true).set_color(Yellow).set_bold(true)
}

pub fn error_style(s: S) -> S {
    s.set_intense(true).set_color(Red).set_bold(true)
}
