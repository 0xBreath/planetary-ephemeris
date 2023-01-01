
use std::path::PathBuf;
use log::{debug, LevelFilter};
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

  // println!("\t\t### PLANET PAIR ALIGNMENT MATRIX ###\t\t");
  // PlanetMatrix::test_planet_matrix(
  //   &PathBuf::from(TICKER_DATA_PATH),
  //   1,
  //   1.0,
  //   10
  // ).await;
  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### LUNAR ZERO DECLINATION CROSS ###\t\t");
  // LunarDeclination::test_lunar_declination(-720, Time::today(), 10, 1).await;
  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### SQUARE OF NINE ###\t\t");
  // SquareOfNine::test_square_of_nine();
  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### IDENTIFY MARKET STRUCTURE ###\t\t");
  // MarketStructure::test_market_structure(5, &PathBuf::from(TICKER_DATA_PATH));
  // println!("----------------------------------------------------------------------------------------");
  println!("\t\t### PRICE PLANET HARMONICS ###\t\t");
  PricePlanet::new(
    PathBuf::from(TICKER_DATA_PATH),
    PathBuf::from(RESULTS_PATH),
    10,
    100.0,
  ).await.single_price_planet_signal(0.01,0).await;
}

pub fn init_logger() {
  TermLogger::init(
    LevelFilter::Info,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  ).expect("failed to initialize logger");
}
