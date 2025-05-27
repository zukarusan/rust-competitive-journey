use std::io::{self, stdout, BufRead, Write};

use rust_competitive_journey::hackerrank;

fn main() {
    println!("Welcome to your Rust competitive journey");
    loop {
        _ = _main() && break;
    };
}

fn _main() -> bool {
    println!("================================================");
    println!("Select what to do:");
    println!("1. Load problem from URL");
    println!("2. Run solution");
    println!("q. Exit");
    
    let Some(opt) = get_option() else {
        println!("Unable to get option");
        return false;
    };
    match opt.as_str() {
        "1" => (do_load_problem(), false).1,
        "2" => (do_run_solution(), false).1,
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
fn do_run_solution() {
    hackerrank::LIST;
}