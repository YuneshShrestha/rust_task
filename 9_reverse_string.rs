// Reverse a string in Rust
use std::io::{self, Write};
fn reverse_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut j = len - 1;
    while i < j {
        let temp = chars[i];
        chars[i] = chars[j];
        chars[j] = temp;
        i += 1;
        j -= 1;
    }

    let reversed_string: String = chars.iter().collect();
    reversed_string    
}

fn main() {
    let mut input = String::new();
    print!("Enter a string: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let result = reverse_string(input);
    println!("Reversed string is {}", result);

}