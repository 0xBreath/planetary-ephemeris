use std::path::PathBuf;
use ephemeris::*;
use time_series::*;

pub const TICKER_DATA_PATH: &str = "BTCUSD.csv";
pub const RESULTS_PATH: &str = "BTCUSD_results.csv";

pub type Matrix = Vec<(Planet, Planet, Vec<(Time, f32, Alignment)>)>;
#[derive(Debug, Clone, PartialEq)]
pub struct PlanetPairAlignmentWinRate {
  pub planet_1: Planet,
  pub planet_2: Planet,
  pub alignment: Alignment,
  pub alignment_total_count: u32,
  pub win_count: u32
}

#[derive(Debug, Clone)]
pub struct PlanetMatrix {
  /// Vector of harmonic angles between two planets for a period of time.
  /// Compares all combinations of planets; a matrix of planetary alignments.
  pub matrix: Matrix
}
impl PlanetMatrix {
  /// Compare geocentric right ascension of two planets.
  /// Compare each planet to all other planets (matrix).
  pub async fn new(
    origin: Origin,
    start_time: &Time,
    period_days: i64,
    alignment_margin_error: f32,
  ) -> Self {
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
    Self { matrix }
  }

  /// Find the total count of each planet pair alignment
  pub fn build_planet_pair_alignment_counts(&self) -> Vec<PlanetPairAlignmentWinRate> {
    let mut vec = Vec::<PlanetPairAlignmentWinRate>::new();
    for (planet_a, planet_b, alignments) in self.matrix.iter() {
      // index follows `Alignment` enum order
      let mut alignment_counts = vec![0; 16];

      for data in alignments.iter() {
        let alignment = &data.2;
        match alignment {
          Alignment::Conjunct => alignment_counts[0] += 1,
          Alignment::Opposite => alignment_counts[1] += 1,
          Alignment::Trine120 => alignment_counts[2] += 1,
          Alignment::Trine240 => alignment_counts[3] += 1,
          Alignment::Square90 => alignment_counts[4] += 1,
          Alignment::Square270 => alignment_counts[5] += 1,
          Alignment::Quintile72 => alignment_counts[6] += 1,
          Alignment::Quintile144 => alignment_counts[7] += 1,
          Alignment::Quintile216 => alignment_counts[8] += 1,
          Alignment::Quintile288 => alignment_counts[9] += 1,
          Alignment::Sextile60 => alignment_counts[10] += 1,
          Alignment::Sextile300 => alignment_counts[11] += 1,
          Alignment::Octile45 => alignment_counts[12] += 1,
          Alignment::Octile135 => alignment_counts[13] += 1,
          Alignment::Octile225 => alignment_counts[14] += 1,
          Alignment::Octile315 => alignment_counts[15] += 1,
        }
      }
      for (index, alignment) in Alignment::iter().iter().enumerate() {
        vec.push(PlanetPairAlignmentWinRate {
          planet_1: planet_a.clone(),
          planet_2: planet_b.clone(),
          alignment: alignment.clone(),
          alignment_total_count: alignment_counts[index],
          win_count: 0
        });
      }
    }
    vec
  }

  /// Search for a `PlanetPairAlignmentWinRate` by two `Planet` and their `Alignment`
  pub fn get_planet_pair_alignment_win_rate(
    alignment_counts: &[PlanetPairAlignmentWinRate],
    planet_a: &Planet,
    planet_b: &Planet,
    alignment: &Alignment
  ) -> Option<PlanetPairAlignmentWinRate> {
    let mut value: Option<PlanetPairAlignmentWinRate> = None;
    for data in alignment_counts.iter() {
      if data.planet_1.eq(planet_a) && data.planet_2.eq(planet_b) && data.alignment.eq(alignment) {
        value = Some(data.clone());
        break;
      }
    }
    value
  }

  /// NOTE: expects `TICKER_DATA_PATH: &str = "BTCUSD.csv"` to be in root directory
  /// Compare each planet to one other planet for all possible harmonics.
  /// Compare all possibilities of planet pairs as a "matrix" of possibilities.
  /// For each planet pair in the matrix, find if the date (+/- a margin of days) equals a known reversal date (analyzed from existing price data).
  /// A "win" is considered if the alignment occurred on the same day as a known reversal.
  /// Compute the win rate of each planet pair. How likely could the planet pair predict a reversal? (across all harmonics)
  /// Win rate is across all harmonics for a given planet pair.
  pub async fn test_planet_matrix(results_file: &PathBuf, margin_of_error_days: u32, alignment_margin_error: f32, candle_range: usize) {
    let ticker_data = TickerData::new_from_csv(results_file);
    let reversals = ticker_data.find_reversals(candle_range);
    if ticker_data.candles.is_empty() {
      return
    }
    let earliest_candle_date = &ticker_data.get_candles()[0].date;
    let max_history_days = Time::today().diff_days(earliest_candle_date);

    let planet_matrix = PlanetMatrix::new(
      Origin::Geocentric,
      &Time::today(),
      max_history_days,
      alignment_margin_error
    ).await;
    println!("PLANET PAIR\tALIGNMENT\tWIN RATE\tWIN EVENTS\tTOTAL EVENTS");

    let alignment_counts = planet_matrix.build_planet_pair_alignment_counts();

    for (planet_a, planet_b, alignments) in planet_matrix.matrix.into_iter() {
      for (time, _ra, alignment) in alignments.iter() {
        let planet_pair_alignment_win_rate: Option<PlanetPairAlignmentWinRate> =
          PlanetMatrix::get_planet_pair_alignment_win_rate(&alignment_counts, &planet_a, &planet_b, alignment);

        if let Some(mut ppawn) = planet_pair_alignment_win_rate {
          // backtest planet-pair-alignment signal against known reversals
          for reversal in reversals.iter() {
            let range_start = time.delta_date(-(margin_of_error_days as i64));
            let range_end = time.delta_date(margin_of_error_days as i64);
            if reversal.candle.date.within_range(range_start, range_end) {
              // increment the win count for this planet pair alignment
              ppawn.win_count += 1;
              break;
            }
          }
        }
      }
    }
    // print result for each PlanetPairAlignmentCount
    for ppawn in alignment_counts.iter() {
      let win_rate = ((ppawn.win_count as f32 / ppawn.alignment_total_count as f32) * 100.0).round();
      if win_rate.is_nan() {
        println!(
          "{}-{}\t{}\t-\t\t{}\t\t{}",
          ppawn.planet_1.to_str(),
          ppawn.planet_2.to_str(),
          ppawn.alignment.to_str(),
          ppawn.win_count,
          ppawn.alignment_total_count
        );
      } else {
        println!(
          "{}-{}\t{}\t{}%\t\t{}\t\t{}",
          ppawn.planet_1.to_str(),
          ppawn.planet_2.to_str(),
          ppawn.alignment.to_str(),
          win_rate,
          ppawn.win_count,
          ppawn.alignment_total_count
        );
      }
    }
  }
}