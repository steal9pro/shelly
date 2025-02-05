use crate::repl::Repl;

pub mod config;
pub mod repl;

fn main() {
    let mut repl = Repl::build();

    repl.start()
}
