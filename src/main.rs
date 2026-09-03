use std::path::*;
use std::sync::{LazyLock, Mutex};

mod git;
mod search;
mod tui;

#[warn(unused)]
static ERROR_BUF: Mutex<String> = Mutex::new(String::new());

static CHECK_STACK: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static IGNORE_STACK: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static READ_STACK: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static FILE_STACK: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

static SHOW_HIDDEN: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
static SEARCH_DEEP: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
static EXPLICIT_FILE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

static GITIGNORE: LazyLock<Mutex<PathBuf>> =
    LazyLock::new(|| Mutex::new(std::path::PathBuf::from(".gitignore")));

enum Stacks {
    CheckStack,
    ReadStack,
    IgnoreStack,
    FileStack,
}

enum Toggles {
    ShowHidden,
    SearchDeep,
    ExplicitFile,
}

fn add_to_stack(file: String, _stack: Stacks) {
    match _stack {
        Stacks::CheckStack => {
            let mut stack = CHECK_STACK.lock().unwrap();
            stack.push(file);
        }
        Stacks::ReadStack => {
            let mut stack = READ_STACK.lock().unwrap();
            stack.push(file);
        }
        Stacks::IgnoreStack => {
            let mut stack = IGNORE_STACK.lock().unwrap();
            stack.push(file);
        }
        Stacks::FileStack => {
            let mut stack = FILE_STACK.lock().unwrap();
            stack.push(file);
        }
    }
}

fn release_stacks(stacks: Stacks) {
    print!("\nReleasing stack->");

    match stacks {
        Stacks::CheckStack => {
            println!("Check Stack");

            let mut stack = CHECK_STACK.lock().unwrap();
            while let Some(val) = stack.pop() {
                println!("Popped: {val}");
            }
        }
        Stacks::ReadStack => {
            println!("Read Stack");

            let mut stack = READ_STACK.lock().unwrap();
            while let Some(val) = stack.pop() {
                println!("Popped: {val}");
            }
        }
        Stacks::IgnoreStack => {
            println!("Ignore Stack");

            let mut stack = IGNORE_STACK.lock().unwrap();
            while let Some(val) = stack.pop() {
                println!("Popped: {val}");
            }
        }
        Stacks::FileStack => {
            println!("File Stack");

            let mut stack = FILE_STACK.lock().unwrap();
            while let Some(val) = stack.pop() {
                println!("Popped: {val}");
            }
        }
    }
}

// toggles and bool checks
fn toggle_bools(toggle: Toggles) {
    match toggle {
        Toggles::ShowHidden => {
            if let Ok(mut lock) = SHOW_HIDDEN.lock() {
                *lock = !*lock;
            }
        }
        Toggles::SearchDeep => {
            if let Ok(mut lock) = SEARCH_DEEP.lock() {
                *lock = !*lock;
            }
        }
        Toggles::ExplicitFile => {
            if let Ok(mut lock) = EXPLICIT_FILE.lock() {
                *lock = !*lock;
            }
        }
    }
}

pub fn is_show_hidden() -> bool {
    SHOW_HIDDEN.lock().map(|lock| *lock).unwrap_or(false)
}

pub fn is_search_deep() -> bool {
    SEARCH_DEEP.lock().map(|lock| *lock).unwrap_or(false)
}

pub fn is_search_explicit() -> bool {
    EXPLICIT_FILE.lock().map(|lock| *lock).unwrap_or(false)
}

// main
fn main() {
    #[warn(unused_variables)]
    let current_err = ERROR_BUF.lock().unwrap();

    let mut depth: i32 = 0;

    // Get arguments
    let mut args = std::env::args().skip(1);
    let mut check_dir = String::from(".");

    // Parse arguments
    while let Some(arg) = args.next() {
        if "--tui" == arg {
            tui::tui_view(&current_err);
        } else if "--dir" == arg {
            if let Some(dir) = args.next() {
                check_dir = dir;
            }
        } else if "--show_hidden" == arg {
            toggle_bools(Toggles::ShowHidden);
        } else if "--full" == arg {
            toggle_bools(Toggles::SearchDeep);
            // println!("{}", arg);
        } else if "--depth" == arg {
            toggle_bools(Toggles::SearchDeep);
            if let Some(deep) = args.next() {
                if let Ok(parsed) = deep.parse::<i32>() {
                    depth = parsed;
                }
            }
            println!("{}", arg);
        } else if "--explict" == arg {
            toggle_bools(Toggles::ExplicitFile);
        } else {
            let file: String = arg.clone();
            add_to_stack(file, Stacks::FileStack);
            continue;
        }
    }

    if let Ok(mut ignore) = GITIGNORE.lock() {
        *ignore = std::path::PathBuf::from(&check_dir).join(".gitignore");
    }

    // get files/folders in current directory
    let path = &Path::new(&check_dir);
    git::check_directory(path, depth);

    if let Err(err) = search::search() {
        eprint!("{}", &err.to_string());
    }

    free_stacks();
    return;
}

fn free_stacks() {
    release_stacks(Stacks::CheckStack);
    release_stacks(Stacks::ReadStack);
    release_stacks(Stacks::IgnoreStack);
    release_stacks(Stacks::FileStack);
}

// fn set_major_error(msg: &str) {
//     eprintln!("error -> {}", msg);
//     free_stacks();
// }
