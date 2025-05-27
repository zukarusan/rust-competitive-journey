use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'rotate_left' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER d
 *  2. INTEGER_ARRAY arr
 */
fn modulo(x: i32, m: usize) -> usize {
    let _m = m as i32;
    (((x % _m) + _m) % _m) as usize
}
fn rotate_left(d: i32, arr: &[i32]) -> Vec<i32> {
    let len = arr.len();
    let mut shift = vec![0; len];
    for i in 0..len {
        shift[i] = arr[modulo(i as i32 - d, len)];
    }
    shift
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

    let d = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = rotate_left(d, &arr);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
