// https://www.hackerrank.com/challenges/picking-numbers/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'picking_numbers' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn picking_numbers(a: &[i32]) -> i32 {
    let mut _a = a.to_vec();
    let mut nums: Vec<i32> = vec![];
    let mut counts: Vec<usize> = vec![];
    _a.sort();
    let mut cur = -1;
    let mut count: &mut usize = &mut 0;
    for e in _a {
        if cur != e {
            nums.push(e);
            counts.push(0);
            cur = e;
            count = counts.last_mut().unwrap();
        }
        *count = *count + 1usize;
    }
    let mut max = counts[0];
    for i in 1..nums.len() {
        if nums[i]-nums[i-1] == 1 {
            max = max.max(counts[i]+counts[i-1]);
        } else {
            max = max.max(counts[i]);
        }
    }
    max as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = picking_numbers(&a);

    writeln!(&mut fptr, "{}", result).ok();
}
