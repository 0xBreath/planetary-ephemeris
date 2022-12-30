pub mod quantities;
pub mod query;
pub mod step_size;
pub mod target;
pub mod time;
pub mod data_type;
pub mod alignment;
pub mod origin;

pub use quantities::*;
pub use query::*;
pub use step_size::*;
pub use target::*;
pub use time::*;
pub use data_type::*;
pub use alignment::*;
pub use origin::*;

/// Vector of harmonic angles between two planets for a period of time.
/// Compares all combinations of planets; a matrix of planetary alignments.
pub type PlanetMatrix = Vec<(Planet, Planet, Vec<(Time, f32, Alignment)>)>;

/// Compare geocentric right ascension of two planets.
/// Compare each planet to all other planets (matrix).
pub async fn planetary_matrix(
  origin: Origin,
  start_time: &Time,
  period_days: i64,
  alignment_margin_error: f32,
) -> PlanetMatrix {
  let planets = vec![
    Planet::Sun,
    Planet::Mercury,
    Planet::Venus,
    Planet::Mars,
    Planet::Jupiter,
    Planet::Saturn,
    Planet::Uranus,
    Planet::Neptune,
    Planet::Pluto,
  ];

  let mut matrix: PlanetMatrix = Vec::new();

  let mut planet_alignments = Vec::new();
  for planet in planets.iter() {
    planet_alignments.push(Query::query(
      origin,
      planet,
      DataType::RightAscension,
      *start_time,
      period_days
    ).await);
  }

  for (index, planet_a_alignments) in planet_alignments.iter().enumerate() {
    for planet_b_index in (index+1)..planet_alignments.len() {
      let planet_a = &planets[index];
      let planet_b = &planets[planet_b_index];
      let planet_b_alignments = planet_alignments[planet_b_index].clone();

      let mut vec: Vec<(Time, f32, Alignment)> = Vec::new();
      for (
        (time, planet_a_ra),
        (_, planet_b_ra)
      ) in planet_a_alignments.iter().zip(planet_b_alignments.iter()) {
        let angle = (planet_a_ra - planet_b_ra).abs();
        let alignment = Alignment::find_alignment(*planet_a_ra, *planet_b_ra, alignment_margin_error);
        if let Some(alignment) = alignment {
          vec.push((*time, angle, alignment));
        }
      }
      vec = Query::remove_duplicate_values(&mut vec);
      matrix.push((planet_a.clone(), planet_b.clone(), vec));
    }
  }
  matrix
}

/// Compute lunar declination
pub async fn lunar_declination(period_days: i64, start_time: Time) -> Vec<(Time, f32)> {
  let moon = Query::query(
    Origin::Geocentric,
    &Planet::Moon,
    DataType::Declination,
    start_time,
    period_days
  ).await;

  let mut zero_cross = Vec::new();
  let mut index = 0;
  while index < moon.len() - 2 {
    if (moon[index].1 < 0.0 && moon[index+1].1 > 0.0) || (moon[index].1 > 0.0 && moon[index+1].1 < 0.0){
      zero_cross.push(moon[index]);
    }
    index += 1;
  }
  zero_cross
}