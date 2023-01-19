use reqwest::Response;
use ephemeris::Time;
use crate::Candle;

pub const QUANDL_API_URL_PREFIX: &str = "https://data.nasdaq.com/api/v3/datasets/WIKI/";
pub const QUANDL_API_URL_SUFFIX: &str = "&order=asc&column_index=4&collapse=daily&transformation=none";
pub const QUANDL_API_KEY: &str = "y6fK4vxt6TxY8yU-f8Di";
pub const QUANDL_BTCUSD_URL: &str = "https://data.nasdaq.com/api/v3/datasets/BITFINEX/BTCUSD.csv?api_key=y6fK4vxt6TxY8yU-f8Di";


#[derive(Debug)]
pub struct QuandlApiWrapper {
  pub start_date: Time,
  pub end_date: Time,
  pub candles: Vec<Candle>
}

impl QuandlApiWrapper {
  /// Wired to only query BTCUSD
  pub async fn new(start_date: Time, end_date: Time) -> Self {
    // BTCUSD available history 2015-present in CSV format
    let csv_ticker_data = reqwest::get(QUANDL_BTCUSD_URL).await.expect("failed to get response from QUANDL API")
      .text().await.expect("failed to get text from QUANDL API response");

    let mut candles = Vec::new();
    // Date,High,Low,Mid,Last,Bid,Ask,Volume
    for (index, line) in csv_ticker_data.lines().enumerate() {
      if index < 2 {
        continue;
      }
      let record = line.split(",").collect::<Vec<&str>>();
      let record_before = csv_ticker_data.lines().nth(index - 1).unwrap().split(",").collect::<Vec<&str>>();
      let date = Time::from_quandl_api_date_format(record[0]);
      candles.push(Candle {
        date,
        open: record_before[4].parse::<f64>().unwrap(),
        high: record[1].parse::<f64>().unwrap(),
        low: record[2].parse::<f64>().unwrap(),
        close: record[4].parse::<f64>().unwrap(),
      })
    }

    Self {
      start_date,
      end_date,
      candles
    }
  }
}