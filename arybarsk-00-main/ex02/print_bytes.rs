fn print_bytes(s: &str)
{
	for b in s.bytes()
	{
		std::println!("{:?}", b);
	}
}

/*fn main() {
	let s: &str = "Déjà Vu\n";
	
	print_bytes(s);
}*/
