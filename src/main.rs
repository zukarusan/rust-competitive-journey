use std::{io::{self, stdout, Stdin, Write}};

use rust_competitive_journey::{error::{AsErrorResult, IntoError}, hackerrank, input, platform_from_url, Platform, Res};


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

    let opt = input(stdin)?;
    match opt.as_str() {
        "1" => hackerrank::run_solution(stdin),
        _ => "Invalid module".into_err(),
    }
}
