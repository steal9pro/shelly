#[allow(unused_imports)]
use std::process;
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
        let (cmd, args) = parse_command(&input);

        if cmd == "exit" {
            process::exit(0x0100);
        }

        not_found(input)
    }
}

fn wait_input(input: &mut String) {
    // Wait for user input
    let stdin = io::stdin();
    stdin.read_line(input).unwrap();
}

fn parse_command(input: &String) -> (&str, &str) {
   let mut parts = input.split(" "); 

   let cmd = parts.next().unwrap();

   let arg = parts.next().unwrap_or("");

   return (cmd, arg)
}

fn not_found(cmd_name: String) {
    let len = cmd_name.len();
    let mut cmd_name = cmd_name;

    cmd_name.truncate(len - 1);
    print!("{}: command not found\n", cmd_name);
    io::stdout().flush().unwrap()
}
