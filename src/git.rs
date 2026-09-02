use std::fs::{File, read_dir};
use std::io::{BufReader, prelude::*};
use std::path::*;

use crate::add_to_stack;
use crate::is_show_hidden;
use crate::{CHECK_STACK, GITIGNORE, IGNORE_STACK, Stacks};
// use crate::set_error;

pub fn read_gitignore() {
    let path = GITIGNORE.lock().unwrap();
    let file = File::open(&*path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let trimmed = line.trim().trim_matches('\u{feff}');

        // skip empty lines and gitignore comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // clean leading slashes so "/target" becomes "target"
        let cleaned = trimmed
            .trim_start_matches('/')
            .trim_start_matches("./")
            .to_string();

        add_to_stack(cleaned, Stacks::IgnoreStack);
    }
}

pub fn check_directory(path: &Path, _depth: i32) {
    for file in read_dir(path).unwrap() {
        let entry = file.unwrap();
        let filename = entry.file_name().to_string_lossy().to_string();

        if filename == ".gitignore" {
            read_gitignore();
            continue;
        } else if filename.starts_with('.') {
            if !is_show_hidden() {
                continue;
            }
        }

        add_to_stack(filename, Stacks::CheckStack);
    }

    let ignore_stack = IGNORE_STACK.lock().unwrap();
    let check_stack = CHECK_STACK.lock().unwrap();

    for ignore_file in ignore_stack.iter() {
        for check_file in check_stack.iter() {
            // println!("ignore: {} -> check: {}", ignore_file, check_file);
            if ignore_file == check_file {
                // println!("check: {} ->  ignore", ignore_file);
                continue;
            } else {
                // println!("check: {} ->  no ignore", check_file);
                add_to_stack(check_file.clone(), Stacks::ReadStack);
                continue;
            }
        }
    }
}
