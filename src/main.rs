use std::io::{self, stdout, BufRead, Write};

enum Opt {
    Load,
    Run,
    Exit
}

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
    
    let opt = match get_option() {
        Some(o) => o,
        None => {
            println!("Please select one of the options only.");
            return false;
        }
    };
    match opt {
        Opt::Load => (do_load_problem(), false).1,
        Opt::Run => (do_run_solution(), false).1,
        Opt::Exit => true
    }
}

fn get_option() -> Option<Opt> {
    print!("> ");
    stdout().flush().ok()?;
    let stdin = io::stdin();
    let mut iter_in = stdin.lock().lines();
    iter_in.next()?.ok().map(|opt| match opt.as_str() {
        "1" => Some(Opt::Load),
        "2" => Some(Opt::Run),
        "q" => Some(Opt::Exit),
        _ => None
    })?
}

fn do_load_problem() {
    println!("test")
}

fn do_run_solution() {
    list_modules::here!();
}