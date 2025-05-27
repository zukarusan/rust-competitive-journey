use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'dynamic_array' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. 2D_INTEGER_ARRAY queries
 */

fn dynamic_array(n: i32, queries: &[Vec<i32>]) -> Vec<i32> {
    let mut arr: Vec<Vec<i32>> = vec![vec![]; n as usize];
    let mut answers: Vec<i32> = vec![];

    let mut idx:usize;
    let mut last_answer = 0;

    for i in queries.iter() {
        let q = i[0];
        let x = i[1];
        let y = i[2];
        idx = ((x ^ last_answer) % n) as usize;
        if q == 1 {
            arr[idx].push(y);
        } else if q == 2 {
            let a_idx = y as usize % arr[idx].len();
            last_answer = arr[idx][a_idx];
            answers.push(last_answer);
        }
    }

    answers
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

    let q = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let mut queries: Vec<Vec<i32>> = Vec::with_capacity(q as usize);

    for i in 0..q as usize {
        queries.push(Vec::with_capacity(3_usize));

        queries[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = dynamic_array(n, &queries);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
