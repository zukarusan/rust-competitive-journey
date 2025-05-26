use std::io::{self, BufRead, Lines, StdinLock};

enum Opt {
    Load,
    Exit
}

fn main()  {
    println!("Welcome to your Rust competitive journey");
    loop {
        println!();
        println!("Select what to do:");
        println!("1. Load problem from URL");
        println!("q. Exit");
        let stdin = io::stdin();
        let mut iter_in = stdin.lock().lines();
        
        let opt = match get_option(&mut iter_in) {
            Some(o) => o,
            None => {
                println!("Please select one of the options only.");
                continue;
            }
        };
        match opt {
            Opt::Load => do_load(),
            Opt::Exit => break
        }
    };
}

fn get_option(iter_in: &mut Lines<StdinLock<'static>>) -> Option<Opt> {
    iter_in.next()?.ok().map(|opt| match opt.as_str() {
        "1" => Some(Opt::Load),
        "q" => Some(Opt::Exit),
        _ => None
    })?
}

fn do_load() {
    println!("test")
}