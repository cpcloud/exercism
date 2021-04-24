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

#[derive(Debug, Copy, Clone)]
enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl ArithOp {
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

#[derive(Debug, Clone)]
struct Definition {
    name: String,
    exprs: Vec<Expr>,
}

#[derive(Debug, Clone)]
enum Expr {
    Value(Value),
    Name(String),
}

#[derive(Debug, Clone)]
enum Stmt {
    Exprs(Vec<Expr>),
    Definition(Definition),
}

pub struct Forth {
    stack: Vec<Value>,
    env: HashMap<String, Def>,
    builtins: [&'static str; 8],
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

fn parse_number(input: &str) -> IResult<&str, Value> {
    map_res(recognize(tuple((opt(char('-')), digit1))), |res| {
        Value::from_str_radix(res, 10)
    })(input)
}

const BUILTIN_OPS: [&str; 8] = ["dup", "drop", "swap", "over", "+", "-", "*", "/"];

fn parse_builtin_op(input: &str) -> IResult<&str, BuiltinOp> {
    alt((
        value(BuiltinOp::Dup, tag_no_case("DUP")),
        value(BuiltinOp::Drop, tag_no_case("DROP")),
        value(BuiltinOp::Swap, tag_no_case("SWAP")),
        value(BuiltinOp::Over, tag_no_case("OVER")),
        value(BuiltinOp::Arith(ArithOp::Add), char('+')),
        value(BuiltinOp::Arith(ArithOp::Sub), char('-')),
        value(BuiltinOp::Arith(ArithOp::Mul), char('*')),
        value(BuiltinOp::Arith(ArithOp::Div), char('/')),
    ))(input)
}

fn parse_definition(input: &str) -> IResult<&str, Definition> {
    map(
        tuple((
            preceded(tuple((char(':'), space1)), parse_name),
            delimited(space1, parse_expr, tuple((space0, char(';')))),
        )),
        |(name, exprs)| Definition {
            name: name.to_owned(),
            exprs,
        },
    )(input)
}

fn parse_name(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(one_of("+-*/")),
        recognize(tuple((
            alpha1,
            many0(satisfy(|c| c.is_alphanumeric() || c == '-')),
        ))),
    ))(input)
}

fn parse_single_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(parse_number, Expr::Value),
        map(parse_name, |result| Expr::Name(result.to_lowercase())),
    ))(input)
}

fn parse_expr(input: &str) -> IResult<&str, Vec<Expr>> {
    separated_list1(space1, parse_single_expr)(input)
}

fn parse_stmts(input: &str) -> IResult<&str, Vec<Stmt>> {
    separated_list1(
        space1,
        alt((
            map(parse_definition, Stmt::Definition),
            map(parse_expr, Stmt::Exprs),
        )),
    )(input)
}

#[derive(Debug, Clone)]
struct Def {
    exprs: Vec<Expr>,
    env: HashMap<String, Def>,
}

impl Forth {
    pub fn new() -> Self {
        Self {
            stack: Default::default(),
            env: Default::default(),
            builtins: BUILTIN_OPS,
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    fn eval_builtin_op(&mut self, op: BuiltinOp) -> ForthResult {
        match op {
            BuiltinOp::Dup => {
                let value = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(value);
                self.stack.push(value);
            }
            BuiltinOp::Drop => {
                self.stack.pop().ok_or(Error::StackUnderflow)?;
            }
            BuiltinOp::Swap => {
                let num_exprs = self.stack.len();
                self.stack.swap(
                    num_exprs.checked_sub(2).ok_or(Error::StackUnderflow)?,
                    num_exprs.checked_sub(1).ok_or(Error::StackUnderflow)?,
                )
            }
            BuiltinOp::Over => {
                let num_exprs = self.stack.len();
                let penultimate =
                    self.stack[num_exprs.checked_sub(2).ok_or(Error::StackUnderflow)?];
                self.stack.push(penultimate);
            }
            BuiltinOp::Arith(op) => {
                let rhs = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let lhs = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(op.eval(lhs, rhs)?);
            }
        }
        Ok(())
    }

    fn eval_stack(&mut self, exprs: Vec<Expr>, parent: HashMap<String, Def>) -> ForthResult {
        for expr in exprs.into_iter() {
            match expr {
                Expr::Value(value) => self.stack.push(value),
                Expr::Name(name) => {
                    if let Some(Def { exprs, env }) = parent.get(&name).cloned() {
                        self.eval_stack(exprs, env.clone())?;
                    } else if let Some(def) = self.env.get(&name).cloned() {
                        self.eval_stack(def.exprs, def.env.clone())?;
                    } else if self.builtins.contains(&name.as_str()) {
                        let (_, builtin_op) =
                            parse_builtin_op(&name).map_err(|_| Error::InvalidWord)?;
                        self.eval_builtin_op(builtin_op)?;
                    } else {
                        return Err(Error::UnknownWord);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let (_, stmts) = parse_stmts(input).map_err(|_| Error::InvalidWord)?;
        for stmt in stmts.into_iter() {
            match stmt {
                Stmt::Definition(def) => {
                    self.env.insert(
                        def.name.to_lowercase(),
                        Def {
                            exprs: def.exprs,
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
}
