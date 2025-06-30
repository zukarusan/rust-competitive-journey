// https://www.hackerrank.com/challenges/utopian-tree/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'utopian_tree' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER n as parameter.
 */

fn utopian_tree(n: i32) -> i32 {
    let mut a = 1;
    for i in 1..=n {
        if i % 2 == 0 {
            a += 1;
        } else {
            a *= 2;
        }
    }
    a
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let result = utopian_tree(n);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
