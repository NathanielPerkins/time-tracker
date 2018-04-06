extern crate chrono;

use std::{thread, time};
use std::process::Command;

use chrono::Local;

fn main() {
    let delay = time::Duration::from_secs(1);
    loop {
        thread::sleep(delay);
        let output = Command::new("xdotool")
                             .arg("getwindowfocus")
                             .arg("getwindowname")
                             .output()
                             .expect("failed to execute process");
        let now = Local::now();
        println!("{}: {}", now.format("%Y-%m-%d][%H:%M:%S").to_string(), String::from_utf8_lossy(&output.stdout))
    }
}
