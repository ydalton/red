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

pub fn print_lines(start_number: usize, amount: usize, buffer: &Vec<String>) {
    if (start_number + amount) > buffer.len() {
        panic!("Too large!");
    }

    for n in start_number..(start_number+amount) {
        println!("{}", &buffer[n]);
    }
}
