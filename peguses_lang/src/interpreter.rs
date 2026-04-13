use std::collections::HashMap;

use crate::ast::{BinOp, Expr, Stmt, UnaryOp};

type Result<T> = std::result::Result<T, String>;

/// Loop control flow
#[derive(Debug, Clone, Copy, PartialEq)]
enum LoopControl {
    None,
    Break,
    Continue,
}

/// Runtime value representation
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    Boolean(bool),
    String(String),
}


impl Value {
    fn as_number(&self) -> Result<i64> {
        match self {
            Value::Number(n) => Ok(*n),
            Value::Boolean(b) => Err(format!("Type error: expected number, got boolean '{}'", b)),
            Value::String(s) => Err(format!("Type error: expected number, got string '{}'", s)),
        }
    }
    #[allow(dead_code)]    fn as_boolean(&self) -> Result<bool> {
        match self {
            Value::Boolean(b) => Ok(*b),
            Value::Number(n) => Err(format!("Type error: expected boolean, got number '{}'", n)),
            Value::String(s) => Err(format!("Type error: expected boolean, got string '{}'", s)),
        }
    }
    
    fn as_string(&self) -> Result<String> {
        match self {
            Value::String(s) => Ok(s.clone()),
            Value::Number(n) => Err(format!("Type error: expected string, got number '{}'", n)),
            Value::Boolean(b) => Err(format!("Type error: expected string, got boolean '{}'", b)),
        }
    }
    
    fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0,
            Value::String(s) => !s.is_empty(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

/// The interpreter that executes Peguses programs
pub struct Interpreter {
    env: HashMap<String, Value>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            env: HashMap::new(),
        }
    }
}

impl Interpreter {
    /// Create a new interpreter with an empty environment
    pub fn new() -> Self {
        Self::default()
    }

    /// Execute a program (list of statements)
    pub fn run(&mut self, program: &[Stmt]) -> Result<()> {
        for stmt in program {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<LoopControl> {
        match stmt {
            Stmt::Let { name, value } => {
                let result = self.eval_expr(value)?;
                self.env.insert(name.clone(), result);
                Ok(LoopControl::None)
            }
            Stmt::Assign { name, value } => {
                if !self.env.contains_key(name) {
                    return Err(format!("Assignment error: variable '{}' is not defined. Use 'let' to declare it first.", name));
                }
                let result = self.eval_expr(value)?;
                self.env.insert(name.clone(), result);
                Ok(LoopControl::None)
            }
            Stmt::Print { value } => {
                let result = self.eval_expr(value)?;
                println!("{}", result);
                Ok(LoopControl::None)
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
            } => {
                let cond_value = self.eval_expr(condition)?;
                if cond_value.is_truthy() {
                    for stmt in then_block {
                        let ctrl = self.execute(stmt)?;
                        if ctrl != LoopControl::None {
                            return Ok(ctrl);
                        }
                    }
                } else if let Some(else_stmts) = else_block {
                    for stmt in else_stmts {
                        let ctrl = self.execute(stmt)?;
                        if ctrl != LoopControl::None {
                            return Ok(ctrl);
                        }
                    }
                }
                Ok(LoopControl::None)
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond_value = self.eval_expr(condition)?;
                    if !cond_value.is_truthy() {
                        break;
                    }
                    for stmt in body {
                        let ctrl = self.execute(stmt)?;
                        match ctrl {
                            LoopControl::Break => return Ok(LoopControl::None),
                            LoopControl::Continue => break,
                            LoopControl::None => {},
                        }
                    }
                }
                Ok(LoopControl::None)
            }
            Stmt::For { var, start, end, body } => {
                let start_val = self.eval_expr(start)?.as_number()
                    .map_err(|e| format!("For loop start: {}", e))?;
                let end_val = self.eval_expr(end)?.as_number()
                    .map_err(|e| format!("For loop end: {}", e))?;
                
                for i in start_val..end_val {
                    self.env.insert(var.clone(), Value::Number(i));
                    for stmt in body {
                        let ctrl = self.execute(stmt)?;
                        match ctrl {
                            LoopControl::Break => return Ok(LoopControl::None),
                            LoopControl::Continue => break,
                            LoopControl::None => {},
                        }
                    }
                }
                Ok(LoopControl::None)
            }
            Stmt::Break => Ok(LoopControl::Break),
            Stmt::Continue => Ok(LoopControl::Continue),
        }
    }

    fn eval_expr(&self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            
            Expr::String(s) => Ok(Value::String(s.clone())),

            Expr::Ident(name) => {
                self.env
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("Reference error: variable '{}' is not defined", name))
            }
            
            Expr::Unary { op, expr } => {
                let val = self.eval_expr(expr)?;
                match op {
                    UnaryOp::Not => Ok(Value::Boolean(!val.is_truthy())),
                    UnaryOp::Neg => {
                        let n = val.as_number()
                            .map_err(|e| format!("Unary negation error: {}", e))?;
                        Ok(Value::Number(-n))
                    }
                }
            }

            Expr::Binary { op, left, right } => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;

                match op {
                    // Arithmetic operators
                    BinOp::Add => {
                        // Support both number addition and string concatenation
                        match (&left_val, &right_val) {
                            (Value::Number(l), Value::Number(r)) => {
                                Ok(Value::Number(l.checked_add(*r)
                                    .ok_or_else(|| "Addition error: integer overflow".to_string())?))
                            }
                            (Value::String(l), Value::String(r)) => {
                                Ok(Value::String(format!("{}{}", l, r)))
                            }
                            _ => Err("Addition error: operands must be both numbers or both strings".to_string())
                        }
                    }
                    BinOp::Sub => {
                        let l = left_val.as_number()
                            .map_err(|e| format!("Subtraction error: left operand - {}", e))?;
                        let r = right_val.as_number()
                            .map_err(|e| format!("Subtraction error: right operand - {}", e))?;
                        Ok(Value::Number(l.checked_sub(r)
                            .ok_or_else(|| "Subtraction error: integer overflow".to_string())?))
                    }
                    BinOp::Mul => {
                        let l = left_val.as_number()
                            .map_err(|e| format!("Multiplication error: left operand - {}", e))?;
                        let r = right_val.as_number()
                            .map_err(|e| format!("Multiplication error: right operand - {}", e))?;
                        Ok(Value::Number(l.checked_mul(r)
                            .ok_or_else(|| "Multiplication error: integer overflow".to_string())?))
                    }
                    BinOp::Div => {
                        let l = left_val.as_number()
                            .map_err(|e| format!("Division error: left operand - {}", e))?;
                        let r = right_val.as_number()
                            .map_err(|e| format!("Division error: right operand - {}", e))?;
                        if r == 0 {
                            Err("Division error: cannot divide by zero".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    BinOp::Mod => {
                        let l = left_val.as_number()
                            .map_err(|e| format!("Modulo error: left operand - {}", e))?;
                        let r = right_val.as_number()
                            .map_err(|e| format!("Modulo error: right operand - {}", e))?;
                        if r == 0 {
                            Err("Modulo error: cannot compute modulo by zero".to_string())
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
                        let l = left_val.as_number()
                            .map_err(|e| format!("Comparison error (<): left operand - {}", e))?;
                        let r = right_val.as_number()
                            .map_err(|e| format!("Comparison error (<): right operand - {}", e))?;
                        Ok(Value::Boolean(l < r))
                    }
                    BinOp::Greater => {
                        let l = left_val.as_number()
                            .map_err(|e| format!("Comparison error (>): left operand - {}", e))?;
                        let r = right_val.as_number()
                            .map_err(|e| format!("Comparison error (>): right operand - {}", e))?;
                        Ok(Value::Boolean(l > r))
                    }
                    BinOp::LessEqual => {
                        let l = left_val.as_number()
                            .map_err(|e| format!("Comparison error (<=): left operand - {}", e))?;
                        let r = right_val.as_number()
                            .map_err(|e| format!("Comparison error (<=): right operand - {}", e))?;
                        Ok(Value::Boolean(l <= r))
                    }
                    BinOp::GreaterEqual => {
                        let l = left_val.as_number()
                            .map_err(|e| format!("Comparison error (>=): left operand - {}", e))?;
                        let r = right_val.as_number()
                            .map_err(|e| format!("Comparison error (>=): right operand - {}", e))?;
                        Ok(Value::Boolean(l >= r))
                    }
                    
                    // Logical operators
                    BinOp::And => {
                        Ok(Value::Boolean(left_val.is_truthy() && right_val.is_truthy()))
                    }
                    BinOp::Or => {
                        Ok(Value::Boolean(left_val.is_truthy() || right_val.is_truthy()))
                    }
                    
                    // String operators
                    BinOp::Concat => {
                        let l = left_val.as_string()
                            .map_err(|e| format!("Concatenation error: left operand - {}", e))?;
                        let r = right_val.as_string()
                            .map_err(|e| format!("Concatenation error: right operand - {}", e))?;
                        Ok(Value::String(format!("{}{}", l, r)))
                    }
                }
            }
        }
    }
}