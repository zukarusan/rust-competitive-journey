// https://www.hackerrank.com/challenges/crush/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

/*
 * Complete the 'array_manipulation' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. 2D_INTEGER_ARRAY queries
 */

fn array_manipulation(n: i32, queries: &[Vec<i32>]) -> i64 {
    let _n = n as usize;
    let mut arr = vec![0i64; _n];
    for q in queries {
        let (a, b) = (q[0] as usize, q[1] as usize);
        let k = q[2] as i64;
        arr[a-1] += k;
        if b < _n {
            arr[b] -= k;
        }
    }
    let (mut cur_val, mut max) = (0, 0);
    for change in arr {
        cur_val += change;
        max = max.max(cur_val);
    }
    max

}

fn main() {
    let stdin = BufReader::new(File::open("inputs/s_crush_in.txt").unwrap());
    let mut stdin_iterator = stdin.lines();

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
