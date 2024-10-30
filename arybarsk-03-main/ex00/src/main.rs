fn choose<T>(values: &[T]) -> &T
{
    let length = <[T]>::len(values) as i32;
    let index = ftkit::random_number(0..length) as usize;
    &values[index]
}

fn main() {
    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    println!("Chosen value: {}", choose(&values));
}

#[cfg(test)]
mod tests {
	use super::*;

#[test]
fn test_u32s() {
    let values2: &[u32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    println!("Chosen value: {}", choose(&values2));
}

#[test]
fn test_i8s() {
    let values2: &[i8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    println!("Chosen value: {}", choose(&values2));
}

#[test]
fn test_floats() {
    let values3: &[f32] = &[1.0, 2.0, 3.0, 4.0, 5.0];
    
    println!("Chosen value: {}", choose(&values3));
}
}