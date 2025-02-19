use crate::config::Config;
use std::ffi::{OsStr, OsString};
use std::io::{self, Write};
use std::os::unix::ffi::OsStrExt;
use std::process::{exit, Command};

pub struct Repl {
    config: Config,
}

impl Repl {
    pub fn build() -> Self {
        let config = Config::build();

        Repl { config }
    }

    pub fn start(&mut self) {
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
                    "type" => self.type_fn(args),
                    "pwd" => self.pwd(),
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
    fn lauch<T>(&mut self, cmd: String, args: T)
    where
        T: IntoIterator,
        T::Item: AsRef<OsStr>,
    {
        let args: Vec<_> = args
            .into_iter()
            .map(|a| a.as_ref().to_os_string())
            .collect();

        if which::which(&cmd).is_err() {
            eprintln!("{cmd}: command not found");
            return;
        }

        let output = Command::new(cmd)
            .args(args)
            .output()
            .expect("command to start");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    fn type_fn(&mut self, args: Vec<String>) {
        self.config.scan_binary().unwrap();

        let cmd_name = args[0].clone();
        match self.config.check_binary(&cmd_name) {
            Some(path) => println!("{cmd_name} is {path}"),
            None => println!("{}: not found", cmd_name),
        }
    }

    fn echo(&self, args: Vec<String>) {
        let line = args.join(" ");
        println!("{line}")
    }

    fn pwd(&self) {
        let path = std::env::current_dir().unwrap();
        let os_str = path.as_os_str().to_str().unwrap();

        let new_line_str = "\n".to_string();
        let str_list = vec![os_str, new_line_str.as_str()];

        let res = str_list.join("");

        io::stdout().write_all(res.as_bytes()).unwrap()
    }

    fn not_found(&self, cmd_name: String) {
        let len = cmd_name.len();
        let mut cmd_name = cmd_name;

        cmd_name.truncate(len - 1);
        print!("{}: command not found\n", cmd_name);
        io::stdout().flush().unwrap()
    }
}
