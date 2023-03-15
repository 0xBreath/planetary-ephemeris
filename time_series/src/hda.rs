use chrono::Duration;
use chrono::{Local, NaiveDate, TimeZone};
use crate::*;
use plotters::prelude::*;

/// Historical Date Analysis
pub struct HDA {
  /// Reversal on this date
  pub date: Time,
  /// How many years had a reversal on this date
  pub mode: u32
}

impl HDA {
  pub fn new(date: Time, mode: u32) -> Self {
    Self { date, mode }
  }
}

pub struct PlotHDA {
  /// Start date to plot daily HDA
  pub start_date: Time,
  /// End date to plot daily HDA
  pub end_date: Time,
  /// Define a candle reversal by local max or min +/- margin
  pub reversal_margin: usize,
  /// Candle is within this margin of a reversal (valid HDA)
  pub hda_margin: usize,
}

impl PlotHDA {
  pub fn new(start_date: Time, end_date: Time, reversal_margin: usize, hda_margin: usize) -> Self {
    Self {
      start_date,
      end_date,
      reversal_margin,
      hda_margin
    }
  }

  /// Compute Historical Date Analysis
  /// Compares the same date of each year for similar price action
  pub fn hda(&self, ticker_data: &TickerData) -> Vec<HDA> {
    let mut daily_hda = Vec::<HDA>::new();

    // compute number of cycles possible in candle history
    let earliest_date = ticker_data.earliest_date();
    let earliest_candle_year = earliest_date.year;
    let latest_date = ticker_data.latest_date();
    let latest_candle_year = latest_date.year;
    let total_years_back = latest_candle_year - earliest_candle_year;

    let time_period = self.start_date.time_period(&self.end_date);
    for date in time_period.iter() {
      // HDA for this date (frequency of reversals on this date across all years)
      let mut hda = 0;
      // find candle X cycles back
      for (index, candle) in ticker_data.candles.iter().enumerate() {
        if index == 0 {
          continue;
        }
        // iterate over each year back
        for years_back in 0..total_years_back {
          // candle date X years back
          let cycle_date = Time::new(date.year - years_back, &date.month, &date.day);
          // found candle in previous year on this date
          let prev_candle = ticker_data.candles.get(index - 1).expect("Failed to get previous candle");
          if prev_candle.date < cycle_date && candle.date >= cycle_date {
            // if candle is within margin of local high or low
            if ticker_data.candle_is_high(candle, self.reversal_margin, self.hda_margin) || ticker_data.candle_is_low(candle, self.reversal_margin, self.hda_margin) {
              hda += 1;
            }
          }
        }
      }
      daily_hda.push(HDA {
        date: *date,
        mode: hda
      });
    }
    daily_hda
  }

  pub fn plot_hda(&self, daily_hda: &[HDA], out_file: &str, plot_title: &str, plot_color: &RGBColor) {
    // get daily PFS data
    let data = self.get_data(daily_hda);
    // draw chart
    let root = BitMapBackend::new(out_file, (2048, 1024)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    // PFS start date
    let from_date_index = self.find_date_index(&data, &self.start_date);
    let from_date_input = self.parse_time(&data[from_date_index].0);
    let from_date = from_date_input - Duration::days(1);
    println!("HDA Start Date: {}", from_date);
    // PFS end date
    let to_date_index = self.find_date_index(&data, &self.end_date);
    let to_date_input = self.parse_time(&data[to_date_index].0);
    let to_date = to_date_input + Duration::days(1);
    println!("HDA End Date: {}", to_date);
    // label chart
    let mut chart = ChartBuilder::on(&root)
      .x_label_area_size(40)
      .y_label_area_size(40)
      .caption(plot_title, ("sans-serif", 50.0).into_font())
      .build_cartesian_2d(from_date..to_date, 0f32..20f32).unwrap();
    chart.configure_mesh().light_line_style(WHITE).draw().unwrap();
    // plot PFS values
    chart.draw_series(
      LineSeries::new(data.iter().map(|x| (self.parse_time(&x.0), x.1)), plot_color)
    ).unwrap();
    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", out_file);
  }

  fn get_data(&self, daily_hda: &[HDA]) -> Vec<(String, f32)> {
    let mut data = Vec::new();
    for hda in daily_hda.iter() {
      data.push((
        hda.date.as_string(),
        hda.mode as f32,
      ));
    }
    data
  }

  fn find_date_index(&self, data: &[(String, f32)], date: &Time) -> usize {
    for (i, (d, _)) in data.iter().enumerate() {
      if d == &date.as_string() {
        return i;
      }
    }
    let mut not_found = true;
    let mut change_date = *date;
    while not_found {
      change_date = change_date.delta_date(-1);
      // get previous index in data
      for (i, (d, _)) in data.iter().enumerate() {
        if d == &change_date.as_string() {
          not_found = false;
          return i;
        }
      }
    }
    panic!("Date not found");
  }

  fn parse_time(&self, t: &str) -> NaiveDate {
    Local
      .datetime_from_str(&format!("{} 0:0", t), "%Y-%m-%d %H:%M")
      .unwrap()
      .date_naive()
  }
}