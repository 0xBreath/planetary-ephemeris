
pub struct Time {
  pub year: u32,
  pub month: Month,
  pub day: Day,
  pub value: String
}

impl Time {
  pub fn new(year: u32, month: Month, day: Day) -> Self {
    Self {
      year,
      month: month.clone(),
      day: day.clone(),
      value: format!("{}-{}-{}", year, month.to_string(), day.to_string()),
    }
  }
  pub fn start_time(year: u32, month: Month, day: Day) -> Self {
    Self {
      year,
      month: month.clone(),
      day: day.clone(),
      value: format!("&START_TIME='{}-{}-{}'", year, month.to_string(), day.to_string()),
    }
  }

  pub fn stop_time(year: u32, month: Month, day: Day) -> Self {
    Self {
      year,
      month: month.clone(),
      day: day.clone(),
      value: format!("&STOP_TIME='{}-{}-{}'", year, month.to_string(), day.to_string()),
    }
  }
  /// Example: 2022-Nov-01
  pub fn convert_response(date: &str) -> Self {
    let month_delim = date.find("-").unwrap();
    let year = &date[..month_delim];
    let year = year.parse::<u32>().unwrap();
    let month_abbrev = &date[(month_delim+1)..(month_delim+4)];
    let month = Month::from_abbrev(month_abbrev);

    let day = &date[(month_delim+5)..];
    Time::new(year, month, Day::from_str(day))
  }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Month {
  January,
  February,
  March,
  April,
  May,
  June,
  July,
  August,
  September,
  October,
  November,
  December,
}
impl Month {
  fn to_string(&self) -> &str {
    match self {
      Month::January => "01",
      Month::February => "02",
      Month::March => "03",
      Month::April => "04",
      Month::May => "05",
      Month::June => "06",
      Month::July => "07",
      Month::August => "08",
      Month::September => "09",
      Month::October => "10",
      Month::November => "11",
      Month::December => "12",
    }
  }

  fn from_abbrev(abbrev: &str) -> Self {
    match abbrev {
      "Jan" => Month::January,
      "Feb" => Month::February,
      "Mar" => Month::March,
      "Apr" => Month::April,
      "May" => Month::May,
      "Jun" => Month::June,
      "Jul" => Month::July,
      "Aug" => Month::August,
      "Sep" => Month::September,
      "Oct" => Month::October,
      "Nov" => Month::November,
      "Dec" => Month::December,
      _ => panic!("Invalid month abbreviation: {}", abbrev),
    }
  }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Day {
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Eleven,
  Twelve,
  Thirteen,
  Fourteen,
  Fifteen,
  Sixteen,
  Seventeen,
  Eighteen,
  Nineteen,
  Twenty,
  TwentyOne,
  TwentyTwo,
  TwentyThree,
  TwentyFour,
  TwentyFive,
  TwentySix,
  TwentySeven,
  TwentyEight,
  TwentyNine,
  Thirty,
  ThirtyOne,
}
impl Day {
  fn to_string(&self) -> &str {
    match self {
      Day::One => "01",
      Day::Two => "02",
      Day::Three => "03",
      Day::Four => "04",
      Day::Five => "05",
      Day::Six => "06",
      Day::Seven => "07",
      Day::Eight => "08",
      Day::Nine => "09",
      Day::Ten => "10",
      Day::Eleven => "11",
      Day::Twelve => "12",
      Day::Thirteen => "13",
      Day::Fourteen => "14",
      Day::Fifteen => "15",
      Day::Sixteen => "16",
      Day::Seventeen => "17",
      Day::Eighteen => "18",
      Day::Nineteen => "19",
      Day::Twenty => "20",
      Day::TwentyOne => "21",
      Day::TwentyTwo => "22",
      Day::TwentyThree => "23",
      Day::TwentyFour => "24",
      Day::TwentyFive => "25",
      Day::TwentySix => "26",
      Day::TwentySeven => "27",
      Day::TwentyEight => "28",
      Day::TwentyNine => "29",
      Day::Thirty => "30",
      Day::ThirtyOne => "31",
    }
  }

  pub fn from_str(day: &str) -> Self {
    match day {
      "01" => Day::One,
      "02" => Day::Two,
      "03" => Day::Three,
      "04" => Day::Four,
      "05" => Day::Five,
      "06" => Day::Six,
      "07" => Day::Seven,
      "08" => Day::Eight,
      "09" => Day::Nine,
      "10" => Day::Ten,
      "11" => Day::Eleven,
      "12" => Day::Twelve,
      "13" => Day::Thirteen,
      "14" => Day::Fourteen,
      "15" => Day::Fifteen,
      "16" => Day::Sixteen,
      "17" => Day::Seventeen,
      "18" => Day::Eighteen,
      "19" => Day::Nineteen,
      "20" => Day::Twenty,
      "21" => Day::TwentyOne,
      "22" => Day::TwentyTwo,
      "23" => Day::TwentyThree,
      "24" => Day::TwentyFour,
      "25" => Day::TwentyFive,
      "26" => Day::TwentySix,
      "27" => Day::TwentySeven,
      "28" => Day::TwentyEight,
      "29" => Day::TwentyNine,
      "30" => Day::Thirty,
      "31" => Day::ThirtyOne,
      _ => panic!("Invalid day: {}", day),
    }
  }
}






