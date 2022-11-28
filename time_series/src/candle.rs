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