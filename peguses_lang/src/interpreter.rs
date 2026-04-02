use std::collections::HashMap;

use crate::ast::{BinOp, Expr, Stmt, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    Boolean(bool),
}

impl Value {
    fn as_number(&self) -> Result<i64, String> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err("Expected a number".to_string()),
        }
    }

    fn as_boolean(&self) -> Result<bool, String> {
        match self {
            Value::Boolean(b) => Ok(*b),
            _ => Err("Expected a boolean".to_string()),
        }
    }
    
    fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}

pub struct Interpreter {
    env: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: &[Stmt]) -> Result<(), String> {
        for stmt in program {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let { name, value } => {
                let result = self.eval_expr(value)?;
                self.env.insert(name.clone(), result);
                Ok(())
            }
            Stmt::Assign { name, value } => {
                if !self.env.contains_key(name) {
                    return Err(format!("Undefined variable: {}", name));
                }
                let result = self.eval_expr(value)?;
                self.env.insert(name.clone(), result);
                Ok(())
            }
            Stmt::Print { value } => {
                let result = self.eval_expr(value)?;
                println!("{}", result);
                Ok(())
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
            } => {
                let cond_value = self.eval_expr(condition)?;
                if cond_value.is_truthy() {
                    for stmt in then_block {
                        self.execute(stmt)?;
                    }
                } else if let Some(else_stmts) = else_block {
                    for stmt in else_stmts {
                        self.execute(stmt)?;
                    }
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond_value = self.eval_expr(condition)?;
                    if !cond_value.is_truthy() {
                        break;
                    }
                    for stmt in body {
                        self.execute(stmt)?;
                    }
                }
                Ok(())
            }
        }
    }

    fn eval_expr(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),

            Expr::Ident(name) => {
                self.env
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            }
            
            Expr::Unary { op, expr } => {
                let val = self.eval_expr(expr)?;
                match op {
                    UnaryOp::Not => Ok(Value::Boolean(!val.is_truthy())),
                    UnaryOp::Neg => Ok(Value::Number(-val.as_number()?)),
                }
            }

            Expr::Binary { op, left, right } => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;

                match op {
                    // Arithmetic operators
                    BinOp::Add => {
                        let l = left_val.as_number()?;
                        let r = right_val.as_number()?;
                        Ok(Value::Number(l + r))
                    }
                    BinOp::Sub => {
                        let l = left_val.as_number()?;
                        let r = right_val.as_number()?;
                        Ok(Value::Number(l - r))
                    }
                    BinOp::Mul => {
                        let l = left_val.as_number()?;
                        let r = right_val.as_number()?;
                        Ok(Value::Number(l * r))
                    }
                    BinOp::Div => {
                        let l = left_val.as_number()?;
                        let r = right_val.as_number()?;
                        if r == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    BinOp::Mod => {
                        let l = left_val.as_number()?;
                        let r = right_val.as_number()?;
                        if r == 0 {
                            Err("Modulo by zero".to_string())
                        } else {
                            Ok(Value::Number(l % r))
                        }
                    }
                    
                    // Comparison operators
                    BinOp::Equal => {
                        Ok(Value::Boolean(left_val == right_val))
                    }
                    BinOp::NotEqual => {
                        Ok(Value::Boolean(left_val != right_val))
                    }
                    BinOp::Less => {
                        let l = left_val.as_number()?;
                        let r = right_val.as_number()?;
                        Ok(Value::Boolean(l < r))
                    }
                    BinOp::Greater => {
                        let l = left_val.as_number()?;
                        let r = right_val.as_number()?;
                        Ok(Value::Boolean(l > r))
                    }
                    BinOp::LessEqual => {
                        let l = left_val.as_number()?;
                        let r = right_val.as_number()?;
                        Ok(Value::Boolean(l <= r))
                    }
                    BinOp::GreaterEqual => {
                        let l = left_val.as_number()?;
                        let r = right_val.as_number()?;
                        Ok(Value::Boolean(l >= r))
                    }
                    
                    // Logical operators
                    BinOp::And => {
                        Ok(Value::Boolean(left_val.is_truthy() && right_val.is_truthy()))
                    }
                    BinOp::Or => {
                        Ok(Value::Boolean(left_val.is_truthy() || right_val.is_truthy()))
                    }
                }
            }
        }
    }
}