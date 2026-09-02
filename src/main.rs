use miamore::{
    self, clear_origin, draw_border, draw_text, init_miamore, manage_cursor, position_t,
    wait_for_seconds,
};
use std::sync::Mutex;

static ERROR_BUF: Mutex<String> = Mutex::new(String::new());

fn tui_view(_err: &str) {
    init_miamore(true, true);
    draw_border("Rust example", miamore::theme_t::thick_l);

    manage_cursor(miamore::cursor_t::move_, Some(position_t { x: 3, y: 4 }));
    draw_text("Hello from Ametrine!");

    wait_for_seconds(8.0);
    clear_origin();

    let err = "error here";
    set_error(err);
}

fn main() {
    println!("Search - Ametrine Foundation");

    let current_err = ERROR_BUF.lock().unwrap();
    tui_view(&current_err);

    return;
}

fn set_error(msg: &str) {
    println!("error -> {}", msg);
}
