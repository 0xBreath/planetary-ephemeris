use std::path::PathBuf;
use log::LevelFilter;
use simplelog::{
  ColorChoice, Config, TerminalMode, TermLogger,
};
#[allow(unused_imports)]
use ephemeris::*;
#[allow(unused_imports)]
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

  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### PLANET PAIR ALIGNMENTS FOR PERIOD ###\t\t");
  // let planet_matrix = PlanetMatrix::new(
  //   Origin::Geocentric,
  //   &Time::new(2013, &Month::from_num(1), &Day::from_num(1)),
  //   &Time::new(2023, &Month::from_num(1), &Day::from_num(1)),
  //   1.0,
  //   Planet::to_vec(),
  //   Alignment::to_vec()
  // ).await.unwrap();
  // let planet_filter = Planet::to_vec(); //vec![Planet::Venus, Planet::Uranus];
  // let alignment_filter = vec![
  //   Alignment::Conjunct, Alignment::Opposite,
  //   Alignment::Square90, Alignment::Square270,
  //   Alignment::Trine120, Alignment::Trine240
  // ];
  // let filtered_matrix = planet_matrix.filter_matrix(1, planet_filter, alignment_filter);
  // planet_matrix.print_filtered_matrix(filtered_matrix);

  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### SQUARE OF NINE ###\t\t");
  // SquareOfNine::test_square_of_nine(41);

  // println!("----------------------------------------------------------------------------------------");
  // let price_planet = PricePlanet::new(
  //   PathBuf::from(PRICE_PLANET_RESULTS_PATH),
  //   5,
  //   0.1,
  //   0.01,
  //   Time::new(2019, &Month::from_num(7), &Day::from_num(1)),
  //   Time::new(2020, &Month::from_num(1), &Day::from_num(1)),
  // ).await.unwrap();
  // // println!("\t\t### SINGLE PRICE PLANET HARMONICS ###\t\t");
  // // price_planet.single_signal(0.02, 1).await;
  // println!("\t\t### CONFLUENT PRICE PLANET HARMONICS ###\t\t");
  // price_planet.confluent_signals(0.02, 1).await;

  // println!("----------------------------------------------------------------------------------------");
  // println!("\t\t### RETROGRADE ###\t\t");
  // let retrograde = Retrograde::new(
  //   Time::new(2015, &Month::from_num(1), &Day::from_num(1)),
  //   Time::new(2023, &Month::from_num(3), &Day::from_num(1)),
  //   &vec![Planet::Sun, Planet::Mercury, Planet::Venus, Planet::Mars, Planet::Jupiter, Planet::Saturn, Planet::Uranus, Planet::Neptune, Planet::Pluto],
  // ).await.unwrap();
  // // retrograde.backtest(10, 1.0, 2).await.unwrap();
  // retrograde.confluent_retrograde(2);

  // println!("----------------------------------------------------------------------------------------");
  println!("\t\t### ECLIPSE CONFLUENT SIGNALS ###\t\t");
  let eclipses = Eclipses::new(
    &PathBuf::from(SOLAR_ECLIPSE_CSV),
    &PathBuf::from(LUNAR_ECLIPSE_CSV)
  );
  eclipses.test_confluence(
    Time::new(2013, &Month::from_num(1), &Day::from_num(1)),
    Time::new(2023, &Month::from_num(1), &Day::from_num(1)),
    3,
    1.0,
    vec![Planet::Sun, Planet::Jupiter, Planet::Saturn, Planet::Uranus, Planet::Neptune, Planet::Pluto],
    vec![Alignment::Conjunct, Alignment::Opposite, Alignment::Square90, Alignment::Square270, Alignment::Trine120, Alignment::Trine240],
  ).await.unwrap();

}

pub fn init_logger() {
  TermLogger::init(
    LevelFilter::Info,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  ).expect("failed to initialize logger");
}
