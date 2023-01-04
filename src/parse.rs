#[derive(PartialEq, Eq)]
pub enum InstructionType {
    Append,
    Write,
    Print,
    Quit,
    None,
}

pub struct Command {
    pub instruction: InstructionType,
    pub start: usize,
    pub amount: usize,
    pub operand: String,
}


pub fn parse_command(cmd: &String) -> Command {
    // how the parser works:
    // read the command and push it into a string
    // when it hits a letter, stop pushing to the string.
    // this above will be the range of the command.
    // -> for example: 5,10p prints lines 5 to 10.
    // the first letter it encounters will be the
    // instruction.
    // -> for example, w, p, a, c.
    // the write command takes an operand.
    let mut first_part = String::new();
    let mut tempstring = String::new();
    let mut is_operand = false;
    let mut instr: InstructionType = InstructionType::None;
    for letter in cmd.chars() {
        if (letter != ' ')
        & !letter.is_alphabetic()
        & !is_operand {
            tempstring.push(letter);
        } else if letter.is_alphabetic()
        & !is_operand {
            is_operand = true;
            first_part = tempstring.to_string();
            instr = {
                match letter {
                    'q' => InstructionType::Quit,
                    'p' => InstructionType::Print,
                    'w' => InstructionType::Write,
                    'a' => InstructionType::Append,
                    _ => InstructionType::None,
                }
            };
            tempstring = "".to_string();
        } else if is_operand
        & (letter != ' ') {
            tempstring.push(letter);
        }
    }

    let mut start: usize = 0;
    let mut amount: usize = 0;

    if first_part != "" {
        let ranges: Vec<&str> = first_part.split(",").collect();
        dbg!(&ranges);
        start = {
            if ranges[0] != "" {
                ranges[0].parse::<usize>().unwrap()
            } else {
                1
            }
        };

        let end: usize = {
            if ranges.len() < 2 {
                start
            } else {
                if ranges[1] != "" {
                    ranges[1].parse::<usize>().unwrap()
                } else {
                    0
                }
            }
        };
        // dbg!(end);

        amount = {
            if start < end {
                end - start + 1
            } else if start == end {
                1
            } else {
                0
            }
        }
    }

    // dbg!(start);
    // dbg!(amount);

    if (instr == InstructionType::Append)
    | (instr == InstructionType::Quit) {
        tempstring = "".to_string();
    }

    let parsed_instruction = Command {
        instruction: instr,
        start: start,
        amount: amount,
        operand: tempstring,
    };
    parsed_instruction
}
