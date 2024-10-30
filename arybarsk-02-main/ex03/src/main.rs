#[derive(Default, Debug, Clone, PartialOrd, PartialEq)]

struct MyType {
    pub active: bool,
    pub passive: bool,
}

fn main() {
    let instance = MyType::default();

    let other_instance = instance.clone();

    println!("the default value of MyType is {instance:?}");
    println!("the clone of `instance` is {other_instance:#?}");
    assert_eq!(
        instance,
        other_instance,
        "the clone isn't the same :/"
    );
    assert!(
        instance == other_instance,
        "why would the clone be less or greater than the original?",
    );
}

#[cfg(test)]
mod tests {
	use super::*;

#[test]
fn test_cases() {
    let mut instance2 = MyType::default();
    instance2.active = true;

    let other_instance2 = instance2.clone();

    println!("the default value of MyType is {instance2:?}");
    println!("the clone of `instance` is {other_instance2:#?}");
    assert_eq!(
        instance2,
        other_instance2,
        "the clone isn't the same :/"
    );
    assert!(
        instance2 == other_instance2,
        "why would the clone be less or greater than the original?",
    );
}
}