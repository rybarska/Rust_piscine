fn min(a: i32, b: i32) -> i32
{
	if a > b
		{b}
	else
		{a}
}

fn main() {
	let a = 5i32;
	let b = 7i32;

	std::println!("{}", min(a, b));
}
