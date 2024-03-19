use std::{
    error::Error,
    fmt::{Debug, Display},
    fs::{File, OpenOptions},
    io::{stdin, stdout, Error as IOError, Read, Result as IOResult, StdinLock, StdoutLock, Write},
    path::Path,
};

pub enum Reader<'a> {
    Stdin(StdinLock<'a>),
    File(File),
}

impl<'a> Default for Reader<'a> {
    fn default() -> Self {
        Reader::Stdin(stdin().lock())
    }
}

impl<'a> Reader<'a> {
    pub fn from_file<P: AsRef<Path>>(p: P) -> IOResult<Reader<'a>> {
        OpenOptions::new().read(true).open(p).map(Reader::File)
    }
}

impl<'a> Read for Reader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        match self {
            Self::File(file) => file.read(buf),
            Self::Stdin(stdin) => stdin.read(buf),
        }
    }
}

pub enum Writer<'a> {
    Stdout(StdoutLock<'a>),
    File(File),
}

impl<'a> Default for Writer<'a> {
    fn default() -> Self {
        Self::Stdout(stdout().lock())
    }
}

impl<'a> Writer<'a> {
    pub fn from_file<P: AsRef<Path>>(p: P) -> IOResult<Self> {
        OpenOptions::new().write(true).open(p).map(Self::File)
    }
}

impl<'a> Write for Writer<'a> {
    fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        match self {
            Self::File(file) => file.write(buf),
            Self::Stdout(stdout) => stdout.write(buf),
        }
    }

    fn flush(&mut self) -> IOResult<()> {
        match self {
            Self::File(file) => file.flush(),
            Self::Stdout(stdout) => stdout.flush(),
        }
    }
}

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

impl Error for LineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

pub fn error_line(lineno: Option<usize>) -> impl Fn(IOError) -> LineError {
    move |io_error| LineError {
        lineno,
        error: io_error,
    }
}
