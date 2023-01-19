use std::path::PathBuf;
use crate::{Candle, Reversal, ReversalType, TickerData};

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
  pub reversal_candle_range: usize,
}

impl MarketStructure {
  /// Identify market structure in vector of reversals .
  /// by finding higher highs and higher lows for positive market structure,
  /// and lower highs and lower lows for negative market structure.
  pub fn new(ticker_data: &TickerData, candle_range: usize) -> Self {
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
      candles: ticker_data.candles.clone(),
      reversals,
      trends,
      latest_high,
      latest_low,
      reversal_candle_range: candle_range
    }
  }

  pub fn test_market_structure(candle_range: usize, results_file: &PathBuf) {
    let ticker_data = TickerData::new_from_csv(results_file);
    let market_structure = MarketStructure::new(&ticker_data, candle_range);

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
}