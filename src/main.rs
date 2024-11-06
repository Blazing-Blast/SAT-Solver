mod condition;
mod tokenizer;

use tokenizer::Tokenizer;
fn main() {
    let tokenizer = Tokenizer::new("alpha | beta & !gamma".to_string());
    let mut state = tokenizer.to_state();
    println!("Solution: {}", state.solve());
}
