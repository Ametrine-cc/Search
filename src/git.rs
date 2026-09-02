use std::fs::{File, read_dir};
use std::io::{BufReader, prelude::*};
use std::path::*;

use crate::Stacks;
use crate::add_to_stack;
use crate::is_show_hidden;
// use crate::set_error;

pub fn read_gitignore() {
    let file = File::open(".gitignore").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        add_to_stack(line, Stacks::IgnoreStack);
    }
}

pub fn check_file(path: &Path) {
    for file in read_dir(path).unwrap() {
        let filename: String = file.unwrap().file_name().to_string_lossy().into_owned();

        if filename == ".gitignore" {
            println!("found gitignore {:#?}", filename);
            read_gitignore();
        } else if filename.starts_with(".") {
            if is_show_hidden() {
                println!("starts with . : {}", filename);
            } else {
                continue;
            }
        } else {
            println!("{:#?}", filename);
            add_to_stack(filename, Stacks::CheckStack);
        }
    }
}
