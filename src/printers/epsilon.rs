use crate::floats::Float;
use crate::printers::Printer;

pub struct UnitInLastPlacePrinter;

impl Printer for UnitInLastPlacePrinter {
  fn name(&self) -> &str {
    "ULP"
  }

  fn description(&self) -> &str {
    "Prints the units in last place (ULP)"
  }

  fn print(&self, val: &Float) -> Vec<String> {
    if !val.classify().finite() {
      return vec!["Undefined".into()];
    }
    let exponent = val.exponent_logical();
    let epsilon_exp = exponent - val.params().sig_bits as i64;
    vec![format!("2^{}", epsilon_exp)]
  }
}