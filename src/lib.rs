//! A bunch of personal utilities.

#![warn(missing_docs)]

pub mod log;
mod timer;

pub use timer::Timer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
