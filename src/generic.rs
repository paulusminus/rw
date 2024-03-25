pub mod reader {
    use std::fs::{File, OpenOptions};
    use std::io::{stdin, Read, Stdin};
    use std::path::Path;

    pub struct Reader<R: Read = Stdin> {
        inner: R,
    }

    impl Reader {
        pub fn new() -> Self {
            Self { inner: stdin() }
        }

        pub fn try_from_file<P: AsRef<Path>>(path: P) -> Result<Reader<File>, std::io::Error> {
            OpenOptions::new()
                .read(true)
                .open(path)
                .map(|file| Reader { inner: file })
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
}

pub mod writer {
    use std::fs::{File, OpenOptions};
    use std::io::{stdout, Stdout, Write};
    use std::path::Path;

    pub struct Writer<W: Write = Stdout> {
        inner: W,
    }

    impl Writer {
        pub fn new() -> Self {
            Self { inner: stdout() }
        }

        pub fn try_to_file<P: AsRef<Path>>(path: P) -> Result<Writer<File>, std::io::Error> {
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(path)
                .map(|file| Writer { inner: file })
        }
    }

    impl Default for Writer {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Write for Writer {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.inner.write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.inner.flush()
        }
    }
}
