use std::fmt;

use colored::Colorize;

#[derive(Debug, PartialEq, Clone)]
pub struct Loc {
    pub file: String,
    pub line: usize,
    pub col: usize
}

pub struct Error {
    pub loc: Loc,
    pub message: String
}

pub struct Note {
    pub loc: Loc,
    pub message: String
}

pub fn printerr(message: String) {
    eprintln!("{} {}", "ERROR:".red().bold().underline(), message);
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", format!("{}", self.loc).red().underline(), "ERROR:".red().bold().underline(),  self.message)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", format!("{}", self.loc).underline(), "NOTE:".bold().underline(), self.message)
    }
}