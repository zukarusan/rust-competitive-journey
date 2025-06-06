use headless_chrome::Browser;

use crate::{error::{AsErrorResult}, Res, OK};

pub fn load_problem(url: &str) -> Res {
    let browser = Browser::default().wrap_err()?;
    OK
}