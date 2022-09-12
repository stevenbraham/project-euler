fn main() {
    // Sum of the squares of the natural numbers
    let mut totalSumSquared = 0;

    // Sum of the natural numbers
    let mut totalSum = 0;

    for i in 1..101 {
        totalSum += i;
        totalSumSquared += i * i;
    }

    println!("{}", totalSum * totalSum - totalSumSquared);
}
