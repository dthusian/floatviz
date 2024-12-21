use std::fmt::{Pointer, Write};
use crate::floats::{Float, FloatParameters};
use crate::printers::{bit2char, int_length, print_bitset, Printer, CYAN, DARK_CYAN, DARK_GREEN, DARK_YELLOW, GREEN, RESET, YELLOW};

fn print_float(f: &mut dyn Write, val: &Float) -> std::fmt::Result {
  let msb_idx_size = int_length(val.params().total_length() - 1);
  f.write_str(&" ".repeat(msb_idx_size - 1))?;
  f.write_str(CYAN)?;
  f.write_char(bit2char(val.sign()))?;
  f.write_char(' ')?;
  f.write_str(GREEN)?;
  print_bitset(f, val.exponent_bits())?;
  f.write_char(' ')?;
  f.write_str(YELLOW)?;
  print_bitset(f, val.significand_bits())?;
  f.write_str(RESET)?;
  Ok(())
}

fn print_guide_markers(f: &mut dyn Write, params: &FloatParameters) -> std::fmt::Result {
  write!(f, "{}{}", DARK_CYAN, params.total_length() - 1)?;
  fn print_field(f: &mut dyn Write, color: &str, upper: usize, lower: usize) -> std::fmt::Result {
    let spaces = (upper - lower + 1).saturating_sub(int_length(upper)).saturating_sub(int_length(lower));
    if spaces != 0 {
      write!(f, " {}{}{}{}", color, upper, " ".repeat(spaces), lower)
    } else {
      // if no space to put guide markers, don't
      write!(f, " {}{}", color, " ".repeat(upper - lower + 1))
    }
  }
  print_field(f, DARK_GREEN, params.total_length() - 2, params.sig_bits)?;
  print_field(f, DARK_YELLOW, params.sig_bits - 1, 0)?;
  f.write_str(RESET)?;
  Ok(())
}

pub struct BinaryPrinter;

impl Printer for BinaryPrinter {
  fn name(&self) -> &str {
    "Binary"
  }

  fn description(&self) -> &str {
    "Prints the binary representation"
  }

  fn print(&self, val: &Float) -> Vec<String> {
    let mut s = String::new();
    print_float(&mut s, val).unwrap();
    vec![s]
  }
}

pub struct BinaryPrinterWithGuide;

impl Printer for BinaryPrinterWithGuide {
  fn name(&self) -> &str {
    "Binary"
  }

  fn description(&self) -> &str {
    "Prints the binary representation with guide markers"
  }

  fn print(&self, val: &Float) -> Vec<String> {
    let mut s1 = String::new();
    let mut s2 = String::new();
    print_float(&mut s1, val).unwrap();
    print_guide_markers(&mut s2, val.params()).unwrap();
    vec![s1, s2]
  }
}