use std::io::{self, Write};
fn check_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut j = len - 1;
    while i < j {
        if chars[i] != chars[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

fn main() {
    let mut input = String::new();
    print!("Enter a string: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let result = check_palindrome(input);
    if result {
        println!("{} is a palindrome", input);
    } else {
        println!("{} is not a palindrome", input);
    }

}