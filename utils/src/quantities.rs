
pub struct Quantities {
  pub value: String,
}

/// [Documentation](https://ssd.jpl.nasa.gov/horizons/manual.html#output)
impl Quantities {
  /// 23 = Sun-Observer-Target ELONGATION angle
  /// 24 = Sun-Target-Observer ~PHASE angle
  pub fn geocentric() -> Self {
    Self {
      value: String::from("&QUANTITIES='24'"),
    }
  }
  /// 18 = Heliocentric ecliptic longitude & latitude
  pub fn heliocentric() -> Self {
    Self {
      value: String::from("&QUANTITIES='18'"),
    }
  }
}