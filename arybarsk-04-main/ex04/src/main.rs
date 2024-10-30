use std::env;
use std::iter::*;
use std::process::{Command, Stdio};
use std::vec::Vec;
use std::io;
use std::io::{stdout, Write, Read};

fn main() -> io::Result<()> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    
    if arguments.is_empty()
    {
        eprintln!("No commands were passed");
        return Ok(());
    }

    let joined = arguments.join(" ");
    let cmd_vec: Vec<&str> = joined.split(',').map(|s| s.trim()).collect();

    for cmd in cmd_vec
    {
        let mut cmd_splits: Vec<&str> = cmd.split_whitespace().collect();
        let cmd_zero = cmd_splits.remove(0);
        
        let mut child_cmd = Command::new(cmd_zero);
        child_cmd.args(&cmd_splits[0..]);
        child_cmd.stdout(Stdio::piped());
        child_cmd.stderr(Stdio::null());

        if let Ok(mut child) = child_cmd.spawn()
        {
            let mut output = String::new();

            let mut _stdout = stdout().lock();
            if let Some(mut stdout) = child.stdout.take()
            {
                let _ = stdout.read_to_string(&mut output);
            }

            let _result = child.wait();

            let _ = writeln!(io::stdout(), "==========");
            _ = writeln!(io::stdout(), "{}", output);
            _ = writeln!(io::stdout(),);
        }
    }   

    Ok(())
}
