fn main() {
    let mut sum = 0;
    for i in 1..1000 {
        // check if the number is divisible by 3 or 5, if this the case add it to the sum
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("{}", sum);
}
