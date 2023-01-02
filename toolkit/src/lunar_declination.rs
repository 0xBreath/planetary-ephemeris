use std::path::PathBuf;
use log::debug;
use ephemeris::{DataType, Origin, Planet, Query, Time};
use time_series::{Direction, ReversalType, TickerData};
use crate::TICKER_DATA_PATH;

pub type ZeroDeclinationCrosses = Vec<(Time, f32, Direction)>;

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
      if moon[index].1 < 0.0 && moon[index+1].1 > 0.0  {
        let (time, angle) = moon[index];
        zero_declinations.push((time, angle, Direction::Up));
      } else if moon[index].1 > 0.0 && moon[index+1].1 < 0.0 {
        let (time, angle) = moon[index];
        zero_declinations.push((time, angle, Direction::Down));
      }
      index += 1;
    }
    Self { zero_declinations }
  }

  pub async fn test_lunar_declination(start_date: Time, candle_range: usize, error_margin_days: i64) {
    let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
    let earliest_date = ticker_data.get_candles()[0].date;
    let period_days = Time::today().diff_days(&earliest_date);
    let reversals = ticker_data.find_reversals(candle_range);
    let lunar_declination = LunarDeclination::new(period_days, start_date).await;

    // iterate over lunar_declinations, identify if it is within +/- error_margin_days of a reversal
    // if so, increment win count
    println!("DATE\t\tDECLINATION\tDIRECTION");
    let mut win_count = 0;
    let total_count = lunar_declination.zero_declinations.len();
    for (time, declination, direction) in lunar_declination.zero_declinations {
      println!("{}\t{}°\t{:?}", time.as_string(), declination, direction);
      for reversal in reversals.iter() {
        let range_start = time.delta_date(-(error_margin_days));
        let range_end = time.delta_date(error_margin_days);
        if reversal.candle.date.within_range(range_start, range_end) {
          debug!("{}\t{}°\t{:?}", time.as_string(), declination, direction);

          match reversal.reversal_type {
            // price goes down, moon goes from - to + declination
            ReversalType::Top => {
              match direction {
                Direction::Up => win_count += 1,
                Direction::Down => win_count += 0
              }
            },
            // price goes up, moon goes from + to - declination
            ReversalType::Bottom => {
              match direction {
                Direction::Up => win_count += 0,
                Direction::Down => win_count += 1
              }
            }
          }
        }
      }
    }
    let win_rate = win_count as f64 / total_count as f64 * 100.0;
    println!("Win Rate: {}%\t\tWin Events: {}\t\tTotal Events: {}", win_rate, win_count, total_count);
  }
}