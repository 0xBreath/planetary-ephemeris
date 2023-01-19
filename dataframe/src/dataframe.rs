use std::path::PathBuf;
use toolkit::*;
use time_series::*;
#[allow(unused_imports)]
use ephemeris::*;
#[allow(unused_imports)]
use csv::{Writer, WriterBuilder};
#[allow(unused_imports)]
use log::info;


// pub async fn planet_matrix_dataframe(
//   ticker_data_path: &PathBuf,
//   alignment_margin_error: f32,
// ) {
//   let ticker_data = TickerData::new_from_csv(ticker_data_path);
//   if ticker_data.candles.is_empty() {
//     return
//   }
//   let earliest_candle_date = &ticker_data.get_candles()[0].date;
//   let latest_candle_date = &ticker_data.get_candles()[ticker_data.get_candles().len() - 1].date;
//
//   let planet_matrix = PlanetMatrix::new(
//     Origin::Geocentric,
//     &Time::new(2022, &Month::December, &Day::ThirtyOne),
//     latest_candle_date,
//     alignment_margin_error
//   ).await;
//
//   // TODO: iterate PlanetMatrix and organize/label by alignment, planet pair, and date
// }


/// CSV dataframe format:
///
/// date: Time
///
/// retro_type -> 0 for no retro signal, 1 for start retro, 2 for end retro
///
/// planet -> 0 when no signal, else `Planet::to_num() + 1`
pub async fn retrograde_dataframe(
  ticker_data_file_path: &PathBuf,
  results_csv_path: &PathBuf,
) {
  let ticker_data = TickerData::new_from_csv(ticker_data_file_path);
  if ticker_data.candles.is_empty() {
    return
  }
  let earliest_date = &ticker_data.earliest_date();
  println!("earliest_date: {}", earliest_date.as_string());
  let latest_date = &ticker_data.latest_date();
  println!("latest_date: {}", latest_date.as_string());

  let retrograde = Retrograde::new(
    **earliest_date,
    **latest_date,
    &Planet::to_vec()
  ).await.unwrap();

  // dataframe format:
  // date: Time
  // retro_type: i32 -> 0 for no retro signal, 1 for start retro, 2 for end retro
  // planet: i32 -> 0 when no signal, else (Planet::to_num() + 1)
  let mut wtr = WriterBuilder::new()
    .has_headers(true)
    .from_path(results_csv_path)
    .expect("failed to create csv writer");
  wtr.write_field("Date").expect("failed to write date field");
  wtr.write_field("retro_type").expect("failed to write retro_type field");
  wtr.write_field("planet").expect("failed to write planet field");
  wtr.write_record(None::<&[u8]>).expect("failed to write record");
  wtr.flush().expect("failed to flush csv writer");

  for candle in ticker_data.get_candles().iter() {
    // search for retrograde events on this candle date
    let mut signal_on_date = false;
    for event in retrograde.retrogrades.iter() {
      let planet_index = event.planet.to_num();

      if event.start_date == candle.date {
        // retrograde start
        wtr.write_record(&[
          candle.date.as_string(),
          format!("{}", 1),
          format!("{}", planet_index + 1),
        ]).expect("failed to write record");
        wtr.flush().expect("failed to flush");

        signal_on_date = true;
      }
      else if event.end_date == candle.date {
        // retrograde end
        wtr.write_record(&[
          candle.date.as_string(),
          format!("{}", 2),
          format!("{}", planet_index + 1),
        ]).expect("failed to write record");

        wtr.flush().expect("failed to flush");

        signal_on_date = true;
      }
    }
    if !signal_on_date {
      wtr.write_record(&[
        candle.date.as_string(),
        format!("{}", 0),
        format!("{}", 0)
      ]).expect("failed to write record");

      wtr.flush().expect("failed to flush");
    }
  }
}


pub async fn ticker_dataframe(ticker_data_path: &PathBuf, results_csv_path: &PathBuf) {
  let ticker_data = TickerData::new_from_csv(ticker_data_path);
  if ticker_data.candles.is_empty() {
    return
  }

  let mut wtr = WriterBuilder::new()
    .has_headers(true)
    .from_path(results_csv_path)
    .expect("failed to create csv writer");
  wtr.write_field("date").expect("failed to write date field");
  wtr.write_field("close").expect("failed to write price field");
  wtr.write_field("open").expect("failed to write price field");
  wtr.write_field("high").expect("failed to write price field");
  wtr.write_field("low").expect("failed to write price field");
  wtr.write_record(None::<&[u8]>).expect("failed to write record");
  wtr.flush().expect("failed to flush csv writer");

  for candle in ticker_data.get_candles().iter() {
    wtr.write_record(&[
      candle.date.as_string(),
      candle.close.to_string(),
      candle.open.to_string(),
      candle.high.to_string(),
      candle.low.to_string(),
    ]).expect("failed to write record");
    wtr.flush().expect("failed to flush");
  }
}

