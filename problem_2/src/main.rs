fn main() {
    let mut sum = 0;

    let mut number = 1;

    loop {
        let fibonach = get_fibonacci(number);

        if fibonach % 2 == 0 {
            sum += fibonach;
        }

        if fibonach > 4000000 {
            break;
        }

        number += 1;
    }

    println!("{}", sum);
}

fn get_fibonacci(n: u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    }
    get_fibonacci(n - 1) + get_fibonacci(n - 2)
}

#[test]
fn test_get_fibonacci() {
    assert_eq!(get_fibonacci(1), 1);
    assert_eq!(get_fibonacci(2), 1);
    assert_eq!(get_fibonacci(9), 34);
    assert_eq!(get_fibonacci(40), 102334155);
}
