use std::env::args;
use std::fmt::Write;
use clap::{Parser, Subcommand};
use crate::floats::{Float, FloatParameters};
use crate::printers::binary::BinaryPrinterWithGuide;
use crate::printers::epsilon::UnitInLastPlacePrinter;
use crate::printers::human::ExactDecimalPrinter;
use crate::printers::{collect_printers, Printer, RED, RESET};

mod floats;
mod ops;
mod printers;
mod str_conv;
mod fenv;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = false)]
pub struct Cli {
  /// Which representations the float should be printed in.
  #[arg(short, long, default_values = ["binary", "exact", "ulp"])]
  show: Vec<String>,
  #[command(subcommand)]
  command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
  /// Prints various information about a float.
  Show {
    /// The type of float.
    /// Can be a C type (float, double), Rust type (f32, f64),
    /// or a custom float type custom(<exponent>, <significand>)
    #[arg(id = "type")]
    type_: String,
    /// The value of the float. Can be a decimal number (0.34), or a hexadecimal or binary representation (prefixed with 0x or 0b)
    fvalue: String,
  },
  /// List all supported printers that can be used with the --show (-s) flag.
  Printers {}
}

fn print_using_printer(printer: &dyn Printer, val: &Float) {
  let strs = printer.print(&val);
  let pname = printer.name();
  println!("{}: {}", pname, &strs[0]);
  strs.iter().skip(1).for_each(|v| {
    println!("{}  {}", " ".repeat(pname.len()), v);
  });
}

fn main() {
  let args = Cli::parse();
  let printers = collect_printers();
  match args.command {
    Commands::Show { type_, fvalue } => {
      let ftype = FloatParameters::parse(&type_);
      let Some(ftype) = ftype else {
        eprintln!("{}Error parsing type \"{}\"{}", RED, type_, RESET);
        return;
      };
      let fvalue_v = Float::parse(&fvalue, &ftype);
      let Ok(fvalue) = fvalue_v else {
        eprintln!("{}Error parsing float \"{}\": {:?}{}", RED, fvalue, fvalue_v.unwrap_err(), RESET);
        return;
      };

      let printers = args.show.iter().filter_map(|v| {
        let p = printers.get(v).cloned();
        if p.is_none() {
          eprintln!("{}Unknown printer: {}{}", RED, v, RESET);
        }
        p
      }).collect::<Vec<_>>();

      printers.iter().for_each(|v| print_using_printer(v.as_ref(), &fvalue));
    }
    Commands::Printers { .. } => {
      printers.iter().for_each(|(k, v)| {
        println!("{}: {}", k, v.description());
      })
    }
  }
}
