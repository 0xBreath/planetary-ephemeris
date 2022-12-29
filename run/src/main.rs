use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use log::{debug, LevelFilter};
use simplelog::{
  ColorChoice, Config, TermLogger, TerminalMode,
};
use ephemeris::*;
use time_series::*;

pub const TICKER_DATA_PATH: &str = "BTCUSD.csv";
pub const RESULTS_PATH: &str = "BTCUSD_results.csv";

#[tokio::main]
async fn main() {
  init_logger();

  //test_planetary_matrix().await;
  //test_square_of_nine();
  price_planet_harmonics(
    -1500,
    1.0,
    10,
    0.015
  ).await;
}

pub fn init_logger() {
  TermLogger::init(
    LevelFilter::Info,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  ).expect("failed to initialize logger");
}

pub async fn test_lunar_declination() {
  let lunar_declinations = lunar_declination(-365, Time::today()).await;
  // for (time, declination) in lunar_declinations {
  //   println!("{}\t{}", time.as_string(), declination);
  // }
  let candle_range: usize = 10;
  let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
  let local_highs = ticker_data.find_local_highs(candle_range);
  let local_lows = ticker_data.find_local_lows(candle_range);

  // iterate over lunar_declinations, identify if it is within 1 day of a local high or low
  // if so, print the date and the declination
  for (time, declination) in lunar_declinations {
    for high in local_highs.iter() {
      if time.delta_date(-1).as_string() == high.date.as_string() {
        println!("HIGH DAY BEFORE\t{}\t{}°", time.as_string(), declination);
      }
      else if time.as_string() == high.date.as_string() {
        println!("HIGH SAME DAY\t{}\t{}°", time.as_string(), declination);
      }
      else if time.delta_date(1).as_string() == high.date.as_string() {
        println!("HIGH DAY AFTER\t{}\t{}°", time.as_string(), declination);
      }
    }
    for low in local_lows.iter() {
      if time.delta_date(-1).as_string() == low.date.as_string() {
        println!("LOW DAY BEFORE\t{}\t{}°", time.as_string(), declination);
      }
      else if time.as_string() == low.date.as_string() {
        println!("LOW SAME DAY\t{}\t{}°", time.as_string(), declination);
      }
      else if time.delta_date(1).as_string() == low.date.as_string() {
        println!("LOW DAY AFTER\t{}\t{}°", time.as_string(), declination);
      }
    }
  }
}

pub async fn test_planetary_matrix() {
  let candle_range: usize = 10;
  let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
  let local_highs = ticker_data.find_local_highs(candle_range);
  let local_lows = ticker_data.find_local_lows(candle_range);

  let matrix = ephemeris::planetary_matrix(
    Origin::Geocentric,
    &Time::today(),
    -365,
  ).await;
  for (planet_a, planet_b, alignments) in matrix.into_iter() {
    if !alignments.is_empty() {
      println!("{}-{}", planet_a.to_str(), planet_b.to_str());
    }
    for (time, _, alignment) in alignments.iter() {
      println!("{}\t{}", time.as_string(), alignment.to_str());
    }
    println!();

    for (time, _ra, alignment) in alignments.iter() {
      //println!("\t{}\t{}\t{}", time.as_string(), ra, alignment.to_str());
      for high in local_highs.iter() {
        if time.delta_date(-1).as_string() == high.date.as_string() {
          println!("HIGH DAY BEFORE\t{}\t{}", time.as_string(), alignment.to_str());
        }
        else if time.as_string() == high.date.as_string() {
          println!("HIGH SAME DAY\t{}\t{}", time.as_string(), alignment.to_str());
        }
        else if time.delta_date(1).as_string() == high.date.as_string() {
          println!("HIGH DAY AFTER\t{}\t{}", time.as_string(), alignment.to_str());
        }
      }
      for low in local_lows.iter() {
        if time.delta_date(-1).as_string() == low.date.as_string() {
          println!("LOW DAY BEFORE\t{}\t{}", time.as_string(), alignment.to_str());
        }
        else if time.as_string() == low.date.as_string() {
          println!("LOW SAME DAY\t{}\t{}", time.as_string(), alignment.to_str());
        }
        else if time.delta_date(1).as_string() == low.date.as_string() {
          println!("LOW DAY AFTER\t{}\t{}", time.as_string(), alignment.to_str());
        }
      }
    }
    println!();

  }
}

pub fn test_square_of_nine() {
  let dimension = 11;
  let square_of_nine = SquareOfNine::new(1, 1.0, dimension);
  if dimension < 13 {
    for y in square_of_nine.matrix.iter() {
      for x in y {
        match x.harmonic {
          Some(harmonic) => print!("{}\t", harmonic as i32),
          None => print!("-\t"),
        }
      }
      println!();
    }
    println!("--------------------------------------------------------------------------------------------------------");
    for y in square_of_nine.matrix.iter() {
      for x in y {
        print!("{}\t", x.value)
      }
      println!();
    }
    println!("--------------------------------------------------------------------------------------------------------");
  }
  //used to check size of outermost square of nine ring
  let zero_zero = square_of_nine.matrix[0][0].value;
  let one_one = square_of_nine.matrix[1][1].value;
  let two_two = square_of_nine.matrix[2][2].value;
  println!("Size of outermost ring: {:?}", (zero_zero - one_one) as u32);
  println!("Size of second outermost ring: {:?}", (one_one - two_two) as u32);
  println!("--------------------------------------------------------------------------------------------------------");
  println!("PRICE\tHARMONIC\t\tDEGREES OF ARC");
  for point in square_of_nine.values.iter() {
    match point.harmonic {
      None => {
        match point.arc {
          None => println!("{:?}\t{:?}\t\t\t{}\r", point.value, point.harmonic, "-"),
          Some(arc) => println!("{:?}\t{:?}\t\t\t{:?}\r", point.value, point.harmonic, arc),
        }
      },
      Some(Harmonic::Zero) => println!("{:?}\t{:?}\t\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
      Some(Harmonic::OneEighth) => println!("{:?}\t{:?}\t\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
      Some(Harmonic::OneFourth) => println!("{:?}\t{:?}\t\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
      Some(Harmonic::ThreeEighths) => println!("{:?}\t{:?}\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
      Some(Harmonic::OneHalf) => println!("{:?}\t{:?}\t\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
      Some(Harmonic::FiveEighths) => println!("{:?}\t{:?}\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
      Some(Harmonic::ThreeFourths) => println!("{:?}\t{:?}\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
      Some(Harmonic::SevenEighths) => println!("{:?}\t{:?}\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
    }
  }
  println!("--------------------------------------------------------------------------------------------------------");
  let harmonics_zero = square_of_nine.find_price_equals_time(0.0);
  println!("ZERO HARMONICS");
  for point in harmonics_zero.iter() {
    match point.harmonic {
      None => {
        match point.arc {
          None => println!("{:?}\t{:?}\t\t\t{}\r", point.value, point.harmonic, "-"),
          Some(arc) => println!("{:?}\t{:?}\t\t\t{:?}\r", point.value, point.harmonic, arc),
        }
      },
      Some(Harmonic::Zero) => println!("{:?}\t{:?}\t\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
      Some(Harmonic::OneEighth) => println!("{:?}\t{:?}\t\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
      Some(Harmonic::OneFourth) => println!("{:?}\t{:?}\t\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
      Some(Harmonic::ThreeEighths) => println!("{:?}\t{:?}\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
      Some(Harmonic::OneHalf) => println!("{:?}\t{:?}\t\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
      Some(Harmonic::FiveEighths) => println!("{:?}\t{:?}\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
      Some(Harmonic::ThreeFourths) => println!("{:?}\t{:?}\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
      Some(Harmonic::SevenEighths) => println!("{:?}\t{:?}\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Backtest {
  planet: Option<Planet>,
  harmonic: Option<f32>,
  win_count: u64,
  total_count: u64,
  win_rate: f64,
}
impl Backtest {
  pub fn new() -> Backtest {
    Backtest {
      planet: None,
      harmonic: None,
      win_count: 0,
      total_count: 0,
      win_rate: 0.0,
    }
  }
  pub fn increment_win_count(&mut self) {
    self.win_count += 1;
    self.set_win_rate();
  }
  pub fn increment_total_count(&mut self) {
    self.total_count += 1;
    self.set_win_rate();
  }
  fn set_win_rate(&mut self) {
    self.win_rate = self.win_count as f64 / self.total_count as f64;
  }
  pub fn get_win_rate(&self) -> f64 {
    self.win_rate
  }
  pub fn get_planet(&self) -> Option<Planet> {
    self.planet.clone()
  }
  pub fn set_planet(&mut self, planet: Planet) {
    self.planet = Some(planet);
  }
  pub fn get_harmonic(&self) -> Option<f32> {
    self.harmonic
  }
  pub fn set_harmonic(&mut self, harmonic: f32) {
    self.harmonic = Some(harmonic);
  }
}

/// Identify all dates where the price longitude on a Square of Nine equals the longitude of all possible planets (includeing Sun and Moon)
/// Backtest the Time=Price against actual reversals in the market
/// Compute win rate each planet longitude (time) and price longitude (price) for a all possible harmonics (90, 120, 180, etc)
/// Return matrix of all possibilities of (planet, harmonic, win rate), and print results to file.
pub async fn price_planet_harmonics(time_period: i64, square_of_nine_step: f64, reversal_candle_range: usize, margin_of_error: f64) {
  let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
  let mut local_highs = ticker_data.find_local_highs(reversal_candle_range);
  let mut local_lows = ticker_data.find_local_lows(reversal_candle_range);
  let mut reversals = Vec::new();
  reversals.append(&mut local_highs);
  reversals.append(&mut local_lows);

  let dimension = 271;
  let origin = 1;
  let square_of_nine = SquareOfNine::new(origin, square_of_nine_step, dimension);

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

  let mut backtest_matrix = vec![vec![Backtest::new(); 16]; planets.len()];

  for (index, planet) in planets.iter().enumerate() {
    let daily_longitudes = Query::query(
      Origin::Geocentric,
      planet,
      DataType::RightAscension,
      Time::today(),
      time_period
    ).await;

    for (time, angle) in daily_longitudes.into_iter() {
      // conjunction
      match win_rate_of_signal(&square_of_nine, time, angle, 0.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][0].increment_win_count();
          }
          backtest_matrix[index][0].increment_total_count();
          backtest_matrix[index][0].set_planet(planet.clone());
          backtest_matrix[index][0].set_harmonic(0.0);
        },
      };
      // opposition
      match win_rate_of_signal(&square_of_nine, time, angle, 180.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][1].increment_win_count();
          }
          backtest_matrix[index][1].increment_total_count();
          backtest_matrix[index][1].set_planet(planet.clone());
          backtest_matrix[index][1].set_harmonic(180.0);
        },
      };
      // square 90
      match win_rate_of_signal(&square_of_nine, time, angle, 90.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][2].increment_win_count();
          }
          backtest_matrix[index][2].increment_total_count();
          backtest_matrix[index][2].set_planet(planet.clone());
          backtest_matrix[index][2].set_harmonic(90.0);
        },
      };
      // square 270
      match win_rate_of_signal(&square_of_nine, time, angle, 270.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][3].increment_win_count();
          }
          backtest_matrix[index][3].increment_total_count();
          backtest_matrix[index][3].set_planet(planet.clone());
          backtest_matrix[index][3].set_harmonic(270.0);
        },
      };
      // trine 120
      match win_rate_of_signal(&square_of_nine, time, angle, 120.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][4].increment_win_count();
          }
          backtest_matrix[index][4].increment_total_count();
          backtest_matrix[index][4].set_planet(planet.clone());
          backtest_matrix[index][4].set_harmonic(120.0);
        },
      };
      // trine 240
      match win_rate_of_signal(&square_of_nine, time, angle, 240.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][5].increment_win_count();
          }
          backtest_matrix[index][5].increment_total_count();
          backtest_matrix[index][5].set_planet(planet.clone());
          backtest_matrix[index][5].set_harmonic(240.0);
        },
      };
      // octile 45
      match win_rate_of_signal(&square_of_nine, time, angle, 45.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][6].increment_win_count();
          }
          backtest_matrix[index][6].increment_total_count();
          backtest_matrix[index][6].set_planet(planet.clone());
          backtest_matrix[index][6].set_harmonic(45.0);
        },
      };
      // octile 135
      match win_rate_of_signal(&square_of_nine, time, angle, 135.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][7].increment_win_count();
          }
          backtest_matrix[index][7].increment_total_count();
          backtest_matrix[index][7].set_planet(planet.clone());
          backtest_matrix[index][7].set_harmonic(135.0);
        },
      };
      // octile 225
      match win_rate_of_signal(&square_of_nine, time, angle, 225.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][8].increment_win_count();
          }
          backtest_matrix[index][8].increment_total_count();
          backtest_matrix[index][8].set_planet(planet.clone());
          backtest_matrix[index][8].set_harmonic(225.0);
        },
      };
      // octile 315
      match win_rate_of_signal(&square_of_nine, time, angle, 315.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][9].increment_win_count();
          }
          backtest_matrix[index][9].increment_total_count();
          backtest_matrix[index][9].set_planet(planet.clone());
          backtest_matrix[index][9].set_harmonic(315.0);
        },
      };
      // sextile 60
      match win_rate_of_signal(&square_of_nine, time, angle, 60.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][10].increment_win_count();
          }
          backtest_matrix[index][10].increment_total_count();
          backtest_matrix[index][10].set_planet(planet.clone());
          backtest_matrix[index][10].set_harmonic(60.0);
        },
      };
      // sextile 300
      match win_rate_of_signal(&square_of_nine, time, angle, 300.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][11].increment_win_count();
          }
          backtest_matrix[index][11].increment_total_count();
          backtest_matrix[index][11].set_planet(planet.clone());
          backtest_matrix[index][11].set_harmonic(300.0);
        },
      };
      // quintile 72
      match win_rate_of_signal(&square_of_nine, time, angle, 72.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][12].increment_win_count();
          }
          backtest_matrix[index][12].increment_total_count();
          backtest_matrix[index][12].set_planet(planet.clone());
          backtest_matrix[index][12].set_harmonic(72.0);
        },
      };
      // quintile 144
      match win_rate_of_signal(&square_of_nine, time, angle, 144.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][13].increment_win_count();
          }
          backtest_matrix[index][13].increment_total_count();
          backtest_matrix[index][13].set_planet(planet.clone());
          backtest_matrix[index][13].set_harmonic(144.0);
        },
      };
      // quintile 216
      match win_rate_of_signal(&square_of_nine, time, angle, 216.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][14].increment_win_count();
          }
          backtest_matrix[index][14].increment_total_count();
          backtest_matrix[index][14].set_planet(planet.clone());
          backtest_matrix[index][14].set_harmonic(216.0);
        },
      };
      // quintile 288
      match win_rate_of_signal(&square_of_nine, time, angle, 288.0, reversals.clone(), margin_of_error) {
        None => debug!("None"),
        Some(points) => {
          if points.len() > 0 {
            backtest_matrix[index][15].increment_win_count();
          }
          backtest_matrix[index][15].increment_total_count();
          backtest_matrix[index][15].set_planet(planet.clone());
          backtest_matrix[index][15].set_harmonic(288.0);
        },
      };
    }
  }

  // write results to console and file
  println!("Number of Reversals in last {} days: {}\r", time_period, reversals.len());
  println!("Reversal defined by price extreme of +/- the adjacent {} candles", reversal_candle_range);
  println!("Margin of Error within actual reversal candle close: {}%", (margin_of_error * 100.0));
  println!("Square of Nine step interval: {}", square_of_nine_step);
  let result_file_buf = &PathBuf::from(RESULTS_PATH);
  let mut file = File::create(result_file_buf).unwrap();
  let _ = file.write(format!("Number of Reversals in last {} days: {}\r", time_period, reversals.len()).as_bytes())
    .expect("Unable to write to file");
  let _ = file.write(format!("Reversal defined by price extreme of +/- the adjacent {} candles\r", reversal_candle_range).as_bytes())
    .expect("Unable to write to file");
  let _ = file.write(format!("Margin of Error within actual reversal candle close: {}%\r", (margin_of_error * 100.0)).as_bytes())
    .expect("Unable to write to file");
  let _ = file.write(format!("Square of Nine step interval: {}\r", square_of_nine_step).as_bytes())
    .expect("Unable to write to file");

  for (index, planet) in backtest_matrix.iter().enumerate() {
    println!();
    println!("{:?}", planets[index].to_str());
    let _ = file.write(format!("{:?}\r", planets[index].to_str()).to_string().as_bytes()).expect("Unable to write to file");
    for backtest in planet.iter() {
      if let Some(harmonic) = backtest.get_harmonic() {
        // println!(
        //   "\tHarmonic: {:?}\t\tWin Rate: {:?}%\t\tWin Count: {:?}\t\tTotal Count: {:?}",
        //   harmonic.round(),
        //   (backtest.get_win_rate() * 100.0).round(),
        //   backtest.win_count,
        //   backtest.total_count
        // );
        println!(
          "\tHarmonic: {:?}\t\tWin Rate: {:?}%",
          harmonic.round(),
          (backtest.get_win_rate() * 100.0).round()
        );
        let _ = file.write(format!(
          "\tHarmonic: {:?}\t\tWin Rate: {:?}%\r",
          harmonic.round(),
          (backtest.get_win_rate() * 100.0).round()
        ).to_string().as_bytes()).expect("Unable to write to file");
      }
    }
  }
  println!("\r
      The “win rates” are the odds the algorithm would have known the day a reversal will occur\r
      and been within {}% of entering a trade at the close of that reversal candle.\r
      When and where almost perfectly sniped.\r", (margin_of_error * 100.0)
  );
  let _ = file.write(format!("\r
      The “win rates” are the odds the algorithm would have known the day a reversal will occur\r
      and been within {}% of entering a trade at the close of that reversal candle.\r
      When and where almost perfectly sniped.\r", (margin_of_error * 100.0)).as_bytes()
  ).expect("Unable to write to file");
}

/// Identify all Square of Nine prices at the harmonic relative to a planet: `planet_angle + angle_diff`.
/// Backtest those price points to actual reversals to determine win rate.
pub fn win_rate_of_signal(
  square_of_nine: &SquareOfNine,
  date: Time,
  planet_angle: f32,
  angle_diff: f32,
  reversals: Vec<Candle>,
  margin_of_error: f64
) -> Option<Vec<Point>> {
  let price_equals_time = square_of_nine.find_price_equals_time(planet_angle + angle_diff);
  // vector of Points that align with local high on the same day. Returns win rate as percentage.
  compare_signals(reversals, price_equals_time, date, margin_of_error)
}

/// Search vector of price reversals for reversal on date.
/// Search SquareOfNine price longitudes for price on date of reversal.
/// Return Vector of SquareOfNine Points that align with price reversals on date, date, and win rate
pub fn compare_signals(price_reversals: Vec<Candle>, harmonic_points: Vec<Point>, harmonic_date: Time, margin_of_error: f64) -> Option<Vec<Point>> {
  let reversal_candle = get_reversal_at_date(price_reversals, harmonic_date);

  // reversal candle found on date
  if let Some(reversal_candle) = reversal_candle {
    let mut signals = Vec::new();
    for point in harmonic_points.iter() {
      let margin = point.value * margin_of_error;
      if point.value - margin <= reversal_candle.close && point.value + margin >= reversal_candle.close {
        signals.push(*point);
      }
    }
    Some(signals)
  // no reversal candle found on date
  } else {
    None
  }
}

/// Search vector of price reversals for reversal on date.
pub fn get_reversal_at_date(reversals: Vec<Candle>, date: Time) -> Option<Candle> {
  for reversal in reversals.iter() {
    if date.as_string() == reversal.date.as_string() {
      return Some(reversal.clone());
    }
  }
  None
}
