use std::io::{self, Write};
#[allow(unused_imports)]
use std::process;

fn main() {
    repl()
}

fn repl() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        wait_input(&mut input);
        let (cmd, args) = parse_command(&input);

        match cmd {
            "exit" => process::exit(0x0100),
            "echo" => echo(args),
            _ => not_found(input),
        }
    }
}

fn wait_input(input: &mut String) {
    // Wait for user input
    let stdin = io::stdin();
    stdin.read_line(input).unwrap();
}

fn parse_command(input: &String) -> (&str, &str) {
    let input = input.trim();

    let (cmd, args) = match input.split_once(" ") {
        Some(res) => res,
        None => {
            return (input, "");
        }
    };

    return (cmd, args);
}

fn echo(args: &str) {
    println!("{args}")
}

fn not_found(cmd_name: String) {
    let len = cmd_name.len();
    let mut cmd_name = cmd_name;

    cmd_name.truncate(len - 1);
    print!("{}: command not found\n", cmd_name);
    io::stdout().flush().unwrap()
}
