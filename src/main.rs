use std::env::args;
use std::process::Command;

pub fn escape(mut i: String) -> String {
    for c in ['`', '\"', '\'', '(', ')', '[', ']', '&', ' '] {
        i = i.replace(c, format!("\\{}", c).as_ref());
    }
    i
}

fn main() {
    let (conn, _) = xcb::Connection::connect(None).unwrap();
    let focused = xcb::get_input_focus(&conn).get_reply().unwrap().focus();
    let command = args().map(escape).skip(1).collect::<Vec<_>>().join(" ");
    xcb::unmap_window_checked(&conn, focused)
        .request_check()
        .unwrap();
    conn.flush();
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    xcb::map_window_checked(&conn, focused)
        .request_check()
        .unwrap();
    conn.flush();
}
