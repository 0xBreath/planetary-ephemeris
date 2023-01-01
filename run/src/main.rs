
use std::path::PathBuf;
use log::{debug, LevelFilter};
use simplelog::{
  ColorChoice, Config, TerminalMode, TermLogger,
};
use ephemeris::*;
use time_series::*;
use toolkit::*;

pub const TICKER_DATA_PATH: &str = "BTCUSD.csv";
pub const RESULTS_PATH: &str = "BTCUSD_results.csv";

#[tokio::main]
async fn main() {
  init_logger();

  // println!("\t\t### PLANET PAIR ALIGNMENT MATRIX ###\t\t");
  // PlanetMatrix::test_planet_matrix(
  //   &PathBuf::from(TICKER_DATA_PATH),
  //   2,
  //   1.5,
  //   10
  // ).await;
  // println!("\t\t### LUNAR ZERO DECLINATION CROSS ###\t\t");
  // LunarDeclination::test_lunar_declination(-720, Time::today(), 10).await;
  // println!("----------------------------------------------------------------------------------------");
  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### SQUARE OF NINE ###\t\t");
  // test_square_of_nine();
  // println!("----------------------------------------------------------------------------------------");
  println!("\t\t### IDENTIFY MARKET STRUCTURE ###\t\t");
  MarketStructure::test_market_structure(10, &PathBuf::from(TICKER_DATA_PATH));
  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### PRICE PLANET HARMONICS ###\t\t");
  // price_planet_harmonics(
  //   200.0,
  //   10,
  //   0.01,
  //   0
  // ).await;
}

pub fn init_logger() {
  TermLogger::init(
    LevelFilter::Info,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  ).expect("failed to initialize logger");
}

/// Identify all dates where the price longitude on a Square of Nine equals the longitude of all possible planets (including Sun and Moon)
/// Backtest the Time=Price against actual reversals in the market
/// Compute win rate each planet longitude (time) and price longitude (price) for a all possible harmonics (90, 120, 180, etc)
/// Return matrix of all possibilities of (planet, harmonic, win rate), and print results to file.
pub async fn price_planet_harmonics(square_of_nine_step: f64, reversal_candle_range: usize, error_margin_price: f64, error_margin_days: u32) {
  let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
  let market_structure = MarketStructure::new(ticker_data.clone(), reversal_candle_range);
  let reversals = market_structure.reversals.clone();
  if ticker_data.candles.is_empty() {
    return
  }
  let earliest_candle_date = &ticker_data.get_candles()[0].date;
  //let time_period = Time::today().diff_days(earliest_candle_date);
  let time_period = -500;
  println!("Time Period: {} days from today {}", time_period, Time::today().as_string());

  // TODO: param for max SquareOfNine value -> determines dimension internally
  let dimension = 2001;
  let origin = 1;
  let square_of_nine = SquareOfNine::new(origin, square_of_nine_step, dimension);

  // let angle = 36.0;
  // let range_min = 359.0;
  // let range_max = 1.0;
  // if range_min < range_max {
  //   if angle >= range_min && angle < range_max {
  //     println!("ALPHA");
  //   }
  // } else if (range_min > range_max && angle >= range_min && angle < 360.0) || (range_min > range_max && angle < range_max && angle >= 0.0) {
  //   println!("BRAVO");
  // }

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

  let harmonics = Alignment::iter().iter().map(|a| a.to_num()).collect::<Vec<f32>>();

  let mut backtest_matrix = vec![vec![Backtest::default(); harmonics.len()]; planets.len()];
  let reversal_candles = reversals.iter().map(|reversal| reversal.candle.clone()).collect::<Vec<Candle>>();
  let price_history = ticker_data.get_candles();

  for (index, planet) in planets.iter().enumerate() {
    let daily_longitudes = Query::query(
      Origin::Geocentric,
      planet,
      DataType::RightAscension,
      Time::today(),
      time_period
    ).await;

    for (time, angle) in daily_longitudes.into_iter() {
      for (harmonic_index, angle_diff) in harmonics.iter().enumerate() {
        // find all prices at angle on the SquareOfNine
        let angle = angle_safe_increment(angle, *angle_diff);
        let price_equals_time = square_of_nine.find_price_equals_time(angle);
        // check if price history candle on the specified date equals (within margin of error)
        // any of the SquareOfNine price points at the harmonic angle relative to the planet.
        let harmonic_price_on_date = check_candle_hit_price_on_date(
          &time,
          &price_equals_time,
          price_history,
          error_margin_price,
          error_margin_days
        );

        // signal occurred
        if let Some(harmonic_price_on_date) = harmonic_price_on_date {
          debug!(
            "Price: {}\tTime: {}\tRing Size: {:?}",
            harmonic_price_on_date.value,
            time.as_string(),
            square_of_nine.ring_size_of_cell_value(harmonic_price_on_date.value)
          );

          // returns Some if true positive signal, None if false positive signal
          let backtested_signal: Option<Candle> = backtest_signal(
            &reversal_candles,
            harmonic_price_on_date,
            time,
            error_margin_price,
          );
          if let Some(_backtested_signal) = backtested_signal {
            backtest_matrix[index][harmonic_index].increment_win_count();
            backtest_matrix[index][harmonic_index].set_harmonic(*angle_diff);
          }
          backtest_matrix[index][harmonic_index].increment_total_count();
        } else {
          debug!("No signal on {}", time.as_string());
        }
      }
    }
  }

  // write results to console and file
  let results_file = &PathBuf::from(RESULTS_PATH);
  market_structure.print(
    results_file,
    time_period,
    reversal_candle_range,
    error_margin_price,
    square_of_nine_step,
    &planets,
    &backtest_matrix
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
  price_reversals: &Vec<Candle>,
  harmonic_price_on_date: Point,
  date: Time,
  margin_of_error: f64
) -> Option<Candle> {
  let reversal_candle = get_reversal_at_date(price_reversals, date);

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
