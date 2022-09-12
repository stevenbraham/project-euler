fn main() {
    let mut prime_numbers_found = 0;
    let mut number_to_check = 0;

    while prime_numbers_found < 10001 {
        number_to_check += 1;
        if is_prime_number(number_to_check) {
            prime_numbers_found += 1;
        }
    }

    println!("{}", number_to_check);
}

fn is_prime_number(number: u32) -> bool {
    if number > 1 {
        for i in 2..number {
            if number % i == 0 {
                return false;
            }
        }
        return true;
    }
    false
}

#[test]
fn test_is_prime_number() {
    assert!(!is_prime_number(1));
    assert!(is_prime_number(2));
    assert!(!is_prime_number(12));
    assert!(is_prime_number(13));
    assert!(!is_prime_number(28));
    assert!(is_prime_number(29));
}
