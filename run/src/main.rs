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

  //range_low();
  //two_planets_align_with_start
  //query_planet().await;
  square_of_nine();
}

fn square_of_nine() {
  let square_of_nine = SquareOfNine::<7>::new(Input::Angle(1.0));
  for y in square_of_nine.matrix.iter() {
    for x in y {
      // match x.value {
      //   Input::Date(time) => print!("{}\t", time.as_string()),
      //   Input::Angle(angle) => print!("{}\t", angle),
      // }
      match x.harmonic {
        Some(harmonic) => print!("{}\t", harmonic as i32),
        None => print!("-\t"),
      }
    }
    println!();
  }
}

async fn query_planet() {
  let vec = Query::query(
    Origin::Geocentric,
    &Planet::Venus,
    Time::new(2021, &Month::November, &Day::Eight),
    500,
  ).await;
  for (time, ra) in vec {
    println!("{}\t{}", time.as_string(), ra);
  }
}

async fn two_planets_align_with_start() {
  let matrix = ephemeris::two_planets_align_with_start_angle(
    Origin::Geocentric,
    &Time::new(2017, &Month::December, &Day::Sixteen),
    1000,
  ).await;
  for (planet_a, planet_b, alignments) in matrix.into_iter() {
    if !alignments.is_empty() {
      println!("{}-{}", planet_a.to_str(), planet_b.to_str());
    }
    for (time, ra, alignment) in alignments {
      println!("\t{}\t{}\t{}", time.as_string(), ra, alignment.to_str());
    }
  }
}

async fn range_low() {
  let range_high_date = Time::new(2017, &Month::December, &Day::Sixteen);
  let planet = Planet::Venus;
  let (range_high_ra, end_date, range_low_ra) = ephemeris::range_low_using_planet_at_range_high(
    Origin::Geocentric,
    &range_high_date,
    &planet,
  ).await.expect("planet moves too slowly to find range low");
  println!("{} Range High\t{}\t{}", planet.to_str(), range_high_date.as_string(), range_high_ra);
  println!("{} Range Low\t{}\t{}", planet.to_str(), end_date.as_string(), range_low_ra);
}


fn init_logger() {
  TermLogger::init(
    LevelFilter::Info,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  ).expect("failed to initialize logger");
}
