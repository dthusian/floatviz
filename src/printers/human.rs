use crate::floats::{Float, FloatClass};
use crate::printers::Printer;
use crate::str_conv::float_to_str;

pub struct HumanPrinter;

impl Printer for HumanPrinter {
  fn print(&self, val: &Float) -> Vec<String> {
    let s = match val.classify() {
      FloatClass::PositiveInf => "+Inf".to_owned(),
      FloatClass::PositiveNormal | FloatClass::NegativeNormal | FloatClass::PositiveZero | FloatClass::NegativeZero => float_to_str(val),
      FloatClass::PositiveSubnormal | FloatClass::NegativeSubnormal => float_to_str(val) + " (subnormal)",
      FloatClass::NegativeInf => "-Inf".to_owned(),
      FloatClass::SignallingNaN => "sNaN".to_owned(),
      FloatClass::QuietNaN => "NaN".to_owned()
    };
    vec![s]
  }
}