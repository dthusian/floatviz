use crate::floats::{Float, FloatClass};
use crate::printers::Printer;
use crate::str_conv::float_to_exact_str;

pub struct ExactDecimalPrinter;

impl Printer for ExactDecimalPrinter {
  fn name(&self) -> &str {
    "Exact Decimal"
  }

  fn print(&self, val: &Float) -> Vec<String> {
    let s = match val.classify() {
      FloatClass::PositiveInf => "+Inf".to_owned(),
      FloatClass::PositiveNormal | FloatClass::NegativeNormal | FloatClass::PositiveZero | FloatClass::NegativeZero => float_to_exact_str(val),
      FloatClass::PositiveSubnormal | FloatClass::NegativeSubnormal => float_to_exact_str(val) + " (subnormal)",
      FloatClass::NegativeInf => "-Inf".to_owned(),
      FloatClass::SignallingNaN => "sNaN".to_owned(),
      FloatClass::QuietNaN => "NaN".to_owned()
    };
    vec![s]
  }
}