use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::File;
use std::path::PathBuf;
use csv;
use log::debug;
use crate::Time;
use crate::*;
use std::io::Error;
use std::str::FromStr;


#[derive(Debug, Clone)]
pub struct ReversalPrediction {
  pub date: Time,
  pub candle: Option<Candle>
}

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
  hashmap: HashMap<u64, Candle>,
  hasher: CandleHasher
}

impl Default for TickerData {
  fn default() -> Self {
    Self {
      candles: Vec::<Candle>::new(),
      hashmap: HashMap::new(),
      hasher: CandleHasher::new()
    }
  }
}

impl TickerData {
  pub fn new() -> Self {
    Self::default()
  }

  /// Read candles from CSV file.
  /// Handles duplicate candles and sorts candles by date.
  /// Expects date of candle to be in UNIX timestamp format.
  /// CSV format: date,open,high,low,close,volume
  pub fn add_csv_series(&mut self, csv_path: &PathBuf) -> Result<(), Error> {
    let file_buffer = File::open(csv_path).unwrap();
    let mut csv = csv::Reader::from_reader(file_buffer);

    let mut headers = Vec::new();
    if let Ok(result) = csv.headers() {
      for header in result {
        headers.push(String::from(header));
      }
    }

    for record in csv.records().flatten() {
      let date = Time::from_unix(
        record[0].parse::<i64>().expect("failed to parse candle UNIX timestamp into i64")
      );
      let volume = match record[5].parse::<String>() {
        Err(_) => None,
        Ok(vol) => {
          if vol == "NaN" {
            None
          } else {
            let vol = f64::from_str(&vol).expect("failed to parse candle volume into f64");
            Some(vol)
          }
        }
      };
      let candle = Candle {
        date,
        open: f64::from_str(&record[1]).expect("failed to parse candle volume into f64"),
        high: f64::from_str(&record[2]).expect("failed to parse candle volume into f64"),
        low: f64::from_str(&record[3]).expect("failed to parse candle volume into f64"),
        close: f64::from_str(&record[4]).expect("failed to parse candle volume into f64"),
        volume
      };
      self.append_candle(&candle);
    }
    Ok(())
  }

  /// Append vector of candles received from an API to existing candles.
  /// Handles duplicate candles and sorts candles by date.
  pub fn add_series(&mut self, new_candles: Vec<Candle>) -> Result<(), Error> {
    for candle in new_candles.into_iter() {
      self.append_candle(&candle);
    }
    Ok(())
  }

  /// If candle does not exist in self.candles, append candle to self.candles.
  /// Sort candles by date.
  fn append_candle(&mut self, candle: &Candle) {
    let key = self.hasher.hash_candle(candle);
    if let Entry::Vacant(e) = self.hashmap.entry(key) {
      e.insert(candle.clone());
      self.candles.push(candle.clone());
      self.candles.sort_by(|a, b| a.date.partial_cmp(&b.date).expect("Failed to partial compare candle dates"));
    }
  }

  pub fn scale(&mut self, factor: f64) -> Result<(), Error> {
    let mut candles = Vec::<Candle>::new();
    for candle in self.candles.iter() {
      let mut copy = candle.clone();
      copy.open *= factor;
      copy.high *= factor;
      copy.low *= factor;
      copy.close *= factor;
      candles.push(copy);
    }
    self.candles = candles;
    Ok(())
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

  pub fn candle_is_high(&self, candle: &Candle, reversal_margin: usize, error_margin: usize) -> bool {
    let local_highs = self.find_local_highs(reversal_margin);
    for local_high in local_highs.iter() {
      if local_high.date.delta_date(-(error_margin as i64)) <= candle.date &&
        local_high.date.delta_date(error_margin as i64) >= candle.date
      {
        return true;
      }
    }
    false
  }

  pub fn candle_is_low(&self, candle: &Candle, reversal_margin: usize, error_margin: usize) -> bool {
    let local_lows = self.find_local_highs(reversal_margin);
    for local_low in local_lows.iter() {
      if local_low.date.delta_date(-(error_margin as i64)) <= candle.date &&
        local_low.date.delta_date(error_margin as i64) >= candle.date
      {
        return true;
      }
    }
    false
  }

  fn get_candle_by_date(&self, date: &Time) -> Option<Candle> {
    for candle in self.candles.iter() {
      if candle.date == *date {
        return Some(candle.clone())
      }
    }
    return None
  }

  fn get_square_price_periods(&self, reversal: &Reversal) -> Vec<u32> {
    let price_extreme = match reversal.reversal_type {
      ReversalType::Top => reversal.candle.high,
      ReversalType::Bottom => reversal.candle.low
    }.to_string();

    let price_pieces = price_extreme.split('.').collect::<Vec<&str>>();
    let price: String = match price_pieces.len() > 1 {
      false => {
        let price = price_pieces.first().unwrap().to_string();
        price
      },
      true => {
        let integer = price_pieces.first().unwrap().to_string();
        let decimal = *price_pieces.get(1).unwrap();
        let price = integer + decimal;
        price
      }
    };
    let period = price[0..2].parse::<u32>().unwrap();
    // TODO: best cutoff to use single digit instead of double digit period?
    //  50 day period becomes 5 days
    if period > 50 {
      vec![period, period / 10]
    } else {
      vec![period]
    }
  }

  pub fn square_price_reversals(&self, candle_range: usize) -> Vec<ReversalPrediction> {
    let mut time_cycle_reversals = Vec::<ReversalPrediction>::new();
    let reversals = self.find_reversals(candle_range);
    // finds all reversals defined as +/- candle_range, which is 20 right now.
    for reversal in reversals.iter() {
      // get price extreme for that reversal, which is high or low depending
      // 1-2 periods. $15000, then it returns 15. If $60000, then it returns 60 and 6
      let square_price_periods: Vec<u32> = self.get_square_price_periods(reversal);
      for period in square_price_periods.iter() {
        let future_reversal_date = reversal.candle.date.delta_date(*period as i64);
        match self.get_candle_by_date(&future_reversal_date) {
          Some(future_reversal_candle) => {
            time_cycle_reversals.push(ReversalPrediction {
              date: future_reversal_date,
              candle: Some(future_reversal_candle)
            });
          },
          None => {
            time_cycle_reversals.push(ReversalPrediction {
              date: future_reversal_date,
              candle: None
            });
          }
        }
      }
    }
    time_cycle_reversals.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());
    time_cycle_reversals
  }

  // TODO: better system for finding reversals
  /// Find price extremes (highs and lows) in a given range of candles +/- the extreme candle.
  pub fn find_reversals(&self, candle_range: usize) -> Vec<Reversal> {
    let mut reversals = Vec::<Reversal>::new();
    for (index, index_candle) in self.candles.iter().enumerate() {
      if index < candle_range || index + candle_range > self.candles.len() - 1 {
        continue;
      }
      let range = &self.candles[index - (candle_range)..(index + candle_range)];
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