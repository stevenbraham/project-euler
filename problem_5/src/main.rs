use rayon::prelude::*;
use std::process;

fn main() {
    (20..2000000000).into_par_iter().for_each(|n| {
        if is_divisible_by_1_to_20(n) {
            println!("{}", n);
            process::exit(0x0100);
        }
    });
}

fn is_divisible_by_1_to_20(number: u64) -> bool {
    for n in 1..20 {
        if (number % n) != 0 {
            // number is not divisible by a number from 1 to 20
            return false;
        }
    }
    true
}
