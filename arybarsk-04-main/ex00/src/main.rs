use std::io::{self, Write};
use std::writeln;

fn main() {
    let mut stdout = io::stdout();

    for i in 0..11 {
        if writeln!(stdout, "{}", i).is_err()
            { break ; }
    }
}

/* fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for i in 0..11 {
        if writeln!(handle, "{}", i).is_err()
            { break ; }
    }

    Ok(())
} */