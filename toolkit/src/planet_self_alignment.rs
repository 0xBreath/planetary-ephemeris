use std::io::{Error, ErrorKind};
use ephemeris::*;


#[derive(Debug, Clone)]
pub struct SelfAlignment {
  pub planet: Planet,
  pub date: Time,
  pub alignment: Alignment,
}

#[derive(Debug, Clone)]
pub struct PlanetSelfAlignments {
  pub relative_date: Time,
  pub alignment_error_margin: f32,
  pub alignments: Vec<SelfAlignment>,
}

#[derive(Debug, Clone)]
pub struct PlanetAlignments {
  pub earliest_date: Time,
  pub latest_date: Time,
  /// Vector follow `Planet::to_vec()` order
  pub planet_angles: Vec<Vec<(Time, f32)>>
}

impl PlanetAlignments {
  pub async fn new(earliest_date: Time, latest_date: Time) -> Self {

    let mut planet_angles = Vec::<Vec<(Time, f32)>>::new();
    for planet in Planet::to_vec() {
      let angles = Query::query(
        Origin::Geocentric,
        &planet,
        DataType::RightAscension,
        earliest_date,
        latest_date
      ).await.expect("failed to query planet angles");
      planet_angles.push(angles);
    }

    Self {
      earliest_date,
      latest_date,
      planet_angles
    }
  }

  pub fn self_relative_alignments(
    &self,
    relative_date: Time,
    error_margin_degrees: f32
  ) -> Result<PlanetSelfAlignments, Error> {
    if relative_date < self.earliest_date {
      return Err(Error::new(ErrorKind::NotFound, "relative date is before earliest ephemeride date"));
    }
    else if relative_date > self.latest_date {
      return Err(Error::new(ErrorKind::NotFound, "relative date is after latest ephemeride date"));
    }

    let all_planet_angles = &self.planet_angles;
    let mut self_alignments = Vec::<SelfAlignment>::new();

    for (planet_index, planet_angles) in all_planet_angles.iter().enumerate() {
      let planet = &Planet::to_vec()[planet_index];
      // find planet angle on `relative_date`
      // all alignments in `self_alignments` are relative to `start_angle`
      let mut start_index: Option<usize> = None;
      let mut start_angle: Option<f32> = None;
      for (index, (time, _)) in planet_angles.iter().enumerate() {
        if &relative_date == time {
          start_index = Some(index);
          start_angle = Some(planet_angles[index].1);
          break;
        }
      }

      if start_index.is_none() || start_angle.is_none() {
        return Err(Error::new(ErrorKind::NotFound, "relative date not found in planet angles"));
      }
      let start_index = start_index.unwrap();
      let start_angle = start_angle.unwrap();

      let mut vec: Vec<(Time, f32, Alignment)> = Vec::new();
      for index in start_index..planet_angles.len() {
        let (time, angle) = planet_angles[index];

        let alignment = Alignment::find_alignment(angle, start_angle, error_margin_degrees);
        if let Some(alignment) = alignment {
          let relative_angle = (angle - start_angle).abs();
          vec.push((time, relative_angle, alignment));
        }
      }
      // finds duplicate Alignments on consecutive dates where f32 is within margin of error
      // filters for the date with f32 closest to the actual Alignment angle
      vec = Query::remove_duplicate_values(&mut vec);
      // cast Vec<(Time, f32, Alignment)> to Vec<SelfAlignment>
      for (time, _, alignment) in vec.iter() {
        self_alignments.push(SelfAlignment {
          planet: planet.clone(),
          date: *time,
          alignment: alignment.clone()
        });
      };
    }

    Ok(PlanetSelfAlignments {
      relative_date,
      alignment_error_margin: error_margin_degrees,
      alignments: self_alignments
    })
  }
}