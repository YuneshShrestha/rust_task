fn returns_shortest_string(text: &str) -> &str {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut shortest_word = words[0];
    for &word in &words {
        if word.len() < shortest_word.len() {
            shortest_word = word;
        }
    }
    shortest_word
}

fn main() {
    println!("{}", returns_shortest_string("This is string."));
}