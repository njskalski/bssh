#[cfg(test)]
mod tests {

    use std::io::{Error, ErrorKind, Read, Result, Write};

    pub struct MockReadStream {
        pub input: Vec<u8>,
        pub pos: usize,
    }

    impl MockReadStream {
        pub fn new(input: Vec<u8>) -> MockReadStream {
            MockReadStream {
                input: input,
                pos: 0,
            }
        }
    }

    impl Read for MockReadStream {
        fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
            if buf.len() > (self.input.len() - self.pos) {
                Err(Error::new(ErrorKind::BrokenPipe, "")) //TODO ok errorkind?
            } else {
                for i in 0..buf.len() {
                    buf[i] = self.input[self.pos + i];
                }
                self.pos += buf.len();
                Ok(())
            }
        }

        fn read(&mut self, _: &mut [u8]) -> Result<usize> {
            panic!();
        }
    }

    pub struct MockWriteStream {
        pub output: Vec<u8>,
    }

    impl MockWriteStream {
        pub fn new() -> MockWriteStream {
            MockWriteStream { output: Vec::new() }
        }
    }

    impl Write for MockWriteStream {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.output.extend_from_slice(buf);
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    pub struct MockReadStreamInfitnite {}

    impl Read for MockReadStreamInfitnite {
        fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
            for i in 0..buf.len() {
                buf[i] = b'.';
            }
            Ok(())
        }

        fn read(&mut self, _: &mut [u8]) -> Result<usize> {
            panic!();
        }
    }
}
