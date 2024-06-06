fn find_longest_common_prefix<'a>(arr: &'a [&'a str]) -> &'a str {
    if arr.is_empty() {
        return "";
    }
    let mut prefix = arr[0];
    for word in &arr[1..] {
        while !word.starts_with(prefix) {
            prefix = &prefix[0..prefix.len() - 1];
            if prefix.is_empty() {
                return "";
            }
        }
    }
    prefix
}

fn main() {
    let arr = ["hello", "hell", "he"];
    println!("{}", find_longest_common_prefix(&arr));
}