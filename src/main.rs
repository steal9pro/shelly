use crate::repl::Repl;

pub mod config;
pub mod repl;

fn main() {
    let repl = Repl::build();

    repl.start()
}
