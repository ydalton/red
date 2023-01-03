use std::env;
use std::process;

mod io;
mod parse;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    // check if there is more than 1 argument.
    if args.len() > 1 {
        for arg in args {
            match arg.as_str() {
                "-h" | "--help" | _ => {
                    utils::help();
                    process::exit(0);
                }
            }
        }
    }


    let mut command = String::new();
    let mut buffer: Vec<String> = Vec::new();
    let mut running = true;
    let mut tried_to_exit = false;
    let mut modified = false;
    // let mut verbose = false;

    while running {
        std::io::stdin()
            .read_line(&mut command)
            .expect("Failed!");

        // remove the new line character at the end
        command.pop();
        parse::parse_command(&command);
        if command == "q" {
            if buffer.is_empty()
                | tried_to_exit
                | !modified {
                running = false;
            } else {
                tried_to_exit = true;
                println!("?");
            }
        } else {
            tried_to_exit = false;
            match command.as_str() {
                // append to buffer
                "a" => modified = utils::edit(&mut buffer),
                // print lines
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
                // write to file
                "w" => {
                    if buffer.is_empty() {
                        println!("?");
                    } else {
                        modified = false;
                        let num_bytes = io::write_to_file("test", &buffer);
                        // print number of bytes saved.
                        println!("{num_bytes}");
                    }
                }
                "!" => {
                    process::Command::new("sh -c");
                }
                _ => println!("?"),
            }
        }
        // clear string
        command = "".to_string();
    }
}
