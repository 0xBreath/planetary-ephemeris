use crate::{Alignment, Origin, Planet, Query, Reversal, ReversalInfo, Time};

/// Vector of harmonic angles between two planets for a period of time.
/// Compares all combinations of planets; a matrix of planetary alignments.
pub type PlanetAlignmentMatrix = Vec<(Planet, Planet, Vec<(Time, f32, Alignment)>)>;
/// Vector of harmonic angles of a planet relative to its starting angle for a period of time.
/// A matrix with itself.
pub type SinglePlanetAlignments = Vec<(Planet, f32, Vec<(Time, f32, Alignment)>)>;

/// TODO: query each planet alignments and cache to reference. Right now there are duplicate queries.
/// Compare geocentric right ascension of two planets.
/// Compare each planet to all other planets (matrix).
pub async fn two_planets_absolute_align(
  origin: Origin,
  start_time: &Time,
  period_days: i64,
) -> PlanetAlignmentMatrix {
  let planets = vec![
    Planet::Sun,
    Planet::Moon,
    Planet::Mercury,
    Planet::Venus,
    Planet::Mars,
    Planet::Jupiter,
    Planet::Saturn,
    Planet::Uranus,
    Planet::Neptune,
    Planet::Pluto,
  ];

  let mut matrix: PlanetAlignmentMatrix = Vec::new();

  for (index, planet_a) in planets.iter().enumerate() {
    let planet_a_alignments = Query::query(
      origin,
      planet_a,
      start_time.clone(),
      period_days
    ).await;
    for planet_b_index in (index+1)..planets.len() {
      let planet_b = &planets[planet_b_index];
      let planet_b_alignments = Query::query(
        origin,
        planet_b,
        start_time.clone(),
        period_days
      ).await;

      let mut vec: Vec<(Time, f32, Alignment)> = Vec::new();
      for (
        (time, planet_a_ra),
        (_, planet_b_ra)
      ) in planet_a_alignments.iter().zip(planet_b_alignments.iter()) {
        let angle = (planet_a_ra - planet_b_ra).abs();
        let alignment = Alignment::find_alignment(*planet_a_ra, *planet_b_ra, 7.0);
        if let Some(alignment) = alignment {
          vec.push((time.clone(), angle, alignment));
        }
      }
      Query::remove_duplicate_values(&mut vec);
      matrix.push((planet_a.clone(), planet_b.clone(), vec));
    }
  }
  matrix
}

/// TODO: query each planet alignments and cache to reference. Right now there are duplicate queries.
/// Compare geocentric right ascension of two planets relative to start date.
/// Compare each planet to all other planets (matrix).
/// Subtract alignment of start date to get "relative" alignment for a given date.
pub async fn two_planets_align_with_start_angle(
  origin: Origin,
  start_time: &Time,
  period_days: i64,
) -> PlanetAlignmentMatrix {
  let planets = vec![
    Planet::Sun,
    Planet::Moon,
    Planet::Mercury,
    Planet::Venus,
    Planet::Mars,
    Planet::Jupiter,
    Planet::Saturn,
    Planet::Uranus,
    Planet::Neptune,
    Planet::Pluto,
  ];

  let mut matrix: PlanetAlignmentMatrix = Vec::new();

  for (index, planet_a) in planets.iter().enumerate() {
    let planet_a_alignments = Query::query(
      origin,
      planet_a,
      start_time.clone(),
      period_days
    ).await;
    for planet_b_index in (index+1)..planets.len() {
      let planet_b = &planets[planet_b_index];
      let planet_b_alignments = Query::query(
        origin,
        planet_b,
        start_time.clone(),
        period_days
      ).await;

      let mut vec: Vec<(Time, f32, Alignment)> = Vec::new();
      let start_alignment = (planet_a_alignments[0].1 - planet_b_alignments[0].1).abs();
      //println!("{}-{}\tStart Alignment: {}", planet_a.to_str(), planet_b.to_str(), start_alignment);
      for (
        (time, planet_a_ra),
        (_, planet_b_ra)
      ) in planet_a_alignments.iter().zip(planet_b_alignments.iter()) {
        let angle = (planet_a_ra - planet_b_ra).abs();
        let alignment = Alignment::find_alignment(angle, start_alignment, 2.0);
        if let Some(alignment) = alignment {
          vec.push((time.clone(), (angle - start_alignment).abs(), alignment));
        }
      }
      Query::remove_duplicate_values(&mut vec);
      matrix.push((planet_a.clone(), planet_b.clone(), vec));
    }
  }
  matrix
}

/// Square of 9 method: double range low planet angle to find range high planet angle.
/// Return (starting range low angle, date of range high angle, range high angle)
pub async fn range_high_using_planet_at_range_low(
  origin: Origin,
  start_time: &Time,
  planet: &Planet,
) -> Option<(f32, Time, f32)> {

  let alignments = Query::query(
    origin,
    planet,
    start_time.clone(),
    5000
  ).await;

  let start_ra = alignments[0].1;
  let range_high = Alignment::square_of_nine_compute(ReversalInfo {
    reversal_type: Reversal::RangeLow,
    reversal_angle: start_ra,
    alignment: Alignment::Conjunct,
  });

  for (index, (time, ra)) in alignments.into_iter().enumerate() {
    // TODO: better system for skipping immediate conjunct alignment for first days after start
    if index < 10 {
      continue;
    }
    let alignment = Alignment::find_alignment(ra, range_high, 3.0);
    if let Some(alignment) = alignment {
      if alignment == Alignment::Conjunct {
        return Some((start_ra, time, range_high))
      }
    }
  }
  None
}

/// Square of 9 method: halve range high planet angle to find range low planet angle.
/// Return (starting range high angle, date of range low angle, range low angle)
pub async fn range_low_using_planet_at_range_high(
  origin: Origin,
  start_time: &Time,
  planet: &Planet,
) -> Option<(f32, Time, f32)> {

  let alignments = Query::query(
    origin,
    planet,
    start_time.clone(),
    5000
  ).await;

  let start_ra = alignments[0].1;
  let range_low = Alignment::square_of_nine_compute(ReversalInfo {
    reversal_type: Reversal::RangeHigh,
    reversal_angle: start_ra,
    alignment: Alignment::Conjunct,
  });

  for (index, (time, ra)) in alignments.into_iter().enumerate() {
    // TODO: better system for skipping immediate conjunct alignment for first days after start
    if index < 10 {
      continue;
    }
    let alignment = Alignment::find_alignment(ra, range_low, 3.0);
    if let Some(alignment) = alignment {
      if alignment == Alignment::Conjunct {
        return Some((start_ra, time, range_low))
      }
    }
  }
  None
}

/// Compute dates of lunar phases
pub async fn lunar_phases(period_days: i64, start_time: Time) -> Vec<(Time, f32, Alignment)> {
  let moon = Query::query(
    Origin::Geocentric,
    &Planet::Moon,
    start_time.clone(),
    period_days
  ).await;
  let sun = Query::query(
    Origin::Geocentric,
    &Planet::Sun,
    start_time.clone(),
    period_days
  ).await;

  let mut vec: Vec<(Time, f32, Alignment)> = Vec::new();
  for (
    (moon_time, moon_ra),
    (_, sun_ra)
  ) in moon.iter().zip(sun.iter()) {
    let angle = (moon_ra - sun_ra).abs();

    let alignment = Alignment::find_alignment(*moon_ra, *sun_ra, 7.0);
    if let Some(alignment) = alignment {
      match alignment {
        Alignment::Conjunct => {
          println!("{:?}", moon_time.as_string());
          println!("\t {}°, New Moon", angle);
          vec.push((moon_time.clone(), angle, alignment));
        },
        Alignment::Opposite => {
          println!("{:?}", moon_time.as_string());
          println!("\t {}°, Full Moon", angle);
          vec.push((moon_time.clone(), angle, alignment));
        },
        Alignment::Square => {
          println!("{:?}", moon_time.as_string());
          println!("\t {}°, Quarter", angle);
          vec.push((moon_time.clone(), angle, alignment));
        },
        _ => {}
      }
    }
  }
  vec
}














