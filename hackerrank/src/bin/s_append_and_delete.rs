// https://www.hackerrank.com/challenges/append-and-delete/problem?isFullScreen=true

use std::{env, fs::File, io::{self, BufRead, Write}, ops::Index};

fn append_and_delete(s: &str, t: &str, k: usize) -> String {
    let common = s.chars()
        .zip(t.chars().into_iter())
        .take_while(|&(x, y)| x == y )
        .count();

    let s_len = s.len();
    let t_len = t.len();
    let min_k = s_len - common + (t_len - common);
    if k < min_k {
        return "No".to_string();
    }
    if min_k == k {
        return "Yes".to_string();
    }
    let diff_k = k - min_k;

    if diff_k % 2 == 0 {
        return "Yes".to_string();
    } else if diff_k > common * 2 {
        return "Yes".to_string();
    }
    "No".to_string()
}


fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    let s = iterator.next().unwrap().unwrap().trim().to_string();
    let t = iterator.next().unwrap().unwrap().trim().to_string();
    let k = iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    writeln!(&mut fptr, "{}", append_and_delete(&s, &t, k)).unwrap();
}
