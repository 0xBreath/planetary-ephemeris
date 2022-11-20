use crate::target::Target;
use crate::quantities::Quantities;
use crate::step_size::StepSize;
use crate::time::Time;

pub const BASE_QUERY: &str = "https://ssd.jpl.nasa.gov/api/horizons.api?format=text";

pub struct Query {
    pub value: String
}

impl Query {
  pub fn new(
    command: Target,
    start_time: Time,
    stop_time: Time,
    quantities: Quantities,
  ) -> Self {
    let default_args = "&OBJ_DATA='YES'&MAKE_EPHEM='YES'&EPHEM_TYPE='OBSERVER'&CENTER='500@399'";
    let step_size = StepSize::default().value;
    let value = format!(
      "{}{}{}{}{}{}{}",
      BASE_QUERY, command.value, default_args, start_time.value, stop_time.value, step_size, quantities.value
    );
    Self {
      value,
    }
  }
}