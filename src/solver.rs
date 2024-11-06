#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    Or {
        l: Box<Condition>,
        r: Box<Condition>,
    },
    Not {
        a: Box<Condition>,
    },
    Variable {
        index: usize,
    },
    Constant {
        b: bool,
    },
}

#[derive(Debug)]
pub enum SolveState {
    Solved {
        vars: Vec<Option<bool>>,
        ids: Vec<String>,
    },
    Unsolvable,
}

#[derive(Debug, Clone)]
pub struct State {
    vars: Vec<Option<bool>>,
    conditions: Vec<Condition>,
    ids: Vec<String>,
}

impl Condition {
    #[allow(dead_code)]
    fn evaluate(&self, state: &State) -> Option<bool> {
        match self {
            Condition::Or { l, r } => match (l.evaluate(state), r.evaluate(state)) {
                (Some(a), Some(b)) => Some(a || b),
                // We only need to know one side to be true to know the whole OR is true.
                (Some(x), None) => {
                    if x {
                        Some(x)
                    } else {
                        None
                    }
                }
                (None, Some(x)) => {
                    if x {
                        Some(x)
                    } else {
                        None
                    }
                }
                (None, None) => None,
            },
            Condition::Not { a } => match a.evaluate(state) {
                Some(x) => Some(!x),
                None => None,
            },
            Condition::Variable { index } => state.get(*index),
            Condition::Constant { b } => Some(*b),
        }
    }
    fn simplify(&mut self, state: &State) -> Option<bool> {
        let out: Option<bool> = match self {
            Condition::Or { l, r } => match (l.simplify(state), r.simplify(state)) {
                (Some(a), Some(b)) => Some(a || b),
                // We only need to know one side to be true to know the whole OR is true.
                (Some(x), None) => {
                    if x {
                        Some(x)
                    } else {
                        *self = (**r).clone();
                        None
                    }
                }
                (None, Some(x)) => {
                    if x {
                        Some(x)
                    } else {
                        *self = (**l).clone();
                        None
                    }
                }
                (None, None) => None,
            },
            Condition::Not { a } => match a.simplify(state) {
                Some(x) => Some(!x),
                None => None,
            },
            Condition::Variable { index } => state.get(*index),
            Condition::Constant { b } => Some(*b),
        };
        match out {
            Some(x) => {
                *self = Condition::Constant { b: x };
                Some(x)
            }
            None => None,
        }
    }
    fn simplify_fully(&mut self, state: &State) -> Option<bool> {
        let mut new: Self = self.clone();
        let mut out;
        loop {
            out = new.simplify(state);
            if new == *self {
                break;
            }
            *self = new.clone();
        }
        out
    }

    fn find_first_var(&self) -> usize {
        match self {
            Condition::Or { l, .. } => l.find_first_var(),
            Condition::Not { a } => a.find_first_var(),
            Condition::Variable { index } => *index,
            Condition::Constant { .. } => panic!("Unreachable code reached"),
        }
    }

    fn degree(&self) -> usize {
        match self {
            Condition::Or { l, r } => l.degree() + r.degree(),
            Condition::Not { a } => a.degree(),
            Condition::Variable { .. } => 1,
            Condition::Constant { .. } => 0,
        }
    }
}

impl std::fmt::Display for SolveState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SolveState::Solved { vars, ids } => {
                    let mut out: String = String::new();
                    for i in 0..vars.len() {
                        out += format!(
                            "\n{}: {}",
                            ids[i],
                            match vars[i] {
                                Some(true) => "true",
                                Some(false) => "false",
                                None => "free",
                            }
                        )
                        .as_str();
                    }
                    out
                }
                SolveState::Unsolvable => "Unsolvable".to_string(),
            }
        )
    }
}

impl State {
    fn get(&self, index: usize) -> Option<bool> {
        self.vars[index]
    }

    pub fn simplify(&mut self) -> Option<bool> {
        let mut new_conditions = self.conditions.clone();
        let mut out = Some(true);
        for cond in &mut new_conditions {
            match cond.simplify_fully(&self) {
                Some(b) => {
                    if !b {
                        return Some(false);
                    }
                }
                None => out = None,
            }
        }
        new_conditions.retain(|x| *x != Condition::Constant { b: true });
        new_conditions.dedup();
        new_conditions.sort_by(|x, y| x.degree().cmp(&y.degree()));
        self.conditions = new_conditions;
        out
    }

    pub fn new(conditions: Vec<Condition>, ids: Vec<String>) -> Self {
        Self {
            vars: vec![None; ids.len()],
            conditions,
            ids,
        }
    }

    pub fn solve(&mut self) -> SolveState {
        let before = self.conditions.clone();
        match self.simplify() {
            Some(x) => {
                if x {
                    SolveState::Solved {
                        vars: self.vars.to_vec(),
                        ids: self.ids.clone(),
                    }
                } else {
                    SolveState::Unsolvable
                }
            }
            None => {
                if !self.vars.contains(&None) {
                    SolveState::Unsolvable
                } else {
                    let index = self.conditions[0].find_first_var();
                    self.vars[index] = Some(true);
                    match self.solve() {
                        SolveState::Solved { vars, ids } => {
                            return SolveState::Solved { vars, ids }
                        }
                        SolveState::Unsolvable => (),
                    };
                    self.vars[index] = Some(false);
                    let out = match self.solve() {
                        SolveState::Solved { vars, ids } => SolveState::Solved { vars, ids },
                        SolveState::Unsolvable => SolveState::Unsolvable,
                        // SolveState::Undetermined => SolveState::Undetermined,
                    };
                    // Backtracking
                    self.vars[0] = None;
                    self.conditions = before;
                    out
                }
            }
        }
    }
}
