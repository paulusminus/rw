use std::{
    fs::{File, OpenOptions},
    io::{stdin, StdinLock},
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
    pub fn from_file<P: AsRef<Path>>(p: P) -> Result<Reader<'a>, std::io::Error> {
        OpenOptions::new().read(true).open(p).map(Reader::File)
    }
}

impl<'a> std::io::Read for Reader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Reader::File(file) => file.read(buf),
            Reader::Stdin(stdin) => stdin.read(buf),
        }
    }
}
