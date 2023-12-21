use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn mini_max_sum(arr: &[i32]) {
    // sort arr
    let mut arr = arr.to_vec();
    arr.sort_unstable(); // We don't care about the order of equal elements and
                         // we are working with primitive types, so this is fine
    let min_sum: i64 = arr
        .iter() // Creates an iterator for the array
        .take(4) // Takes the first 4 elements
        .map(|&x| x as i64) //map takes a function and applies it to each element
        .sum(); // Sums the elements, returning an i64
    let max_sum: i64 = arr
        .iter()
        .skip(1) // Skips a number of elements, doesn't take index into account
        .take(4)
        .map(|&x| x as i64)
        .sum();
    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    mini_max_sum(&arr);
}
