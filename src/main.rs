mod interface;
mod solver;
mod tokenizer;

use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

use interface::solve_file;
fn main() {
    let mut paths: Vec<PathBuf> = fs::read_dir("./examples/")
        .unwrap()
        .map(|x| x.unwrap().path())
        .collect();
    paths.sort();
    for path in paths {
        println!("\n{}: {}", read_to_string(&path).unwrap(), solve_file(path));
    }
    // let mut state = tokenizer::Tokenizer::new("!a | !b & b".to_string()).to_state();
    // dbg!(&state);
    // println!("Solution: {}", state.solve());
}
