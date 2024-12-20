use std::env::args;
use std::fmt::Write;
use crate::floats::{Float, FloatParameters};
use crate::printers::binary::BinaryPrinterWithGuide;
use crate::printers::human::HumanPrinter;
use crate::printers::Printer;

mod floats;
mod ops;
mod printers;
mod str_conv;

fn print_using_printer(name: &str, printer: impl Printer, val: &Float) {
  let strs = printer.print(&val);
  println!("{} = {}", name, &strs[0]);
  strs.iter().skip(1).for_each(|v| {
    println!("{}   {}", " ".repeat(name.len()), v);
  });
}

fn main() {
  let mut args = args().collect::<Vec<_>>();
  let a_type = &args[1];
  let a_type = FloatParameters::parse(a_type).unwrap();
  let a_val = &args[2];
  let a_val = Float::parse(a_val, &a_type).unwrap();
  print_using_printer("A", BinaryPrinterWithGuide, &a_val);
  print_using_printer("A", HumanPrinter, &a_val);
}
