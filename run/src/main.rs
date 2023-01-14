
use std::path::PathBuf;
use log::LevelFilter;
use simplelog::{
  ColorChoice, Config, TerminalMode, TermLogger,
};
#[allow(unused_imports)]
use ephemeris::*;
use time_series::*;
use toolkit::*;

#[tokio::main]
async fn main() {
  init_logger();

  // let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
  // let reversals = ticker_data.find_reversals();
  // for reversal in reversals.iter() {
  //   println!("{}\t{}", reversal.candle.date.as_string(), reversal.reversal_type.as_string());
  // }

  // println!("\t\t### PLANET PAIR ALIGNMENTS FOR PERIOD ###\t\t");
  // let planet_matrix = PlanetMatrix::new(
  //   Origin::Geocentric,
  //   &Time::today(),
  //   -200,
  //   1.0
  // ).await;
  // planet_matrix.print_alignments(&PathBuf::from(PLANET_MATRIX_RESULTS_PATH));
  // println!("----------------------------------------------------------------------------------------");

  // println!("\t\t### PLANET PAIR ALIGNMENT MATRIX ###\t\t");
  // PlanetMatrix::test_planet_matrix(
  //   &PathBuf::from(TICKER_DATA_PATH),
  //   2,
  //   5.0,
  //   10
  // ).await;

  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### PLANET ENTER/EXIT RETROGRADE ###\t\t");
  // let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
  // let retrograde = Retrograde::new(ticker_data).await;
  // retrograde.backtest(20);
  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### PLANET ZERO DECLINATION CROSS ###\t\t");
  // PlanetEquatorCrosses::test_declinations(
  //   Time::today(),
  //   5,
  //   0,
  // ).await;
  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### SQUARE OF NINE ###\t\t");
  // SquareOfNine::test_square_of_nine();
  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### IDENTIFY MARKET STRUCTURE ###\t\t");
  // MarketStructure::test_market_structure(5, &PathBuf::from(TICKER_DATA_PATH));
  // println!("----------------------------------------------------------------------------------------");

  // let price_planet = PricePlanet::new(
  //   PathBuf::from(TICKER_DATA_PATH),
  //   PathBuf::from(PRICE_PLANET_RESULTS_PATH),
  //   10,
  //   100.0,
  // ).await;
  // println!("\t\t### CONFLUENT PRICE PLANET HARMONICS ###\t\t");
  // price_planet.confluent_signals(0.005, 0).await;

  // println!();
  // println!("\t\t### SINGLE PRICE PLANET HARMONICS ###\t\t");
  //price_planet.single_signal(0.005, 0).await;

  // dataframe::retrograde_dataframe(
  //   "BITCOIN".to_string(),
  //   Time::new(2023, &Month::from_num(1), &Day::from_num(1)),
  //   Time::today(),
  //   &PathBuf::from(RETROGRADE_RESULTS_CSV)
  // ).await;

  // dataframe::ticker_dataframe(
  //   &PathBuf::from(TICKER_DATA_PATH),
  //   &PathBuf::from(TICKER_DATAFRAME_CSV)
  // ).await;

  let eclipses = Eclipses::new(
    &PathBuf::from(SOLAR_ECLIPSE_CSV),
    &PathBuf::from(LUNAR_ECLIPSE_CSV)
  );
  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### PLANET SELF ALIGNMENT IN TWO ECLIPSES ###\t\t");
  // eclipses.find_planet_self_alignment_in_two_eclipses(
  //   Time::new(2019, &Month::from_num(1), &Day::from_num(1)),
  //   Time::new(2024, &Month::from_num(1), &Day::from_num(1)),
  //   1,
  //   1.5,
  // ).await;
  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### PLANET EQUATOR CROSS IN TWO ECLIPSES ###\t\t");
  // eclipses.find_planet_equator_cross_in_two_eclipses(
  //   Time::new(1990, &Month::from_num(1), &Day::from_num(1)),
  //   Time::today(),
  //   1
  // ).await;
  println!("----------------------------------------------------------------------------------------");
  println!("\t\t### PLANET PAIR ALIGNMENTS ON ECLIPSE ###\t\t");
  eclipses.find_planet_matrix_alignments_on_eclipse(
    Time::new(2017, &Month::from_num(1), &Day::from_num(1)),
    Time::new(2024, &Month::from_num(1), &Day::from_num(1)),
    1.5,
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
