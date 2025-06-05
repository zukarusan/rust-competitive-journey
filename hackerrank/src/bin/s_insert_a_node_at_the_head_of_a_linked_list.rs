// https://www.hackerrank.com/challenges/insert-a-node-at-the-head-of-a-linked-list/problem?isFullScreen=true

use std::{collections::LinkedList, env, fs::File, io::{self, BufRead, Write}};


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut file = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut list: LinkedList<i32> = LinkedList::new();
    for _ in 0..n {
        let el = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        list.push_front(el);
    }
    list.iter().for_each(|x| {
        writeln!(&mut file, "{}", x).ok();
    });
}