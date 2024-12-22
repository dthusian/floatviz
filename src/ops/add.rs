use std::cmp::{max, min};
use std::fmt::{Write};
use std::iter::repeat;
use std::mem::swap;
use num_bigint::{BigInt, Sign};
use crate::fenv::FloatingPointEnv;
use crate::floats::{BitVec, Float, FloatClass, FloatParameters};
use crate::ops::{Exception, Op};
use crate::printers::{bit2char, print_bitset, DARK_GRAY, PINK, RESET, YELLOW};

pub struct AddSub(pub bool);

impl Op for AddSub {
  fn num_params(&self) -> usize {
    2
  }

  fn execute(&self, env: &FloatingPointEnv, params: &[Float], output_type: &FloatParameters) -> (Float, Exception) {
    todo!()
  }

  fn execute_visual(&self, f: &mut dyn Write, env: &FloatingPointEnv, params: &[Float], output_type: &FloatParameters) -> Result<(Float, Exception), std::fmt::Error> {
    let mut a = &params[0];
    let mut b = &params[1];

    writeln!(f, "\n1. Classify inputs\n")?;
    let a_class = params[0].classify();
    let b_class = params[1].classify();
    // check nans
    if a_class.nan() {
      writeln!(f, "- Input A is NaN, return NaN")?;
      return Ok((Float::nan(&output_type), Exception::INVALID_OPERATION))
    }
    if b_class.nan() {
      writeln!(f, "- Input B is NaN, return NaN")?;
      return Ok((Float::nan(&output_type), Exception::INVALID_OPERATION))
    }

    // check infs
    let a_inf = a_class.inf();
    let b_inf = b_class.inf();
    let sub = a.sign() ^ b.sign() ^ self.0;
    match (a_inf, b_inf) {
      (true, true) => {
        if sub {
          writeln!(f, "- Operation simplifies to Infinity - Infinity, return NaN")?;
          return Ok((Float::nan(&output_type), Exception::INVALID_OPERATION))
        }
        if a.sign() {
          writeln!(f, "- Operation simplifies to -(Infinity + Infinity), return -Infinity")?;
          return Ok((Float::inf(&output_type, true), Exception::default()))
        } else {
          writeln!(f, "- Operation simplifies to Infinity + Infinity, return Infinity")?;
          return Ok((Float::inf(&output_type, false), Exception::default()))
        }
      }
      (true, false) | (false, true) => {
        if (a_inf && a.sign()) || (b_inf && b.sign()) {
          writeln!(f, "- Operation simplifies to -Infinity +/- Finite, return -Infinity")?;
          return Ok((Float::inf(&output_type, true), Exception::default()))
        } else {
          writeln!(f, "- Operation simplifies to Infinity +/- Finite, return Infinity")?;
          return Ok((Float::inf(&output_type, false), Exception::default()))
        }
      }
      _ => {}
    }

    // we are now sure that the numbers are finite
    debug_assert!(a_class.finite());
    debug_assert!(b_class.finite());
    writeln!(f, "- Both inputs are finite")?;

    if sub && a.sign() {
      writeln!(f, "- Swapping a and b to turn -a + b into b - a")?;
      swap(&mut a, &mut b);
    }

    writeln!(f, "\n2. Align significands and {}\n", if sub { "subtract" } else { "add" })?;

    let a_sig = a.significand_logical();
    let b_sig = b.significand_logical();
    let a_exp = a.exponent_logical();
    let b_exp = b.exponent_logical();
    let left_digit = max(a_exp, b_exp); // = max_exp
    let min_exp = min(a_exp, b_exp);
    let right_digit = min(a_exp - a_sig.len() as i64, b_exp - b_sig.len() as i64);
    debug_assert!(left_digit - right_digit >= 0);
    let diff = (left_digit - right_digit) as usize;

    const EXTRA_PREPAD: usize = 2;
    const EXTRA_POSTPAD: usize = 1;
    fn print_significand(f: &mut dyn Write, sig: &BitVec, prepad: usize, postpad: usize) -> std::fmt::Result {
      write!(f, "{}{}{}{}", " ".repeat(prepad), PINK, bit2char(*sig.last().unwrap()), YELLOW)?;
      print_bitset(f, &sig[0..sig.len() - 1])?;
      write!(f, "{}{}...{}\n", DARK_GRAY, "0".repeat(postpad), RESET)?;
      Ok(())
    }
    print_significand(f, &a_sig, (left_digit - a_exp) as usize + EXTRA_PREPAD, (a_exp - a_sig.len() as i64 - right_digit) as usize + EXTRA_POSTPAD)?;
    print_significand(f, &b_sig, (left_digit - b_exp) as usize + EXTRA_PREPAD, (b_exp - a_sig.len() as i64 - right_digit) as usize + EXTRA_POSTPAD)?;
    //todo account for diff too large

    writeln!(f, "-{}-", "-".repeat(diff))?;

    let mut ai = BigInt::from_slice(Sign::Plus, a_sig.as_raw_slice());
    ai <<= a_exp - min_exp;
    let mut bi = BigInt::from_slice(Sign::Plus, b_sig.as_raw_slice());
    bi <<= b_exp - min_exp;
    let qi = if sub { ai - bi } else { ai + bi };
    let mut q_sig = BitVec::from_vec(qi.to_u32_digits().1);
    // remove zeroes from the MSB end
    if let Some(last_one) = q_sig.last_one() {
      q_sig.truncate(last_one + 1);
    }

    let q_sign = qi.sign() == Sign::Minus;
    if q_sign {
      write!(f, "-")?;
    } else {
      write!(f, " ")?;
    }
    print_significand(f, &q_sig, diff + EXTRA_PREPAD - 1 - q_sig.len(), EXTRA_POSTPAD)?;

    // compute q_exp before messing with the significand
    let q_exp = right_digit + q_sig.len() as i64;
    // pad zeroes on the LSB end until it's long enough
    // todo n^2
    while q_sig.len() < output_type.sig_bits + 1 {
      q_sig.insert(0, false);
    }

    writeln!(f, "\n3. Round to destination format.\n")?;
    writeln!(f, "- The current rounding mode is: {:?}", env.rounding_mode)?;
    let q_chop = q_sig.len().saturating_sub(output_type.sig_bits).saturating_sub(1);
    let q_sig_truncated = &q_sig[q_chop..];
    let q_low_bits = q_sig[..q_chop].any();
    // at least 2^emax * (2 - ulp) => wraps to inf
    // equivalent to if exp = max_exp && all sig bits are 1
    if q_exp > output_type.max_exp() || q_exp == output_type.max_exp() && q_sig_truncated.all() {
      // overflow
      writeln!(f, "- The output is too large, wrapping to infinity")?;
      Ok((Float::inf(output_type, a.sign()), Exception::OVERFLOW))
    } else if q_exp < output_type.min_exp() {
      // subnormal
      writeln!(f, "- The raw exponent of the output is {}, but the minimum possible exponent is {}, encoding as subnormal", q_exp, output_type.min_exp())?;
      let shift = output_type.min_exp() - q_exp;
      let mut q_sig = q_sig_truncated.to_owned();
      q_sig.shift_left(shift as usize); // this shifts towards zero
      Ok((Float::from_parts(output_type, q_sign, output_type.min_exp(), &q_sig[..output_type.sig_bits]), Exception::default()))
    } else {
      // normal
      writeln!(f, "- The raw exponent of the output is {}, encoding as normal", q_exp)?;
      Ok((Float::from_parts(output_type, q_sign, q_exp, &q_sig_truncated[..output_type.sig_bits]), Exception::default()))
    }

    //todo round properly
  }
}