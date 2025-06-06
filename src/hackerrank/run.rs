use std::{io::{stdout, Stdin, Write}, process::Command};

use crate::{error::{AsErrorResult, IntoError}, hackerrank, input, vec_to_set, Res};

pub fn run_solution(stdin: &Stdin) -> Res {
    for m in hackerrank::LIST {
        println!("{}", m);
    }
    print!("Type solution to run > ");
    stdout().flush().wrap_err()?;
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
    

    "Running solution is not yet implemented".into_err()
}