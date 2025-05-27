use std::{collections::HashSet, hash::Hash, io::{self, stdout, BufRead, Write}};
use rust_competitive_journey::hackerrank;

fn vec_to_set<T>(vec: Vec<T>) -> HashSet<T> where T: Eq + Hash {
    HashSet::from_iter(vec)
}
fn main() {
    println!("Welcome to your Rust competitive journey");
    loop {
        _ = _main() && break;
    };
}

fn _main() -> bool {
    println!();
    println!("================================================");
    println!("Select what to do:");
    println!("1. Load problem from URL");
    println!("2. Run solution");
    println!("q. Exit");
    
    let opt = get_option().unwrap_or_default();
    match opt.as_str() {
        "1" => (do_load_problem(), false).1,
        "2" => do_run_solution().unwrap_or(false),
        "q" => true,
        _ => (println!("Please select one of the options only."), false).1
    }
}

fn get_option() -> Option<String> {
    print!("> ");
    stdout().flush().ok()?;
    let stdin = io::stdin();
    let mut iter_in = stdin.lock().lines();
    iter_in.next()?.ok()
}

fn do_load_problem() {
    println!("test")
}
fn do_run_solution() -> Option<bool> {
    println!("Select module:");
    println!("1. HackerRank");
    print!("> ");
    stdout().flush().ok()?;
    let stdin = io::stdin();
    let opt = {
        let mut iter_in = stdin.lock().lines();
        iter_in.next()?.ok()?
    };
    Some(match opt.as_str() {
        "1" => do_run_solution_hackerrank().unwrap_or(false),
        _ => (println!("Invalid module"), false).1
    })
}
fn do_run_solution_hackerrank() -> Option<bool> {
    for m in hackerrank::LIST {
        println!("{}", m);
    }
    print!("Type solution to run > ");
    stdout().flush().ok()?;
    let stdin = io::stdin();
    let mut iter_in = stdin.lock().lines();
    let name = iter_in.next()?.ok()?;

    let names = vec_to_set(hackerrank::LIST.to_vec());
    if !names.contains(name.as_str()) {
        println!("There's no solution named {}", name);
        return Some(false);
    }
    
    Some(true)
}