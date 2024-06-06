// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.
fn get_index_of_first_occurent(data: &[i32], value: i32) -> i32 {
    let mut start = 0;
    let mut end = data.len() as i32 - 1;
    let mut result = -1;
    while start <= end {
        let mid = start + (end - start) / 2;
        if data[mid as usize] == value {
            result = mid;
            end = mid - 1;
        } else if data[mid as usize] > value {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    result
}

fn main() {
    // Assuming given array is already sorted
    let data = [1, 2, 2, 4, 5, 6, 9, 9, 10];
    println!("{}", get_index_of_first_occurent(&data, 9));
}