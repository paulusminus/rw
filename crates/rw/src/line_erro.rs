use std::error::Error as StdErr;
use std::{
    fmt::{Debug, Display},
    io::Error as IOError,
};

pub struct LineError {
    lineno: Option<usize>,
    error: IOError,
}

impl Debug for LineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.lineno {
            Some(no) => write!(f, "Line {} {}", no, self.error),
            None => write!(f, "{}", self.error),
        }
    }
}

impl Display for LineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.lineno {
            Some(no) => write!(f, "Error in line {}: {}", no, self.error),
            None => write!(f, "Error: {}", self.error),
        }
    }
}

impl StdErr for LineError {
    fn source(&self) -> Option<&(dyn StdErr + 'static)> {
        self.error.source()
    }

    fn cause(&self) -> Option<&dyn StdErr> {
        self.source()
    }
}

pub fn error_line(lineno: Option<usize>) -> impl Fn(IOError) -> LineError {
    move |io_error| LineError {
        lineno,
        error: io_error,
    }
}
