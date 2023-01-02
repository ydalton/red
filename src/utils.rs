use std::io;
use std::convert::TryFrom;

const NAME: &str = "red";

pub fn help() {
    println!(
            "Usage: {NAME} [options] [file]\n\
            Options:\n\
            \t-h, --help                  display this help and exit\n \
            \t-h, --help                  display this help and exit\n
            ");
}

pub fn edit(buffer: &mut Vec<String>) -> bool {
    let mut line = String::new();
    let mut in_loop = true;
    let mut changed = false;

    while in_loop {
        io::stdin()
            .read_line(&mut line)
            .expect("Failed!");
        // get rid of the newline character
        line.pop();
        if line.eq(".") {
            in_loop = false;
        } else {
            let _ = &buffer.push(line);
            changed = true;
        }
        line = "".to_string();
    }

    changed
}

pub fn print_lines(start_number: u32, amount: u32, buffer: &Vec<String>) {
    if (start_number + amount) > buffer.len()
                                        .try_into()
                                        .unwrap() {
        panic!("Too large!");
    }

    for n in start_number..(start_number+amount) {
        let n_us = usize::try_from(n).unwrap();
        println!("{}", &buffer[n_us]);
    }
}
