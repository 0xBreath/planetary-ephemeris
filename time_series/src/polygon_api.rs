use reqwest::Response;
use ephemeris::Time;

pub const POLYGON_API_BASE_QUERY: &str = "https://api.polygon.io/v2/aggs/ticker/";
pub const POLYGON_API_KEY: &str = "pgK3BU2e2n2IJVNFVEytYNa9NB78Z9E_";

#[derive(Debug)]
pub enum TickerType {
  Stocks,
  Crypto,
}

#[derive(Debug)]
pub enum TickerSymbol {
  Stocks(String),
  Crypto(String),
}

impl TickerSymbol {
  pub fn to_query_format(&self) -> String {
    match self {
      TickerSymbol::Stocks(ticker) => ticker.to_string(),
      TickerSymbol::Crypto(ticker) => format!("X:{}", ticker.to_string()),
    }
  }

  pub fn to_string(self) -> String {
    match self {
      TickerSymbol::Stocks(ticker) => ticker.to_string(),
      TickerSymbol::Crypto(ticker) => ticker.to_string(),
    }
  }
}

#[derive(Debug)]
pub struct PolygonApiWrapper {
  pub ticker: TickerSymbol,
  pub start_date: Time,
  pub end_date: Time,
  pub response: serde_json::Value
}

impl PolygonApiWrapper {
  pub async fn new(ticker: TickerSymbol, start_date: Time, end_date: Time) -> Self {
    let symbol = ticker.to_query_format();
    let query = &format!(
      "{}{}/range/1/day/{}/{}?adjusted=true&sort=asc&limit=50000&apiKey={}",
      POLYGON_API_BASE_QUERY, symbol,
      start_date.as_string(), end_date.as_string(),
      POLYGON_API_KEY
    );
    println!("{}", query);
    let data = reqwest::get(query).await.expect("failed to get response from polygon api")
      .text().await.expect("failed to get text from polygon api response");

    let response: serde_json::Value = serde_json::from_str(data.as_str())
      .expect("failed to parse Polygon API response into JSON");
    println!("{:?}", response);

    Self {
      ticker,
      start_date,
      end_date,
      response
    }
  }
}