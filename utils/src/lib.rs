pub mod quantities;
pub mod query;
pub mod step_size;
pub mod target;
pub mod time;
pub mod right_ascension;
pub mod alignment;

pub use quantities::*;
pub use query::*;
pub use step_size::*;
pub use target::*;
pub use time::*;
pub use right_ascension::*;
pub use alignment::*;

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














