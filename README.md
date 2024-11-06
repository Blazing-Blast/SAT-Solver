# SAT-Solver

> A naive SAT-solver written in Rust.

## Usage

Place all the formulas you want solved in the `examples/` directory, and run `cargo run`.

## Syntax

The parser recognizes the following symbols:

- `( )` Order of operations
- `0 1` Truth values
- `| ∨` Or
- `& ∧` And
- `! ¬` Not

The parser ignores whitespace, and everything not yet mentioned will be interpreted as a variable name.
Variable names are not allowed to contain whitespace.
This is for the sole reason of being able to raise an error when you accidentally omit an OR or AND.

All statements must be in Conjunctive Normal Form.
This means that a formula like `!(a | b)` is not allowed and must be rewritten as `!a & !b`.
And the formula `!(a & b)` must be rewritten as `!a | !b`.

Example: `alpha | beta & !gamma` will be interpreted as `(alpha OR beta) AND (NOT gamma)`.
A valid output will be

```plaintext
alpha: true
beta: true
gamma: false
```

## Solver

The solver relies on a relatively naive algorithm:d
It takes the first unknown variable of the least complex expression, and considers the cases of it being true and false.
In both cases, it simplifies the formula as far as possible.
If a formula fully simplifies to `true`, then it returns the current set variables.
If both cases simplify to `false`, then it is known that the current combination of set variables does not yield a solution, so the algorithm backtracks and attempts a new combination.
Finally, if the formula only partially simplifies, then the algorithm sets another variable, and probes deeper.

If all cases are exhausted and there has still not been a single combination of variables that yields a `true` formula, then the algorithm concludes that there exists no solution.

Future improvements should focus on optimizing which variable to set, and what value to set it to.

## Memory model

All boolean expressions are stored on the heap. For example, `Q OR R` will be stored as a label that identifies it as an `OR` expression, and two pointers to the heap locations where `Q` and `R` are stored. As in the process of simplification the formula will always become more compact, it is also possible to store these expressions in a single heap allocation. It is unsure whether this will have any performance benefits, but it would be something worth trying.
