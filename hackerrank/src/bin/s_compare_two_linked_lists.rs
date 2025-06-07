// https://www.hackerrank.com/challenges/compare-two-linked-lists/problem?isFullScreen=true

use std::{collections::LinkedList, env, fs::File, io::{self, BufRead, Write}};

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let file = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    
    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    for _ in 0..t {
        let mut n_list: LinkedList<i32> = LinkedList::new();
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        for _ in 0..n {
            let el = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
            n_list.push_back(el);
        }
        let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
        let value = n_list.iter().rev().skip(p).next().unwrap();
        writeln!(&file, "{}", value).ok();
    }
}