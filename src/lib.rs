//! A bunch of personal utilities.
#![warn(missing_docs)]

#[cfg(feature = "sdl")]
#[macro_use]
extern crate rental;

pub mod fs;
pub mod log;
pub mod prelude;

#[cfg(feature = "chrono")]
mod timer;
#[cfg(feature = "chrono")]
pub use timer::Timer;

#[cfg(feature = "sdl")]
pub mod sdl;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
