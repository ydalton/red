use std::io;

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
        if line == ".".to_string() {
            in_loop = false;
        } else {
            let _ = &buffer.push(line);
            changed = true;
        }
        line = "".to_string();
    }
    changed
}

pub fn print_lines(start_number: usize,
                   amount: usize,
                   buffer: &Vec<String>) -> bool {
    let end_number = {
        if amount == 0 {
            buffer.len()
        } else {
            start_number + amount - 1
        }
    };

    if (start_number > buffer.len() + 1)
        | (end_number > buffer.len()) {
        return false;
    }

    for n in (start_number - 1)..end_number {
        println!("{}", &buffer[n]);
    }
    return true;
}
