fn next_number(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn main() {
    let mut n = 5;
    print!("{} ", n);

    // Some starter code to test
    while n != 1 {
        n = next_number(n);
        print!("{} ", n);
    }
    println!();
}
