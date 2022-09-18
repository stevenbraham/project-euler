use rayon::prelude::*;

fn main() {
    let mut number = 1;

    loop {
        let traingle_number = get_traingle_number(number);
        let divisors = get_divisors(traingle_number);
        if divisors.len() > 500 {
            println!("{}", traingle_number);
            break;
        }

        number += 1;
    }
}

/// Calculates the nth triangle number
fn get_traingle_number(number: u32) -> u32 {
    (1..=number).into_par_iter().sum()
}

struct DivisionResult {
    number: u32,
    divisble: bool,
}

fn get_divisors(number: u32) -> Vec<u32> {
    (1..=number)
        .into_par_iter()
        .map(|i| {
            
            DivisionResult {
                number: i,
                divisble: number % i == 0,
            }
        })
        .filter(|result| result.divisble)
        .map(|result| result.number)
        .collect()
}

#[test]
fn test_get_traingle_number() {
    assert_eq!(get_traingle_number(1), 1);
    assert_eq!(get_traingle_number(2), 3);
    assert_eq!(get_traingle_number(3), 6);
    assert_eq!(get_traingle_number(4), 10);
    assert_eq!(get_traingle_number(5), 15);
    assert_eq!(get_traingle_number(6), 21);
    assert_eq!(get_traingle_number(7), 28);
    assert_eq!(get_traingle_number(8), 36);
    assert_eq!(get_traingle_number(9), 45);
    assert_eq!(get_traingle_number(10), 55);
}
