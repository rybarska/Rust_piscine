use std::env;
use std::process::Command;
//use std::os::unix::process::CommandExt;
use std::io;
use std::io::stdin;
use std::vec::Vec;
use std::iter::*;

fn main() -> io::Result<()> {
    let mut arguments = env::args().skip(1);
    let cmd = match arguments.next()
    {
        Some(c) => c,
        None =>
        {
            println!("No valid args");
            return Ok(());
        },
    };

    let cmd_args: Vec<String> = arguments.collect();

    let mut input = String::new();

    let mut lines: Vec<String> = Vec::new();

    while stdin().read_line(&mut input)? > 0
    {
        lines.push(input.trim().to_string());
        input.clear();
    }
    
    let mut child = Command::new(cmd)
                    .args(&cmd_args)
                    .args(&lines)
                    .spawn()
                    .expect("Command failed to start");

    let _result = child.wait().unwrap();

    Ok(())
}
