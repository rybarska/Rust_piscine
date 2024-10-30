pub fn deduplicate(list: &mut Vec<i32>)
{
	for lelem in list.iter()
		{
			if 
		}
}

/* pub fn sort_boxes(boxes: &mut [[u32; 2]])
{
	let mut i = 0;

	while i < boxes.len()
	{
		for b in 0..boxes.len() - 1
		{
			if boxes[b][0] <= boxes[b + 1][0] && boxes[b][1] <= boxes[b + 1][1]
			{
				boxes.swap(b, b+1);
			}
		}
		i += 1;
	}
	for b in 0..boxes.len() - 1
		{ assert!(boxes[b][0] >= boxes[b + 1][0] && boxes[b][1] >= boxes[b + 1][1]) }
}
 */