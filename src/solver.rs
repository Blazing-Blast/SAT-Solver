use bit_set::BitSet;

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
        vars: BitSet,
        len: usize,
        ids: Vec<String>,
    },
    Unsolvable,
    // Undetermined,
}

#[derive(Debug, Clone)]
pub struct State {
    vars: BitSet,
    conditions: Vec<Condition>,
    // The index from where the undetermined bits start
    cutoff: usize,
    length: usize,
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
}

impl std::fmt::Display for SolveState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SolveState::Solved { vars, len, ids } => {
                    let mut out: String = String::new();
                    for i in 0..*len {
                        out += format!("\n{}: {}", ids[i], vars.contains(i)).as_str();
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
        if index >= self.cutoff {
            return None;
        }
        Some(self.vars.contains(index))
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
        self.conditions = new_conditions;
        out
    }

    pub fn new(var_num: usize, conditions: Vec<Condition>, ids: Vec<String>) -> Self {
        Self {
            vars: BitSet::with_capacity(var_num),
            conditions,
            cutoff: 0,
            length: var_num,
            ids,
        }
    }

    pub fn solve(&mut self) -> SolveState {
        let before = self.clone();
        match self.simplify() {
            Some(x) => {
                if x {
                    SolveState::Solved {
                        vars: self.vars.clone(),
                        len: self.length,
                        ids: self.ids.clone(),
                    }
                } else {
                    *self = before;
                    SolveState::Unsolvable
                }
            }
            None => {
                if self.cutoff >= self.length {
                    SolveState::Unsolvable
                } else {
                    self.cutoff += 1;
                    self.vars.insert(self.cutoff - 1);
                    match self.solve() {
                        SolveState::Solved { vars, len, ids } => {
                            return SolveState::Solved { vars, len, ids }
                        }
                        SolveState::Unsolvable => (),
                    };
                    self.vars.remove(self.cutoff - 1);
                    let out = match self.solve() {
                        SolveState::Solved { vars, len, ids } => {
                            SolveState::Solved { vars, len, ids }
                        }
                        SolveState::Unsolvable => SolveState::Unsolvable,
                        // SolveState::Undetermined => SolveState::Undetermined,
                    };
                    *self = before; //backtracking
                    out
                }
            }
        }
    }
}
