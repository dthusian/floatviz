use std::fmt::{Write};
use crate::floats::{Float, FloatParameters};

pub mod add;

pub trait Op {
  fn num_params(&self) -> usize;

  fn execute(&self, params: &[Float], output_type: &FloatParameters) -> Float;

  fn execute_visual(&self, fomatter: &mut dyn Write, params: &[Float], output_type: &FloatParameters) -> Float;
}