// https://www.hackerrank.com/challenges/time-conversion/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'time_conversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn time_conversion(s: &str) -> String {
    let am_pm = s[s.len()-2..].to_lowercase();
    let mut times: Vec<i32> = s[..s.len()-2].trim_end().split(':').map(|x| x.parse::<i32>().unwrap()).collect();
    let hour = times[0];
    if am_pm == "pm" {
        times[0] = if hour == 12 { hour } else { hour + 12 };
    } else if am_pm == "am" {
        times[0] = if hour == 12 { 0 } else { hour };
    }
    format!("{:02}:{:02}:{:02}", times[0], times[1], times[2])
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = time_conversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
