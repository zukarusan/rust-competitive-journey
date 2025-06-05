pub mod hackerrank;
pub mod leetcode;
use regex::Regex;
pub const OK: Result<(), String> = Ok(());
pub enum Platform {
    Hackerrank,
    Leetcode
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