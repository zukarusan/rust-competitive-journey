// https://www.hackerrank.com/challenges/migratory-birds/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratory_birds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];
    for i in arr {
        counts[*i as usize] += 1;
    }
    let mut max = (0, 1);
    for i in 1..6 {
        let c = counts[i];
        if c > max.0 {
            max.0 = c;
            max.1 = i;
        }
    }
    max.1 as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratory_birds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
