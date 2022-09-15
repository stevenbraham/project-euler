use rayon::prelude::*;
use std::process;

fn main() {
    (1..1000).into_par_iter().for_each(|a| {
        (1..1000).into_par_iter().for_each(|b| {
            (1..1000).into_par_iter().for_each(|c| {
                if
                // requirement 1: a + b + c, should equal 1000
                a + b + c == 1000 && 
                // requirement 2: a^2 + b^2 = c^2
                (a * a) + (b * b) == (c * c) &&
                // requirement 3: a < b < c
                a < b && b < c 
                {
                    println!("A: {} B: {} C: {}", a ,b,c);
                    println!("Product: {}", a * b * c);
                    process::exit(0);
                }
            });
        });
    })  ;
}