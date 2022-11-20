use utils::*;

pub async fn mercury_past_month() -> String {
  let command = Target::from_earth(&Planet::Mercury);
  let start_time = Time::start_time(2022, Month::October, Day::Thirty);
  let stop_time = Time::stop_time(2022, Month::November, Day::Five);
  let quantities = Quantities::geocentric();
  let query = Query::new(command, start_time, stop_time, quantities);

  let data = reqwest::get(query.value)
    .await
    .expect("failed to request data for Mercury past month")
    .text()
    .await
    .expect("failed to read response");
  extract_data(data)
}

pub async fn mars_past_month() -> String {
  let command = Target::from_earth(&Planet::Mars);
  let start_time = Time::start_time(2022, Month::October, Day::Thirty);
  let stop_time = Time::stop_time(2022, Month::November, Day::Five);
  let quantities = Quantities::geocentric();
  let query = Query::new(command, start_time, stop_time, quantities);

  let data = reqwest::get(query.value)
    .await
    .expect("failed to request data for Mars past month")
    .text()
    .await
    .expect("failed to read response");
  extract_data(data)
}