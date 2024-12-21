use std::env::args;
use std::fmt::Write;
use crate::floats::{Float, FloatParameters};
use crate::printers::binary::BinaryPrinterWithGuide;
use crate::printers::epsilon::EpsilonPrinter;
use crate::printers::human::ExactDecimalPrinter;
use crate::printers::Printer;

mod floats;
mod ops;
mod printers;
mod str_conv;
mod fenv;

fn print_using_printer(printer: &dyn Printer, val: &Float) {
  let strs = printer.print(&val);
  let pname = printer.name();
  println!("{}: {}", pname, &strs[0]);
  strs.iter().skip(1).for_each(|v| {
    println!("{}  {}", " ".repeat(pname.len()), v);
  });
}

fn main() {
  let mut args = args().collect::<Vec<_>>();
  let a_type = &args[1];
  let a_type = FloatParameters::parse(a_type).unwrap();
  let a_val = &args[2];
  let a_val = Float::parse(a_val, &a_type).unwrap();
  let printers = vec![
    Box::new(BinaryPrinterWithGuide) as Box<dyn Printer>,
    Box::new(ExactDecimalPrinter),
    Box::new(EpsilonPrinter)
  ];
  printers.iter().for_each(|v| print_using_printer(v.as_ref(), &a_val));
}
