// Implement a function that returns the kth smallest element in a given array.


fn kth_smallest_element(mut arr: Vec<i32>, k: usize) -> i32 {
    arr.sort();
    arr[k - 1]
}

fn main() {
    let arr = vec![12, 3, 5, 7, 19];
    let k = 2;
    println!("{}", kth_smallest_element(arr, k));
}