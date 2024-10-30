fn main() {
	for x in 1..=100
	{
		match (x % 3, x % 5, x % 11)
		{
			(0, 0, _) => std::println!("fizzbuzz"),
			(0, _, _) => std::println!("fizz"),
			(_, 0, _) => std::println!("buzz"),
			(_, _, 3) => std::println!("FIZZ"),
			(_, _, 5) => std::println!("BUZZ"),
			_ => std::println!("{}", x),
		}
	}
}
