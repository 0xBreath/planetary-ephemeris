use crate::{Alignment, Origin, Planet, RightAscension};
use crate::target::Target;
use crate::quantities::Quantities;
use crate::step_size::StepSize;
use crate::time::Time;

pub const BASE_QUERY: &str = "https://ssd.jpl.nasa.gov/api/horizons.api?format=text";

pub struct Query {
    pub value: String
}

impl Query {
  /// Compute alignment of a planet relative to the Origin (Earth or Sun) for a period of time.
  /// Return a vector of dates with significant alignments.
  pub async fn query(
    origin: Origin,
    planet: &Planet,
    mut start_time: Time,
    period_days: i64,
  ) -> Vec<(Time, f32)> {
    let mut stop_time = start_time.delta_date(period_days);
    // swap start and stop time if period is historical rather than for the future
    if period_days < 0 {
      std::mem::swap(&mut start_time, &mut stop_time);
    }
    let query = Query::build_query(
      Target::new(planet),
      start_time,
      stop_time,
      Quantities::default(),
      origin
    );

    let data = reqwest::get(query.value)
      .await
      .expect("failed to request data")
      .text()
      .await
      .expect("failed to read response");
    let data = Self::extract_data(data);
    Self::format_to_vec(data)
  }

  /// Construct a query to interact with the 'Horizon API'
  fn build_query(
    command: Target,
    start_time: Time,
    stop_time: Time,
    quantities: Quantities,
    origin: Origin
  ) -> Self {
    let origin = origin.as_query();
    let default_args = "&OBJ_DATA='YES'&MAKE_EPHEM='YES'&EPHEM_TYPE='OBSERVER'";
    let step_size = StepSize::default().value;
    let value = format!(
      "{}{}{}{}{}{}{}{}",
      BASE_QUERY, command.value, default_args, origin, start_time.start_time(), stop_time.stop_time(), step_size, quantities.value
    );
    Self {
      value,
    }
  }

  /// Isolate planetary ephemeral data from API response
  pub fn extract_data(response: String) -> String {
    let mut data = String::new();
    let mut lines = response.lines();
    while let Some(line) = lines.next() {
      if line.contains("$$SOE") {
        for line in lines.by_ref() {
          if line.contains("$$EOE") {
            break;
          }
          data.push_str(line);
          data.push_str("\r\n");
        }
      }
    }
    data
  }

  /// Format API response of into vector of (Time, right ascension as degrees)
  pub fn format_to_vec(data: String) -> Vec<(Time, f32)> {
    let mut vec = Vec::new();
    let lines = data.lines();
    for line in lines {
      let mut line = line.split_whitespace();
      let time = String::from(line.next().unwrap());
      let time = Time::convert_api_response(&time);
      // next after YYYY-MM-DD is seconds... skip it
      line.next().unwrap();
      // next 3 values are Right Ascension (HH MM SS.SS)
      let ra_hh = line
        .next().unwrap()
        .parse::<f32>().unwrap();
      let ra_mm = line
        .next().unwrap()
        .parse::<f32>().unwrap();
      let ra_ssss = line
        .next().unwrap()
        .parse::<f32>().unwrap();
      let ra = RightAscension::new(ra_hh as i32, ra_mm as i32, ra_ssss);
      let ra_degrees = ra.to_degrees();

      vec.push((time, ra_degrees));
    }
    vec
  }

  /// TODO: take alignment as param, take closest value in vector to alignment. Right now it takes the last value
  /// Remove duplicate Alignment values for consecutive dates
  pub fn remove_duplicate_values(vec: &mut Vec<(Time, f32, Alignment)>) {
    if vec.is_empty() {
      return;
    }
    let mut i = 0;
    while i < vec.len() - 1 {
      if vec[i].2 == vec[i + 1].2 {
        vec.remove(i);
      } else {
        i += 1;
      }
    }
  }
}