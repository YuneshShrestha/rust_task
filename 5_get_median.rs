// Given a sorted array of integers, implement a function that returns the median of the array.

fn get_median(arr: &[i32]) -> f64 {
    let n = arr.len();
    if n % 2 == 0 {
        return (arr[n / 2] + arr[n / 2 - 1]) as f64 / 2.0;
    } else {
        return arr[n / 2] as f64;
    }
}

fn main() {
    // Assuming given array is already sorted
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    println!("{}", get_median(&arr));
}