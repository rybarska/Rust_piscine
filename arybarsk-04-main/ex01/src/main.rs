use std::io;
use std::io::Write;
use std::fs::File;
use std::env;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|e|
        {
            eprintln!("Could not read input: '{}'", e);
            e
        })?;
    //let trimmed_input = input.trim();
    print!("{}", input);
    for argument in env::args().skip(1)
    {
        match File::create(&argument)
        {
            Ok(mut file) =>
            {
                if let Err(e) = file.write_all(input.as_bytes())
                    { eprintln!("Could not write to file '{}': {}", argument, e)}
            },
            Err(e) => eprintln!("Could not create file '{}': {}", argument, e),
        }
    }
    Ok(())
}