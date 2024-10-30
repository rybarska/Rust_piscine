use std::fmt::Debug;

trait FortyTwo {
    fn forty_two() -> Self;
}

fn print_forty_two<T: Debug + FortyTwo>()
{
    let val = <T as FortyTwo>::forty_two();
    println!("{:?}", val);
}

impl FortyTwo for u32
{
    fn forty_two() -> u32
    {
        42 
    }
}

impl FortyTwo for String
{
    fn forty_two() -> String
    {
        42.to_string()
    }
}

fn main() {
    print_forty_two::<u32>();
    print_forty_two::<String>();
}