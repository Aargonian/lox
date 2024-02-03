#![allow(dead_code)]
pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn report(line: usize, were: &str, message: &str) {
    eprintln!("[Line {line}] Error {were}: {message}");
}
