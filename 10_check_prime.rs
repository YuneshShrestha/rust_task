// Check if a number is prime in Rust

use std::io::{self, Write};
fn check_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap();
    let result = check_prime(input);
    if result {
        println!("Prime");
    } else {
        println!("Not Prime");
    }
}