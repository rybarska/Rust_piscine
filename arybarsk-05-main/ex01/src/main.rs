//use std::time::Duration;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::io;
use std::io::stdout;

struct Logger<W> {
    buffer: Box<[u8]>,
    writer: W,
    position: usize,
}

impl<W> Logger<W> {
    pub fn new(threshold: usize, writer: W) -> Self
    {
        Self
        {
            buffer: vec![0; threshold].into_boxed_slice(),
            writer,
            position:0,
        }
    }

}

impl<W: io::Write> Logger<W> {
    pub fn log(&mut self, message: &str) -> io::Result<()>
    {
        if self.buffer.len() != 0 && message.is_empty()
        {
            self.buffer[self.position] = b'\n';
            self.position += 1;
        }
        let message = format!("{}\n", message);
        let bytes = message.as_bytes();

        if self.buffer.len() == 0
        {
            self.writer.write_all(bytes)?;
            self.writer.flush()?;
            return Ok(());
        }
        if bytes.len() > self.buffer.len()
        {
            self.flush();
            self.writer.write_all(bytes)?;
            self.writer.flush()?;
            return Ok(());
        }
        if self.position + bytes.len() > self.buffer.len()
            { self.flush() }
        self.buffer[self.position..self.position + bytes.len()].copy_from_slice(bytes);
        self.position += bytes.len();
        if self.position >= self.buffer.len()
            { self.flush() }
        Ok(())
    }

    pub fn flush(&mut self)
    {
        if self.position > 0
        {
            if let Err(e) = self.writer.write_all(&self.buffer[..self.position])
                { eprintln!("Could not write to writer, {}", e) }
            if let Err(e) = self.writer.flush()
                { eprintln!("Could not flush writer, {}", e) }
            self.position = 0;
        }
    }
}

fn main() {
    let logger = Arc::new(Mutex::new(Logger::new( 64, stdout())));

    let mut manager = vec![];

    for i in 0..10
    {
        let logger = logger.clone();
        let t = thread::spawn(move ||
        {
            let mut locked_logger = logger.lock().unwrap();
            for j in 0..10
            {
                let message = format!("hello {} from thread {}", j, i);
                locked_logger.log(&message).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });
        manager.push(t);
    }

    for t in manager
    {
        t.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    // Test 1: No buffer (zero size buffer)
    #[test]
    fn no_buffer_flushes_immediately() {
        let mut buffer = Vec::new();
        let mut logger = Logger::new(0, &mut buffer);  // No buffer, flush immediately

        logger.log("hello").expect("failed");
        logger.log("world").expect("failed");

        let output = String::from_utf8(buffer).expect("Invalid UTF-8");
        let actual_output = "hello\nworld\n";
        assert_eq!(output, actual_output);
    }

    // Test 2: Small buffer, multiple logs fit (buffering works)
    #[test]
    fn small_buffer_multiple_logs() {
        let mut buffer = Vec::new();
        let mut logger = Logger::new(15, &mut buffer); // Buffer can hold 15 bytes max

        logger.log("first").expect("failed");  // 6 bytes (including newline)
        logger.log("second").expect("failed"); // 7 bytes (including newline)
        // Total 13 bytes, fits in buffer, no flush yet

        logger.flush();  // Now we flush manually

        let output = String::from_utf8(buffer).expect("Invalid UTF-8");
        let actual_output = "first\nsecond\n";
        assert_eq!(output, actual_output);
    }

    // Test 3: Auto flush when buffer is full
    #[test]
    fn auto_flush_when_buffer_full() {
        let mut buffer = Vec::new();
        let mut logger = Logger::new(10, &mut buffer); // Buffer can hold 10 bytes max

        logger.log("12345").expect("failed"); // 6 bytes (5 + newline)
        logger.log("abcde").expect("failed"); // 6 bytes (5 + newline)

        // Should have flushed after the first message because buffer is full
        logger.flush(); // Ensure buffer is flushed after logging

        let output = String::from_utf8(buffer).expect("Invalid UTF-8");
        let actual_output = "12345\nabcde\n"; // Full buffer should have been flushed
        assert_eq!(output, actual_output);
    }

    // Test 4: Large message exceeding buffer triggers immediate flush
    #[test]
    fn large_message_triggers_immediate_flush() {
        let mut buffer = Vec::new();
        let mut logger = Logger::new(8, &mut buffer); // Buffer can hold 8 bytes

        logger.log("this is a long message").expect("failed"); // Message is larger than buffer
        
        // Should auto-flush immediately since the message exceeds buffer size
        let output = String::from_utf8(buffer).expect("Invalid UTF-8");
        let actual_output = "this is a long message\n";
        assert_eq!(output, actual_output);
    }

    // Test 5: Manual flush ensures remaining content is written
    #[test]
    fn manual_flush_writes_remaining_content() {
        let mut buffer = Vec::new();
        let mut logger = Logger::new(20, &mut buffer); // Buffer can hold 20 bytes

        logger.log("short message").expect("failed"); // Fits in buffer, no flush yet
        logger.flush();  // Now force flush

        let output = String::from_utf8(buffer).expect("Invalid UTF-8");
        let actual_output = "short message\n"; // Should flush the buffered content
        assert_eq!(output, actual_output);
    }

    // Test 6: Concurrent logging with multiple threads
    #[test]
    fn concurrent_logging_is_thread_safe() {
        let logger = Arc::new(Mutex::new(Logger::new(64, Vec::new())));
        let mut threads = vec![];

        for i in 0..5 {
            let logger = Arc::clone(&logger);
            let t = thread::spawn(move || {
                let mut locked_logger = logger.lock().unwrap();
                for j in 0..10 {
                    let message = format!("thread {} message {}", i, j);
                    locked_logger.log(&message).expect("logging failed");
                    thread::sleep(Duration::from_millis(1));
                }
            });
            threads.push(t);
        }

        for t in threads {
            t.join().unwrap();
        }

        let logger = logger.lock().unwrap();
        let output = String::from_utf8(logger.writer.to_vec()).expect("Invalid UTF-8");

        // Check that the output contains the expected logs from all threads
        for i in 0..5 {
            for j in 0..10 {
                let message = format!("thread {} message {}\n", i, j);
                assert!(output.contains(&message));
            }
        }
    }
}
