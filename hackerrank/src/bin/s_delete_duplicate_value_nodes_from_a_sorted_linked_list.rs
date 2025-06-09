// https://www.hackerrank.com/challenges/delete-duplicate-value-nodes-from-a-sorted-linked-list/problem?isFullScreen=true


use std::{collections::LinkedList, env, fs::File, io::{self, BufRead, Write}};

fn keep_popping_until_not_equal(list: &mut LinkedList<i32>, dup_value: i32) {
    while let Some(value) = list.front().map(|x| *x) {
        if value == dup_value {
            list.pop_front();
        } else {
            break;
        }
    }
}

fn remove_duplicates(list: &mut LinkedList<i32>) {
    let dup_value: Option<i32> = list.pop_front();
    if let Some(value) = dup_value {
        keep_popping_until_not_equal(list, value);
        remove_duplicates(list);
        list.push_front(value);
    }
}

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
        remove_duplicates(&mut n_list);
        let mut sep = "";
        for e in n_list.iter() {
            write!(&file, "{}", sep).ok();
            write!(&file, "{}", e).ok();
            sep = " ";
        }
        writeln!(&file).ok();
    }
}