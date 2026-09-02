use std::io::Result;

use crate::{CHECK_STACK, FILE_STACK};

pub fn search() -> Result<()> {
    println!("searching...");

    // List out all files needing to be found
    {
        let stack = FILE_STACK.lock().unwrap();
        for file in stack.iter() {
            println!("seaching for -> {}", file);
        }
    }

    // List of files being checked
    {
        let stack = CHECK_STACK.lock().unwrap();
        for file in stack.iter() {
            println!("seaching through -> {}", file);
        }
    }

    Ok(())
}
