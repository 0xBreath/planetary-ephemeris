use std::fs::File;
use std::path::PathBuf;
use csv;
use log::debug;
use ephemeris::Time;
use crate::*;

#[derive(Debug, Clone)]
pub enum FirstMove {
  EngulfingHigh,
  EngulfingLow
}

#[derive(Debug, Clone)]
pub enum ReversalType {
  Top,
  Bottom
}
impl ReversalType {
  pub fn as_string(&self) -> String {
    match self {
      ReversalType::Top => "Top".to_string(),
      ReversalType::Bottom => "Bottom".to_string()
    }
  }
}

impl PartialEq for ReversalType {
  fn eq(&self, other: &Self) -> bool {
    matches!((self, other), (ReversalType::Top, ReversalType::Top) | (ReversalType::Bottom, ReversalType::Bottom))
  }
}

#[derive(Debug, Clone)]
pub struct Reversal {
  pub candle: Candle,
  pub reversal_type: ReversalType,
}

#[derive(Clone, Debug)]
pub struct TickerData {
  /// Candlestick history of a ticker.
  pub candles: Vec<Candle>,
}

impl TickerData {
  pub fn new_from_csv(csv_path: &PathBuf) -> Self {
    let file_buffer = File::open(csv_path).unwrap();
    let mut csv = csv::Reader::from_reader(file_buffer);

    let mut headers = Vec::new();
    if let Ok(result) = csv.headers() {
      for header in result {
        headers.push(String::from(header));
      }
    }

    let mut data = Vec::new();
    for record in csv.records().flatten() {
      let date = Time::from_unix(
        record[0].parse::<i64>().expect("failed to parse candle UNIX timestamp into i64")
      );
      let candle = Candle {
        date,
        open: record[1].parse().unwrap(),
        high: record[2].parse().unwrap(),
        low: record[3].parse().unwrap(),
        close: record[4].parse().unwrap(),
      };
      data.push(candle);
    }

    Self { candles: data }
  }

  /// Limited to 2 years of historical date (free plan)
  pub async fn new_polygon_api(ticker_symbol: TickerSymbol, start_date: Time, end_date: Time) -> Self {
    let polygon = PolygonApiWrapper::new(ticker_symbol, start_date, end_date).await;

    let bars = polygon.response["results"].as_array().unwrap();
    let mut candles = Vec::<Candle>::new();
    for bar in bars.iter() {
      candles.push(Candle {
        date: Time::from_unix_msec(bar["t"].as_i64().unwrap()),
        open: bar["o"].as_f64().unwrap(),
        high: bar["h"].as_f64().unwrap(),
        low: bar["l"].as_f64().unwrap(),
        close: bar["c"].as_f64().unwrap(),
      });
    }

    Self { candles }
  }

  pub async fn new_quandl_api(start_date: Time, end_date: Time, factor: f64) -> Self {
    let quandl = QuandlApiWrapper::new(start_date, end_date).await;
    let mut data = Self { candles: quandl.candles };
    data = data.scale(factor);
    data
  }

  pub fn from_candles(candles: Vec<Candle>) -> Self {
    Self { candles }
  }

  pub fn scale(&self, factor: f64) -> Self {
    let mut candles = Vec::<Candle>::new();
    for candle in self.candles.iter() {
      let mut copy = candle.clone();
      copy.open *= factor;
      copy.high *= factor;
      copy.low *= factor;
      copy.close *= factor;
      candles.push(copy);
    }
    Self::from_candles(candles)
  }

  /// Get reference to `Vec<Candle>` from `TickerData`.
  pub fn get_candles(&self) -> &Vec<Candle> {
    &self.candles
  }

  pub fn earliest_date(&self) -> &Time {
    &self.get_candles()[0].date
  }

  pub fn latest_date(&self) -> &Time {
    &self.get_candles()[self.candles.len() - 1].date
  }

  /// Find price extreme (highs) in a given range of candles +/- the extreme candle.
  pub fn find_local_highs(&self, candle_range: usize) -> Vec<Candle> {
    // identify a daily reversal by checking maximum/minimum for period (day - candle_range)..(day + candle_range)
    let mut local_highs = Vec::<Candle>::new();
    for (index, index_candle) in self.candles.iter().enumerate() {
      if index < candle_range || index + 10 > self.candles.len() - 1 {
        continue;
      }
      let range = &self.candles[index - candle_range..index + candle_range];
      let mut max_candle: &Candle = range.get(0).unwrap();
      for (index, candle) in range.iter().enumerate() {
        if index >= self.candles.len() {
          break;
        }
        if candle.close >= max_candle.close {
          max_candle = candle;
        }
      }
      if max_candle == index_candle {
        debug!("High: {:?}\t{:?}", max_candle.close, max_candle.date.as_string());
        local_highs.push(index_candle.clone());
      }
    }
    local_highs
  }

  pub fn find_highest_high(&self, candle_range: usize) -> Candle {
    let local_highs = self.find_local_highs(candle_range);
    // compare Highs. If LowerHigh occurs, then previous High is HTF_High
    let mut highest_high = local_highs.get(0).unwrap().clone();
    for local_high in local_highs.into_iter() {
      if local_high.close > highest_high.close {
        highest_high = local_high;
      }
    }
    highest_high
  }

  /// Find price extreme (lows) in a given range of candles +/- the extreme candle.
  pub fn find_local_lows(&self, candle_range: usize) -> Vec<Candle> {
    // identify a daily reversal by checking maximum/minimum for period (day - 5) .. (day + 5)
    let mut local_lows = Vec::<Candle>::new();
    for (index, index_candle) in self.candles.iter().enumerate() {
      if index < candle_range || index + 10 > self.candles.len() - 1 {
        continue;
      }
      let range = &self.candles[index - candle_range..index + candle_range];
      let mut min_candle: &Candle = range.get(0).unwrap();
      for (index, candle) in range.iter().enumerate() {
        if index >= self.candles.len() {
          break;
        }
        if candle.close <= min_candle.close {
          min_candle = candle;
        }
      }
      if min_candle == index_candle {
        debug!("Low: {:?}\t{:?}", min_candle.close, min_candle.date.as_string());
        local_lows.push(index_candle.clone());
      }
    }
    local_lows
  }

  pub fn find_lowest_low(&self, candle_range: usize) -> Candle {
    let local_lows = self.find_local_highs(candle_range);
    // compare Highs. If LowerHigh occurs, then previous High is HTF_High
    let mut lowest_low = local_lows.get(0).unwrap().clone();
    for local_low in local_lows.into_iter() {
      if local_low.close < lowest_low.close {
        lowest_low = local_low;
      }
    }
    lowest_low
  }

  // TODO: better system for finding reversals
  /// Find price extremes (highs and lows) in a given range of candles +/- the extreme candle.
  pub fn find_reversals(&self, candle_range: usize) -> Vec<Reversal> {
    let mut reversals = Vec::<Reversal>::new();
    for (index, index_candle) in self.candles.iter().enumerate() {
      if index < candle_range || index + candle_range > self.candles.len() - 1 {
        continue;
      }
      let range = &self.candles[index - (candle_range / 2)..(index + candle_range)];
      let mut min_candle: &Candle = range.get(0).unwrap();
      let mut max_candle: &Candle = range.get(0).unwrap();
      for (index, candle) in range.iter().enumerate() {
        if index >= self.candles.len() {
          break;
        }
        if candle.close <= min_candle.close {
          min_candle = candle;
        }
        else if candle.close >= max_candle.close {
          max_candle = candle;
        }
      }
      if min_candle == index_candle {
        debug!("Low: {:?}\t{:?}", min_candle.close, min_candle.date.as_string());
        reversals.push(Reversal {
          candle: index_candle.clone(),
          reversal_type: ReversalType::Bottom,
        });
      }
      else if max_candle == index_candle {
        debug!("High: {:?}\t{:?}", max_candle.close, max_candle.date.as_string());
        reversals.push(Reversal {
          candle: index_candle.clone(),
          reversal_type: ReversalType::Top
        }.clone()
        );
      }
    }
    reversals
  }

  /// Find candles with a Z-Score >1.0 from the candle one period before.
  pub fn find_sushi_roll_reversals(&self, period: usize) -> Vec<Reversal> {
    let mut reversals = Vec::<Reversal>::new();
    for (index, index_candle) in self.candles.iter().enumerate() {
      if index < period {
        continue;
      }
      let period_first_half = period / 2;
      let period_last_half = period - period_first_half;
      let sum_first_half_highs = self.candles[index - period..index - period_last_half].iter().fold(0.0, |sum, candle| sum + candle.high);
      let sum_first_half_lows = self.candles[index - period..index - period_last_half].iter().fold(0.0, |sum, candle| sum + candle.low);
      let mean_high = sum_first_half_highs / period_first_half as f64;
      let mean_low = sum_first_half_lows / period_first_half as f64;
      debug!("Date: {}\tMean High: {}\tMean Low: {}", index_candle.date.as_string(), mean_high, mean_low);

      let last_half_range = &self.candles[index - period_last_half..index];
      let mut first_move: Option<FirstMove> = None;
      for candle in last_half_range.iter() {
        // bullish engulfing high made, wait for bearish engulfing low
        if candle.high > mean_high {
          match first_move {
            // no engulfing candle occurred yet, this is the first move
            None => first_move = Some(FirstMove::EngulfingHigh),
            Some(FirstMove::EngulfingHigh) => continue,
            // bearish engulfing high already made, this engulfing high is the reversal signal
            Some(FirstMove::EngulfingLow) => {
              reversals.push(Reversal {
                candle: candle.clone(),
                reversal_type: ReversalType::Bottom
              });
              break;
            }
          }
        }
        // bearish engulfing low made, wait for bullish engulfing high
        else if candle.low < mean_low {
          match first_move {
            // no engulfing candle occurred yet, this is the first move
            None => first_move = Some(FirstMove::EngulfingLow),
            Some(FirstMove::EngulfingLow) => continue,
            // bullish engulfing high already made, this engulfing low is the reversal signal
            Some(FirstMove::EngulfingHigh) => {
              reversals.push(Reversal {
                candle: candle.clone(),
                reversal_type: ReversalType::Top
              });
              break;
            }
          }
        }
      }
    }
    Self::remove_duplicate_reversals(reversals)
  }

  /// Find price extremes (highs and lows) in a given range of candles +/- the extreme candle.
  pub fn find_engulfing_candle_reversals(&self, candle_range: usize) -> Vec<Reversal> {
    let mut reversals = Vec::<Reversal>::new();
    for (index, index_candle) in self.candles.iter().enumerate() {
      if index < candle_range {
        continue;
      }
      let previous_candle = &self.candles[index - 1];
      let range = &self.candles[index - candle_range..index - 1];
      let mut min_candle: &Candle = range.get(0).unwrap();
      let mut max_candle: &Candle = range.get(0).unwrap();
      for candle in range.iter() {
        if candle.close <= min_candle.close {
          min_candle = candle;
        }
        else if candle.close >= max_candle.close {
          max_candle = candle;
        }
      }
      if min_candle == previous_candle {
        // check index_candle is bullish engulfing
        debug!("Low: {:?}\t{:?}", min_candle.close, min_candle.date.as_string());
        reversals.push(Reversal {
          candle: index_candle.clone(),
          reversal_type: ReversalType::Bottom,
        });
      }
      else if max_candle == previous_candle {
        // check index_candle is bearish engulfing
        debug!("High: {:?}\t{:?}", max_candle.close, max_candle.date.as_string());
        reversals.push(Reversal {
          candle: index_candle.clone(),
          reversal_type: ReversalType::Top
        });
      }
    }
    reversals
  }

  /// Compute mean candle close for `self.period` candles back from `candle`.
  pub fn mean(&self, candle: &Candle, period: usize) -> Option<f64> {
    // search self.candles for candle,
    // if index candle == candle and index > self.period, return index
    // else return None
    for (index, index_candle) in self.candles.iter().enumerate() {
      if index_candle == candle && index >= period {
        let range = &self.candles[(index-period)..index];
        let mut sum = 0.0;
        for candle in range.iter() {
          sum += candle.close;
        }
        return Some(sum / range.len() as f64);
      }
    }
    None
  }

  /// Compute standard deviation of a candle close for `self.period` candles back from `candle`.
  fn std_dev(&self, candle: &Candle, period: usize) -> Option<f64> {
    if !self.candles.is_empty() {
      match self.mean(candle, period) {
        Some(mean_price) => {
          let start_index = self.candles.len() - 1 - period;
          let variance = self.candles.iter().map(|candle| {
            let diff = mean_price - candle.close;
            diff * diff
          }).sum::<f64>() / self.candles.len() as f64;

          Some(variance.sqrt() as f64)
        },
        _ => None
      }
    } else {
      None
    }
  }

  /// Compute Z-Score of a candle for `self.period` candles back from `candle`.
  /// Z-Score is the number of standard deviations a candle's close spans away from the mean of the data set.
  /// >3 standard deviations is significant.
  pub fn z_score(&self, candle: &Candle, period: usize) -> Option<f64> {
    if !self.candles.is_empty() {
      let mean = self.mean(candle, period).expect("Mean is not defined");
      let std_dev = self.std_dev(candle, period).expect("Std dev is not defined");
      let z_score = (candle.close - mean) / std_dev;
      Some(z_score)
    } else {
      None
    }
  }

  /// Remove duplicate Candles from the data set.
  pub fn remove_duplicate_reversals(mut signals: Vec<Reversal>) -> Vec<Reversal> {
    signals.sort_by(|a, b| a.candle.date.partial_cmp(&b.candle.date).expect("failed to compare dates"));
    signals.dedup_by(|a, b| a.candle.date == b.candle.date);
    signals
  }
}