// https://www.hackerrank.com/challenges/print-the-elements-of-a-linked-list-in-reverse/problem?isFullScreen=true

use std::{collections::LinkedList, env, fs::File, io::{self, BufRead, Write}};


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut file = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    
    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        let mut list: LinkedList<i32> = LinkedList::new();
        for _ in 0..n {
            let el = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
            list.push_back(el);
        }
        
        list.iter().rev().for_each(|x| {
            writeln!(&mut file, "{}", x).ok();
        });
    }
}