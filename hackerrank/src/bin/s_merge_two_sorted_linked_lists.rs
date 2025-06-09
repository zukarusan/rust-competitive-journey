// https://www.hackerrank.com/challenges/merge-two-sorted-linked-lists/problem?isFullScreen=true

use std::{collections::LinkedList, env, fs::File, io::{self, BufRead, Write}};

fn merge_list(into: &mut LinkedList<i32>, from: &mut LinkedList<i32>) {
    if let Some(insert_value) = from.pop_front() {
        let c = into.iter().take_while(|x| **x <= insert_value).count();
        let mut split = into.split_off(c);
        merge_list(&mut split, from);
        split.push_front(insert_value);
        into.append(&mut split);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let file = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    
    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    for _ in 0..t {
        let mut n_list: LinkedList<i32> = LinkedList::new();
        let mut m_list: LinkedList<i32> = LinkedList::new();
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        for _ in 0..n {
            let el = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
            n_list.push_back(el);
        }
        let m = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        for _ in 0..m {
            let el = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
            m_list.push_back(el);
        }
        let (mut into, mut from) = (n_list, m_list);
        if from.len() > into.len() {
            (from, into) = (into, from);
        }
        merge_list(&mut into, &mut from);
        let mut sep = "";
        for e in into.iter() {
            write!(&file, "{}", sep).ok();
            write!(&file, "{}", e).ok();
            sep = " ";
        }
        writeln!(&file).ok();
    }
}