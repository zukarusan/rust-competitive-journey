// https://www.hackerrank.com/challenges/sparse-arrays/problem?isFullScreen=true

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead, Write};
use std::iter::TakeWhile;

/*
 * Complete the 'matching_strings' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. STRING_ARRAY string_list
 *  2. STRING_ARRAY queries
 */
fn matching_strings(string_list: &[String], queries: &[String]) -> Vec<i32> { // this is faster ig
    let mut answers:Vec<i32> = vec![];
    let mut map = HashMap::<&str, i32>::new();
    for s in string_list {
        let _s = s.as_str();
        map.entry(_s)
            .and_modify(|m| *m += 1)
            .or_insert(1);
    }
    for q in queries {
        let _q = &q.as_str();
        answers.push(*map.get(_q).unwrap_or(&0));
    }
    answers
}

fn matching_strings_v2(string_list: &[String], queries: &[String]) -> Vec<i32> {
    let mut answer:Vec<i32> = vec![];
    let mut string_vec = string_list.to_vec();
    let mut map = HashMap::<&str, i32>::new();
    
    string_vec.sort_unstable();
    for q in queries {
        map.entry(q)
        .and_modify(|a| {
            answer.push(*a);
        })
        .or_insert_with(|| {
            if let Ok(idx) = string_vec.binary_search(q) {
                let mut count = 1;
                count += v2_count_occurence(&string_vec[..], &q, idx, true);
                count += v2_count_occurence(&string_vec[..], &q, idx, false);
                answer.push(count);
                count
            } else {
                answer.push(0);
                0
            }
        });
    }
    answer
}

fn v2_count_occurence(arr: &[String] , find: &str, idx: usize, left_or_right: bool) -> i32 {
    let len = arr.len();
    let step = if left_or_right { -1i32 } else { 1 };
    let start = idx as i32 + step;
    let mut i = start;
    let mut count = 0;
    loop {
        let _i = i as usize;
        if i < 0 || _i >= len {
            break;
        }
        if arr[_i] != find {
            break;
        }
        count += 1;
        i += step;
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let stringList_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut string_list: Vec<String> = Vec::with_capacity(stringList_count as usize);

    for _ in 0..stringList_count {
        let stringList_item = stdin_iterator.next().unwrap().unwrap();
        string_list.push(stringList_item);
    }

    let queries_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut queries: Vec<String> = Vec::with_capacity(queries_count as usize);

    for _ in 0..queries_count {
        let queries_item = stdin_iterator.next().unwrap().unwrap();
        queries.push(queries_item);
    }

    let res = matching_strings(&string_list, &queries); // by this we know that the simplest approach "often" means the correct one

    for i in 0..res.len() {
        write!(&mut fptr, "{}", res[i]).ok();

        if i != res.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
