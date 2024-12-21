use num_bigint::{BigInt, Sign};
use crate::floats::Float;

// Assumes finite
pub fn float_to_exact_str(f: &Float) -> String {
  let params = &f.params();
  let mut significand = BigInt::from_slice(Sign::Plus, f.significand_logical().as_raw_slice());
  let exponent = f.exponent_logical();
  if exponent >= params.sig_bits as i64 {
    // precision greater than 1, shift significant left and convert to dec
    significand <<= exponent;
    let mut s = significand.to_string();
    if f.sign() {
      s.insert(0, '-');
    }
    s
  } else {
    // compute significand * 10^? / 2^(sig_bits-exp)
    let required_zeroes = (params.sig_bits as i64 - exponent) as usize;
    fn count_trailing_zeroes_big(b: &BigInt) -> usize {
      for (i, d) in b.iter_u32_digits().enumerate() {
        let tz = d.trailing_zeros() as usize;
        if tz != 32 {
          return i * 32 + tz;
        }
      }
      usize::MAX
    }

    let mut decimal_shift = 0;
    while count_trailing_zeroes_big(&significand) < required_zeroes {
      significand *= 10;
      decimal_shift += 1;
    }
    significand >>= required_zeroes;

    let mut s = significand.to_string();
    if decimal_shift <= s.len() && decimal_shift != 0 {
      s.insert(s.len() - decimal_shift, '.');
    } else if decimal_shift > s.len() {
      let insert_zeroes = decimal_shift - s.len();
      s = format!(".{}{}", "0".repeat(insert_zeroes), s);
    }
    if f.sign() {
      s.insert(0, '-');
    }
    s
  }
}