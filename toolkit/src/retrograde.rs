use std::path::PathBuf;
use log::debug;
use ephemeris::*;
use time_series::*;


pub type ConfluentRetrograde = (Time, Vec<RetrogradeEvent>);

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RetrogradeEvent {
  pub planet: Planet,
  pub start_date: Time,
  pub start_angle: f32,
  pub end_date: Time,
  pub end_angle: f32,
}

#[derive(Debug, Clone)]
pub struct Retrograde {
  pub retrogrades: Vec<RetrogradeEvent>,
  pub start_date: Time,
  pub end_date: Time
}

impl Retrograde {
  /// Search time period for retrograde events
  pub async fn new(start_date: Time, end_date: Time, planets: &Vec<Planet>) -> std::io::Result<Self> {
    if start_date > end_date {
      return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Start date must be before end date"));
    }
    let mut retrogrades = Vec::new();
    for planet in planets.iter() {
      let daily_angles = Query::query(
        Origin::Geocentric,
        planet,
        DataType::RightAscension,
        start_date,
        end_date
      ).await.expect("failed to query planet angles");

      // retrograde identified as longitude decreasing (except)
      // covers case where angle passes through 360
      let mut in_retrograde = false;
      let mut retro_start_date: Option<Time> = None;
      let mut retro_start_angle: Option<f32> = None;
      for index in 0..daily_angles.len() - 1 {
        let (date, angle) = daily_angles[index];
        let (_, next_angle) = daily_angles[index + 1];
        // start of retrograde
        if Self::is_retrograde(angle, next_angle) && !in_retrograde {
          in_retrograde = true;
          retro_start_date = Some(date);
          retro_start_angle = Some(angle);
          continue;
        }
        // end of retrograde
        else if !Self::is_retrograde(angle, next_angle) && in_retrograde {
          in_retrograde = false;
          if retro_start_date.is_some() && retro_start_angle.is_some() {
            let start_date = retro_start_date.expect("failed to unwrap retrograde start date");
            let start_angle = retro_start_angle.expect("failed to unwrap retrograde start angle");
            retrogrades.push(RetrogradeEvent {
              planet: planet.clone(),
              start_date,
              start_angle,
              end_date: date,
              end_angle: angle,
            });
            debug!(
              "{}\t{} at {} to {} at {}",
              planet.to_str(), start_date.as_string(), start_angle, date.as_string(), angle
            );
            retro_start_date = None;
            retro_start_angle = None;
          } else if retro_start_date.is_none() {
            panic!("Retrograde start date not set");
          }
          else if retro_start_angle.is_none() {
            panic!("Retrograde start angle not set");
          }
        }
      }
    }
    Ok(Self {
      retrogrades,
      start_date,
      end_date
    })
  }

  // TODO: identify when >1 retrograde events happens within error margin of days
  pub fn confluent_retrograde(&self, error_margin_days: u8) -> Vec<ConfluentRetrograde> {
    // guaranteed to be positive i64 because of error catch in Self::new()
    let period = self.start_date.diff_days(&self.end_date);
    let mut confluent_retrogrades = Vec::<ConfluentRetrograde>::new();
    for index in 0..period {
      let date = self.start_date.delta_date(index);
      let range_low = date.delta_date(-(error_margin_days as i64));
      let range_high = date.delta_date(error_margin_days as i64);
      let mut events_on_date = Vec::<RetrogradeEvent>::new();
      for retrograde in self.retrogrades.iter() {
        if retrograde.start_date.within_range(range_low, range_high)
          || retrograde.end_date.within_range(range_low, range_high) {
          events_on_date.push(retrograde.clone());
        }
      }
      if events_on_date.len() > 1 {
        confluent_retrogrades.push((date, events_on_date.clone()));
        println!("{}\t{}", date.as_string(), events_on_date.len());
      }
    }
    confluent_retrogrades
  }

  pub fn is_retrograde(first: f32, second: f32) -> bool {
    second - first < 0.0
  }

  // TODO: correlate retrograde entry/exit with top/bottom reversal
  /// Search `Candle` history for reversals on retrograde start or end dates
  pub async fn backtest(
    &self,
    reversal_candle_range: usize,
    price_factor: f64,
    error_margin_days: u8
  ) -> std::io::Result<()> {
    let ticker_data = TickerData::new_quandl_api(self.start_date, self.end_date, price_factor).await;
    let start = &ticker_data.earliest_date();
    let end = &ticker_data.latest_date();
    println!("ticker data earliest date: {}", ticker_data.earliest_date().as_string());
    println!("ticker data latest date: {}", ticker_data.latest_date().as_string());

    println!("Reversal defined by +/- {} candles adjacent to reversal are higher/lower", reversal_candle_range);
    println!("Time period: {} to {}", start.as_string(), end.as_string());
    let reversals = ticker_data.find_reversals(reversal_candle_range);

    // iterate over reversals
    // iterate over retrograde events
    // if `RetrogradeEvent` start_date or end_date is within margin of error of candle date
    // reversal is considered a win
    let mut start_win_counts_top = vec![0; Planet::to_vec().len()];
    let mut start_win_counts_bottom = vec![0; Planet::to_vec().len()];
    let mut end_win_counts_top = vec![0; Planet::to_vec().len()];
    let mut end_win_counts_bottom = vec![0; Planet::to_vec().len()];
    let mut total_counts = vec![0; Planet::to_vec().len()];
    for retrograde in self.retrogrades.iter() {
      let start_date_margin_low = retrograde.start_date.delta_date(-(error_margin_days as i64));
      let start_date_margin_high = retrograde.start_date.delta_date(error_margin_days as i64);
      let end_date_margin_low = retrograde.end_date.delta_date(-(error_margin_days as i64));
      let end_date_margin_high = retrograde.end_date.delta_date(error_margin_days as i64);

      let planet_index = Planet::to_vec().iter().position(|p| p == &retrograde.planet).unwrap();
      for reversal in reversals.iter() {
        if reversal.candle.date.within_range(start_date_margin_low, start_date_margin_high) {
          match reversal.reversal_type {
            ReversalType::Top => start_win_counts_top[planet_index] += 1,
            ReversalType::Bottom => start_win_counts_bottom[planet_index] += 1,
          }
          println!("{}\tSTART\t{}", retrograde.planet.to_str(), retrograde.end_date.as_string());
          // retrograde event should only line up with one reversal candle, so break afterwards
          break;
        } else if reversal.candle.date.within_range(end_date_margin_low, end_date_margin_high) {
          match reversal.reversal_type {
            ReversalType::Top => end_win_counts_top[planet_index] += 1,
            ReversalType::Bottom => end_win_counts_bottom[planet_index] += 1,
          }
          println!("{}\tEND\t{}", retrograde.planet.to_str(), retrograde.end_date.as_string());
          // retrograde event should only line up with one reversal candle, so break afterwards
          break;
        }
      }
      total_counts[planet_index] += 1;
    }
    println!("PLANET\t\tWIN RATE\tSTART TOP\tSTART BOTTOM\tEND TOP\t\tEND BOTTOM\tTOTAL COUNT");
    for (index, planet) in Planet::to_vec().iter().enumerate() {
      let start_win_count_top = start_win_counts_top[index];
      let end_win_count_top = end_win_counts_top[index];
      let start_win_count_bottom = start_win_counts_bottom[index];
      let end_win_count_bottom = end_win_counts_bottom[index];
      let total_count = total_counts[index];
      let total_win_count_start = start_win_count_top + start_win_count_bottom;
      let total_win_count_end = end_win_count_top + end_win_count_bottom;
      let total_win_count = total_win_count_start + total_win_count_end;

      let win_rate = (total_win_count as f32 / total_count as f32) * 100.0;
      let _start_win_rate = (total_win_count_start as f32 / total_count as f32) * 100.0;
      let _end_win_rate = (total_win_count_end as f32 / total_count as f32) * 100.0;
      let start_win_rate_top = (start_win_count_top as f32 / total_count as f32) * 100.0;
      let start_win_rate_bottom = (start_win_count_bottom as f32 / total_count as f32) * 100.0;
      let end_win_rate_top = (end_win_count_top as f32 / total_count as f32) * 100.0;
      let end_win_rate_bottom = (end_win_count_bottom as f32 / total_count as f32) * 100.0;
      println!(
        "{}\t\t{}%\t\t{}%\t\t{}%\t\t{}%\t\t{}%\t\t{}",
        planet.to_str(), win_rate.round(),
        start_win_rate_top.round(), start_win_rate_bottom.round(),
        end_win_rate_top.round(), end_win_rate_bottom.round(), total_count
      );
    }
    Ok(())
  }
}