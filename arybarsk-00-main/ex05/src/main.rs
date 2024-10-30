pub fn is_leap_year(year: u32) -> bool
{
	assert_ne!(year, 0, "Year 0 is not valid");
	(year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub fn num_days_in_month(year: u32, month: u32) -> u32
{
	assert_ne!(year, 0, "Year 0 is not valid");
	assert!(month >= 1, "Month is not valid");
	assert!(month <= 12, "Month is not valid");
	match (is_leap_year(year), month)
	{
		(true, 2) => 29,
		(false, 2) => 28,
		(_, 1 | 3 | 5 | 7 | 8 | 10 | 12) => 31,
		(_, 4 | 6 | 9 | 11) => 30,
		_ => 0,
	}
	
}

pub fn is_a_friday(thirteenth: u32) -> bool
{
	thirteenth % 7 == 5
}

pub fn format_month(month: u32) -> &'static str
{
	match month
	{
		1 => "January",
		2 => "February",
		3 => "March",
		4 => "April",
		5 => "May",
		6 => "June",
		7 => "July",
		8 => "August",
		9 => "September",
		10 => "October",
		11 => "November",
		12 => "December",
		_ => "Invalid month",
	}
}

fn main()
{
	let mut all_days = 0u32;
	
	for year in 1..=2024
	{
		for month in 1..=12
		{
			if year == 2024 && month == 10
				{ break ; }
			let thirteenth = all_days + 13;
			if is_a_friday(thirteenth)
				{ std::println!("Friday, {} 13, {}", format_month(month), year); }
			all_days += num_days_in_month(year, month);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_is_leap_year()
	{
		assert!(is_leap_year(1600), "1600 should be a leap year");
		assert!(!is_leap_year(1500), "1500 should be a common year");
		assert!(is_leap_year(2004), "2004 should be a leap year");
		assert!(!is_leap_year(2003), "2003 should be a common year");
	}
	
	#[test]
	fn test_num_days_in_month()
	{
		assert_eq!(num_days_in_month(1600, 2), 29, "February in 1600 should have 29 days");
		assert_eq!(num_days_in_month(1500, 2), 28, "February in 1500 should have 28 days");
		assert_eq!(num_days_in_month(1600, 1), 31, "January should have 31 days");
		assert_eq!(num_days_in_month(1500, 1), 31, "January should have 31 days");
		assert_eq!(num_days_in_month(1600, 3), 31, "March should have 31 days");
		assert_eq!(num_days_in_month(1500, 3), 31, "March should have 31 days");
		assert_eq!(num_days_in_month(1600, 5), 31, "May should have 31 days");
		assert_eq!(num_days_in_month(1500, 5), 31, "May should have 31 days");
		assert_eq!(num_days_in_month(1600, 7), 31, "July should have 31 days");
		assert_eq!(num_days_in_month(1500, 7), 31, "July should have 31 days");
		assert_eq!(num_days_in_month(1600, 8), 31, "August should have 31 days");
		assert_eq!(num_days_in_month(1500, 8), 31, "August should have 31 days");
		assert_eq!(num_days_in_month(1600, 10), 31, "October should have 31 days");
		assert_eq!(num_days_in_month(1500, 10), 31, "October should have 31 days");
		assert_eq!(num_days_in_month(1600, 12), 31, "December should have 31 days");
		assert_eq!(num_days_in_month(1500, 12), 31, "December should have 31 days");
		assert_eq!(num_days_in_month(1600, 4), 30, "April should have 30 days");
		assert_eq!(num_days_in_month(1500, 4), 30, "April should have 30 days");
		assert_eq!(num_days_in_month(1600, 6), 30, "June should have 30 days");
		assert_eq!(num_days_in_month(1500, 6), 30, "June should have 30 days");
		assert_eq!(num_days_in_month(1600, 9), 30, "September should have 30 days");
		assert_eq!(num_days_in_month(1500, 9), 30, "September should have 30 days");
		assert_eq!(num_days_in_month(1600, 11), 30, "November should have 30 days");
		assert_eq!(num_days_in_month(1500, 11), 30, "November should have 30 days");
	}
	
	#[test]
	#[should_panic(expected = "Year 0 is not valid")]
	fn invalid_is_leap_year_1()
	{
		is_leap_year(0);
	}
	
	#[test]
	#[should_panic(expected = "Year 0 is not valid")]
	fn invalid_num_days_in_month()
	{
		num_days_in_month(0, 6);
	}
	
	#[test]
	#[should_panic(expected = "Month is not valid")]
	fn invalid_num_days_in_month_2()
	{
		num_days_in_month(3, 0);
	}
	
	#[test]
	#[should_panic(expected = "Month is not valid")]
	fn invalid_num_days_in_month_3()
	{
		num_days_in_month(3, 15);
	}
}
