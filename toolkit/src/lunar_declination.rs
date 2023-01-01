use std::path::PathBuf;
use ephemeris::{DataType, Origin, Planet, Query, Time};
use time_series::TickerData;
use crate::TICKER_DATA_PATH;

pub type ZeroDeclinationCrosses = Vec<(Time, f32)>;

#[derive(Debug, Clone)]
pub struct LunarDeclination {
  /// Vector of dates of lunar declinations with precise declination as f32.
  pub zero_declinations: ZeroDeclinationCrosses
}

impl LunarDeclination {
  /// Compute lunar declination
  pub async fn new(period_days: i64, start_time: Time) -> Self {
    let moon = Query::query(
      Origin::Geocentric,
      &Planet::Moon,
      DataType::Declination,
      start_time,
      period_days
    ).await;

    let mut zero_declinations = Vec::new();
    let mut index = 0;
    while index < moon.len() - 2 {
      if (moon[index].1 < 0.0 && moon[index+1].1 > 0.0) || (moon[index].1 > 0.0 && moon[index+1].1 < 0.0){
        zero_declinations.push(moon[index]);
      }
      index += 1;
    }
    Self { zero_declinations }
  }

  pub async fn test_lunar_declination(period_days: i64, start_date: Time, error_margin_days: i64) {
    let lunar_declination = LunarDeclination::new(period_days, start_date).await;
    let candle_range: usize = 10;
    let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
    let reversals = ticker_data.find_reversals(candle_range);

    // iterate over lunar_declinations, identify if it is within +/- error_margin_days of a reversal
    // if so, print the date and the declination
    println!("DATE\t\tDECLINATION");
    for (time, declination) in lunar_declination.zero_declinations {
      for reversal in reversals.iter() {
        let range_start = time.delta_date(-(error_margin_days));
        let range_end = time.delta_date(error_margin_days);
        if reversal.candle.date.within_range(range_start, range_end) {
          println!("{}\t{}°", time.as_string(), declination);
        }
      }
    }
  }
}