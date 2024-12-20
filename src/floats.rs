use std::iter::repeat;
use bitvec::field::BitField;
use bitvec::macros::internal::funty::Floating;
use bitvec::order::Lsb0;
use thiserror::Error;

pub type BitVec = bitvec::vec::BitVec<u32, Lsb0>;
pub type BitSlice = bitvec::slice::BitSlice<u32, Lsb0>;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct FloatParameters {
  pub exp_bits: usize,
  pub exp_bias: u64,
  pub sig_bits: usize,
  pub pmin: usize,
}

pub const F32_PARAMS: FloatParameters = FloatParameters {
  exp_bits: 8,
  exp_bias: 127,
  sig_bits: 23,
  pmin: 9
};

pub const F64_PARAMS: FloatParameters = FloatParameters {
  exp_bits: 11,
  exp_bias: 1023,
  sig_bits: 52,
  pmin: 17
};

impl FloatParameters {
  pub fn parse(s: &str) -> Option<Self> {
    if s == "double" || s == "f64" {
      Some(F64_PARAMS)
    } else if s == "float" || s == "f32" {
      Some(F32_PARAMS)
    } else {
      None
    }
  }

  pub fn validate(&self) {
    if self.exp_bits == 0 {
      panic!("Exp bits must be greater than 0");
    } else if self.exp_bits >= 64 {
      panic!("Exp bits must be smaller than 64");
    } else if self.exp_bias >= (1 << self.exp_bits) {
      panic!("Exp bias is too large");
    } else if self.sig_bits == 0 {
      panic!("Sig bits must be larger than 0");
    } else if self.sig_bits >= (1 << 63) {
      panic!("Sig bits must be smaller than 2^63");
    }
  }

  pub fn total_length(&self) -> usize {
    self.exp_bits + self.sig_bits + 1
  }
}

pub struct Float {
  params: FloatParameters,
  bits: BitVec
}

impl Float {
  pub fn parse(s: &str, params: &FloatParameters) -> Result<Self, FloatParseError> {
    params.validate();
    let mut bits = if s.starts_with("0x") {
      bitvec_from_hex(&s[2..], params.total_length())?
    } else if s.starts_with("0b") {
      bitvec_from_bitstr(&s[2..], params.total_length())?
    } else if params == &F32_PARAMS {
      //todo manual parsing
      let f = s.parse::<f32>().map_err(|_| FloatParseError::InvalidDecimalLiteral)?;
      let mut bits = BitVec::from_iter(repeat(false).take(32));
      bits.store_le(f.to_bits());
      bits
    } else if params == &F64_PARAMS {
      let f = s.parse::<f64>().map_err(|_| FloatParseError::InvalidDecimalLiteral)?;
      let mut bits = BitVec::from_iter(repeat(false).take(64));
      bits.store_le(f.to_bits());
      bits
    } else {
      return Err(FloatParseError::NoParser);
    };
    while bits.len() < params.total_length() {
      bits.push(false);
    }
    Ok(Float {
      params: params.clone(),
      bits,
    })
  }

  pub fn sign(&self) -> bool {
    *self.bits.last().unwrap()
  }

  pub fn exponent(&self) -> &BitSlice {
    let s = self.params.sig_bits;
    let e = self.params.exp_bits;
    &self.bits[s..s + e]
  }

  pub fn exponent_unbiased(&self) -> u64 {
    let exp = self.exponent();
    let exp_unbiased = exp.load_le::<u64>();
    exp_unbiased
  }

  pub fn exponent_biased(&self) -> i64 {
    self.exponent_unbiased().wrapping_sub(self.params.exp_bias) as i64
  }

  pub fn significand(&self) -> &BitSlice {
    &self.bits[0..self.params.sig_bits]
  }

  pub fn params(&self) -> &FloatParameters {
    &self.params
  }

  pub fn classify(&self) -> FloatClass {
    if self.exponent().all() {
      if self.significand().not_any() {
        if self.sign() {
          FloatClass::NegativeInf
        } else {
          FloatClass::PositiveInf
        }
      } else {
        if *self.significand().last().unwrap() {
          FloatClass::QuietNaN
        } else {
          FloatClass::SignallingNaN
        }
      }
    } else if self.exponent().not_any() {
      if self.significand().not_any() {
        if self.sign() {
          FloatClass::NegativeZero
        } else {
          FloatClass::PositiveZero
        }
      } else {
        if self.sign() {
          FloatClass::NegativeSubnormal
        } else {
          FloatClass::PositiveSubnormal
        }
      }
    } else {
      if self.sign() {
        FloatClass::NegativeNormal
      } else {
        FloatClass::PositiveNormal
      }
    }
  }
}

fn bitvec_from_hex(s: &str, len: usize) -> Result<BitVec, FloatParseError> {
  let mut vec = BitVec::with_capacity(len);
  s.as_bytes().iter().rev().try_for_each(|v| {
    let byte: u8 = match *v {
      b'0' => 0,
      b'1' => 1,
      b'2' => 2,
      b'3' => 3,
      b'4' => 4,
      b'5' => 5,
      b'6' => 6,
      b'7' => 7,
      b'8' => 8,
      b'9' => 9,
      b'a' | b'A' => 0xa,
      b'b' | b'B' => 0xb,
      b'c' | b'C' => 0xc,
      b'd' | b'D' => 0xd,
      b'e' | b'E' => 0xe,
      b'f' | b'F' => 0xf,
      _ => return Err(FloatParseError::InvalidHexDigit)
    };
    vec.push((byte & 1) != 0);
    vec.push((byte & 2) != 0);
    vec.push((byte & 4) != 0);
    vec.push((byte & 8) != 0);
    Ok(())
  }).map(|()| vec)
}

fn bitvec_from_bitstr(s: &str, len: usize) -> Result<BitVec, FloatParseError> {
  let mut vec = BitVec::with_capacity(len);
  s.as_bytes().iter().rev().try_for_each(|v| {
    let byte = match *v {
      b'0' => false,
      b'1' => true,
      _ => return Err(FloatParseError::InvalidBinaryDigit)
    };
    vec.push(byte);
    Ok(())
  }).map(|()| vec)
}

#[derive(Error, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FloatParseError {
  #[error("Invalid hex digit")]
  InvalidHexDigit,
  #[error("Invalid binary digit")]
  InvalidBinaryDigit,
  #[error("Too many float bits")]
  TooLong,
  #[error("Invalid decimal literal")]
  InvalidDecimalLiteral,
  #[error("Must specify hex or binary for non-standard float format")]
  NoParser
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FloatClass {
  PositiveInf,
  PositiveNormal,
  PositiveSubnormal,
  PositiveZero,
  NegativeZero,
  NegativeSubnormal,
  NegativeNormal,
  NegativeInf,
  SignallingNaN,
  QuietNaN,
}

impl FloatClass {
  pub fn inf(self) -> bool {
    matches!(self, FloatClass::PositiveInf | FloatClass::NegativeInf)
  }
  pub fn finite(self) -> bool {
    self.normal() | self.subnormal() | self.zero()
  }
  pub fn normal(self) -> bool {
    matches!(self, FloatClass::PositiveNormal | FloatClass::NegativeNormal)
  }
  pub fn subnormal(self) -> bool {
    matches!(self, FloatClass::PositiveSubnormal | FloatClass::NegativeSubnormal)
  }
  pub fn zero(self) -> bool {
    matches!(self, FloatClass::PositiveZero | FloatClass::NegativeZero)
  }
  pub fn nan(self) -> bool {
    matches!(self, FloatClass::QuietNaN | FloatClass::SignallingNaN)
  }
  pub fn positive(self) -> bool {
    matches!(self, FloatClass::PositiveInf | FloatClass::PositiveNormal | FloatClass::PositiveSubnormal | FloatClass::PositiveZero)
  }
  pub fn negative(self) -> bool {
    matches!(self, FloatClass::NegativeInf | FloatClass::NegativeNormal | FloatClass::NegativeSubnormal | FloatClass::NegativeZero)
  }
}