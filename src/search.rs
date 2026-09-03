use std::io::Result;

use crate::{CHECK_STACK, FILE_STACK, is_search_explicit, verbose_println};

pub fn search() -> Result<()> {
    // List out all files needing to be found
    {
        let stack = FILE_STACK.lock().unwrap();
        for file in stack.iter() {
            verbose_println("list_files", file.to_string());
        }
    }

    // List of files being checked
    {
        let stack = CHECK_STACK.lock().unwrap();
        for file in stack.iter() {
            let msg: &str = file.as_str();
            verbose_println("list_files", msg.to_string());
        }
    }

    let file_stack = FILE_STACK.lock().unwrap();
    let check_stack = CHECK_STACK.lock().unwrap();

    for file in file_stack.iter() {
        for check_file in check_stack.iter() {
            if is_search_explicit() {
                if check_file == file {
                    println!("found {}", check_file);
                }
            } else {
                if file == check_file {
                    println!("found {}", file);
                }
                if check_file.contains(file) {
                    println!("found {}", check_file);
                } else {
                    continue;
                }
            }
        }
    }

    Ok(())
}
