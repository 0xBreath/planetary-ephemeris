use std::path::PathBuf;
use log::LevelFilter;
use simplelog::{
  ColorChoice, Config, TerminalMode, TermLogger,
};
use time_series::*;

const SPX_PFS_FILE: &str = "./SPX/SPX_pfs.png";

#[tokio::main]
async fn main() {
  init_logger();

  // load TickerData with SPX price history
  let spx_1960_2023 = &PathBuf::from("./SPX/1960_2023.csv");
  let mut ticker_data = TickerData::new();
  ticker_data.add_csv_series(&PathBuf::from(spx_1960_2023)).expect("Failed to add CSV to TickerData");

  // stream real-time data from RapidAPI to TickerData
  let rapid_api = RapidApi::new("SPX".to_string());
  let candles = rapid_api.query(Interval::Daily).await;
  ticker_data.add_series(candles).expect("Failed to add API series to TickerData");
  // write full ticker_data history to CSV
  dataframe::ticker_dataframe(&ticker_data, &PathBuf::from("./SPX/SPX_history.csv"));

  let start_date = Time::new(2022, &Month::July, &Day::Fourteen);
  let end_date = Time::new(2023, &Month::January, &Day::Fourteen);
  let cycle_years = 10;

  let pfs = PlotPFS::new(cycle_years, start_date, end_date);
  let daily_pfs = pfs.pfs(&ticker_data);

  pfs.plot_pfs(&daily_pfs,SPX_PFS_FILE, plot_color);
}


pub fn init_logger() {
  TermLogger::init(
    LevelFilter::Info,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  ).expect("failed to initialize logger");
}


// TODO: convert these to tests

// let ticker_data = TickerData::new_from_csv(&PathBuf::from(TICKER_DATA_PATH));
// let reversals = ticker_data.find_reversals();
// for reversal in reversals.iter() {
//   println!("{}\t{}", reversal.candle.date.as_string(), reversal.reversal_type.as_string());
// }

// println!("----------------------------------------------------------------------------------------");
// println!("\t\t### PLANET PAIR ALIGNMENTS FOR PERIOD ###\t\t");
// let planet_matrix = PlanetMatrix::new(
//   Origin::Geocentric,
//   &Time::new(1990, &Month::from_num(1), &Day::from_num(1)),
//   &Time::new(2025, &Month::from_num(3), &Day::from_num(1)),
//   2.0,
//   &Planet::to_vec(),
//   &Alignment::to_vec()
// ).await.unwrap();
// let planet_filter = vec![Planet::Mars];
// let alignment_filter = [vec![Alignment::Conjunct]].concat();
// let filtered_matrix = planet_matrix.filter_matrix(1, planet_filter, alignment_filter);
// planet_matrix.print_filtered_matrix(filtered_matrix);

// println!("----------------------------------------------------------------------------------------");
// println!("\t\t### SINGLE PLANET EPHEMERIS ###\t\t");
// print_planet_ephemeris(
//   &PathBuf::from(MARS_EPHEMERIS),
//   Origin::Heliocentric,
//   Planet::Mars,
//   DataType::RightAscension,
//   Time::new(2010, &Month::from_num(1), &Day::from_num(1)),
//   Time::new(2023, &Month::from_num(3), &Day::from_num(1))
// ).await;

// println!("----------------------------------------------------------------------------------------");
// println!("\t\t### SQUARE OF NINE ###\t\t");
// SquareOfNine::test_square_of_nine(11);

// println!("----------------------------------------------------------------------------------------");
// let price_planet = PricePlanet::new(
//   PathBuf::from(PRICE_PLANET_RESULTS_PATH),
//   10,
//   0.1,
//   0.01,
//   Time::new(2013, &Month::from_num(1), &Day::from_num(1)),
//   Time::new(2023, &Month::from_num(3), &Day::from_num(1)),
// ).await.unwrap();
// // // println!("\t\t### SINGLE PRICE PLANET HARMONICS ###\t\t");
// // // price_planet.single_signal(0.02, 1).await;
// println!("\t\t### CONFLUENT PRICE PLANET HARMONICS ###\t\t");
// price_planet.confluent_signals(0.03, 0).await;

// println!("----------------------------------------------------------------------------------------");
// println!("\t\t### RETROGRADE ###\t\t");
// let retrograde = Retrograde::new(
//   Time::new(2023, &Month::from_num(1), &Day::from_num(1)),
//   Time::new(2023, &Month::from_num(3), &Day::from_num(1)),
//   &Planet::to_vec(),
// ).await.unwrap();
// retrograde.print();
// // retrograde.backtest(10, 1.0, 2).await.unwrap();
// retrograde.confluent_retrograde(2);

// println!("----------------------------------------------------------------------------------------");
// println!("\t\t### ECLIPSE CONFLUENT SIGNALS ###\t\t");
// let eclipses = Eclipses::new(
//   &PathBuf::from(SOLAR_ECLIPSE_CSV),
//   &PathBuf::from(LUNAR_ECLIPSE_CSV)
// );
// eclipses.print(
//   &PathBuf::from(ECLIPSES_DATAFRAME_CSV),
//   &Time::new(2013, &Month::from_num(1), &Day::from_num(1)),
//   &Time::new(2023, &Month::from_num(6), &Day::from_num(1)),
// );
// eclipses.test_confluence(
//   Time::new(2013, &Month::from_num(1), &Day::from_num(1)),
//   Time::new(2023, &Month::from_num(3), &Day::from_num(1)),
//   3,
//   1.0,
//   vec![Planet::Sun, Planet::Jupiter, Planet::Saturn, Planet::Uranus, Planet::Neptune, Planet::Pluto],
//   vec![Alignment::Conjunct, Alignment::Opposite, Alignment::Square90, Alignment::Square270, Alignment::Trine120, Alignment::Trine240],
// ).await.unwrap();
