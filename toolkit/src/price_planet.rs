use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use log::debug;
use ephemeris::*;
use time_series::*;
use time_series::Reversal;

#[derive(Debug, Clone)]
pub struct PlanetLongitudes {
  pub planet: Planet,
  pub angles: Vec<(Time, f32)>,
}

#[derive(Debug, Clone)]
pub struct PricePlanet {
  ticker_data: TickerData,
  market_structure: MarketStructure,
  square_of_nine: SquareOfNine,
  planet_longitudes: Vec<PlanetLongitudes>,
  harmonics: Vec<f32>,
  backtest_matrix: Vec<Vec<Backtest>>,
  reversal_candle_range: usize,
  time_period: i64,
  backtest_results: PathBuf,
}

impl PricePlanet {
  pub async fn new(
    ticker_data_file_path: PathBuf,
    backtest_results_file_path: PathBuf,
    reversal_candle_range: usize,
    square_of_nine_step: f64,
  ) -> Self {
    let ticker_data = TickerData::new_from_csv(&ticker_data_file_path);
    let market_structure = MarketStructure::new(ticker_data.clone(), reversal_candle_range);

    // TODO: param for max SquareOfNine value -> determines dimension internally
    let dimension = 2001;
    let origin = 1;
    let square_of_nine = SquareOfNine::new(origin, square_of_nine_step, dimension);
    //let earliest_candle_date = &ticker_data.get_candles()[0].date;
    //let time_period = Time::today().diff_days(earliest_candle_date);
    let time_period = -100;

    let harmonics = Alignment::iter().iter().map(|a| a.to_num()).collect::<Vec<f32>>();

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
    let mut planet_longitudes = Vec::<PlanetLongitudes>::new();
    for planet in planets.iter() {
      let angles = Query::query(
        Origin::Geocentric,
        planet,
        DataType::RightAscension,
        Time::today(),
        time_period
      ).await;
      planet_longitudes.push(PlanetLongitudes {
        planet: planet.clone(),
        angles
      });
    }
    let backtest_matrix = vec![vec![Backtest::default(); harmonics.len()]; planets.len()];

    Self {
      ticker_data,
      market_structure,
      square_of_nine,
      planet_longitudes,
      harmonics,
      backtest_matrix,
      reversal_candle_range,
      time_period,
      backtest_results: backtest_results_file_path,
    }
  }

  // pub async fn confluent_price_planet_signals(mut self) {
  //   let ticker_data = self.ticker_data;
  //   let market_structure = self.market_structure;
  //   let reversals = market_structure.reversals.clone();
  //   if ticker_data.candles.is_empty() {
  //     return
  //   }
  //   let square_of_nine = self.square_of_nine;
  //   //let earliest_candle_date = &ticker_data.get_candles()[0].date;
  //   //let time_period = Time::today().diff_days(earliest_candle_date);
  //   let time_period = -730;
  //   println!("Time Period: {} days from today {}", time_period, Time::today().as_string());
  //
  //   let harmonics = self.harmonics;
  //   let mut planet_longitudes = self.planet_longitudes;
  //   let mut backtest_matrix = self.backtest_matrix;
  //   let reversal_candles = reversals.iter().map(|reversal| reversal.candle.clone()).collect::<Vec<Candle>>();
  //   let price_history = ticker_data.get_candles();
  //
  //   // TODO: counter and Vec<&Backtest> for signals that occurred on this date
  //   // TODO: if counter > 1, update win count for each Backtest
  //   // TODO: result is signal occurs when two or more harmonics align on the same candle
  //   for (index, planet_longitudes) in planet_longitudes.into_iter().enumerate() {
  //     let daily_longitudes = planet_longitudes.angles;
  //
  //     for (time, angle) in daily_longitudes.into_iter() {
  //       for (harmonic_index, angle_diff) in harmonics.iter().enumerate() {
  //         // find all prices at angle on the SquareOfNine
  //         let angle = angle_safe_increment(angle, *angle_diff);
  //         let price_equals_time = square_of_nine.find_price_equals_time(angle);
  //         // check if price history candle on the specified date equals (within margin of error)
  //         // any of the SquareOfNine price points at the harmonic angle relative to the planet.
  //         let harmonic_price_on_date = check_candle_hit_price_on_date(
  //           &time,
  //           &price_equals_time,
  //           price_history,
  //           error_margin_price,
  //           error_margin_days
  //         );
  //
  //         // signal occurred
  //         if let Some(harmonic_price_on_date) = harmonic_price_on_date {
  //           // returns Some if true positive signal, None if false positive signal
  //           let backtested_signal: Option<Candle> = backtest_signal(
  //             &reversal_candles,
  //             harmonic_price_on_date,
  //             time,
  //             error_margin_price,
  //           );
  //           if let Some(signal) = &backtested_signal {
  //             backtest_matrix[index][harmonic_index].increment_win_count();
  //             backtest_matrix[index][harmonic_index].add_signal((planets[index].clone(), *angle_diff, signal.clone()));
  //           }
  //           backtest_matrix[index][harmonic_index].increment_total_count();
  //         } else {
  //           debug!("No signal on {}", time.as_string());
  //         }
  //       }
  //     }
  //   }
  //
  //   // write results to console and file
  //   let results_file = &self.backtest_results;
  //   market_structure.print(
  //     results_file,
  //     time_period,
  //     reversal_candle_range,
  //     error_margin_price,
  //     square_of_nine_step,
  //     &planets,
  //     &backtest_matrix,
  //     &square_of_nine
  //   );
  // }

  /// Identify all dates where the price longitude on a Square of Nine equals the longitude of all possible planets (including Sun and Moon)
  /// Backtest the Time=Price against actual reversals in the market
  /// Compute win rate each planet longitude (time) and price longitude (price) for a all possible harmonics (90, 120, 180, etc)
  /// Return matrix of all possibilities of (planet, harmonic, win rate), and print results to file.
  pub async fn single_price_planet_signal(self, error_margin_price: f64, error_margin_days: u32) {
    let ticker_data = self.ticker_data;
    let market_structure = self.market_structure;
    let reversals = market_structure.reversals.clone();
    if ticker_data.candles.is_empty() {
      return
    }
    let square_of_nine = &self.square_of_nine;
    println!("Time Period: {} days from today {}", self.time_period, Time::today().as_string());

    let harmonics = self.harmonics;
    let planet_longitudes = self.planet_longitudes;
    let mut backtest_matrix = self.backtest_matrix;
    let reversal_candles = reversals.iter().map(|reversal| reversal.candle.clone()).collect::<Vec<Candle>>();
    let price_history = ticker_data.get_candles();

    // TODO: counter and Vec<&Backtest> for signals that occurred on this date
    // TODO: if counter > 1, update win count for each Backtest
    // TODO: result is signal occurs when two or more harmonics align on the same candle
    for (index, planet_longitudes) in planet_longitudes.into_iter().enumerate() {
      let daily_longitudes = planet_longitudes.angles;
      let planet = planet_longitudes.planet;

      for (time, angle) in daily_longitudes.into_iter() {
        for (harmonic_index, angle_diff) in harmonics.iter().enumerate() {
          // find all prices at angle on the SquareOfNine
          let angle = Self::angle_safe_increment(angle, *angle_diff);
          let price_equals_time = &square_of_nine.find_price_equals_time(angle);
          // check if price history candle on the specified date equals (within margin of error)
          // any of the SquareOfNine price points at the harmonic angle relative to the planet.
          let harmonic_price_on_date = Self::check_candle_hit_price_on_date(
            &time,
            price_equals_time,
            price_history,
            error_margin_price,
            error_margin_days
          );

          // signal occurred
          if let Some(harmonic_price_on_date) = harmonic_price_on_date {
            // returns Some if true positive signal, None if false positive signal
            let backtested_signal: Option<Candle> = Self::backtest_signal(
              &reversal_candles,
              harmonic_price_on_date,
              time,
              error_margin_price,
            );
            if let Some(signal) = &backtested_signal {
              backtest_matrix[index][harmonic_index].increment_win_count();
              backtest_matrix[index][harmonic_index].add_signal((planet.clone(), *angle_diff, signal.clone()));
            }
            backtest_matrix[index][harmonic_index].increment_total_count();
          } else {
            debug!("No signal on {}", time.as_string());
          }
        }
      }
    }

    // write results to console and file
    PricePlanet::print_single_price_planet_results(
      &reversal_candles,
      error_margin_price,
      &self.backtest_results,
      self.reversal_candle_range,
      &self.square_of_nine,
      self.time_period,
      &backtest_matrix,
    );
  }

  #[allow(clippy::too_many_arguments)]
  pub fn print_single_price_planet_results(
    reversals: &Vec<Candle>,
    margin_of_error: f64,
    results_file: &PathBuf,
    reversal_candle_range: usize,
    square_of_nine: &SquareOfNine,
    time_period: i64,
    backtest_matrix: &[Vec<Backtest>]
  ) {
    let square_of_nine_step = square_of_nine.get_step();
    // write results to console and file
    println!("Number of Reversals in last {} days: {}\r", time_period, reversals.len());
    println!("Reversal defined by price extreme of +/- the adjacent {} candles", reversal_candle_range);
    println!("Margin of Error within actual reversal candle close: {}%", (margin_of_error * 100.0));
    println!("Square of Nine step interval: {}", square_of_nine_step);
    println!("Ring Size at 15001: {}", square_of_nine.ring_size_of_cell_value(10001.0).expect("failed to get ring size"));
    println!("Ring Size at 30001: {}", square_of_nine.ring_size_of_cell_value(30001.0).expect("failed to get ring size"));
    println!("Ring Size at 60001: {}", square_of_nine.ring_size_of_cell_value(60001.0).expect("failed to get ring size"));
    let mut file = File::create(results_file).unwrap();
    let _ = file.write(format!("Number of Reversals in last {} days: {}\r", time_period, reversals.len()).as_bytes())
      .expect("Unable to write to file");
    let _ = file.write(format!("Reversal defined by price extreme of +/- the adjacent {} candles\r", reversal_candle_range).as_bytes())
      .expect("Unable to write to file");
    let _ = file.write(format!("Margin of Error within actual reversal candle close: {}%\r", (margin_of_error * 100.0)).as_bytes())
      .expect("Unable to write to file");
    let _ = file.write(format!("Square of Nine step interval: {}\r", square_of_nine_step).as_bytes())
      .expect("Unable to write to file");
    let _ = file.write(format!(
      "Square of Nine ring size at 10001: {}\r",
      square_of_nine.ring_size_of_cell_value(10001.0).expect("failed to get ring size")
    ).as_bytes()).expect("Unable to write to file");
    let _ = file.write(format!(
      "Square of Nine ring size at 30001: {}\r",
      square_of_nine.ring_size_of_cell_value(30001.0).expect("failed to get ring size")
    ).as_bytes()).expect("Unable to write to file");
    let _ = file.write(format!(
      "Square of Nine ring size at 60001: {}\r",
      square_of_nine.ring_size_of_cell_value(60001.0).expect("failed to get ring size")
    ).as_bytes()).expect("Unable to write to file");

    for (index, planet) in backtest_matrix.iter().enumerate() {
      println!();
      println!("PLANET\t\tALIGNMENT\tWIN RATE\tWIN COUNT\tTOTAL ALIGNMENTS");
      let _ = file.write(format!("\nPLANET\t\tALIGNMENT\tWIN RATE\tWIN COUNT\tTOTAL ALIGNMENTS\n").to_string().as_bytes()).expect("Unable to write to file");
      for backtest in planet.iter() {
        if let Some(signals) = &backtest.signals {
          if signals.len() > 1 {
            let harmonic = signals[0].1;
            let planet = signals[0].0.clone();

            let win_rate = (backtest.get_win_rate() * 100.0).round();
            if win_rate == 100.0 {
              if harmonic < 10.0 {
                let _ = file.write(format!(
                  "{}\t\t{:?}\t\t\t{:?}%\t{:?}\t\t\t{:?}\n",
                  planet.to_str(),
                  harmonic.round(),
                  (backtest.get_win_rate() * 100.0).round(),
                  backtest.get_win_count(),
                  backtest.get_total_count()
                ).to_string().as_bytes()).expect("Unable to write to file");
              } else {
                let _ = file.write(format!(
                  "{}\t\t{:?}\t\t{:?}%\t{:?}\t\t\t{:?}\n",
                  planet.to_str(),
                  harmonic.round(),
                  (backtest.get_win_rate() * 100.0).round(),
                  backtest.get_win_count(),
                  backtest.get_total_count()
                ).to_string().as_bytes()).expect("Unable to write to file");
              }
              println!(
                "{}\t\t{:?}\t\t{:?}%\t{:?}\t\t{:?}",
                planet.to_str(),
                harmonic.round(),
                (backtest.get_win_rate() * 100.0).round(),
                backtest.get_win_count(),
                backtest.get_total_count()
              );
            } else {
              if harmonic < 10.0 {
                let _ = file.write(format!(
                  "{}\t\t{:?}\t\t\t{:?}%\t\t{:?}\t\t\t{:?}\n",
                  planet.to_str(),
                  harmonic.round(),
                  (backtest.get_win_rate() * 100.0).round(),
                  backtest.get_win_count(),
                  backtest.get_total_count()
                ).to_string().as_bytes()).expect("Unable to write to file");
              } else {
                let _ = file.write(format!(
                  "{}\t\t{:?}\t\t{:?}%\t\t{:?}\t\t\t{:?}\n",
                  planet.to_str(),
                  harmonic.round(),
                  (backtest.get_win_rate() * 100.0).round(),
                  backtest.get_win_count(),
                  backtest.get_total_count()
                ).to_string().as_bytes()).expect("Unable to write to file");
              }
              println!(
                "{}\t\t{:?}\t\t{:?}%\t\t{:?}\t\t{:?}",
                planet.to_str(),
                harmonic.round(),
                (backtest.get_win_rate() * 100.0).round(),
                backtest.get_win_count(),
                backtest.get_total_count()
              );
            }
          }
        }
      }
    }
    println!("
      The “win rates” are the odds the algorithm would have known the day a reversal will occur\r
      and been within {}% of entering a trade at the close of that reversal candle.\r
      Backtest is for BTCUSD {} days from today {}.\r",
             (margin_of_error * 100.0), time_period, Time::today().as_string()
    );
    let _ = file.write(format!("\n
      The “win rates” are the odds the algorithm would have known the day a reversal will occur\r
      and been within {}% of entering a trade at the close of that reversal candle.\r
      Backtest is for BTCUSD {} days from today {}.\r",
                               (margin_of_error * 100.0), time_period, Time::today().as_string()).as_bytes()
    ).expect("Unable to write to file");
  }

  /// Check if any of the price points on the SquareOfNine at a specific angle
  /// are within the margin of error to the close, high, or low of the actual candle on that date.
  /// Returns a vector of all Candles on the date that were within margin by the candle close, high, or low.
  pub fn check_candle_hit_price_on_date(
    date: &Time,
    angle_price_points: &[Point],
    price_history: &[Candle],
    error_margin_price: f64,
    error_margin_days: u32
  ) -> Option<Point> {
    let candle_on_date = price_history.iter().find(|candle| {
      let range_start = date.delta_date(-(error_margin_days as i64));
      let range_end = date.delta_date(error_margin_days as i64);
      candle.date.within_range(range_start, range_end)
    });

    let mut signal = None;
    if let Some(candle_on_date) = candle_on_date {
      for point in angle_price_points.iter() {
        // SquareOfNine price price within margin of error to actual candle close, high, or low
        if SquareOfNine::within_margin_of_error(candle_on_date.close, point.value, error_margin_price) ||
          SquareOfNine::within_margin_of_error(candle_on_date.high, point.value, error_margin_price) ||
          SquareOfNine::within_margin_of_error(candle_on_date.low, point.value, error_margin_price)
        {
          debug!("Signal: {}\tClose: {}\tHigh: {}\tLow: {}", point.value, candle_on_date.close, candle_on_date.low, candle_on_date.high);
          signal = Some(*point);
        }
      }
    }
    signal
  }

  /// TODO: many prices fall along price equals time. How to know which price the reversal is if many prices could hit in one candle?
  /// Search vector of price reversals for reversal on date.
  /// Search SquareOfNine for all prices (harmonic points) along longitude.
  /// Return Vector of SquareOfNine harmonic points (prices) that equal the reversals price +/- margin of error.
  /// Determines the win rate of "price in alignment with planet" predicting a reversal price (within margin) and date (same day).
  /// Returns None if signal was false positive.
  /// Returns Some(Vec<Point>) if true positive signal occurred. Vec<Point> are all prices hit by that signal candle.
  pub fn backtest_signal(
    price_reversals: &Vec<Candle>,
    harmonic_price_on_date: Point,
    date: Time,
    margin_of_error: f64
  ) -> Option<Candle> {
    let reversal_candle = PricePlanet::get_reversal_at_date(price_reversals, date);

    // reversal candle found on date; true positive signal
    if let Some(reversal_candle) = reversal_candle {
      // price point within margin of reversal close, high, or low.
      if SquareOfNine::within_margin_of_error(reversal_candle.close, harmonic_price_on_date.value, margin_of_error) ||
        SquareOfNine::within_margin_of_error(reversal_candle.high, harmonic_price_on_date.value, margin_of_error) ||
        SquareOfNine::within_margin_of_error(reversal_candle.low, harmonic_price_on_date.value, margin_of_error)
      {
        Some(reversal_candle.clone())
      }
      else {
        // true positive signal, but price point not within margin, so entry missed
        // TODO: better system for handling margin of error on entry to increase win rate
        None
      }
    }
    // false positive signal
    else {
      None
    }
  }

  /// Check if value is within margin of error of a price.
  pub fn angle_safe_increment(mut angle: f32, diff: f32) -> f32 {
    angle += diff;
    if angle >= 360.0 {
      angle -= 360.0;
    } else if angle < 0.0 {
      angle += 360.0;
    }
    angle
  }

  /// Search vector of price reversals for reversal on date.
  pub fn get_reversal_at_date(reversals: &[Candle], date: Time) -> Option<&Candle> {
    reversals.iter().find(|&candle| date.as_string() == candle.date.as_string())
  }
}
