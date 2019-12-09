```rust
#[cfg(test)]
pub mod mock {
    use std::io::{BufRead, BufReader, Read, Write};

    pub struct MockPort {
        count: usize,
        location: usize,
        read_buf: String,
        write_buf: Vec<u8>,
    }

    impl MockPort {
        pub fn new(buf: &str) -> Self {
            Self {
                count: 0,
                location: 0,
                read_buf: buf.to_string(),
                write_buf: vec![],
            }
        }
    }

    impl Write for MockPort {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let mut count: usize = 0;
            for &i in buf {
                self.write_buf.push(i);
                count += 1;
            }
            Ok(count)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl Read for MockPort {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.count = 1;
            let src = self.read_buf.as_bytes();
            for i in 0..buf.len() {
                self.count += 1;
                let b = src[self.location];
                buf[i] = b;
                if i == src.len() - 1 {
                    return Ok(self.count);
                }
                if self.location == src.len() - 1 {
                    return Ok(self.count);
                }
                self.location += 1;
            }
            Ok(self.count)
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn read_test() {
            let expect = "ABC\r";
            let mut buf: Vec<u8> = vec![0, 0, 0, 0];

            let mut port = MockPort::new(expect);
            port.read(&mut buf).unwrap();
            for (i, &v) in expect.as_bytes().iter().enumerate() {
                assert_eq!(buf[i], v);
            }
        }

        #[test]
        fn read_until_test() {
            let expect = "ABC\r";
            let mut buf: Vec<u8> = vec![];

            let mut port = BufReader::new(MockPort::new(expect));
            port.read_until(b'\r', &mut buf).unwrap();
            for (i, &v) in expect.as_bytes().iter().enumerate() {
                assert_eq!(buf[i], v);
            }
        }
    }
}
```
