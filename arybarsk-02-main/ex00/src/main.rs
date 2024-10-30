type Seconds = f32;
type Minutes = f32;

fn seconds_to_minutes(seconds: Seconds) -> Minutes {
    seconds / 60.0
}

fn main() {
    let s: Seconds = 120.0;
    let m: Minutes = seconds_to_minutes(s);

    println!("{s} seconds is {m} minutes");
}
#[cfg(test)]
mod tests {
	use super::*;

#[test]
fn test_cases() {
    let s1: Seconds = 20.0;
    let m1: Minutes = seconds_to_minutes(s1);

    println!("{s1} seconds is {m1} minutes");
}

}