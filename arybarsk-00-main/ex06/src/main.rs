fn main()
{
	let num_to_guess = ftkit::random_number(1..100);
	
	std::println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.");
	
	loop
	{
		let guess = ftkit::read_number();
		match num_to_guess.cmp(&guess)
		{
			std::cmp::Ordering::Equal => {std::println!("That is right! The secret was indeed the number {guess}, which you have brilliantly discovered!");
			break ;
			}
			std::cmp::Ordering::Less => {std::println!("Sometimes I wonder whether I should retire. I would have guessed higher.");}
			std::cmp::Ordering::Greater => {std::println!("This student might not be as smart as I was told. This answer is obviously too weak.");}
		}
	}
}
