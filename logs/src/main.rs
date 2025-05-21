use std::fs;

fn main() {
    let text = fs::read_to_string("logs.txt").expect("Failed to read file");

    let lines: Vec<&str> = text.split("\n").collect();

    for line in lines {
        println!("{}", line);
    }
}
