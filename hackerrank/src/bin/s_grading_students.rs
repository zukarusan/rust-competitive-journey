// https://www.hackerrank.com/challenges/grading/problem?isFullScreen=true
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'grading_students' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY grades as parameter.
 */

fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut result = grades.to_vec();
    for r in result.iter_mut() {
        if *r < 38 {
            continue;
        }
        let diff = (5 - (*r % 5)) % 5;
        if diff < 3 {
            *r = *r + diff;
        }
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let grades_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);

    for _ in 0..grades_count {
        let grades_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        grades.push(grades_item);
    }

    let result = grading_students(&grades);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
