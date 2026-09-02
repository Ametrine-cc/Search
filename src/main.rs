// use miamore::{
// self, clear_origin, draw_border, draw_text, init_miamore, manage_cursor, position_t,
// wait_for_seconds,
// };

use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::sync::{LazyLock, Mutex};

#[warn(unused)]
static ERROR_BUF: Mutex<String> = Mutex::new(String::new());

static IGNORE_STACK: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

// fn tui_view(_err: &str) {
//     init_miamore(true, true);
//     draw_border("Rust example", miamore::theme_t::thick_l);

//     manage_cursor(miamore::cursor_t::move_, Some(position_t { x: 3, y: 4 }));
//     draw_text("Hello from Ametrine!");

//     wait_for_seconds(8.0);
//     clear_origin();

//     let err = "error here";
//     set_error(err);
// }

fn add_ignore_stack(_ignore_file: String) {
    let mut stack = IGNORE_STACK.lock().unwrap();
    stack.push(_ignore_file);
}

fn release_ignore_stack() {
    let mut stack = IGNORE_STACK.lock().unwrap();
    while let Some(val) = stack.pop() {
        println!("Popped: {val}");
    }
}

fn read_gitignore() -> std::io::Result<()> {
    let ignore_file = File::open(".gitignore")?;
    let reader = BufReader::new(ignore_file);

    for line in reader.lines() {
        let line = line?;
        add_ignore_stack(line);
    }

    Ok(())
}

fn main() {
    #[warn(unused_variables)]
    let _current_err = ERROR_BUF.lock().unwrap();

    if let Err(err) = read_gitignore() {
        set_error(&err.to_string());
    }

    // tui_view(&current_err);
    release_ignore_stack();
    return;
}

fn set_error(msg: &str) {
    eprintln!("error -> {}", msg);
    release_ignore_stack();
}
