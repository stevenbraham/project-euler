fn main() {
    let mut largest_number_found: u32 = 0;

    for n1 in 100..1000 {
        for n2 in 100..1000 {
            let number = n1 * n2;
            if is_palindrome_number(number) && number > largest_number_found {
                largest_number_found = number;
            }
        }
    }

    println!("{}", largest_number_found);
}

fn is_palindrome_number(number: u32) -> bool {
    number.to_string() == number.to_string().chars().rev().collect::<String>()
}

#[test]
fn test_is_prime_number() {
    assert!(is_palindrome_number(1));
    assert!(is_palindrome_number(101));
    assert!(!is_palindrome_number(9001));
    assert!(is_palindrome_number(9009));
}
