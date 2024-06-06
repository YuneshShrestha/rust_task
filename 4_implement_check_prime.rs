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
    let result = check_prime(11);
    if result {
        println!("Prime");
    } else {
        println!("Not Prime");
    }
}