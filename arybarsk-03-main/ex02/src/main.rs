use std::fmt;

const FRIENDLY_OUTPUT: &str = "Hey! I'm John.";
const ANGRY_OUTPUT: &str = "Don't try to silence me!";
const INFO_OUTPUT: &str = "John, the man himself.";
const BRAGGING_OUTPUT: &str = "John, the man himself. He's handsome AND formidable.";
const DEFAULT_OUTPUT: &str = "Bip Boop?";

//#[derive(Debug)]
struct John;

impl fmt::Binary for John
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(formatter, "{}", DEFAULT_OUTPUT)
    }
}

impl fmt::Display for John
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        if let Some(precision) = formatter.precision()
        {
            if precision == 0
                { return write!(formatter, "{}", ANGRY_OUTPUT); }
        }
        formatter.pad(FRIENDLY_OUTPUT)
    }
}

impl fmt::Debug for John
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        if formatter.alternate()
        {
            write!(formatter, "{}", BRAGGING_OUTPUT)
        }
        else
        {
            write!(formatter, "{}", INFO_OUTPUT)
        }
    }
}

fn main() {
    let john = John;

    println!("1. {john}");
    println!("2. |{john:<30}|");
    println!("3. |{john:>30}|");
    println!("4. {john:.6}");
    println!("5. {john:.0}");
    println!("6. {john:?}");
    println!("7. {john:#?}");
    println!("8. {john:b}");
}