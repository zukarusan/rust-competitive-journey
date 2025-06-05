// https://www.hackerrank.com/challenges/delete-a-node-from-a-linked-list/problem?isFullScreen=true

use std::{collections::LinkedList, env, fs::File, io::{self, BufRead, Write}};


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut file = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    let mut list: LinkedList<i32> = LinkedList::new();
    for _ in 0..n {
        let el = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        list.push_back(el);
    }
    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let mut tail = list.split_off(p).split_off(1);
    list.append(&mut tail);

    let mut sep = "";
    list.iter().for_each(|x| {
        write!(&mut file, "{}", sep).ok();
        write!(&mut file, "{}", x).ok();
        sep = " ";
    });
}