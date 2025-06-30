// https://www.hackerrank.com/challenges/designer-pdf-viewer/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'designer_pdf_viewer' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY h
 *  2. STRING word
 */

fn designer_pdf_viewer(h: &[i32], word: &str) -> i32 {
    let mut max = 0;
    for c in word.chars() {
        max = max.max(h[c as usize-97]);
    }
    max * word.len() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let h: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let word = stdin_iterator.next().unwrap().unwrap();

    let result = designer_pdf_viewer(&h, &word);

    writeln!(&mut fptr, "{}", result).ok();
}