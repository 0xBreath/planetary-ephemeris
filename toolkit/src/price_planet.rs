use std::fs::File;
use std::path::PathBuf;
use log::debug;
use ephemeris::*;
use time_series::*;
use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};

#[derive(Debug, Clone)]
pub struct PlanetLongitudes {
  pub planet: Planet,
  pub angles: Vec<(Time, f32)>,
}

#[derive(Debug, Clone)]
pub struct PricePlanet {
  ticker_data: TickerData,
  ticker_data_factor: f64,
  market_structure: MarketStructure,
  square_of_nine: SquareOfNine,
  planet_longitudes: Vec<PlanetLongitudes>,
  harmonics: Vec<f32>,
  backtest_matrix: Vec<Vec<Backtest>>,
  reversal_candle_range: usize,
  start_date: Time,
  end_date: Time,
  backtest_results: PathBuf,
}

impl PricePlanet {
  pub async fn new(
    backtest_results_file_path: PathBuf,
    reversal_candle_range: usize,
    square_of_nine_step: f64,
    ticker_data_factor: f64,
    start_date: Time,
    end_date: Time,
  ) -> std::io::Result<Self> {
    let ticker_data = TickerData::new_quandl_api(start_date, end_date, ticker_data_factor).await;

    let market_structure = MarketStructure::new(&ticker_data, reversal_candle_range);

    // TODO: param for max SquareOfNine value -> determines dimension internally
    let dimension = 2001;
    let origin = 1;
    let square_of_nine = SquareOfNine::new(origin, square_of_nine_step, dimension);
    if end_date < start_date {
      return Err(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "end_date must be greater than start_date",
      ));
    }

    let harmonics = Alignment::to_vec().iter().map(|a| a.to_num()).collect::<Vec<f32>>();

    let planets = Planet::to_vec();
    let mut planet_longitudes = Vec::<PlanetLongitudes>::new();
    for planet in planets.iter() {
      let angles = Query::query(
        Origin::Geocentric,
        planet,
        DataType::RightAscension,
        start_date,
        end_date
      ).await.expect("failed to query planet angles");
      planet_longitudes.push(PlanetLongitudes {
        planet: planet.clone(),
        angles
      });
    }
    let backtest_matrix = vec![vec![Backtest::default(); harmonics.len()]; planets.len()];

    Ok(Self {
      ticker_data,
      ticker_data_factor,
      market_structure,
      square_of_nine,
      planet_longitudes,
      harmonics,
      backtest_matrix,
      reversal_candle_range,
      start_date,
      end_date,
      backtest_results: backtest_results_file_path,
    })
  }

  /// Identify all dates where the price longitude on a Square of Nine equals the longitude of all planets (including Sun and Moon).
  ///
  /// Signal occurs when >1 price-planet alignment occurs for the same candle/day; hence the name "confluent signal".
  ///
  /// Backtest the Time=Price against actual reversals in the market.
  ///
  /// Compute win rate each planet longitude (time) and price longitude (price) for a all possible harmonics (90, 120, 180, etc).
  ///
  /// Return matrix of all possibilities of (planet, harmonic, win rate), and print results to file.
  /// TODO: Backtest result print all planets involved in signal
  pub async fn confluent_signals(self, error_margin_price: f64, error_margin_days: u32) {
    stdout()
      .execute(cursor::Hide).unwrap();

    let ticker_data = self.ticker_data;
    let market_structure = self.market_structure;
    let reversals = market_structure.reversals;
    if ticker_data.candles.is_empty() {
      return
    }
    let square_of_nine = &self.square_of_nine;
    println!("Time Period: {} to {}", self.start_date.as_string(), self.end_date.as_string());
    let harmonics = self.harmonics;
    let planet_longitudes = self.planet_longitudes;
    let reversal_candles = reversals.iter().map(|reversal| reversal.candle.clone()).collect::<Vec<Candle>>();
    let price_history = ticker_data.get_candles();
    let mut backtest_matrix = self.backtest_matrix;
    // iterate over each day (`Time`) in the price history
    let dates = planet_longitudes[0].angles.iter().map(|(time, _)| time).collect::<Vec<&Time>>();
    // iterate over each planet and find angle on this day (Time)
    for (index, date) in dates.iter().enumerate() {
      let planet_angles_on_date = planet_longitudes.iter().map(|planet_longitudes| {
        let angle = planet_longitudes.angles.iter().find(|(time, _)| time == *date).unwrap().1;
        (planet_longitudes.planet.clone(), angle)
      }).collect::<Vec<(Planet, f32)>>();
      stdout().queue(cursor::SavePosition).unwrap();
      stdout().write_all(format!("{}% completed...", index / dates.len()).as_bytes()).unwrap();
      stdout().queue(cursor::RestorePosition).unwrap();
      stdout().flush().unwrap();
      stdout().queue(cursor::RestorePosition).unwrap();
      stdout().queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();

      let mut signals = Vec::<(Planet, f32, Point)>::new();

      // compare price longitude to planet longitudes at all possible harmonics
      // finds any valid signal on this date (Time)
      for (_, harmonic) in harmonics.iter().enumerate() {
        for (_, (planet, angle)) in planet_angles_on_date.iter().enumerate() {
          // compute angle of harmonic that price must equal to be in alignment with planet
          let angle = Self::angle_safe_increment(*angle, *harmonic);
          // scan SquareOfNine for all price Points that match this angle (Price=Time)
          let angle_price_points = &square_of_nine.find_price_equals_time(angle);
          // search candle history for any candles that hit a price in `angle_price_points` on this date
          // assumes this trading algorithm would place orders at each price in `angle_price_points` (within reason)
          let harmonic_price_on_date = Self::check_candle_hit_price_on_date(
            date,
            angle_price_points,
            price_history,
            error_margin_price,
            error_margin_days
          );
          if let Some(point) = harmonic_price_on_date {
            signals.push((planet.clone(), *harmonic, point));
          }
        }
      }
      // at this point all signals for this date (Time) have been found
      // if signals.length > 1, then confluence occurred and needs to be backtested
      if signals.len() > 1 {
        // let backtest = Backtest::default();
        // can use any Point in `signals` to backtest
        // because all Points in `signals` are on the same date (Time),
        // thus all Points align with same Candle in the candle history
        let point = signals.get(0).unwrap().2;
        let backtested_signal: Option<Candle> = Self::backtest_signal(
          &reversal_candles,
          point,
          date,
          error_margin_price,
        );
        if let Some(true_positive) = backtested_signal {
          // only adding signals to one cell in the matrix
          // because all signals have various alignment with planets, thus separating them by planet is meaningless
          // TODO: system for separating signals by planet combination
          // TODO: which entails add_signal() to matrix cell and display results in some reasonable way
          backtest_matrix[0][0].increment_win_count();
        }
        backtest_matrix[0][0].increment_total_count();
      }
      stdout().execute(cursor::Show).unwrap();
    }

    // print results to file
    let candles = ticker_data.get_candles();
    Self::print_confluent_price_planet_results(
      candles,
      self.ticker_data_factor,
      error_margin_price,
      &self.backtest_results,
      self.reversal_candle_range,
      square_of_nine,
      &self.start_date,
      &self.end_date,
      &backtest_matrix
    );

  }

  /// Identify all dates where the price longitude on a Square of Nine equals the longitude of all planets (including Sun and Moon).
  ///
  /// Signal occurs when a single price-planet alignment occurs for the same candle/day.
  ///
  /// Backtest the Time=Price against actual reversals in the market.
  ///
  /// Compute win rate each planet longitude (time) and price longitude (price) for a all possible harmonics (90, 120, 180, etc).
  ///
  /// Return matrix of all possibilities of (planet, harmonic, win rate), and print results to file.
  pub async fn single_signal(self, error_margin_price: f64, error_margin_days: u32) {
    let ticker_data = self.ticker_data;
    let market_structure = self.market_structure;
    let reversals = market_structure.reversals;
    if ticker_data.candles.is_empty() {
      return
    }
    let square_of_nine = &self.square_of_nine;
    println!("Time Period: {} to {}", self.start_date.as_string(), self.end_date.as_string());

    let harmonics = self.harmonics;
    let planet_longitudes = self.planet_longitudes;
    let mut backtest_matrix = self.backtest_matrix;
    let reversal_candles = reversals.iter().map(|reversal| reversal.candle.clone()).collect::<Vec<Candle>>();
    let price_history = ticker_data.get_candles();

    stdout()
      .execute(cursor::Hide).unwrap();

    for (index, planet_data) in planet_longitudes.iter().enumerate() {
      let daily_longitudes = &planet_data.angles;
      let planet = &planet_data.planet;

      for (time, angle) in daily_longitudes.iter() {
        for (harmonic_index, angle_diff) in harmonics.iter().enumerate() {
          // find all prices at angle on the SquareOfNine
          let angle = Self::angle_safe_increment(*angle, *angle_diff);
          let angle_price_points = &square_of_nine.find_price_equals_time(angle);
          // check if price history candle on the specified date equals (within margin of error)
          // any of the SquareOfNine price points at the harmonic angle relative to the planet.
          let harmonic_price_on_date = Self::check_candle_hit_price_on_date(
            time,
            angle_price_points,
            price_history,
            error_margin_price,
            error_margin_days
          );

          // candle hit a harmonic price on this date
          if let Some(point) = harmonic_price_on_date {
            // returns Some if true positive signal, None if false positive signal
            let backtested_signal: Option<Candle> = Self::backtest_signal(
              &reversal_candles,
              point,
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
      stdout().queue(cursor::SavePosition).unwrap();
      stdout().write_all(format!("{}% completed...", index / planet_longitudes.len()).as_bytes()).unwrap();
      stdout().queue(cursor::RestorePosition).unwrap();
      stdout().flush().unwrap();
      stdout().queue(cursor::RestorePosition).unwrap();
      stdout().queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
    }
    stdout().execute(cursor::Show).unwrap();

    // write results to console and file
    PricePlanet::print_single_price_planet_results(
      &reversal_candles,
      self.ticker_data_factor,
      error_margin_price,
      &self.backtest_results,
      self.reversal_candle_range,
      &self.square_of_nine,
      &self.start_date,
      &self.end_date,
      &backtest_matrix,
    );
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
    price_reversals: &[Candle],
    harmonic_price_on_date: Point,
    date: &Time,
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
  pub fn get_reversal_at_date<'a, 'b>(reversals: &'b [Candle], date: &'a Time) -> Option<&'b Candle> {
    reversals.iter().find(|&candle| date.as_string() == candle.date.as_string())
  }

  #[allow(clippy::too_many_arguments)]
  pub fn print_confluent_price_planet_results(
    reversals: &Vec<Candle>,
    ticker_data_factor: f64,
    margin_of_error: f64,
    results_file: &PathBuf,
    reversal_candle_range: usize,
    square_of_nine: &SquareOfNine,
    start_date: &Time,
    end_date: &Time,
    backtest_matrix: &[Vec<Backtest>]
  ) {
    let square_of_nine_step = square_of_nine.get_step();
    // write results to console and file
    println!("Number of Reversals in last {} days: {}\r", end_date.diff_days(start_date), reversals.len());
    println!("Reversal defined by price extreme of +/- the adjacent {} candles", reversal_candle_range);
    println!("Margin of Error within actual reversal candle close: {}%", (margin_of_error * 100.0));
    println!("Square of Nine step interval: {}", square_of_nine_step);
    println!("Ticker Data price factor: {}", ticker_data_factor);
    let mut file = File::create(results_file).unwrap();
    let _ = file.write(format!("Number of Reversals in last {} days: {}\r", end_date.diff_days(start_date), reversals.len()).as_bytes())
      .expect("Unable to write to file");
    let _ = file.write(format!("Reversal defined by price extreme of +/- the adjacent {} candles\r", reversal_candle_range).as_bytes())
      .expect("Unable to write to file");
    let _ = file.write(format!("Margin of Error within actual reversal candle close: {}%\r", (margin_of_error * 100.0)).as_bytes())
      .expect("Unable to write to file");
    let _ = file.write(format!("Square of Nine step interval: {}\r", square_of_nine_step).as_bytes())
      .expect("Unable to write to file");
    let _ = file.write(format!("Ticker Data price factor: {}", ticker_data_factor).as_bytes())
      .expect("Unable to write to file");

    println!();
    println!("WIN RATE\t\tWIN COUNT\t\tTOTAL ALIGNMENTS");
    let _ = file.write("\nWIN RATE\t\tWIN COUNT\t\tTOTAL ALIGNMENTS\n".to_string().as_bytes()).expect("Unable to write to file");

    let backtest = &backtest_matrix[0][0];
    println!("backtest: {:?}", backtest);
    if let Some(signals) = &backtest.signals {
      println!("signals length: {}", signals.len());
      if signals.len() > 1 {
        // let harmonic = signals[0].1;
        // let planet = signals[0].0.clone();

        let win_rate = (backtest.get_win_rate() * 100.0).round();
        let _ = file.write(format!(
          "{:?}%\t\t{:?}\t\t\t{:?}\n",
          win_rate,
          backtest.get_win_count(),
          backtest.get_total_count()
        ).to_string().as_bytes()).expect("Unable to write to file");
        println!(
          "{:?}%\t\t{:?}\t\t\t{:?}",
          win_rate,
          backtest.get_win_count(),
          backtest.get_total_count()
        );
      }
    }
    println!("
    The “win rates” are the odds the algorithm would have known the day a reversal will occur\r
    and been within {}% of entering a trade at the close of that reversal candle.\r
    Backtest is for BTCUSD {} to {}.\r",
             (margin_of_error * 100.0), start_date.as_string(), end_date.as_string()
    );
    let _ = file.write(format!("\n
    The “win rates” are the odds the algorithm would have known the day a reversal will occur\r
    and been within {}% of entering a trade at the close of that reversal candle.\r
    Backtest is for BTCUSD {} to {}.\r",
                               (margin_of_error * 100.0), start_date.as_string(), end_date.as_string()).as_bytes()
    ).expect("Unable to write to file");
  }

  #[allow(clippy::too_many_arguments)]
  pub fn print_single_price_planet_results(
    reversals: &Vec<Candle>,
    ticker_data_factor: f64,
    margin_of_error: f64,
    results_file: &PathBuf,
    reversal_candle_range: usize,
    square_of_nine: &SquareOfNine,
    start_date: &Time,
    end_date: &Time,
    backtest_matrix: &[Vec<Backtest>]
  ) {
    let square_of_nine_step = square_of_nine.get_step();
    // write results to console and file
    println!("Number of Reversals from {} to {}\r", start_date.as_string(), end_date.as_string());
    println!("Reversal defined by price extreme of +/- the adjacent {} candles", reversal_candle_range);
    println!("Margin of Error within actual reversal candle close: {}%", (margin_of_error * 100.0));
    println!("Square of Nine step interval: {}", square_of_nine_step);
    println!("Ticker Data price factor: {}", ticker_data_factor);
    let mut file = File::create(results_file).unwrap();
    let _ = file.write(format!("Number of Reversals from {} to {}\r", start_date.as_string(), end_date.as_string()).as_bytes())
      .expect("Unable to write to file");
    let _ = file.write(format!("Reversal defined by price extreme of +/- the adjacent {} candles\r", reversal_candle_range).as_bytes())
      .expect("Unable to write to file");
    let _ = file.write(format!("Margin of Error within actual reversal candle close: {}%\r", (margin_of_error * 100.0)).as_bytes())
      .expect("Unable to write to file");
    let _ = file.write(format!("Square of Nine step interval: {}\r", square_of_nine_step).as_bytes())
      .expect("Unable to write to file");
    let _ = file.write(format!("Ticker Data price factor: {}\r", ticker_data_factor).as_bytes())
      .expect("Unable to write to file");

    for planet in backtest_matrix.iter() {
      println!();
      println!("PLANET\t\tALIGNMENT\tWIN RATE\tWIN COUNT\tTOTAL ALIGNMENTS");
      let _ = file.write("\nPLANET\t\tALIGNMENT\tWIN RATE\tWIN COUNT\tTOTAL ALIGNMENTS\n".to_string().to_string().as_bytes()).expect("Unable to write to file");
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
                  win_rate,
                  backtest.get_win_count(),
                  backtest.get_total_count()
                ).to_string().as_bytes()).expect("Unable to write to file");
              } else {
                let _ = file.write(format!(
                  "{}\t\t{:?}\t\t{:?}%\t{:?}\t\t\t{:?}\n",
                  planet.to_str(),
                  harmonic.round(),
                  win_rate,
                  backtest.get_win_count(),
                  backtest.get_total_count()
                ).to_string().as_bytes()).expect("Unable to write to file");
              }
              println!(
                "{}\t\t{:?}\t\t{:?}%\t{:?}\t\t{:?}",
                planet.to_str(),
                harmonic.round(),
                win_rate,
                backtest.get_win_count(),
                backtest.get_total_count()
              );
            } else {
              if harmonic < 10.0 {
                let _ = file.write(format!(
                  "{}\t\t{:?}\t\t\t{:?}%\t\t{:?}\t\t\t{:?}\n",
                  planet.to_str(),
                  harmonic.round(),
                  win_rate,
                  backtest.get_win_count(),
                  backtest.get_total_count()
                ).to_string().as_bytes()).expect("Unable to write to file");
              } else {
                let _ = file.write(format!(
                  "{}\t\t{:?}\t\t{:?}%\t\t{:?}\t\t\t{:?}\n",
                  planet.to_str(),
                  harmonic.round(),
                  win_rate,
                  backtest.get_win_count(),
                  backtest.get_total_count()
                ).to_string().as_bytes()).expect("Unable to write to file");
              }
              println!(
                "{}\t\t{:?}\t\t{:?}%\t\t{:?}\t\t{:?}",
                planet.to_str(),
                harmonic.round(),
                win_rate,
                backtest.get_win_count(),
                backtest.get_total_count()
              );
            }
          }
        }
      }
    }
    println!("
      The “win rates” are the odds the algorithm would have known the day a reversal will occur\n
      and been within {}% of entering a trade at the close of that reversal candle.\n
      Backtest is for BTCUSD {} to {}.\n",
             (margin_of_error * 100.0), start_date.as_string(), end_date.as_string()
    );
    let _ = file.write(format!("\n
      The “win rates” are the odds the algorithm would have known the day a reversal will occur\n
      and been within {}% of entering a trade at the close of that reversal candle.\n
      Backtest is for BTCUSD {} to {}.\n",
                               (margin_of_error * 100.0), start_date.as_string(), end_date.as_string()).as_bytes()
    ).expect("Unable to write to file");
  }
}
