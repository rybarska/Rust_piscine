#![forbid(unsafe_op_in_unsafe_fn)]

pub fn ft_swap<T>(a: &mut T, b: &mut T)
{
    unsafe
    {
        let temp_a = std::ptr::read(a);
        let temp_b = std::ptr::read(b);

        std::ptr::write(a, temp_b);
        std::ptr::write(b, temp_a);
    }
}

/// Calculates the length of the string pointed to by s,
/// excluding the terminating null byte ('\0')
///
/// # Safety
///
/// Ensure the pointer is:
/// -valid for reading,
/// -properly aligned,
/// -points to a C-style null-terminated string.
/// 
/// If the pointer is null or the string is not null-terminated,
/// the behaviour is undefined.
pub unsafe fn ft_strlen(s: *const u8) -> usize
{
    // SAFETY:
    // Ensure the pointer is not null.
    assert!(!s.is_null());

    let	mut count: usize = 0;

	// SAFETY:
    // Assume s is a valid pointer to a null terminated string.
    // The length of the string can be counted by advancing the pointer
    // until the null value.
    unsafe
    {
        while *s.add(count) != 0
            { count += 1; }
    }

	count
}

/// Copies the string pointed to by src into the memory pointed to by dst,
/// including the terminating null byte ('\0')
///
/// # Safety
///
/// Ensure the src pointer is:
/// -valid for reading,
/// -properly aligned,
/// -points to a C-style null-terminated string.
/// 
/// Ensure the memory pointed to by dst pointer is large enough
/// to hold the string pointed to by src.
/// 
/// If those conditions are not met, the behaviour is undefined.
pub unsafe fn ft_strcpy(dst: *mut u8, src: *const u8)
{
    // SAFETY:
    // Ensure both pointers are not null.
    assert!(!src.is_null());
    assert!(!dst.is_null());

    let	mut count: usize = 0;

    // SAFETY:
    // Assume both pointers are valid.
    // The string can be copied character by character,
    // by advancing both pointers until src reaches the null value.
    unsafe
    {
        while *src.add(count) != 0
        {
            *dst.add(count) = std::ptr::read(src.add(count));
            count += 1;
        }
        *dst.add(count) = 0;
    }
}