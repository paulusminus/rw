pub use {reader::Reader, writer::Writer};

mod reader {
    use std::fs::{File, OpenOptions};
    use std::io::{stdin, Cursor, Read, Stdin};
    use std::path::Path;

    pub struct Reader<R: Read = Stdin> {
        inner: R,
    }

    impl Reader {
        fn new() -> Self {
            Self { inner: stdin() }
        }

        pub fn try_from_file<P: AsRef<Path>>(path: P) -> Result<Reader<File>, std::io::Error> {
            OpenOptions::new()
                .read(true)
                .open(path)
                .map(|file| Reader { inner: file })
        }

        pub fn from_string<S: AsRef<str>>(s: S) -> Reader<Cursor<Vec<u8>>> {
            Reader {
                inner: Cursor::new(s.as_ref().as_bytes().to_vec()),
            }
        }
    }

    impl Default for Reader {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<R: Read> Read for Reader<R> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.inner.read(buf)
        }
    }

    #[cfg(test)]
    mod test {
        use super::Reader;
        use std::io::read_to_string;

        #[test]
        fn from_string() {
            const INPUT: &str = "Hallo allemaal";
            let reader = Reader::from_string(INPUT);
            let s = read_to_string(reader).unwrap();
            assert_eq!(s, *INPUT);
        }

        #[test]
        fn try_from_file() {
            const FILENAME: &str = "Cargo.toml";
            let reader = Reader::try_from_file(FILENAME).unwrap();
            let s = read_to_string(reader).unwrap();
            assert!(s.starts_with("[package]\n"));
        }
    }
}

mod writer {
    use std::fs::{File, OpenOptions};
    use std::io::{stdout, Cursor, Stdout, Write};
    use std::path::Path;

    pub struct Writer<W: Write = Stdout> {
        inner: W,
    }

    impl Writer {
        fn new() -> Self {
            Self { inner: stdout() }
        }

        pub fn try_to_file<P: AsRef<Path>>(path: P) -> Result<Writer<File>, std::io::Error> {
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(path)
                .map(|file| Writer { inner: file })
        }

        pub fn to_string() -> Writer<Cursor<Vec<u8>>> {
            Writer {
                inner: Cursor::new(Vec::<u8>::new()),
            }
        }
    }

    impl Default for Writer {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<W: Write> Write for Writer<W> {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.inner.write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.inner.flush()
        }
    }

    #[cfg(test)]
    mod test {
        use crate::generic::{reader::Reader, writer::Writer};
        use std::io::{read_to_string, Write};

        #[test]
        fn to_string() {
            let mut writer = Writer::to_string();
            writer.write("Hallo allemaal\n".as_bytes()).unwrap();
            writer.flush().unwrap();
        }

        #[test]
        fn to_file() {
            const TEST_STRING: &str = "Hallo allemaal";

            let file = tempfile::NamedTempFile::new().unwrap();
            let filename = file.into_temp_path().canonicalize().unwrap();

            {
                let mut writer = Writer::try_to_file(filename.as_path()).unwrap();
                writer.write(TEST_STRING.as_bytes()).unwrap();
                writer.flush().unwrap();
            }

            {
                let reader = Reader::try_from_file(filename.as_path()).unwrap();
                let s = read_to_string(reader).unwrap();
                assert_eq!(s, *TEST_STRING);
            }
        }
    }
}
