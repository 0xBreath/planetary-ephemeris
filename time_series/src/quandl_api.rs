use reqwest::Response;
use ephemeris::Time;

pub const QUANDL_API_URL_PREFIX: &str = "https://data.nasdaq.com/api/v3/datasets/WIKI/";
pub const QUANDL_API_URL_SUFFIX: &str = "&order=asc&column_index=4&collapse=daily&transformation=none";
pub const QUANDL_API_KEY: &str = "y6fK4vxt6TxY8yU-f8Di";


#[derive(Debug)]
pub struct QuandlApiWrapper {
  pub ticker: String,
  pub start_date: Time,
  pub end_date: Time,
  pub response: serde_json::Value
}

impl QuandlApiWrapper {
  pub async fn new(ticker: String, start_date: Time, end_date: Time) -> Self {
    let query = &format!(
      "{}{}.json?start_date={}&end_date={}{}&api_key={}",
      QUANDL_API_URL_PREFIX, ticker,
      start_date.as_string(), end_date.as_string(),
      QUANDL_API_URL_SUFFIX, QUANDL_API_KEY
    );

    let data = reqwest::get(query).await.expect("failed to get response from QUANDL API")
      .text().await.expect("failed to get text from QUANDL API response");

    let response: serde_json::Value = serde_json::from_str(data.as_str())
      .expect("failed to parse QUANDL API response into JSON");
    println!("{:?}", response);

    Self {
      ticker,
      start_date,
      end_date,
      response
    }
  }
}