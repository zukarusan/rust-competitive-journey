// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem?isFullScreen=true
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'breaking_records' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY scores as parameter.
 */

fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut max = scores[0];
    let mut min = scores[0];
    let mut c_max = 0;
    let mut c_min = 0;
    for s in scores {
        if *s > max {
            max = *s;
            c_max += 1;
        }
        if *s < min {
            min = *s;
            c_min += 1;
        }
    }
    vec![c_max, c_min]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = breaking_records(&scores);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
