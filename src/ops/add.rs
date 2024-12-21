use std::fmt::{Write};
use crate::floats::{Float, FloatParameters};
use crate::ops::Op;

pub struct AddSub(pub bool);

impl Op for AddSub {
  fn num_params(&self) -> usize {
    2
  }

  fn execute(&self, params: &[Float], output_type: &FloatParameters) -> Float {
    todo!()
  }

  fn execute_visual(&self, fomatter: &mut dyn Write, params: &[Float], output_type: &FloatParameters) -> Float {
    todo!()
  }
}