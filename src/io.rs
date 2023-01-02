use std::{fs::File, io::Write};
pub fn write_to_file(file_name: &str, buffer: &Vec<String>) -> u32 {
    let mut to_be_written = String::new();
    let mut bytes: u32 = 0;

    for line in buffer {
        for character in line.chars() {
            to_be_written.push(character);
            bytes += 1;
        }
        to_be_written.push('\n');
        bytes += 1;
    }

    let mut file = File::create(file_name).expect("Unable to open file!");
    file.write_all(to_be_written.as_str().as_bytes()).expect("Unable to write to file!");

    bytes
}