// https://www.hackerrank.com/challenges/day-of-the-programmer/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'day_of_programmer' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER year as parameter.
 */

fn day_of_programmer(year: i32) -> String {
    let mut days_of_month: [usize; 12] = [31, 0, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    days_of_month[1] = match year {
        1700..=1917 => if year % 4 == 0 { 29 } else { 28 }, // julian
        1919..=2700 => if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) { 29 } else { 28 }, // gregorian
        1918 => 15, // transition year
        _ => panic!("time traveler error")
    };
    let mut days = 256usize;
    let mut i = 0usize;
    while days > days_of_month[i] {
        days -= days_of_month[i];
        i += 1;
    }
    format!("{:02}.{:02}.{year}", days, i+1)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let year = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = day_of_programmer(year);

    writeln!(&mut fptr, "{}", result).ok();
}
