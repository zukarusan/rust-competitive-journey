// https://www.hackerrank.com/challenges/find-digits/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'find_digits' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER n as parameter.
 */

fn find_digits(n: i32) -> i32 {
    let mut _n = n;
    let mut count = 0;
    while _n != 0 {
        let d = _n % 10;
        if d != 0 && n % d == 0 {
            count += 1;
        }
        _n /= 10;
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let result = find_digits(n);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
