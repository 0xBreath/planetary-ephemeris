use std::cmp::Ordering;
use ephemeris::Time;

/// Event for a single candlestick for a given ticker.
#[derive(Clone, Debug)]
pub struct Candle {
  /// UNIX timestamp in seconds
  pub date: Time,
  /// Open price
  pub open: f64,
  /// High price
  pub high: f64,
  /// Low price
  pub low: f64,
  /// Close price
  pub close: f64,
}

impl PartialEq for Candle {
  fn eq(&self, other: &Self) -> bool {
    self.date.as_string() == other.date.as_string() && self.close == other.close
  }
}