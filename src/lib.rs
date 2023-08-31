use std::{io::Write, thread, time::Duration};

/// Wraps a writer and delays its output after each newline.
/// 
/// `DelayWriter<W>` can be used to display text with multiple lines with
/// a smooth animation. Waits for a `Duration` after each newline (`\n`)
/// in the input buffer.
/// 
/// # Examples
/// 
/// ```
/// use std::{time::Duration, io::Write};
/// use delay_writer::DelayWriter;
///
/// let stdout = std::io::stdout();
/// let mut writer = DelayWriter::new(stdout, Duration::from_millis(100));
///
/// let text = "Hello\nWorld!";
///
/// writer.write_all(text.as_bytes()).unwrap();
/// ```
pub struct DelayWriter<W: Write> {
    inner: W,
    delay: Duration,
}

impl<W: Write> DelayWriter<W> {
    /// Creates a new `DelayWriter<W>` with a specified delay.
    pub const fn new(writer: W, delay: Duration) -> Self {
        Self {
            inner: writer,
            delay,
        }
    }
}

impl<W: Write> Write for DelayWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for (index, line) in buf.split(|byte| *byte == b'\n').enumerate() {
            if index != 0 {
                self.inner.write_all(&[b'\n'])?;
            }

            self.inner.write_all(line)?;

            thread::sleep(self.delay);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let out = std::io::stdout();
        let writer = DelayWriter::new(out, Duration::from_millis(500));

        assert_eq!(Duration::from_millis(500), writer.delay);
    }

    #[test]
    fn write() {
        let mut buf = Vec::new();
        let mut writer = DelayWriter::new(&mut buf, Duration::from_millis(500));

        _ = writer.write(b"Hello\nWorld");

        assert_eq!("Hello\nWorld", String::from_utf8(buf).unwrap());
    }

    #[test]
    fn bytes_written() {
        let mut buf = Vec::new();
        let mut writer = DelayWriter::new(&mut buf, Duration::from_millis(500));

        let result = writer.write(b"Hello\nWorld");

        assert_eq!(11, result.unwrap());
    }
}
