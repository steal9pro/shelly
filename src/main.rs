#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    run_loop()
}

fn run_loop() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        wait_input(&mut input);
        not_found(input)
    }
}

fn wait_input(input: &mut String) {
    // Wait for user input
    let stdin = io::stdin();
    stdin.read_line(input).unwrap();
}

fn not_found(cmd_name: String) {
    let len = cmd_name.len();
    let mut cmd_name = cmd_name;

    cmd_name.truncate(len - 1);
    print!("{}: command not found\n", cmd_name);
    io::stdout().flush().unwrap()
}
