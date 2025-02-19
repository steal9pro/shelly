use std::collections::{HashMap, HashSet};
use std::fs::{self};
use std::path::Path;
use std::{env, io};

pub struct Config {
    paths: Vec<String>,
    binaries: HashMap<String, String>,
    buildin_binaries: HashSet<String>,
}

impl Config {
    pub fn build() -> Self {
        let path = env::var("PATH").unwrap_or_default();
        let paths: Vec<String> = path.split(':').map(|s| s.to_string()).collect();

        let mut config = Config {
            paths,
            binaries: HashMap::new(),
            buildin_binaries: HashSet::new(),
        };

        config.fill_buildin();
        config.scan_binary();

        config
    }

    fn fill_buildin(&mut self) {
        self.buildin_binaries.insert("echo".to_string());
        self.buildin_binaries.insert("type".to_string());
        self.buildin_binaries.insert("pwd".to_string());
        self.buildin_binaries.insert("exit".to_string());
    }

    pub fn scan_binary(&mut self) -> io::Result<()> {
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
                            let file_name = file_name.to_string();
                            if !self.binaries.contains_key(&file_name) {
                                let path = entry.path().to_str().unwrap().to_string();
                                self.binaries.insert(file_name.to_string(), path);
                            }
                        }
                    }
                }
                Err(err) => eprintln!("Failed to read directory {}: {}", p, err),
            }
        }

        Ok(())
    }

    pub fn check_binary(&self, search: &String) -> Option<String> {
        if self.buildin_binaries.contains(search) {
            return Some("a shell builtin".to_string());
        }
        self.binaries.get(search).cloned()
    }
}
