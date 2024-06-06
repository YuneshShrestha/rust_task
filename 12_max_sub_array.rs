fn max_sub_array_sum(a: &[i32]) -> i32 {
    let mut curr_sum = a[0];
    let mut max_sum = a[0];
    for &num in &a[1..] {
        curr_sum = curr_sum + num;
        if num > curr_sum {
            curr_sum = num;
        }
        if curr_sum > max_sum {
            max_sum = curr_sum;
        }
    }
    max_sum
}

fn main() {
    let a = vec![1, 3, -2, 4];
    println!("{}", max_sub_array_sum(&a));
}