pub mod binary;
pub mod human;
pub mod epsilon;

use std::collections::{BTreeMap, HashMap};
use std::fmt::{Write};
use std::rc::Rc;
use crate::floats::{Float, BitSlice};
use crate::printers::binary::{BinaryPrinter, BinaryPrinterWithGuide};
use crate::printers::epsilon::UnitInLastPlacePrinter;
use crate::printers::human::ExactDecimalPrinter;

pub const RESET: &str = "\x1b[0m";
pub const BLACK: &str = "\x1b[30m";
pub const DARK_RED: &str = "\x1b[31m";
pub const DARK_GREEN: &str = "\x1b[32m";
pub const DARK_YELLOW: &str = "\x1b[33m";
pub const DARK_BLUE: &str = "\x1b[34m";
pub const DARK_PINK: &str = "\x1b[35m";
pub const DARK_CYAN: &str = "\x1b[36m";
pub const GRAY: &str = "\x1b[37m";

pub const DARK_GRAY: &str = "\x1b[90m";
pub const RED: &str = "\x1b[91m";
pub const GREEN: &str = "\x1b[92m";
pub const YELLOW: &str = "\x1b[93m";
pub const BLUE: &str = "\x1b[94m";
pub const PINK: &str = "\x1b[95m";
pub const CYAN: &str = "\x1b[96m";
pub const WHITE: &str = "\x1b[97m";

pub fn bit2char(bit: bool) -> char {
  if bit { '1' } else { '0' }
}

pub fn int_length(x: usize) -> usize {
  if x == 0 {
    return 1;
  }
  x.ilog10() as usize + 1
}

pub fn print_bitset(f: &mut dyn Write, bitset: &BitSlice) -> std::fmt::Result {
  for i in 0..bitset.len() {
    let i2 = bitset.len() - i - 1;
    f.write_char(bit2char(bitset[i2]))?;
  }
  Ok(())
}


pub trait Printer {
  fn name(&self) -> &str;
  fn description(&self) -> &str;
  fn print(&self, val: &Float) -> Vec<String>;
}

pub fn collect_printers() -> BTreeMap<String, Rc<dyn Printer>> {
  let mut h: BTreeMap<String, Rc<dyn Printer>> = BTreeMap::new();
  h.insert("binary".into(), Rc::new(BinaryPrinterWithGuide));
  h.insert("exact".into(), Rc::new(ExactDecimalPrinter));
  h.insert("ulp".into(), Rc::new(UnitInLastPlacePrinter));
  h
}