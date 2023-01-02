use ephemeris::*;
use time_series::*;
use time_series::Reversal;
use crate::PlanetLongitudes;

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
  pub retrogrades: Vec<RetrogradeEvent>
}

impl Retrograde {
  /// Search time period for retrograde events
  pub async fn new(time_period: i64) -> Self {
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
            println!(
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
    Self { retrogrades }
  }

  pub fn is_retrograde(first: f32, second: f32) -> bool {
    second - first < 0.0
  }

  // /// Search `Candle` history for reversals on retrograde start or end dates
  // pub fn backtest(&self, reversals: &Vec<Reversal>) {}
}