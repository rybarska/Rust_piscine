use std::cell::Cell;

pub fn swap_u32<'a>(a: &'a Cell<u32>, b: &'a Cell<u32>)
{
    let temp_a: Cell<u32> = a.clone();
    let temp_b: Cell<u32> = b.clone();

    a.swap(b);

    assert_eq!(b, &temp_a);
    assert_eq!(a, &temp_b);
}

pub fn swap_string<'a>(a: &'a Cell<String>, b: &'a Cell<String>)
{
    a.swap(b);
}