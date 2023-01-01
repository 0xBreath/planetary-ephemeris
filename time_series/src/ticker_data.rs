use std::fs::File;
use std::path::PathBuf;
use csv;
use log::debug;
use ephemeris::Time;
use crate::Candle;

#[derive(Debug, Clone)]
pub enum ReversalType {
  Top,
  Bottom
}
impl PartialEq for ReversalType {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (ReversalType::Top, ReversalType::Top) => true,
      (ReversalType::Bottom, ReversalType::Bottom) => true,
      _ => false,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Reversal {
  pub candle: Candle,
  pub reversal_type: ReversalType,
}

#[derive(Clone, Debug)]
pub struct TickerData {
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

  /// Get reference to `Vec<Candle>` from `TickerData`.
  pub fn get_candles(&self) -> &Vec<Candle> {
    &self.candles
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

  /// Find price extremes (highs and lows) in a given range of candles +/- the extreme candle.
  pub fn find_reversals(&self, candle_range: usize) -> Vec<Reversal> {
    let mut reversals = Vec::<Reversal>::new();
    for (index, index_candle) in self.candles.iter().enumerate() {
      if index < candle_range || index + 10 > self.candles.len() - 1 {
        continue;
      }
      let range = &self.candles[index - candle_range..index + candle_range];
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

  /// Compute volatility for a candle.
  /// Returns a ratio (percentage / 100).
  pub fn candle_volatility(candle: &Candle) -> f64 {
    if candle.high > candle.low {
      candle.high / candle.low
    } else {
      candle.low / candle.high
    }
  }

  /// Compute mean volatility for candles.
  /// Returns mean as ratio (percentage / 100).
  pub fn mean_volatility_candles(&self) -> Option<f64> {
    let mut sum_spans = 0.0;
    for candle in self.candles.iter() {
      sum_spans += Self::candle_volatility(candle);
    }
    match self.candles.len() {
      positive if positive > 0 => Some(sum_spans / self.candles.len() as f64),
      _ => None
    }
  }

  /// Compute standard deviation volatility for candles.
  /// Returns std dev as ratio (percentage / 100).
  fn std_dev_volatility_candles(&self) -> Option<f64> {
    if !self.candles.is_empty() {
      match self.mean_volatility_candles() {
        Some(mean_vol) => {
          let variance = self.candles.iter().map(|candle| {
            let diff = mean_vol - ((candle.high / candle.low) as f64);
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

  /// Compute Z-Score volatility for a candle.
  /// Z-Score is the number of standard deviations a candle's volatility spans away from the mean of the data set.
  /// >2 standard deviations is significant. 3 is extreme.
  pub fn candles_z_score_volatility(&self, candle: &Candle) -> Option<f64> {
    if !self.candles.is_empty() {
      let mean = self.mean_volatility_candles().expect("Mean volatility is not defined");
      let std_dev = self.std_dev_volatility_candles().expect("Std dev volatility is not defined");
      let candle_vol = Self::candle_volatility(candle);
      let z_score = candle_vol - mean / std_dev;
      Some(z_score)
    } else {
      None
    }
  }
}