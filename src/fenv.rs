
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RoundingMode {
  /// Rounds to the nearest float, if two floats are
  /// equally far away, then use the one with even LSB
  TiesToEven,
  /// Rounds to the nearest float, if two floats are
  /// equally far away, then use the one with larger magnitude.
  TiesToAway,
  /// Rounds to the nearest float no less than the number.
  TowardPositive,
  /// Rounds to the nearest float no greater than the number.
  TowardNegative,
  /// Rounds to the nearest float with no greater magnitude than the number.
  TowardZero
}

/// Represents settings that the environment uses when executing floating point operations.
/// These include rounding mode, flushing subnormals to zero, etc.
pub struct FloatingPointEnv {
  pub rounding_mode: RoundingMode,
  pub flush_subnormals_to_zero: bool,
}