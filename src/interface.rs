use std::{fs::read_to_string, path::Path};

use crate::{solver::SolveState, tokenizer::Tokenizer};

pub fn solve_string(string: String) -> SolveState {
    let tokenizer = Tokenizer::new(string);
    let mut state = tokenizer.to_state();
    state.solve()
}

#[allow(dead_code)]
pub fn solve_str(string: &str) -> SolveState {
    solve_string(string.to_string())
}

pub fn solve_file<T>(name: T) -> SolveState
where
    T: AsRef<Path>,
{
    let string = read_to_string(name);
    solve_string(string.expect("Cannot open file"))
}
