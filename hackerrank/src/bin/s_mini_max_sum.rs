// https://www.hackerrank.com/challenges/mini-max-sum/problem?isFullScreen=true

use std::io::{self, BufRead};

/*
 * Complete the 'mini_max_sum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn mini_max_sum(arr: &[i32]) {
    let mut _arr: Vec<u128> = arr.iter().map(|x| (*x) as u128).collect();
    _arr.sort();
    let min:u128 = _arr[0..4].iter().sum();
    let max:u128 = _arr[1..5].iter().sum();
    
    println!("{} {}", min, max);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    mini_max_sum(&arr);
}
