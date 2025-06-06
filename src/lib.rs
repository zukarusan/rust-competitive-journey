pub mod hackerrank;
pub mod leetcode;
pub mod error;
use std::{collections::HashSet, hash::Hash, io::{BufRead, Stdin}};

use crate::error::{AsError, AsErrorResult, MessageError};
use regex::Regex;

pub type Res = Result<(), MessageError>;
pub const OK: Res = Ok(());
pub enum Platform {
    Hackerrank,
    Leetcode
}
pub fn vec_to_set<T>(vec: Vec<T>) -> HashSet<T> where T: Eq + Hash {
    HashSet::from_iter(vec)
}
pub fn platform_from_url(url: &str) -> Option<Platform> {
    let url_re = Regex::new(r"https?:\/\/(?:\w*)?\.?(\w+)\.(?:com)").ok()?;
    let domain = url_re.captures(url)?.get(1)?.as_str();
    match domain {
        "hackerrank" => Some(Platform::Hackerrank),
        "leetcode" => Some(Platform::Leetcode),
        _ => None
    }
}
pub fn input(stdin: &Stdin) -> Result<String, MessageError> {
    stdin.lock()
        .lines()
        .next()
        .as_err("FATAL: iter finished")?
        .wrap_err()
}
