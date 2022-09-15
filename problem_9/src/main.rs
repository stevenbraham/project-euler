fn main() {
    for a in 1..1000 {
        for b in 1..1000 {
            for c in 1..1000 {
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
                    return;
                }
            }
        }
    }
}
