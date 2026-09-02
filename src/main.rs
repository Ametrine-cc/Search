use std::path::*;
use std::sync::{LazyLock, Mutex};

mod git;
mod tui;

#[warn(unused)]
static ERROR_BUF: Mutex<String> = Mutex::new(String::new());

static CHECK_STACK: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static IGNORE_STACK: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));
// static FILE_STACK: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

static SHOW_HIDDEN: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

enum Stacks {
    CheckStack,
    // ReadStack,
    IgnoreStack,
}

fn add_to_stack(file: String, _stack: Stacks) {
    match _stack {
        Stacks::CheckStack => {
            let mut stack = CHECK_STACK.lock().unwrap();
            stack.push(file);
        } // Stacks::ReadStack => {
        // let mut stack = FILE_STACK.lock().unwrap();
        // stack.push(file);
        // }
        Stacks::IgnoreStack => {
            let mut stack = IGNORE_STACK.lock().unwrap();
            stack.push(file);
        }
    }
}

fn release_stacks(stacks: Stacks) {
    match stacks {
        Stacks::CheckStack => {
            let mut stack = CHECK_STACK.lock().unwrap();
            while let Some(val) = stack.pop() {
                println!("Popped: {val}");
            }
        } // Stacks::ReadStack => {
        // let mut stack = FILE_STACK.lock().unwrap();
        // while let Some(val) = stack.pop() {
        // println!("Popped: {val}");
        // }
        // }
        Stacks::IgnoreStack => {
            let mut stack = IGNORE_STACK.lock().unwrap();
            while let Some(val) = stack.pop() {
                println!("Popped: {val}");
            }
        }
    }
}

pub fn toggle_show_hidden() {
    if let Ok(mut lock) = SHOW_HIDDEN.lock() {
        *lock = !*lock;
    }
}

pub fn is_show_hidden() -> bool {
    SHOW_HIDDEN.lock().map(|lock| *lock).unwrap_or(false)
}

fn main() {
    #[warn(unused_variables)]
    let current_err = ERROR_BUF.lock().unwrap();

    // Get arguments
    let mut args = std::env::args().skip(1);
    let mut check_dir = String::new();

    // Parse arguments
    while let Some(arg) = args.next() {
        if "--tui" == arg {
            tui::tui_view(&current_err);
        } else if "--dir" == arg {
            if let Some(dir) = args.next() {
                check_dir = dir;
            }
        } else if "--show_hidden" == arg {
            toggle_show_hidden();
        } else {
            let file: String = arg.clone();
            add_to_stack(file, Stacks::CheckStack);
            continue;
        }
    }

    // List out all files being checked
    {
        let stack = CHECK_STACK.lock().unwrap();
        for file in stack.iter() {
            println!("finding->{}", file);
        }
    }

    // get files/folders in current directory
    // let path = &Path::new(&check_dir);
    // git::check_file(path);

    free_stacks();
    return;
}

fn free_stacks() {
    release_stacks(Stacks::CheckStack);
    // release_stacks(Stacks::ReadStack);
    release_stacks(Stacks::IgnoreStack);
}

// fn set_error(msg: &str) {
//     eprintln!("error -> {}", msg);
//     free_stacks();
// }
