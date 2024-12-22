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
    /// The operation to perform
    op: String,
    /// Arguments, alternative type and value
    args: Vec<String>
  },
  /// List all supported printers that can be used with the --show (-s) flag.
  Printers {},
  /// List all supported operations.
  Operations {}
}

fn print_using_printer(printer: &dyn Printer, val: &Float) {
  let strs = printer.print(&val);
  let pname = printer.name();
  println!("{}: {}", pname, &strs[0]);
  strs.iter().skip(1).for_each(|v| {
    println!("{}  {}", " ".repeat(pname.len()), v);
  });
}

fn print_float(value: &Float, show: &[String], printers: &BTreeMap<String, Rc<dyn Printer>>) {
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

      print_float(&fvalue, &args.show, &printers);
    }
    Commands::Printers { .. } => {
      printers.iter().for_each(|(k, v)| {
        println!("{}: {}", k, v.description());
      })
    }
    Commands::Op { op, args: args2 } => {
      let box_op = ops.get(&op);
      let Some(box_op) = box_op else {
        eprintln!("{}Unknown operation: {}{}", RED, op, RESET);
        return;
      };

      if args2.len() != box_op.num_params() * 2 {
        eprintln!("{}Wrong number of arguments, expected {}{}", RED, box_op.num_params(), RESET);
        return;
      }
      let params = args2.chunks(2).map(|v| {
        let ty = FloatParameters::parse(&v[0]).unwrap();
        let float = Float::parse(&v[1], &ty).unwrap();
        float
      }).collect::<Vec<_>>();

      let letters = "ABCDEFG";
      params.iter().zip(letters.chars()).for_each(|(float, name)| {
        println!("\x1b[1mInput {}\x1b[0m", name);
        print_float(&float, &args.show, &printers);
        println!();
      });

      println!("---");
      let mut s = String::new();
      let (ret, exp) = box_op.execute_visual(&mut s, &FloatingPointEnv {
        rounding_mode: RoundingMode::TiesToEven,
        flush_subnormals_to_zero: false,
      }, &params, &F64_PARAMS).unwrap();
      println!("{}", s);

      println!("---");
      println!();

      println!("\x1b[1mResult\x1b[0m");
      print_float(&ret, &args.show, &printers);
    }
    Commands::Operations { .. } => {
      ops.iter().for_each(|(k, v)| {
        println!("{}", k);
      })
    }
  }
}
