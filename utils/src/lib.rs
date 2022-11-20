pub mod quantities;
pub mod query;
pub mod step_size;
pub mod target;
pub mod time;

pub use quantities::*;
pub use query::*;
pub use step_size::*;
pub use target::*;
pub use time::*;

pub fn extract_data(response: String) -> String {
  let mut data = String::new();
  let mut lines = response.lines();
  while let Some(line) = lines.next() {
    if line.contains("$$SOE") {
      while let Some(line) = lines.next() {
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

pub fn format_to_vec(data: String) -> Vec<(Time, f32)> {
  let mut vec = Vec::new();
  let mut lines = data.lines();
  while let Some(line) = lines.next() {
    let mut line = line.split_whitespace();
    let time = String::from(line.next().unwrap());
    let time = Time::convert_response(&time);
    // next after time is seconds, so skip it
    line.next().unwrap();
    let angle = line.next().unwrap();
    let angle = angle.parse::<f32>().unwrap();
    vec.push((time, angle));
  }
  vec
}
