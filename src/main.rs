use std::collections::BTreeMap;
use std::fmt::Write;
use std::rc::Rc;
use clap::{Parser, Subcommand};
use crate::fenv::{FloatingPointEnv, RoundingMode};
use crate::floats::{Float, FloatParameters, F64_PARAMS};
use crate::ops::collect_ops;
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
    #[arg(id = "TYPE")]
    type_: String,
    /// The value of the float. Can be a decimal number (0.34), or a hexadecimal or binary representation (prefixed with 0x or 0b)
    value: String,
  },
  /// Performs an operation on two numbers.
  Op {
    /// Arguments, alternative type and value
    args: Vec<String>
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

fn print_many(value: &Float, show: &[String], printers: &BTreeMap<String, Rc<dyn Printer>>) {
  let printers = show.iter().filter_map(|v| {
    let p = printers.get(v).cloned();
    if p.is_none() {
      eprintln!("{}Unknown printer: {}{}", RED, v, RESET);
    }
    p
  }).collect::<Vec<_>>();

  printers.iter().for_each(|v| print_using_printer(v.as_ref(), &value));
}

fn main() {
  let args = Cli::parse();
  let printers = collect_printers();
  let ops = collect_ops();
  match args.command {
    Commands::Show { type_, value } => {
      let ftype = FloatParameters::parse(&type_);
      let Some(ftype) = ftype else {
        eprintln!("{}Error parsing type \"{}\"{}", RED, type_, RESET);
        return;
      };
      let fvalue_v = Float::parse(&value, &ftype);
      let Ok(fvalue) = fvalue_v else {
        eprintln!("{}Error parsing float \"{}\": {}{}", RED, value, fvalue_v.unwrap_err(), RESET);
        return;
      };

      print_many(&fvalue, &args.show, &printers);
    }
    Commands::Printers { .. } => {
      printers.iter().for_each(|(k, v)| {
        println!("{}: {}", k, v.description());
      })
    }
    Commands::Op { args: args2 } => {
      //todo undog
      let at = FloatParameters::parse(&args2[0]).unwrap();
      let a = Float::parse(&args2[1], &at).unwrap();
      let bt = FloatParameters::parse(&args2[2]).unwrap();
      let b = Float::parse(&args2[3], &bt).unwrap();

      println!("\x1b[1mInput A\x1b[0m");
      print_many(&a, &args.show, &printers);
      println!();

      println!("\x1b[1mInput A\x1b[0m");
      print_many(&b, &args.show, &printers);
      println!();

      println!("---");

      let add = ops.get("add".into()).unwrap();
      let mut s = String::new();
      let (ret, exp) = add.execute_visual(&mut s, &FloatingPointEnv {
        rounding_mode: RoundingMode::TiesToEven,
        flush_subnormals_to_zero: false,
      }, &[a, b], &F64_PARAMS).unwrap();
      println!("{}", s);

      println!("---");

      println!("\x1b[1mResult\x1b[0m");
      print_many(&ret, &args.show, &printers);
    }
  }
}
