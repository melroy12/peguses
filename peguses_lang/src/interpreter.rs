use std::collections::HashMap;

use crate::ast::{BinOp, Expr, Stmt};

pub struct Interpreter {
    env: HashMap<String, i64>,
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
            Stmt::Print { value } => {
                let result = self.eval_expr(value)?;
                println!("{}", result);
                Ok(())
            }
        }
    }

    fn eval_expr(&self, expr: &Expr) -> Result<i64, String> {
        match expr {
            Expr::Number(n) => Ok(*n),

            Expr::Ident(name) => {
                self.env
                    .get(name)
                    .copied()
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            }

            Expr::Binary { op, left, right } => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;

                match op {
                    BinOp::Add => Ok(left_val + right_val),
                    BinOp::Sub => Ok(left_val - right_val),
                    BinOp::Mul => Ok(left_val * right_val),
                    BinOp::Div => {
                        if right_val == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                }
            }
        }
    }
}