use std::{collections::HashSet, hash::Hash, io::{self, stdout, BufRead, Stdin, Write}, process::Command};

use rust_competitive_journey::{error::{AsError, AsErrorResult, IntoError, MessageError}, hackerrank, platform_from_url, Platform, Res};

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
        _ => "Please select one of the options only.".into_err()
    };
    match res {
        Err(msg) => (println!("{}", msg), true).1,
        Ok(still_run) => still_run
    }
}

fn input(stdin: &Stdin) -> Result<String, MessageError> {
    stdin.lock()
        .lines()
        .next()
        .as_err("FATAL: iter finished")?
        .wrap_err()
}

fn get_option(stdin: &Stdin) -> Option<String> {
    print!("> ");
    stdout().flush().ok()?;
    input(stdin).ok()
}

fn do_load_problem(stdin: &Stdin) -> Res {
    print!("Input URL > ");
    stdout().flush().wrap_err()?;

    let url = input(stdin)?;
    let Some(platform) = platform_from_url(&url) else {
        return "URL not supported".into_err();
    };

    match platform {
        Platform::Hackerrank => hackerrank::load_problem(&url),
        Platform::Leetcode => "Platform not implemented".into_err()
    }
    
}
fn do_run_solution(stdin: &Stdin) -> Res {
    println!("Select module:");
    println!("1. HackerRank");
    print!("> ");
    stdout().flush().wrap_err()?;

    let opt = input(&stdin)?;
    match opt.as_str() {
        "1" => do_run_solution_hackerrank(),
        _ => "Invalid module".into_err(),
    }
}
fn do_run_solution_hackerrank() -> Res {
    for m in hackerrank::LIST {
        println!("{}", m);
    }
    print!("Type solution to run > ");
    stdout().flush().wrap_err()?;
    let stdin = io::stdin();
    let name = input(&stdin)?;

    let names = vec_to_set(hackerrank::LIST.to_vec());
    if !names.contains(name.as_str()) {
        return format!("There's no solution named {name}").into_err();
    }

    let _handle = Command::new("cargo")
        .args([
            "run",
            "-p", "hackerrank",
            "--bin", name.as_str()
        ])
        .spawn()
        .expect("Failed to run solution");
    
    Ok(())
}