// https://www.hackerrank.com/challenges/reverse-a-linked-list/problem?isFullScreen=true

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
            list.push_front(el);
        }
        
        let mut sep = "";
        list.iter().for_each(|x| {
            write!(&mut file, "{}", sep).ok();
            write!(&mut file, "{}", x).ok();
            sep = " ";
        });
        writeln!(&mut file).ok();
    }
}