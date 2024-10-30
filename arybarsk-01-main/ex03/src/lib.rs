pub fn largest_group<'a>(haystack: &'a [u32], needle: & [u32]) -> &'a [u32]
{
	let mut ultimate_start = 0;
	let mut ultimate_stop = 0;
	let mut start = 0;
	let mut stop = 0;
	let mut max_len = 0;

	for i in 0..haystack.len()
	{
		let helem = haystack[i];
		if needle.contains(&helem)
			{ stop += 1; }
		else
		{
			if stop > start
			{
				let subslice: &[u32] = &haystack[start..stop];
				let mut valid = true;
				for nelem in needle
				{
					if !subslice.contains(nelem)
						{ valid = false; }
				}
				if subslice.len() > max_len && valid
				{
					max_len = subslice.len();
					ultimate_start = start;
					ultimate_stop = stop;
				}
			}
			start = stop + 1;
			stop = start;
		}
	}
 	if start < haystack.len()
	{
		let subslice: &[u32] = &haystack[start..];
		let mut valid = true;
		for nelem in needle
		{
			if !subslice.contains(nelem)
				{ valid = false; }
		}
		if subslice.len() > max_len && valid
		{
			ultimate_start = start;
			ultimate_stop = haystack.len();
		}
	}
	&haystack[ultimate_start..ultimate_stop]
}
