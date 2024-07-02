use std::{
  ops::{self, Add},
  str::FromStr,
};

use stack_core::{parser, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Val {
  Integer(i64),
  Float(f64),
}

impl ops::Add for Val {
  type Output = Result<Self, (Self, Self)>;

  fn add(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Self::Integer(lhs), Self::Integer(rhs)) => {
        Ok(Self::Integer(lhs.saturating_add(rhs)))
      }
      (Self::Float(lhs), Self::Float(rhs)) => Ok(Self::Float(lhs + rhs)),

      (lhs, rhs) => Err((lhs, rhs)),
    }
  }
}

impl ops::Sub for Val {
  type Output = Result<Self, (Self, Self)>;

  fn sub(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Self::Integer(lhs), Self::Integer(rhs)) => {
        Ok(Self::Integer(lhs.saturating_sub(rhs)))
      }
      (Self::Float(lhs), Self::Float(rhs)) => Ok(Self::Float(lhs - rhs)),

      (lhs, rhs) => Err((lhs, rhs)),
    }
  }
}

impl ops::Mul for Val {
  type Output = Result<Self, (Self, Self)>;

  fn mul(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Self::Integer(lhs), Self::Integer(rhs)) => {
        Ok(Self::Integer(lhs.saturating_mul(rhs)))
      }
      (Self::Float(lhs), Self::Float(rhs)) => Ok(Self::Float(lhs * rhs)),

      (lhs, rhs) => Err((lhs, rhs)),
    }
  }
}

impl ops::Div for Val {
  type Output = Result<Self, (Self, Self)>;

  fn div(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Self::Integer(lhs), Self::Integer(rhs)) => {
        Ok(Self::Integer(lhs.saturating_div(rhs)))
      }
      (Self::Float(lhs), Self::Float(rhs)) => Ok(Self::Float(lhs / rhs)),

      (lhs, rhs) => Err((lhs, rhs)),
    }
  }
}

impl ops::Rem for Val {
  type Output = Result<Self, (Self, Self)>;

  fn rem(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Self::Integer(lhs), Self::Integer(rhs)) => Ok(Self::Integer(lhs % rhs)),
      (Self::Float(lhs), Self::Float(rhs)) => Ok(Self::Float(lhs % rhs)),

      (lhs, rhs) => Err((lhs, rhs)),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
  Push(Val),
  Intrinsic(Intrinsic),
  End,
}

type Ops = Vec<Op>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum VMError {
  #[default]
  Unknown,

  Halt,
  IPBounds,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VM {
  ops: Ops,
  ip: usize,

  registers: Vec<Val>,
  stack: Vec<Val>,
  sp: usize,
}

impl VM {
  pub fn new() -> Self {
    Self {
      ops: Ops::new(),
      ip: 0,

      registers: Vec::new(),
      stack: Vec::new(),
      sp: 0,
    }
  }

  pub fn stack_pop(&mut self) -> Result<Val, VMError> {
    match self.stack.pop() {
      Some(val) => Ok(val),
      None => Err(VMError::Unknown),
    }
  }

  pub fn stack_push(&mut self, val: Val) {
    self.stack.push(val);
  }

  pub fn compile_expr(&self, expr: Expr) -> Op {
    match expr.kind {
      ExprKind::Nil => todo!(),
      ExprKind::Boolean(_) => todo!(),
      ExprKind::Integer(int) => Op::Push(Val::Integer(int)),
      ExprKind::Float(_) => todo!(),
      ExprKind::String(_) => todo!(),
      ExprKind::Symbol(symbol) => {
        if let Ok(intrinsic) = Intrinsic::from_str(symbol.as_str()) {
          Op::Intrinsic(intrinsic)
        } else {
          todo!()
        }
      }
      ExprKind::Lazy(_) => todo!(),
      ExprKind::List(_) => todo!(),
      ExprKind::Record(_) => todo!(),
      ExprKind::Function { scope, body } => todo!(),
      ExprKind::SExpr { call, body } => todo!(),
      ExprKind::Underscore => todo!(),
    }
  }

  pub fn compile(&mut self, exprs: Vec<Expr>) {
    for expr in exprs.into_iter() {
      self.ops.push(self.compile_expr(expr));
    }

    self.ops.push(Op::End);
  }

  pub fn step(&mut self) -> Result<(), VMError> {
    let op = self.ops.get(self.ip);

    let ip = self.ip.checked_add(1).map(|res| res.min(self.ops.len()));
    if let Some(ip) = ip {
      self.ip = ip;
    } else {
      return Err(VMError::IPBounds);
    }

    if let Some(op) = op {
      match op {
        Op::Push(val) => self.stack.push(*val),
        Op::Intrinsic(intrinsic) => match intrinsic {
          Intrinsic::Add => {
            let rhs = self.stack_pop()?;
            let lhs = self.stack_pop()?;

            let result = match lhs + rhs {
              Ok(res) => res,
              Err(_) => todo!(),
            };

            self.stack_push(result);
          }
          Intrinsic::Sub => todo!(),
          Intrinsic::Mul => todo!(),
          Intrinsic::Div => todo!(),
          Intrinsic::Rem => todo!(),
          Intrinsic::Eq => todo!(),
          Intrinsic::Ne => todo!(),
          Intrinsic::Lt => todo!(),
          Intrinsic::Le => todo!(),
          Intrinsic::Gt => todo!(),
          Intrinsic::Ge => todo!(),
          Intrinsic::Or => todo!(),
          Intrinsic::And => todo!(),
          Intrinsic::Not => todo!(),
          Intrinsic::Assert => todo!(),
          Intrinsic::Drop => todo!(),
          Intrinsic::Dupe => todo!(),
          Intrinsic::Swap => todo!(),
          Intrinsic::Rot => todo!(),
          Intrinsic::Len => todo!(),
          Intrinsic::Nth => todo!(),
          Intrinsic::Split => todo!(),
          Intrinsic::Concat => todo!(),
          Intrinsic::Push => todo!(),
          Intrinsic::Pop => todo!(),
          Intrinsic::Insert => todo!(),
          Intrinsic::Prop => todo!(),
          Intrinsic::Has => todo!(),
          Intrinsic::Remove => todo!(),
          Intrinsic::Keys => todo!(),
          Intrinsic::Values => todo!(),
          Intrinsic::Cast => todo!(),
          Intrinsic::TypeOf => todo!(),
          Intrinsic::Lazy => todo!(),
          Intrinsic::If => todo!(),
          Intrinsic::Halt => todo!(),
          Intrinsic::Call => todo!(),
          Intrinsic::Let => todo!(),
          Intrinsic::Def => todo!(),
          Intrinsic::Set => todo!(),
          Intrinsic::Get => todo!(),
          Intrinsic::Debug => todo!(),
          Intrinsic::Print => todo!(),
          Intrinsic::Pretty => todo!(),
          Intrinsic::Recur => todo!(),
          Intrinsic::OrElse => todo!(),
          Intrinsic::Import => todo!(),
        },
        Op::End => return Err(VMError::Halt),
      }

      Ok(())
    } else {
      todo!("ip out of bounds")
    }
  }
}

fn main() {
  let source = Source::new("", "2 2 +");
  let mut lexer = Lexer::new(source);
  let exprs = parser::parse(&mut lexer).unwrap();

  let mut vm = VM::new();
  vm.compile(exprs);

  loop {
    if let Err(err) = vm.step() {
      eprintln!("{err:?}");
      break;
    }
  }

  println!("{vm:?}");
}
