//! Convenience functions on files.
use flate2::read::GzDecoder;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};

/// Iterator of lines direct from file path.
pub fn read_lines(path: &str) -> io::Result<Lines<BufReader<File>>> {
    Ok(BufReader::new(File::open(path)?).lines())
}

/// Iterator of lines, gunzipped, direct from file path.
pub fn read_gz_lines(path: &str) -> io::Result<Lines<BufReader<GzDecoder<File>>>> {
    Ok(BufReader::new(GzDecoder::new(File::open(path)?)).lines())
}
