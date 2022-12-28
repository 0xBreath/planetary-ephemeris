use std::path::PathBuf;
use log::LevelFilter;
use simplelog::{
  ColorChoice, Config, TermLogger, TerminalMode,
};
use ephemeris::*;
use time_series::*;

pub const TICKER_DATA_PATH: &str = "BTCUSD.csv";

#[tokio::main]
async fn main() {
  init_logger();

  //test_planetary_matrix().await;
  test_square_of_nine();
  //test_square_of_nine_high_res();
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
  let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
  let local_highs = ticker_data.find_local_highs();
  let local_lows = ticker_data.find_local_lows();

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
  let matrix = ephemeris::planetary_matrix(
    Origin::Geocentric,
    &Time::today(),
    -365,
  ).await;
  for (planet_a, planet_b, alignments) in matrix.into_iter() {
    if !alignments.is_empty() {
      println!("{}-{}", planet_a.to_str(), planet_b.to_str());
    }
    for (time, _, alignment) in alignments {
      println!("{}\t{}", time.as_string(), alignment.to_str());
    }
    println!();

    // let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
    // let local_highs = ticker_data.find_local_highs();
    // let local_lows = ticker_data.find_local_lows();
    //
    // for (time, ra, alignment) in alignments {
    //   //println!("\t{}\t{}\t{}", time.as_string(), ra, alignment.to_str());
    //   for high in local_highs.iter() {
    //     if time.delta_date(-1).as_string() == high.date.as_string() {
    //       println!("HIGH DAY BEFORE\t{}\t{}", time.as_string(), alignment.to_str());
    //     }
    //     else if time.as_string() == high.date.as_string() {
    //       println!("HIGH SAME DAY\t{}\t{}", time.as_string(), alignment.to_str());
    //     }
    //     else if time.delta_date(1).as_string() == high.date.as_string() {
    //       println!("HIGH DAY AFTER\t{}\t{}", time.as_string(), alignment.to_str());
    //     }
    //   }
    //   for low in local_lows.iter() {
    //     if time.delta_date(-1).as_string() == low.date.as_string() {
    //       println!("LOW DAY BEFORE\t{}\t{}", time.as_string(), alignment.to_str());
    //     }
    //     else if time.as_string() == low.date.as_string() {
    //       println!("LOW SAME DAY\t{}\t{}", time.as_string(), alignment.to_str());
    //     }
    //     else if time.delta_date(1).as_string() == low.date.as_string() {
    //       println!("LOW DAY AFTER\t{}\t{}", time.as_string(), alignment.to_str());
    //     }
    //   }
    // }
    // println!();

  }
}

pub fn test_square_of_nine() {
  const N: usize = 81;
  let square_of_nine = SquareOfNine::<N>::new(1, 10);
  if N < 13 {
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
  for point in square_of_nine.values.into_iter() {
    match point.harmonic {
      None => {
        match point.arc {
          None => println!("{:?}\t{:?}\t\t\t{}\r", point.value, point.harmonic, "-"),
          Some(arc) => println!("{:?}\t{:?}\t\t\t{:?}\r", point.value, point.harmonic, point.arc.unwrap()),
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
