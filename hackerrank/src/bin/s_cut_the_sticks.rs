// https://www.hackerrank.com/challenges/cut-the-sticks/problem?isFullScreen=true

use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'cut_the_sticks' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn cut_the_sticks(arr: &[i32]) -> Vec<i32> {
    let mut sticks = arr.to_vec();
    sticks.sort_unstable();
    let mut results = vec![];
    let mut prev = 0;
    let n = sticks.len();
    let mut i = 0;
    while i < n {
        let cur = sticks[i];
        if cur != prev {
            results.push((n-i) as i32);
        }
        i += 1;
        prev = cur;
    }
    results
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = cut_the_sticks(&arr);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
