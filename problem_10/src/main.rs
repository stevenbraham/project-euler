use indicatif::ProgressBar;
use rayon::prelude::*;

struct PrimeCheckResult {
    number: u64,
    is_prime: bool,
}

fn main() {
    let limit = 2000000;

    let pb = ProgressBar::new(limit);

    let result: u64 = (1..limit)
        .into_par_iter() // parallelize the loop
        .map(|i| {
            pb.inc(1);
            PrimeCheckResult {
                number: i,
                is_prime: is_prime_number(i),
            }
        })
        .filter(|result| result.is_prime) // remove all non prime numbers
        .map(|result| result.number) // extract the number
        .sum();

    pb.finish_with_message("");

    println!("{}", result);
}

fn is_prime_number(number: u64) -> bool {
    if number > 1 {
        let divisionResults: Vec<bool> = (2..number)
            .into_par_iter()
            .map(|n| number % n == 0)
            .collect();

        return !divisionResults.contains(&true);
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
