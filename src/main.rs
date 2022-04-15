use std::env::args;
use std::process::Command;

macro_rules! xdotool_output {
    ($cmd:expr) => {
        Command::new("sh")
            .arg("-c")
            .arg(format!("xdotool {}", $cmd))
            .output()
            .expect("Please install xdotool.")
            .stdout
            .iter()
            .map(|&b| b as char)
            .collect::<String>()
    };
}

macro_rules! xdotool_spawn {
    ($cmd:expr) => {
        Command::new("sh")
            .arg("-c")
            .arg(format!("xdotool {}", $cmd))
            .spawn()
            .expect("Please install xdotool.")
    };
}

pub fn get_focused_window() -> String {
    xdotool_output!("getwindowfocus")
}

pub fn unmap_window(window: &str) {
    xdotool_spawn!(format!("windowunmap {}", window));
}

pub fn map_window(window: &str) {
    xdotool_spawn!(format!("windowmap {}", window));
}

pub fn escape(mut i: String) -> String {
    for c in ['`', '\"', '\'', '(', ')', '[', ']', '&', ' '] {
        i = i.replace(c, format!("\\{}", c).as_ref());
    }
    i
}

fn main() {
    let focused = get_focused_window();
    // `\"'()[]&
    let command = args().map(escape).skip(1).collect::<Vec<_>>().join(" ");
    println!("{}", command);
    unmap_window(&focused);
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    map_window(&focused);
}
