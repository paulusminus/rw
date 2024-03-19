use std::{
    error::Error,
    fmt::{Debug, Display},
    fs::{File, OpenOptions},
    io::{read_to_string, stdin, stdout, Cursor, Error as IOError, Read, Result as IOResult, StdinLock, StdoutLock, Write},
    path::Path,
};

pub enum Reader<'a> {
    Stdin(StdinLock<'a>),
    File(File),
    Memory(Cursor<Vec<u8>>),
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

    pub fn memory(init: Option<String>) -> IOResult<Reader<'a>> {
        Ok(Reader::Memory(Cursor::new(init.map(|s| s.into_bytes()).unwrap_or_default())))
    }
}

impl<'a> Read for Reader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        match self {
            Self::File(file) => file.read(buf),
            Self::Stdin(stdin) => stdin.read(buf),
            Self::Memory(cursor) => cursor.read(buf),
        }
    }
}

pub enum Writer<'a> {
    Stdout(StdoutLock<'a>),
    File(File),
    Memory(Cursor<Vec<u8>>),
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

    pub fn memory() -> IOResult<Self> {
        Ok(Self::Memory(Cursor::new(vec![])))
    }

    pub fn show_buffer(self) -> Result<String, IOError> {
        match self {
            Writer::Memory(mut cursor) => {
                cursor.set_position(0);
                read_to_string(cursor)
            },
            _ => Ok(String::new())
        }
    } 
}

impl<'a> Write for Writer<'a> {
    fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        match self {
            Self::File(file) => file.write(buf),
            Self::Stdout(stdout) => stdout.write(buf),
            Self::Memory(cursor) => cursor.write(buf),
        }
    }

    fn flush(&mut self) -> IOResult<()> {
        match self {
            Self::File(file) => file.flush(),
            Self::Stdout(stdout) => stdout.flush(),
            Self::Memory(cursor) => cursor.flush(),
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

#[cfg(test)]
mod test {
    use std::io::{BufRead, BufReader, Write};

    use crate::{Reader, Writer};

    const TEST_STRING: &str = "Hallo allemaal\nWat fijn dat u er bent\nHallo Paul\nWat fijn dat jij er bent";

    #[test]
    fn memory_reader() {
        let r = Reader::memory(Some(TEST_STRING.to_owned())).unwrap();
        let mut lines = BufReader::new(r).lines();
        assert_eq!(lines.next().unwrap().unwrap(), "Hallo allemaal".to_owned());
        assert_eq!(lines.next().unwrap().unwrap(), "Wat fijn dat u er bent".to_owned());
        assert_eq!(lines.next().unwrap().unwrap(), "Hallo Paul".to_owned());
        assert_eq!(lines.next().unwrap().unwrap(), "Wat fijn dat jij er bent".to_owned());
    }

    #[test]
    fn write_to_buffer() {
        let mut w = Writer::memory().unwrap();
        w.write("Hallo allemaal\n".as_bytes()).unwrap();
        w.write("Wat fijn dat u er bent\n".as_bytes()).unwrap();
        w.write("Hallo Paul\n".as_bytes()).unwrap();
        w.write("Wat fijn dat jij er bent".as_bytes()).unwrap();
        w.flush().unwrap();

        let s = w.show_buffer().unwrap();
        assert_eq!(s, TEST_STRING.to_owned());
    }
}