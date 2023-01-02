use log::debug;
use ephemeris::*;
use time_series::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RetrogradeEvent {
  planet: Planet,
  start_date: Time,
  start_angle: f32,
  end_date: Time,
  end_angle: f32,
}

#[derive(Debug, Clone)]
pub struct Retrograde {
  pub retrogrades: Vec<RetrogradeEvent>,
  pub ticker_data: TickerData,
}

impl Retrograde {
  /// Search time period for retrograde events
  pub async fn new(ticker_data: TickerData) -> Self {
    let earliest_date = ticker_data.earliest_date();
    let time_period = Time::today().diff_days(&earliest_date);

    let mut retrogrades = Vec::new();
    for planet in Planet::to_vec().iter() {
      let daily_angles = Query::query(
        Origin::Geocentric,
        planet,
        DataType::RightAscension,
        Time::today(),
        time_period
      ).await;

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
    Self {
      retrogrades,
      ticker_data
    }
  }

  pub fn is_retrograde(first: f32, second: f32) -> bool {
    second - first < 0.0
  }

  /// Search `Candle` history for reversals on retrograde start or end dates
  pub fn backtest(&self, reversal_candle_range: usize) {
    println!("Reversal candle range: {}", reversal_candle_range);
    println!("Time period: {}", Time::today().diff_days(&self.ticker_data.earliest_date()));
    let reversals = self.ticker_data.find_reversals(reversal_candle_range);

    // iterate over reversals
    // iterate over retrograde events
    // if `RetrogradeEvent` start_date or end_date is within margin of error of candle date
    // reversal is considered a win
    let mut start_win_counts = vec![0; Planet::to_vec().len()];
    let mut end_win_counts = vec![0; Planet::to_vec().len()];
    let mut total_counts = vec![0; Planet::to_vec().len()];
    for retrograde in self.retrogrades.iter() {
      let planet_index = Planet::to_vec().iter().position(|p| p == &retrograde.planet).unwrap();
      for reversal in reversals.iter() {
        if retrograde.start_date == reversal.candle.date {
          start_win_counts[planet_index] += 1;
          // retrograde event should only line up with one reversal candle, so break afterwards
          break;
        } else if retrograde.end_date == reversal.candle.date {
          end_win_counts[planet_index] += 1;
          // retrograde event should only line up with one reversal candle, so break afterwards
          break;
        }
      }
      total_counts[planet_index] += 1;
    }
    println!("PLANET\t\tWIN RATE\tSTART RATE\tEND RATE\tSTART COUNT\tEND COUNT\tTOTAL COUNT");
    for (index, planet) in Planet::to_vec().iter().enumerate() {
      let start_win_count = start_win_counts[index];
      let end_win_count = end_win_counts[index];
      let total_count = total_counts[index];
      let total_win_count = start_win_count + end_win_count;

      let win_rate = (total_win_count as f32 / total_count as f32) * 100.0;
      let start_win_rate = (start_win_count as f32 / total_count as f32) * 100.0;
      let end_win_rate = (end_win_count as f32 / total_count as f32) * 100.0;
      println!(
        "{}\t\t{}%\t\t{}%\t\t{}%\t\t{}\t\t{}\t\t{}",
        planet.to_str(), win_rate.round(),
        start_win_rate.round(), end_win_rate.round(),
        start_win_count, end_win_count, total_count
      );
    }
  }
}