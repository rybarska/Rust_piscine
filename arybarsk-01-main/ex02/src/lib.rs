pub const fn color_name(color: &[u8; 3]) -> &'static str
{
	match (color[0], color[1], color[2])
	{
		(0, 0, 0) => "pure black",
		(255, 255, 255) => "pure white",
		(255, 0, 0) => "pure red",
		(0, 255, 0) => "pure green",
		(0, 0, 255) => "pure blue",
		(128, 128, 128) => "perfect grey",
		(0..=30, 0..=30, 0..=30) => "almost black",
		(129..=255, 0..=127, 0..=127) => "redish",
		(0..=127, 129..=255, 0..=127) => "greenish",
		(0..=127, 0..=127, 129..=255) => "blueish",
		_ => "unknown",
	}
}