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
}