use std::io;
use red;


fn main() {
    let mut command = String::new();
    let mut buffer: Vec<String> = Vec::new();
    let mut running = true;
    let mut tried_to_exit = false;
    let mut modified = false;
    // let mut verbose = false;

    while running {
        io::stdin()
            .read_line(&mut command)
            .expect("Failed!");

        // remove the new line character at the end
        command.pop();
        if command.as_str() == "q" {
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
                "a" => modified = red::utils::edit(&mut buffer),
                // print lines
                "p" => {
                    if buffer.is_empty() {
                        println!("?");
                    } else {
                    red::utils::print_lines((buffer.len() - 1)
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
                        let num_bytes = red::io::write_to_file("test", &buffer);
                        // print number of bytes saved.
                        println!("{num_bytes}");
                    }
                }
                _ => println!("?"),
            }
        }
        command = "".to_string();
    }
}
