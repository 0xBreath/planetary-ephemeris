use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use csv;
use ephemeris::{Planet, Time};
use crate::{Backtest, Candle, Reversal, ReversalType, TickerData};

#[derive(Debug, Clone)]
pub enum Direction {
  Up,
  Down,
}
impl Direction {
  pub fn as_string(&self) -> &str {
    match self {
      Direction::Up => "Up",
      Direction::Down => "Down",
    }
  }
}

#[derive(Debug, Clone)]
pub struct Trend {
  pub start_candle: Option<Candle>,
  pub end_candle: Option<Candle>,
  pub reversal: Option<Reversal>,
  pub direction: Option<Direction>,
}

#[derive(Clone, Debug)]
pub struct MarketStructure {
  pub candles: Vec<Candle>,
  pub reversals: Vec<Reversal>,
  pub trends: Vec<Trend>,
  pub latest_high: Option<Candle>,
  pub latest_low: Option<Candle>,
}

impl MarketStructure {
  /// Identify market structure in vector of reversals .
  /// by finding higher highs and higher lows for positive market structure,
  /// and lower highs and lower lows for negative market structure.
  pub fn new(ticker_data: TickerData, candle_range: usize) -> Self {
    let mut trends = Vec::<Trend>::new();
    let reversals = ticker_data.find_reversals(candle_range);
    println!("First Candle: {:?}", ticker_data.candles[0].date.as_string());
    println!("Last Candle: {:?}", ticker_data.candles[ticker_data.candles.len() - 1].date.as_string());
    println!("First Reversal: {:?}", reversals[0].candle.date.as_string());
    println!("Last Reversal: {:?}", reversals[reversals.len() - 1].candle.date.as_string());


    let mut direction: Option<Direction> = None;
    let mut start_candle: Option<Candle> = None;
    let mut latest_low: Option<Candle> = None;
    let mut latest_high: Option<Candle> = None;
    // iterate lows and identify series of higher lows
    for reversal in reversals.iter() {
      match direction {
        // no trend established yet
        None => {
          start_candle = Some(reversal.candle.clone());
          match reversal.reversal_type {
            ReversalType::Top => {
              if let Some(latest_high) = &latest_high {
                // positive trend
                if reversal.candle.close > latest_high.close {
                  trends.push(Trend {
                    start_candle: start_candle.clone(),
                    end_candle: None,
                    reversal: Some(reversal.clone()),
                    direction: Some(Direction::Up),
                  });
                  direction = Some(Direction::Up);
                }
                // negative trend
                else {
                  trends.push(Trend {
                    start_candle: start_candle.clone(),
                    end_candle: None,
                    reversal: Some(reversal.clone()),
                    direction: Some(Direction::Down),
                  });
                  direction = Some(Direction::Down);
                }
              }
              latest_high = Some(reversal.candle.clone());
            },
            ReversalType::Bottom => {
              if let Some(latest_low) = &latest_low {
                // positive trend
                if reversal.candle.close > latest_low.close {
                  trends.push(Trend {
                    start_candle: start_candle.clone(),
                    end_candle: None,
                    reversal: Some(reversal.clone()),
                    direction: Some(Direction::Up),
                  });
                  direction = Some(Direction::Up);
                }
                // negative trend
                else {
                  trends.push(Trend {
                    start_candle: start_candle.clone(),
                    end_candle: None,
                    reversal: Some(reversal.clone()),
                    direction: Some(Direction::Down),
                  });
                  direction = Some(Direction::Down);
                }
              }
              latest_low = Some(reversal.candle.clone());
            },
          }
        },
        // positive market structure
        Some(Direction::Up) => {
          match reversal.reversal_type {
            // compare current high to previous high
            ReversalType::Top => {
              if let Some(latest_high) = &latest_high {
                // positive trend continues
                if reversal.candle.close > latest_high.close {
                  trends.push(Trend {
                    start_candle: start_candle.clone(),
                    end_candle: None,
                    reversal: Some(reversal.clone()),
                    direction: Some(Direction::Up),
                  });
                }
                // positive trend ends
                else {
                  trends.push(Trend {
                    start_candle: start_candle.clone(),
                    end_candle: Some(reversal.candle.clone()),
                    reversal: Some(reversal.clone()),
                    direction: None,
                  });
                  direction = None;
                }
              }
              latest_high = Some(reversal.candle.clone());
            },
            // compare current low to previous low
            ReversalType::Bottom => {
              if let Some(latest_low) = &latest_low {
                // positive trend continues
                if reversal.candle.close > latest_low.close {
                  trends.push(Trend {
                    start_candle: start_candle.clone(),
                    end_candle: None,
                    reversal: Some(reversal.clone()),
                    direction: Some(Direction::Down),
                  });
                }
                // positive trend ends
                else {
                  trends.push(Trend {
                    start_candle: start_candle.clone(),
                    end_candle: Some(reversal.candle.clone()),
                    reversal: Some(reversal.clone()),
                    direction: None,
                  });
                  direction = None;
                }
              }
              latest_low = Some(reversal.candle.clone());
            },
          }
        },
        // negative market structure
        Some(Direction::Down) => {
          match reversal.reversal_type {
            // compare current high to previous high
            ReversalType::Top => {
              if let Some(latest_high) = &latest_high {
                // negative trend continues
                if reversal.candle.close < latest_high.close {
                  trends.push(Trend {
                    start_candle: start_candle.clone(),
                    end_candle: None,
                    reversal: Some(reversal.clone()),
                    direction: Some(Direction::Down),
                  });
                }
                // negative trend ends
                else {
                  trends.push(Trend {
                    start_candle: start_candle.clone(),
                    end_candle: Some(reversal.candle.clone()),
                    reversal: Some(reversal.clone()),
                    direction: None,
                  });
                  direction = None;
                }
              }
              latest_high = Some(reversal.candle.clone());
            },
            // compare current low to previous low
            ReversalType::Bottom => {
              if let Some(latest_low) = &latest_low {
                // negative trend continues
                if reversal.candle.close < latest_low.close {
                  trends.push(Trend {
                    start_candle: start_candle.clone(),
                    end_candle: None,
                    reversal: Some(reversal.clone()),
                    direction: Some(Direction::Down),
                  });
                }
                // negative trend ends
                else {
                  trends.push(Trend {
                    start_candle: start_candle.clone(),
                    end_candle: Some(reversal.candle.clone()),
                    reversal: Some(reversal.clone()),
                    direction: None,
                  });
                  direction = None;
                }
              }
              latest_low = Some(reversal.candle.clone());
            },
          }
        },
      }
    }

    Self {
      candles: ticker_data.candles,
      reversals,
      trends,
      latest_high,
      latest_low
    }
  }

  pub fn print(
    &self,
    results_file: &PathBuf,
    time_period: i64,
    reversal_candle_range: usize,
    margin_of_error: f64,
    square_of_nine_step: f64,
    planets: &Vec<Planet>,
    backtest_matrix: &Vec<Vec<Backtest>>
  ) {
    // write results to console and file
    println!("Number of Reversals in last {} days: {}\r", time_period, self.reversals.len());
    println!("Reversal defined by price extreme of +/- the adjacent {} candles", reversal_candle_range);
    println!("Margin of Error within actual reversal candle close: {}%", (margin_of_error * 100.0));
    println!("Square of Nine step interval: {}", square_of_nine_step);
    let mut file = File::create(results_file).unwrap();
    let _ = file.write(format!("Number of Reversals in last {} days: {}\r", time_period, self.reversals.len()).as_bytes())
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
}