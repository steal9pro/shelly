use crate::config::Config;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::process::{exit, Command};

pub struct Repl {
    config: Config,
}

impl Repl {
    pub fn build() -> Self {
        let config = Config::build();

        Repl { config }
    }

    pub fn start(&self) {
        loop {
            print!("$ ");

            if io::stdout().flush().is_err() {
                eprintln!("Error flushing stdout");
                continue;
            }

            let mut input = String::new();
            self.wait_input(&mut input);

            match self.parse_command(&input) {
                Ok((cmd, args)) => match cmd.as_str() {
                    "exit" => {
                        exit(0);
                    }
                    "echo" => self.echo(args),
                    "type" => self.type_fn(cmd),
                    _ => self.lauch(cmd, args),
                    // _ => self.not_found(input),
                },
                Err(err) => eprintln!("{err}"),
            }
        }
    }

    fn wait_input(&self, input: &mut String) {
        // Wait for user input
        let stdin = io::stdin();
        stdin.read_line(input).unwrap();
    }

    fn parse_command(&self, input: &str) -> Result<(String, Vec<String>), String> {
        let input = input.trim();

        let (cmd, args) = match input.split_once(" ") {
            Some((cmd, args)) => (cmd.to_string(), args.to_string()),
            None => (input.to_string(), String::new()),
        };
        let args: Vec<String> = args.split(" ").map(|s| s.to_string()).collect();

        Ok((cmd, args))
    }

    fn lauch<T>(&self, cmd: String, args: T)
    where
        T: IntoIterator,
        T::Item: AsRef<OsStr>,
    {
        Command::new(cmd)
            .args(args)
            .spawn()
            .expect("command to start");
    }

    fn type_fn(&self, arg: String) {
        match self.config.check_binary(&arg) {
            Some(path) => println!("{arg} is {path}"),
            None => println!("{}: not found", arg),
        }
    }

    fn echo(&self, args: Vec<String>) {
        println!("{:#?}", args)
    }

    fn not_found(&self, cmd_name: String) {
        let len = cmd_name.len();
        let mut cmd_name = cmd_name;

        cmd_name.truncate(len - 1);
        print!("{}: command not found\n", cmd_name);
        io::stdout().flush().unwrap()
    }
}
