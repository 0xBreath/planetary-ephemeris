
use std::path::PathBuf;
use log::LevelFilter;
use simplelog::{
  ColorChoice, Config, TerminalMode, TermLogger,
};
use ephemeris::*;
use time_series::*;
use toolkit::*;

pub const TICKER_DATA_PATH: &str = "BTCUSD.csv";
pub const RESULTS_PATH: &str = "BTCUSD_results.txt";

#[tokio::main]
async fn main() {
  init_logger();

  println!("----------------------------------------------------------------------------------------");
  println!("\t\t### PLANET ENTER/EXIT RETROGRADE ###\t\t");
  let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
  let retrograde = Retrograde::new(ticker_data).await;
  retrograde.backtest(20);
  // println!("\t\t### PLANET PAIR ALIGNMENT MATRIX ###\t\t");
  // PlanetMatrix::test_planet_matrix(
  //   &PathBuf::from(TICKER_DATA_PATH),
  //   1,
  //   1.0,
  //   10
  // ).await;
  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### LUNAR ZERO DECLINATION CROSS ###\t\t");
  // LunarDeclination::test_lunar_declination(
  //   Time::today(),
  //   10,
  //   0
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
  //   PathBuf::from(RESULTS_PATH),
  //   10,
  //   100.0,
  // ).await;
  // println!("\t\t### CONFLUENT PRICE PLANET HARMONICS ###\t\t");
  // price_planet.confluent_signals(0.005, 0).await;

  // println!();
  // println!("\t\t### SINGLE PRICE PLANET HARMONICS ###\t\t");
  //price_planet.single_signal(0.005, 0).await;
}

pub fn init_logger() {
  TermLogger::init(
    LevelFilter::Info,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  ).expect("failed to initialize logger");
}
