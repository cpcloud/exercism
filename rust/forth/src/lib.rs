use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{alpha1, char, digit1, one_of, satisfy, space0, space1},
    combinator::{map, map_res, opt, recognize, value},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

/// Sum type for arithmetic operations
#[derive(Debug, Copy, Clone)]
enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl ArithOp {
    /// Evaluate the operation given `lhs` and `rhs` input values.
    fn eval(&self, lhs: Value, rhs: Value) -> Result<Value, Error> {
        Ok(match self {
            ArithOp::Add => lhs + rhs,
            ArithOp::Sub => lhs - rhs,
            ArithOp::Mul => lhs * rhs,
            ArithOp::Div => lhs.checked_div(rhs).ok_or(Error::DivisionByZero)?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum BuiltinOp {
    Dup,
    Drop,
    Swap,
    Over,
    Arith(ArithOp),
}

/// The result of parsing a definition
#[derive(Debug, Clone)]
struct ParsedDefinition {
    /// The name of the definition
    name: String,
    /// The expressions in the definition
    exprs: Vec<Expr>,
}

/// The result of parsing an expression
#[derive(Debug, Clone)]
enum Expr {
    Value(Value),
    Symbol(String),
}

/// The result of parsing a statement
#[derive(Debug)]
enum Stmt {
    /// List of expressions making up a statement
    Exprs(Vec<Expr>),
    /// A variable or function definition
    ParsedDefinition(ParsedDefinition),
}

/// A Forth interpreter
pub struct Forth {
    /// Current evaluated values
    stack: Vec<Value>,
    /// The names visible to the interpreter
    env: HashMap<String, Definition>,
}

/// A ParsedDefinition together with its execution environment
#[derive(Debug, Clone)]
struct Definition {
    /// The expressions making up the defintition
    exprs: Vec<Expr>,
    /// The environment visible at the time of definition
    env: HashMap<String, Definition>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

/// Parse digit strings with optional `-` into Values.
fn parse_number(input: &str) -> IResult<&str, Value> {
    map_res(recognize(tuple((opt(char('-')), digit1))), |res| {
        Value::from_str_radix(res, 10)
    })(input)
}

/// Parse builtin operators and functions
fn parse_builtin_op(input: &str) -> IResult<&str, BuiltinOp> {
    alt((
        value(BuiltinOp::Dup, tag_no_case("dup")),
        value(BuiltinOp::Drop, tag_no_case("drop")),
        value(BuiltinOp::Swap, tag_no_case("swap")),
        value(BuiltinOp::Over, tag_no_case("over")),
        value(BuiltinOp::Arith(ArithOp::Add), char('+')),
        value(BuiltinOp::Arith(ArithOp::Sub), char('-')),
        value(BuiltinOp::Arith(ArithOp::Mul), char('*')),
        value(BuiltinOp::Arith(ArithOp::Div), char('/')),
    ))(input)
}

/// Parse variable and function definitions
fn parse_definition(input: &str) -> IResult<&str, ParsedDefinition> {
    map(
        tuple((
            preceded(tuple((char(':'), space0)), parse_symbol),
            delimited(space1, parse_expr, tuple((space0, char(';')))),
        )),
        |(name, exprs)| ParsedDefinition {
            name: name.to_lowercase(),
            exprs,
        },
    )(input)
}

/// Parse symbols: arithmetic operators or dash separated alphanumeric characters
fn parse_symbol(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(one_of("+-*/")),
        recognize(tuple((
            alpha1,
            many0(satisfy(|c| c.is_alphanumeric() || c == '-' || c == '_')),
        ))),
    ))(input)
}

/// Parse a single expr: either a number or symbol
fn parse_single_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(parse_number, Expr::Value),
        map(parse_symbol, |string| Expr::Symbol(string.to_lowercase())),
    ))(input)
}

/// Parse a whitespace separated list of single expressions
fn parse_expr(input: &str) -> IResult<&str, Vec<Expr>> {
    separated_list1(space1, parse_single_expr)(input)
}

/// Parse a list of definitions or a list of expressions
fn parse_stmts(input: &str) -> IResult<&str, Vec<Stmt>> {
    separated_list1(
        space1,
        alt((
            map(parse_definition, Stmt::ParsedDefinition),
            map(parse_expr, Stmt::Exprs),
        )),
    )(input)
}

impl Forth {
    /// Builtin operations
    const BUILTIN_OPS: [&'static str; 8] = ["dup", "drop", "swap", "over", "+", "-", "*", "/"];

    /// Construct a new
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            stack: Default::default(),
            env: Default::default(),
        }
    }

    /// Return the list of values currently available
    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    /// Evaluate the `input` expression
    pub fn eval(&mut self, input: &str) -> ForthResult {
        let (_, stmts) = parse_stmts(input).map_err(|_| Error::InvalidWord)?;
        for stmt in stmts.into_iter() {
            match stmt {
                Stmt::ParsedDefinition(ParsedDefinition { name, exprs }) => {
                    self.env.insert(
                        name.to_lowercase(),
                        Definition {
                            exprs,
                            env: self.env.clone(),
                        },
                    );
                }
                Stmt::Exprs(exprs) => {
                    self.eval_stack(exprs, self.env.clone())?;
                }
            };
        }
        Ok(())
    }

    /// Compute the second to last index
    fn second_to_last_index(&self) -> Result<usize, Error> {
        self.stack.len().checked_sub(2).ok_or(Error::StackUnderflow)
    }

    /// Evaluate a built operation
    fn eval_builtin_op(&mut self, op: BuiltinOp) -> ForthResult {
        match op {
            BuiltinOp::Dup => {
                self.stack
                    .push(*self.stack.last().ok_or(Error::StackUnderflow)?);
            }
            BuiltinOp::Drop => {
                self.stack.pop().ok_or(Error::StackUnderflow)?;
            }
            BuiltinOp::Swap => {
                let second_to_last_index = self.second_to_last_index()?;
                let last_index = second_to_last_index + 1;
                self.stack.swap(second_to_last_index, last_index)
            }
            BuiltinOp::Over => {
                self.stack.push(self.stack[self.second_to_last_index()?]);
            }
            BuiltinOp::Arith(op) => {
                let rhs = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let lhs = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(op.eval(lhs, rhs)?);
            }
        }
        Ok(())
    }

    /// Evaluate list of expressions against a definition environment
    fn eval_stack(
        &mut self,
        exprs: Vec<Expr>,
        def_env: HashMap<String, Definition>,
    ) -> ForthResult {
        for expr in exprs.into_iter() {
            match expr {
                Expr::Value(value) => self.stack.push(value),
                Expr::Symbol(symbol) => {
                    // Chain lookups from the definition environment to the parent environment
                    match def_env
                        .get(&symbol)
                        .or_else(|| self.env.get(&symbol))
                        .cloned()
                    {
                        Some(Definition { exprs, env }) => {
                            self.eval_stack(exprs, env)?;
                        }
                        // if we didn't find the name in the definition environment or the parent
                        // and the symbol is builtin operation then execute it
                        None if Self::BUILTIN_OPS.contains(&symbol.as_str()) => {
                            let (_, builtin_op) =
                                parse_builtin_op(&symbol).map_err(|_| Error::InvalidWord)?;
                            self.eval_builtin_op(builtin_op)?;
                        }
                        // otherwise we don't know the symbol, so it's an error
                        _ => return Err(Error::UnknownWord),
                    }
                }
            }
        }
        Ok(())
    }
}
