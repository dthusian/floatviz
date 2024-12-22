use std::collections::BTreeMap;
use std::fmt::{Write};
use std::ops::{BitAnd, BitOr, BitXor};
use std::rc::Rc;
use thiserror::Error;
use crate::fenv::FloatingPointEnv;
use crate::floats::{Float, FloatParameters};
use crate::ops::add::AddSub;

pub mod add;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Exception(pub u32);

impl Exception {
  pub const INVALID_OPERATION: Exception = Exception(0x1);
  pub const DIVISION_BY_ZERO: Exception = Exception(0x2);
  pub const OVERFLOW: Exception = Exception(0x4);
  pub const UNDERFLOW: Exception = Exception(0x8);
  pub const INEXACT: Exception = Exception(0x10);
}

macro_rules! exception_op {
    ($op:path, $method:ident, $binop:tt) => {
      impl $op for Exception {
        type Output = Exception;

        fn $method(self, rhs: Self) -> Self::Output {
          Exception(self.0 $binop rhs.0)
        }
      }
    };
}

exception_op!(BitOr, bitor, |);
exception_op!(BitAnd, bitand, &);
exception_op!(BitXor, bitxor, ^);

pub trait Op {
  fn num_params(&self) -> usize;

  fn execute(&self, env: &FloatingPointEnv, params: &[Float], output_type: &FloatParameters) -> (Float, Exception);

  fn execute_visual(&self, fomatter: &mut dyn Write, env: &FloatingPointEnv, params: &[Float], output_type: &FloatParameters) -> Result<(Float, Exception), std::fmt::Error>;
}

pub fn collect_ops() -> BTreeMap<String, Rc<dyn Op>> {
  let mut h = BTreeMap::<String, Rc<dyn Op>>::new();
  h.insert("add".into(), Rc::new(AddSub(false)));
  h.insert("sub".into(), Rc::new(AddSub(true)));
  h
}