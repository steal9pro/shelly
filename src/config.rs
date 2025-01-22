use std::collections::HashSet;
use std::fs::{self};
use std::path::Path;
use std::{env, io};

pub struct Config {
    paths: Vec<String>,
    binaries: HashSet<String>,
}

impl Config {
    pub fn build() -> Self {
        let path = env::var("PATH").unwrap_or_default();
        let paths: Vec<String> = path.split(':').map(|s| s.to_string()).collect();

        let mut config = Config {
            paths,
            binaries: HashSet::new(),
        };
        config.scan_binary();

        config
    }

    fn scan_binary(&mut self) -> io::Result<()> {
        for p in &self.paths {
            let path = Path::new(p);

            if !path.is_dir() {
                eprintln!("Skipping invalid or inaccessible path: {}", p);
                continue;
            }

            match fs::read_dir(p) {
                Ok(entries) => {
                    for entry in entries {
                        let entry = entry?;
                        if let Some(file_name) = entry.file_name().to_str() {
                            self.binaries.insert(file_name.to_string());
                        }
                    }
                }
                Err(err) => eprintln!("Failed to read directory {}: {}", p, err),
            }
        }

        println!("binary len: {:#?}", self.binaries.len());

        Ok(())
    }

    pub fn check_binary(&self, search: &String) -> Option<bool> {
        if self.binaries.contains(search) {
            Some(true)
        } else {
            None
        }
    }
}
