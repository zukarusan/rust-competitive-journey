// https://www.hackerrank.com/challenges/beautiful-days-at-the-movies/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'beautiful_days' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER i
 *  2. INTEGER j
 *  3. INTEGER k
 */

// O(digits) time complexity
fn reverse_number(mut num: i32) -> i32 {
    let mut rev = 0;
    while num != 0 {
        rev = rev * 10 + (num % 10);
        num /= 10;
    }
    rev
}

fn beautiful_days(i: i32, j: i32, k: i32) -> i32 {
    let mut count = 0;
    for d in i..=j {
        if (d - reverse_number(d)).abs() % k == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let i = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let j = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let result = beautiful_days(i, j, k);

    writeln!(&mut fptr, "{}", result).ok();
}
