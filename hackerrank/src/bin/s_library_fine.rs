// https://www.hackerrank.com/challenges/library-fine/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'library_fine' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER d1
 *  2. INTEGER m1
 *  3. INTEGER y1
 *  4. INTEGER d2
 *  5. INTEGER m2
 *  6. INTEGER y2
 */

fn library_fine(d1: i32, m1: i32, y1: i32, d2: i32, m2: i32, y2: i32) -> i32 {
    if y1 < y2 || (m1 < m2 && y1 == y2) || (d1 <= d2 && m1 == m2 && y1 == y2) {
        0
    } else if m1 == m2 && y1 == y2 {
        (d1 - d2) * 15  
    } else if y1 == y2 {
        (m1 - m2) * 500
    } else {
        10000
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let d1 = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m1 = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let y1 = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let second_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let d2 = second_multiple_input[0].trim().parse::<i32>().unwrap();

    let m2 = second_multiple_input[1].trim().parse::<i32>().unwrap();

    let y2 = second_multiple_input[2].trim().parse::<i32>().unwrap();

    let result = library_fine(d1, m1, y1, d2, m2, y2);

    writeln!(&mut fptr, "{}", result).ok();
}
