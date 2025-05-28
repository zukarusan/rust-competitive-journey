// https://www.hackerrank.com/challenges/sparse-arrays/problem?isFullScreen=true

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'matching_strings' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. STRING_ARRAY string_list
 *  2. STRING_ARRAY queries
 */
fn matching_strings(string_list: &[String], queries: &[String]) -> Vec<i32> {
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

    let res = matching_strings(&string_list, &queries);

    for i in 0..res.len() {
        write!(&mut fptr, "{}", res[i]).ok();

        if i != res.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
