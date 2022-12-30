use std::path::PathBuf;
use log::{debug, LevelFilter};
use simplelog::{
  ColorChoice, Config, TerminalMode, TermLogger,
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
    0.01
  ).await;
  println!("----------------------------------------------------------------------------------------");
  test_market_structure();
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

pub fn test_market_structure() {
let candle_range: usize = 10;
  let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
  let market_structure = MarketStructure::new(ticker_data, candle_range);

  match &market_structure.latest_high {
    Some(high) => println!("Latest High: {}", high.date.as_string()),
    None => println!("Latest High: None"),
  };
  match &market_structure.latest_low {
    Some(low) => println!("Latest Low: {}", low.date.as_string()),
    None => println!("Latest Low: None"),
  };
  println!("START\t\tEND\t\tREVERSAL\t\tTREND");
  for trend in market_structure.trends.iter() {
    match &trend.start_candle {
      Some(candle) => print!("{}", candle.date.as_string()),
      None => print!("None"),
    };
    match &trend.end_candle {
      Some(candle) => print!("\t{}", candle.date.as_string()),
      None => print!("\tNone\t"),
    };
    match &trend.reversal {
      Some(reversal) => print!("\t{}\t\t", reversal.candle.date.as_string()),
      None => print!("\tNone\t\t"),
    };
    print!("{:?}", trend.direction.as_ref());
    println!();
  }
}

pub async fn test_planetary_matrix() {
  let candle_range: usize = 10;
  let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
  let local_highs = ticker_data.find_local_highs(candle_range);
  let local_lows = ticker_data.find_local_lows(candle_range);

  let alignment_margin_error = 1.5;
  let matrix = ephemeris::planetary_matrix(
    Origin::Geocentric,
    &Time::today(),
    -365,
    alignment_margin_error
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

/// Identify all dates where the price longitude on a Square of Nine equals the longitude of all possible planets (including Sun and Moon)
/// Backtest the Time=Price against actual reversals in the market
/// Compute win rate each planet longitude (time) and price longitude (price) for a all possible harmonics (90, 120, 180, etc)
/// Return matrix of all possibilities of (planet, harmonic, win rate), and print results to file.
pub async fn price_planet_harmonics(time_period: i64, square_of_nine_step: f64, reversal_candle_range: usize, margin_of_error: f64) {
  let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
  let market_structure = MarketStructure::new(ticker_data, reversal_candle_range);
  let reversals = market_structure.reversals.clone();
  // let mut local_highs = ticker_data.find_local_highs(reversal_candle_range);
  // let mut local_lows = ticker_data.find_local_lows(reversal_candle_range);
  // let mut reversals = Vec::new();
  // reversals.append(&mut local_highs);
  // reversals.append(&mut local_lows);

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

  let mut backtest_matrix = vec![vec![Backtest::default(); 16]; planets.len()];

  for (index, planet) in planets.iter().enumerate() {
    let daily_longitudes = Query::query(
      Origin::Geocentric,
      planet,
      DataType::RightAscension,
      Time::today(),
      time_period
    ).await;

    for (time, angle) in daily_longitudes.into_iter() {
      for harmonic_index in 0..16 {
        let angle_diff = match harmonic_index {
          0 => 0.0,
          1 => 180.0,
          2 => 90.0,
          3 => 270.0,
          4 => 120.0,
          5 => 240.0,
          6 => 45.0,
          7 => 135.0,
          8 => 225.0,
          9 => 315.0,
          10 => 60.0,
          11 => 300.0,
          12 => 72.0,
          13 => 144.0,
          14 => 216.0,
          15 => 288.0,
          _ => panic!("Invalid index"),
        };
        let reversal_candles = reversals.iter().map(|reversal| &reversal.candle).collect::<Vec<&Candle>>();
        match win_rate_of_signal(&square_of_nine, time, angle, angle_diff, reversal_candles, margin_of_error) {
          None => debug!("None"),
          Some(points) => {
            if !points.is_empty() {
              backtest_matrix[index][harmonic_index].increment_win_count();
            }
            backtest_matrix[index][harmonic_index].increment_total_count();
            backtest_matrix[index][harmonic_index].set_planet(planet.clone());
            backtest_matrix[index][harmonic_index].set_harmonic(angle_diff);
          },
        };
      }
    }
  }

  // write results to console and file
  let results_file = &PathBuf::from(RESULTS_PATH);
  market_structure.print(
    results_file,
    time_period,
    reversal_candle_range,
    margin_of_error,
    square_of_nine_step,
    &planets,
    &backtest_matrix
  );
}

/// Identify all Square of Nine prices at the harmonic relative to a planet: `planet_angle + angle_diff`.
/// Backtest those price points to actual reversals to determine win rate.
pub fn win_rate_of_signal(
  square_of_nine: &SquareOfNine,
  date: Time,
  planet_angle: f32,
  angle_diff: f32,
  reversals: Vec<&Candle>,
  margin_of_error: f64
) -> Option<Vec<Point>> {
  let price_equals_time = square_of_nine.find_price_equals_time(planet_angle + angle_diff);
  // vector of Points that align with local high on the same day. Returns win rate as percentage.
  compare_signals(reversals, price_equals_time, date, margin_of_error)
}

/// Search vector of price reversals for reversal on date.
/// Search SquareOfNine price longitudes for price on date of reversal.
/// Return Vector of SquareOfNine Points that align with price reversals on date, date, and win rate
pub fn compare_signals(price_reversals: Vec<&Candle>, harmonic_points: Vec<Point>, harmonic_date: Time, margin_of_error: f64) -> Option<Vec<Point>> {
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
pub fn get_reversal_at_date(reversals: Vec<&Candle>, date: Time) -> Option<&Candle> {
  reversals.into_iter().find(|&candle| date.as_string() == candle.date.as_string())
}
