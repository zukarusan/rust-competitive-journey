// https://www.hackerrank.com/challenges/between-two-sets/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'get_total_x' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn find_common_factor(x: u64, y: u64) -> u64 {
    let max = x.max(y);
    let min = x.min(y);
    for i in 1..=min {
        let f = max * i;
        if f % x == 0 && f % y == 0 {
            return f;
        }
    }
    return 0;
}
fn get_total_x(a: &[i32], b: &[i32]) -> u64 {
    let min_b = *b.iter().min().unwrap() as u64;
    let mut common_least = a[0] as u64;
    for i in a.iter() {
        common_least = find_common_factor(common_least as u64, (*i) as u64);
    }
    let mut count = 0u64;
    let mut factor = common_least;
    while factor <= min_b {
        let mut has_it = true;
        for i in b.iter() {
            if *i as u64 % factor != 0 {
                has_it = false;
                break;
            }
        }
        if has_it {
            count += 1;
        }
        factor += common_least;
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

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = get_total_x(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}
