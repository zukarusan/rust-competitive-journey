// https://www.hackerrank.com/challenges/strange-advertising/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'viral_advertising' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER n as parameter.
 */

fn viral_advertising(n: i32, shares: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let likes = shares / 2;
    likes + viral_advertising(n - 1, likes * 3)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = viral_advertising(n, 5);

    writeln!(&mut fptr, "{}", result).ok();
}
