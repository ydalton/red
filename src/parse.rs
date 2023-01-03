enum InstructionType {
    Write,
    Print,
    Quit,
}

struct Command {
    instruction: InstructionType,
    start: usize,
    amount: usize,
    operand: String,
}


pub fn parse_command(cmd: &String) {
    let old_string: Vec<&str> = cmd.split(' ').collect();
    dbg!(&old_string);
    let mut new_string: Vec<&str> = Vec::new();
    for word in old_string {
        if word != "" {
            new_string.push(word);
        }
    }
}