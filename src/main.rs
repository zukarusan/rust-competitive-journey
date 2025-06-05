use std::{collections::HashSet, hash::Hash, io::{self, stdout, BufRead, Stdin, Write}, process::Command};

use rust_competitive_journey::{hackerrank, platform_from_url, Platform};

fn vec_to_set<T>(vec: Vec<T>) -> HashSet<T> where T: Eq + Hash {
    HashSet::from_iter(vec)
}
fn main() {
    println!("Welcome to your Rust competitive journey");
    loop {
        _ = !_main() && break;
    };
}

fn _main() -> bool {
    println!();
    println!("================================================");
    println!("Select what to do:");
    println!("1. Load problem from URL");
    println!("2. Run solution");
    println!("q. Exit");
    let stdin = io::stdin();
    let opt = get_option(&stdin).unwrap_or_default();
    let res = match opt.as_str() {
        "1" => do_load_problem(&stdin).map(|_| true),
        "2" => do_run_solution(&stdin).map(|_| true),
        "q" => Ok(false),
        _ => Err("Please select one of the options only.".to_string())
    };
    match res {
        Err(msg) => (println!("{}", msg), true).1,
        Ok(still_run) => still_run
    }
}

fn input(stdin: &Stdin) -> Result<String, String> {
    Ok(stdin.lock()
            .lines()
            .next()
            .ok_or("FATAL: iter finished")?
            .map_err(|_| "FATAL: iter next error")?
    )
}

fn get_option(stdin: &Stdin) -> Option<String> {
    print!("> ");
    stdout().flush().ok()?;
    input(stdin).ok()
}

fn do_load_problem(stdin: &Stdin) -> Result<(), String> {
    print!("Input URL > ");
    stdout().flush().map_err(|_| "can't flush")?;

    let url = input(stdin)?;
    let Some(platform) = platform_from_url(&url) else {
        return Err("URL not supported".to_string());
    };

    match platform {
        Platform::Hackerrank => hackerrank::load_problem(),
        Platform::Leetcode => Err("Platform not implemented".to_string())
    }
    
}
fn do_run_solution(stdin: &Stdin) -> Result<(), String> {
    println!("Select module:");
    println!("1. HackerRank");
    print!("> ");
    stdout().flush().map_err(|_| "can't flush")?;

    let opt = input(&stdin)?;
    match opt.as_str() {
        "1" => do_run_solution_hackerrank(),
        _ => Err("Invalid module".to_string()),
    }
}
fn do_run_solution_hackerrank() -> Result<(), String> {
    for m in hackerrank::LIST {
        println!("{}", m);
    }
    print!("Type solution to run > ");
    stdout().flush().map_err(|_| "can't flush")?;
    let stdin = io::stdin();
    let name = input(&stdin)?;

    let names = vec_to_set(hackerrank::LIST.to_vec());
    if !names.contains(name.as_str()) {
        return Err(format!("There's no solution named {}", name.to_string()));
    }

    let handle = Command::new("cargo")
        .args([
            "run",
            "-p", "hackerrank",
            "--bin", name.as_str()
        ])
        .spawn()
        .expect("Failed to run solution");
    
    Ok(())
}