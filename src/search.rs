use std::io::Result;

use crate::{CHECK_STACK, FILE_STACK, is_search_explicit};

pub fn search() -> Result<()> {
    println!("searching...");

    // List out all files needing to be found
    // {
    //     let stack = FILE_STACK.lock().unwrap();
    //     for file in stack.iter() {
    //         println!("seaching for -> {}", file);
    //     }
    // }

    // List of files being checked
    // {
    //     let stack = CHECK_STACK.lock().unwrap();
    //     for file in stack.iter() {
    //         println!("seaching through -> {}", file);
    //     }
    // }

    let file_stack = FILE_STACK.lock().unwrap();
    let check_stack = CHECK_STACK.lock().unwrap();

    for file in file_stack.iter() {
        for check_file in check_stack.iter() {
            if file == check_file {
                println!("found {}", file);
            } else if is_search_explicit() {
                if check_file.contains(file) {
                    println!("found {}", check_file);
                }
            } else {
                continue;
            }
        }
    }

    Ok(())
}
