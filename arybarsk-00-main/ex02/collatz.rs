fn collatz(start: u32)
{
	let mut n = start;
	std::println!("{}", n);
	while n > 1
	{
		if n % 2 == 0
			{n /= 2}
		else
			{n = 3 * n + 1}
		std::println!("{}", n);
		if n == 1
			{ break ; }
	}
}

/*fn main() {
	let start = 5u32;
	
	collatz(start);
}*/
