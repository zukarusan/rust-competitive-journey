// https://www.hackerrank.com/challenges/crush/problem?isFullScreen=true

use std::{env, vec};
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'array_manipulation' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. 2D_INTEGER_ARRAY queries
 */
fn has_intersection(first: (i32, i32), second: (i32, i32)) -> bool {
    return first.0 <= second.1 && first.1 >= second.0;
}
fn split_intersection(range: (i32, i32), val: i64, by_range: (i32, i32), k: i64) -> Vec<((i32, i32), i64)> {
    let mut splits:Vec<((i32, i32), i64)> = vec![];
    let intersected = (range.0.min(by_range.0), range.1.min(by_range.1));
    if range.0 < intersected.0 {
        splits.push(((range.0, intersected.0-1), val));
    }
    if range.1 > intersected.1 {
        splits.push(((intersected.1+1, range.1), val));
    }
    splits.push((intersected, val + k));
    splits
}
fn array_manipulation(n: i32, queries: &[Vec<i32>]) -> i64 {
    let mut stack = vec![];
    let mut result: Vec<((i32, i32), i64)> = vec![];
    stack.push(((1, n), 0i64));
    for q in queries {
        let range = (q[0], q[1]);
        let k = q[2] as i64;
        stack.append(&mut result);
        while let Some(top) = stack.pop() {
            if !has_intersection(range, top.0) {
                result.push(top);
                continue;
            }
            let res = split_intersection(top.0, top.1, range, k);
            result.extend(res);
        }
    }
    result.iter()
        .map(|x| x.1)
        .max()
        .unwrap_or(0)
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

    let mut queries: Vec<Vec<i32>> = Vec::with_capacity(m as usize);

    for i in 0..m as usize {
        queries.push(Vec::with_capacity(3_usize));

        queries[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = array_manipulation(n, &queries);

    writeln!(&mut fptr, "{}", result).ok();
}
