use utils::*;

pub async fn heliocentric(planet: &Planet, period_days: i64) -> String {
  let command = Target::new(planet);
  let today = Time::today();
  let start_time = Time::new(today.year, &today.month, &today.day);
  let stop_time = start_time.delta_date(period_days);
  let quantities = Quantities::default();
  let query = Query::heliocentric(command, start_time, stop_time, quantities);

  let data = reqwest::get(query.value)
    .await
    .expect("failed to request heliocentric data")
    .text()
    .await
    .expect("failed to read response");
  extract_data(data)
}

pub async fn geocentric(planet: &Planet, period_days: i64) -> Vec<(Time, f32)> {
  let command = Target::new(planet);
  let today = Time::today();
  let start_time = Time::new(today.year, &today.month, &today.day);
  let stop_time = start_time.delta_date(period_days);
  let quantities = Quantities::default();
  let query = Query::geocentric(command, start_time, stop_time, quantities);

  let data = reqwest::get(query.value)
    .await
    .expect("failed to request geocentric data")
    .text()
    .await
    .expect("failed to read response");
  let data = extract_data(data);
  format_to_vec(data)
}

pub async fn lunar_phases() -> Vec<(Time, f32, Alignment)> {
  let moon = geocentric(&Planet::Moon, 30).await;
  let sun = geocentric(&Planet::Sun, 30).await;

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

// matrix of planet alignments relative to one another
pub async fn alignment_matrix(period_days: i64) -> Vec<(Planet, Planet, Vec<(Time, f32, Alignment)>)> {
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

  let mut matrix: Matrix = Vec::new();

  for (index, planet_a) in planets.iter().enumerate() {
    let planet_a_alignments = geocentric(planet_a, period_days).await;
    for planet_b_index in (index+1)..planets.len() {
      let planet_b = &planets[planet_b_index];
      let planet_b_alignments = geocentric(planet_b, period_days).await;

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
      remove_duplicate_values(&mut vec);
      matrix.push((planet_a.clone(), planet_b.clone(), vec));
    }
  }
  matrix
}