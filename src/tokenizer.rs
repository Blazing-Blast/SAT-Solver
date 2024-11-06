use std::collections::VecDeque;
use Token::*;

use crate::solver::{Condition, State};
#[derive(Debug, Clone, Copy)]
enum Token {
    Open,
    Close,
    And,
    Or,
    Not,
    Identifier { index: usize },
    Literal { value: bool },
}

pub(crate) struct Tokenizer {
    stream: VecDeque<char>,
    identifiers: Vec<String>,
    peek_buffer: Option<Token>,
}

impl Tokenizer {
    pub fn new(string: String) -> Tokenizer {
        Self {
            stream: VecDeque::from(string.chars().collect::<Vec<char>>()),
            identifiers: Vec::new(),
            peek_buffer: None,
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.stream.is_empty() {
            return None;
        }
        Some(self.stream[0])
    }

    fn next_char(&mut self) -> Option<char> {
        self.stream.pop_front()
    }

    fn commit_char(&mut self) {
        _ = self
            .stream
            .pop_front()
            .expect("Cannot commit an empty stream")
    }

    fn resolve_identifier(&mut self, id: String) -> Token {
        Token::Identifier {
            index: match self.identifiers.iter().position(|x| *x == *id) {
                Some(i) => i,
                None => {
                    self.identifiers.push(id);
                    self.identifiers.len() - 1
                }
            },
        }
    }

    fn create_token(&mut self) -> Option<Token> {
        let c: char;
        loop {
            c = match self.next_char() {
                Some(x) => {
                    if x.is_whitespace() {
                        continue;
                    } else {
                        x
                    }
                }
                None => return None,
            };
            break;
        }
        match c {
            '(' => Some(Open),
            ')' => Some(Close),
            '|' | '∨' => Some(Or),
            '&' | '∧' => Some(And),
            '!' | '¬' => Some(Not),
            '1' => Some(Literal { value: true }),
            '0' => Some(Literal { value: false }),
            x => {
                let mut id: String = x.to_string();
                loop {
                    let c = match self.peek_char() {
                        Some(x) => x,
                        None => break,
                    };
                    match c {
                        '(' | ')' | '!' | '¬' | '|' | '∨' | '&' | '∧' => break,
                        x if x.is_whitespace() => break,
                        x => {
                            self.commit_char();
                            id.push(x);
                        }
                    }
                }
                Some(self.resolve_identifier(id))
            }
        }
    }
    fn peek_token(&mut self) -> Option<Token> {
        match self.peek_buffer {
            Some(t) => Some(t),
            None => {
                self.peek_buffer = self.create_token();
                self.peek_buffer
            }
        }
    }

    fn commit_token(&mut self) {
        self.peek_buffer = None
    }

    fn next_token(&mut self) -> Option<Token> {
        let out = self.peek_token();
        self.commit_token();
        out
    }

    fn next_condition(&mut self) -> Option<Condition> {
        let token = match self.next_token() {
            Some(x) => x,
            None => return None,
        };
        let left = match token {
            Open => match self.next_condition() {
                Some(x) => match self.next_token() {
                    Some(Token::Close) => x,
                    _ => panic!("Unclosed parenthesis"),
                },
                None => return None,
            },
            Close => panic!("Syntax error: expression starts with Closing"),
            And => return None, // Consume the AND at the start of an expression
            Or => panic!("Syntax error: Or without left operand"),
            Not => match self.next_token() {
                Some(Identifier { index }) => Condition::Not {
                    a: Box::new(Condition::Variable { index }),
                },
                None => panic!("Syntax error: expected variable, got EOF"),
                Some(e) => panic!("Syntax error: expected variable, got {:?}", e),
            },
            Identifier { index } => Condition::Variable { index },
            Literal { value } => Condition::Constant { b: value },
        };
        match self.peek_token() {
            Some(t) => match t {
                And | Close => Some(left),
                Or => {
                    self.commit_token();
                    match self.next_condition() {
                        Some(right) => Some(Condition::Or {
                            l: Box::new(left),
                            r: Box::new(right),
                        }),

                        None => panic!("Syntax error: Or without right operand"),
                    }
                }
                x => panic!("Syntax error: Expected Operator, found {:?}", x),
            },
            None => Some(left),
        }
    }

    fn get_conditions(&mut self) -> Vec<Condition> {
        let mut out = Vec::new();
        loop {
            match self.next_condition() {
                Some(c) => out.push(c),
                None => match self.peek_token() {
                    Some(And) => panic!("Syntax error: Expected expression, got AND"),
                    None => break,
                    Some(_) => continue,
                },
            }
        }
        out
    }

    pub fn to_state(mut self) -> State {
        let conditions = self.get_conditions();
        State::new(conditions, self.identifiers)
    }
}
