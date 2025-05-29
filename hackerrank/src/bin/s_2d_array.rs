use std::cmp::max;
// https://www.hackerrank.com/challenges/2d-array/problem?isFullScreen=true
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'hourglass_sum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn hourglass_sum(arr: &[Vec<i32>]) -> i32 {
    let mut max_sum: i32 = -63;
    let mut last1;
    let mut last3;
    let mut sum_w1;
    let mut sum_w2: i32;
    let mut sum_w3;
    for i in 2..6 {
        sum_w1 = arr[i-2].iter().take(2).sum::<i32>();
        sum_w3 = arr[i].iter().take(2).sum::<i32>();
        last1 = 0;
        last3 = 0;
        for j in 2..6 {
            sum_w1 = sum_w1 + arr[i-2][j] - last1;
            sum_w2 = arr[i-1][j-1];
            sum_w3 = sum_w3 + arr[i][j] - last3;
            last1 = arr[i-2][j-2];
            last3 = arr[i][j-2];
            max_sum = max(max_sum, sum_w1 + sum_w2 + sum_w3);
        }
    }
    max_sum
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(6_usize);

    for i in 0..6_usize {
        arr.push(Vec::with_capacity(6_usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = hourglass_sum(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
