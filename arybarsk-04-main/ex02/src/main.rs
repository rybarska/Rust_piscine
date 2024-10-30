use std::fs;
use std::path::Path;
use std::io;
use std::env;

fn match_size(size: u64) ->String
{
    match size
    {
        (0..=999) => {format!("{} bytes", size)},
        (1000..=999999) => {format!("{} kilobytes", size)},
        (1000000..=999999999) => {format!("{} megabytes", size)},
        (1000000000..) => {format!("{} gigabytes", size)},
    }
} 

fn get_size(path: &Path) -> io::Result<u64>
{
    let mut all_size: u64 = 0;
    
    if path.is_dir()
    {
        let entries = fs::read_dir(path)?;
        for entry in entries {
            let dir = entry?;
            let metadata = dir.metadata()?;
            if metadata.is_dir()
                { all_size += get_size(&dir.path())?;}
            else
                { all_size += metadata.len();}
        }
    }
    else if path.is_file()
    {
        all_size = fs::metadata(path)?.len();
    }
    Ok(all_size)
}

fn main() -> io::Result<()> {
    for argument in env::args().skip(1)
    {
        let path = Path::new(&argument);
        match get_size(path)
        {
            Ok(size) =>
            {
                println!("\nSize: {}", match_size(size));
            },
            Err(e) => eprintln!("Could not get size '{}': {}", argument, e),
        }
    }

    Ok(())
}