use std::io;
use red::utils::*;

fn main() {
    let mut command = String::new();
    let mut buffer: Vec<String> = Vec::new();
    let mut running = true;
    let mut tried_to_exit = false;

    while running {
        io::stdin()
            .read_line(&mut command)
            .expect("Failed!");

        // remove the new line character at the end
        command.pop();
        if command.as_str() == "q" {
            if buffer.is_empty()
                | tried_to_exit {
                running = false;
            } else {
                tried_to_exit = true;
                println!("?");
            }
        } else {
            tried_to_exit = false;
            match command.as_str() {
                "a" => utils::edit(&mut buffer),
                "p" => {
                    if buffer.is_empty() {
                        println!("?");
                    } else {
                    utils::print_lines((buffer.len() - 1)
                                        .try_into()
                                        .unwrap(),
                                        1,
                                        &buffer)
                    }
                },
                _ => println!("?"),
            }
        }
        command = "".to_string();
    }
}
