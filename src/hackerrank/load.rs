use std::{env::current_dir, fs::File, io::Write, thread::sleep, time::Duration};

use headless_chrome::{util::Wait, Browser, LaunchOptionsBuilder};
use regex::Regex;

use crate::{error::{AsErrorResult}, Res, OK};

const _INIT_PREFERENCE: &'static str = "{\"codeshellUserOpts\":\"{\"theme\":\"light\",\"tabSize\":4,\"intellisense\":true,\"mode\":\"normal\",\"keyMap\":\"sublime\",\"indentUnit\":4,\"userPreferredLang\":\"rust\"}\"}";

pub fn load_problem(url: &str) -> Res {
    let url_re = Regex::new(r"https?:\/\/(?:\w*)?\.?hackerrank\.(?:com)\/challenges/([^/]+)/").wrap_err()?;
    let mut file_name = url_re.captures(url)
        .ok_or("URL seems not in intended format").wrap_err()?
        .get(1).ok_or("Can't find problem name from url").wrap_err()?
        .as_str()
        .to_string();
    file_name = file_name.replace("-", "_");
    file_name = format!("s_{}.rs", file_name);
    let binding = current_dir().wrap_err()?;
    let cur_dir_path = binding.display();
    let file_path = &format!("{cur_dir_path}/hackerrank/src/bin/{file_name}");
    println!("{file_path}");

    let mut file = File::create(file_path).wrap_err()?;

    writeln!(&mut file, "// {}", url).wrap_err()?;
    writeln!(&mut file).wrap_err()?;
    
    OK
}
// TODO
fn _retrieve_codes(url: &str) -> Res {
    let browser = Browser::new(LaunchOptionsBuilder::default()
            .headless(false) // Ensure it's headless
            .enable_gpu(true)
            .enable_logging(true)
            .devtools(true)
            .sandbox(true)
            .build()
            .wrap_err()?).wrap_err()?;
    let tab = browser.new_tab().wrap_err()?;

    
    tab.navigate_to(url).wrap_err()?;
    tab.wait_until_navigated().wrap_err()?;
    sleep(Duration::from_secs(10));
    tab.set_storage("jStorage", _INIT_PREFERENCE).wrap_err()?;
    
    let isset = tab.get_storage::<String>("jStorage").wrap_err()?;
    println!("CHECK PREF: {isset}");
    
    // tab.reload(false, Some("")).wrap_err()?;
    // tab.wait_until_navigated().wrap_err()?;
    sleep(Duration::from_secs(60 * 30));
    OK
}