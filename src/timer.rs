//! Timer utility class.
use log::{log, Level};
use std::time::{Duration, Instant};

/// RAII timer utility using [`env_logger`].
pub struct Timer<'a> {
    /// Base time to measure against.
    pub start: Instant,
    /// Message to print on log.
    pub msg: &'a str,
    /// [`Level`] to log at.
    pub level: Level,
}

impl Timer<'_> {
    /// Constructor. Will log at the given [`Level`] now and at dropping, both with the given message.
    pub fn new(level: Level, msg: &str) -> Timer {
        log!(level, "[start]{}{}", sep(msg), msg);
        Timer {
            start: Instant::now(),
            msg,
            level,
        }
    }

    /// Convenience constructor for [`Level::Trace`].
    pub fn trace(msg: &str) -> Timer {
        Timer::new(Level::Trace, msg)
    }

    /// Convenience constructor for [`Level::Debug`].
    pub fn debug(msg: &str) -> Timer {
        Timer::new(Level::Debug, msg)
    }

    /// Convenience constructor for [`Level::Info`].
    pub fn info(msg: &str) -> Timer {
        Timer::new(Level::Info, msg)
    }

    /// Convenience constructor for [`Level::Warn`].
    pub fn warn(msg: &str) -> Timer {
        Timer::new(Level::Warn, msg)
    }

    /// Convenience constructor for [`Level::Error`].
    pub fn error(msg: &str) -> Timer {
        Timer::new(Level::Error, msg)
    }

    /// Logs an extra message at the given [`Level`] with the current time elapsed, stored message, and passed 2nd message.
    pub fn log_at(&self, level: Level, msg2: &str) {
        log!(
            level,
            "[{}]{}{}{}{}",
            format_duration(self.start.elapsed()),
            sep(self.msg),
            self.msg,
            sep2(self.msg, msg2),
            msg2
        )
    }

    /// Convenience function to log extra message at the stored [`Level`]
    pub fn log(&self, msg2: &str) {
        self.log_at(self.level, msg2)
    }

    /// Convenience function to log extra message at [`Level::Trace`].
    pub fn log_trace(&self, msg2: &str) {
        self.log_at(Level::Trace, msg2)
    }

    /// Convenience function to log extra message at [`Level::Debug`].
    pub fn log_debug(&self, msg2: &str) {
        self.log_at(Level::Debug, msg2)
    }

    /// Convenience function to log extra message at [`Level::Info`].
    pub fn log_info(&self, msg2: &str) {
        self.log_at(Level::Info, msg2)
    }

    /// Convenience function to log extra message at [`Level::Warn`].
    pub fn log_warn(&self, msg2: &str) {
        self.log_at(Level::Warn, msg2)
    }

    /// Convenience function to log extra message at [`Level::Error`].
    pub fn log_error(&self, msg2: &str) {
        self.log_at(Level::Error, msg2)
    }
}

/// Prints another log at the chosen [`Level`] measuring time elapsed, with the stored message.
impl Drop for Timer<'_> {
    fn drop(&mut self) {
        log!(
            self.level,
            "[{}]{}{}",
            format_duration(self.start.elapsed()),
            sep(self.msg),
            self.msg
        );
    }
}

fn sep(msg: &str) -> &str {
    if msg.is_empty() {
        ""
    } else {
        " "
    }
}

fn sep2(msg: &str, msg2: &str) -> &'static str {
    if msg.is_empty() && msg2.is_empty() {
        ""
    } else if msg.is_empty() || msg2.is_empty() {
        " "
    } else {
        ": "
    }
}

pub fn format_duration(dur: Duration) -> String {
    let ns = dur.as_nanos();
    let (us, ns) = (ns / 1000, ns % 1000);
    if us <= 0 {
        return format!("{}ns", ns);
    }
    let (ms, us) = (us / 1000, us % 1000);
    if ms <= 0 {
        return format!("{}.{:03}µs", us, ns);
    }
    let (sec, ms) = (ms / 1000, ms % 1000);
    if sec <= 0 {
        return format!("{}.{:03}_{:03}ms", ms, us, ns);
    }
    let (min, sec) = (sec / 60, sec % 60);
    if min <= 0 {
        return format!("{}.{:03}_{:03}_{:03}s", sec, ms, us, ns);
    }
    let (hr, min) = (min / 60, min % 60);
    if hr <= 0 {
        return format!("{}:{:02}.{:03}_{:03}_{:03}", min, sec, ms, us, ns);
    }
    let (day, hr) = (hr / 24, hr % 24);
    if day <= 0 {
        return format!("{}:{:02}:{:02}.{:03}_{:03}_{:03}", hr, min, sec, ms, us, ns);
    }
    return format!(
        "{}d{:02}:{:02}:{:02}.{:03}_{:03}_{:03}",
        day, hr, min, sec, ms, us, ns
    );
}
