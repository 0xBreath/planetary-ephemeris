use std::fs::File;
use std::path::PathBuf;
use csv;
use ephemeris::Time;
use crate::Candle;

#[derive(Clone, Debug)]
pub struct TickerData {
  pub headers: Vec<String>,
  pub data: Vec<Candle>,
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

    Self {
      headers,
      data,
    }
  }

  pub fn find_local_highs(&self) -> Vec<Candle> {
    // identify a daily reversal by checking maximum/minimum for period (day - 5) .. (day + 5)
    let mut local_highs = Vec::<Candle>::new();
    for (index, index_candle) in self.data.iter().enumerate() {
      let mut not_max = false;

      for i in (index-10)..(index+10) {
        if i >= self.data.len() {
          continue;
        }
        let candle = &self.data[i];
        // invalid local high
        if candle.close > index_candle.close {
          not_max = true;
        }
      }
      if !not_max {
        local_highs.push(index_candle.clone());
      }
    }
    local_highs
  }

  pub fn find_highest_high(&self) -> Candle {
    let local_highs = self.find_local_highs();
    // compare Highs. If LowerHigh occurs, then previous High is HTF_High
    let mut highest_high = local_highs.get(0).unwrap().clone();
    for local_high in local_highs.into_iter() {
      if local_high.close > highest_high.close {
        highest_high = local_high;
      }
    }
    highest_high
  }

  pub fn find_local_lows(&self) -> Vec<Candle> {
    // identify a daily reversal by checking maximum/minimum for period (day - 5) .. (day + 5)
    let mut local_lows = Vec::<Candle>::new();
    for (index, index_candle) in self.data.iter().enumerate() {
      let mut not_min = false;

      for i in (index-10)..(index+10) {
        if i >= self.data.len() {
          continue;
        }
        let candle = &self.data[i];
        // invalid local high
        if candle.close < index_candle.close {
          not_min = true;
        }
      }
      if !not_min {
        local_lows.push(index_candle.clone());
      }
    }
    local_lows
  }

  pub fn find_lowest_low(&self) -> Candle {
    let local_lows = self.find_local_highs();
    // compare Highs. If LowerHigh occurs, then previous High is HTF_High
    let mut lowest_low = local_lows.get(0).unwrap().clone();
    for local_low in local_lows.into_iter() {
      if local_low.close < lowest_low.close {
        lowest_low = local_low;
      }
    }
    lowest_low
  }
}