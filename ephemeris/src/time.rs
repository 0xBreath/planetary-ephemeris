use chrono::{Datelike, NaiveDate, TimeZone};

#[derive(Clone, Copy, Debug)]
pub struct Time {
  pub year: i32,
  pub month: Month,
  pub day: Day,
}

impl Time {
  pub fn new(year: i32, month: &Month, day: &Day) -> Self {
    Self {
      year,
      month: month.clone(),
      day: day.clone(),
    }
  }
  pub fn as_string(&self) -> String {
    format!("{}-{}-{}", self.year, self.month.to_string(), self.day.to_string())
  }
  pub fn to_naive_date(&self) -> NaiveDate {
    chrono::NaiveDate::from_ymd_opt(
      self.year,
      self.month.to_num(),
      self.day.to_num()
    ).expect("failed to convert Time to chrono::NaiveDate")
  }
  /// Start time for 'Horizon API'
  pub fn start_time(&self) -> String {
    format!("&START_TIME='{}'", self.as_string())
  }
  /// Stop time for 'Horizon API'
  pub fn stop_time(&self) -> String {
      format!("&STOP_TIME='{}'", self.as_string())
  }

  /// Convert 'Horizon API' time response to Self
  /// Example: 2022-Nov-01 -> Time { year: 2022, month: Month::November, day: Day::One }
  pub fn convert_api_response(date: &str) -> Self {
    let month_delim = date.find('-').unwrap();
    let year = &date[..month_delim];
    let year = year.parse::<i32>().unwrap();
    let month_abbrev = &date[(month_delim+1)..(month_delim+4)];
    let month = Month::from_abbrev(month_abbrev);

    let day = &date[(month_delim+5)..];
    Time::new(year, &month, &Day::from_string(day))
  }
  /// Convert `chrono::DateTime` to `Time`
  pub fn today() -> Self {
    let date = chrono::Utc::now();
    let year = date.naive_utc().year();
    let month = Month::from_num(date.naive_utc().month());
    let day = Day::from_num(date.naive_utc().day());
    Time::new(year, &month, &day)
  }
  /// Increment Time by a number of days
  pub fn delta_date(&self, days: i64) -> Self {
    let chrono_date = chrono::NaiveDate::from_ymd_opt(
      self.year,
      self.month.to_num(),
      self.day.to_num()
    ).expect("failed to convert Time to chrono::NaiveDate");
    // convert NaiveDate to NaiveDateTime
    let chrono_date = chrono::NaiveDateTime::new(
      chrono_date,
      chrono::NaiveTime::from_hms_opt(0, 0, 0).expect("failed to create NaiveTime")
    );

    let date = chrono_date + chrono::Duration::days(days);
    let year = date.year();
    let month = Month::from_num(date.month());
    let day = Day::from_num(date.day());
    Time::new(year, &month, &day)
  }

  /// Check if Time is within range of dates
  pub fn within_range(&self, start: Self, stop: Self) -> bool {
    self.to_naive_date() >= start.to_naive_date() && self.to_naive_date() <= stop.to_naive_date()
  }

  /// Difference in days between two dates
  pub fn diff_days(&self, other: &Self) -> i64 {
    let date1 = self.to_naive_date();
    let date2 = other.to_naive_date();
    date2.signed_duration_since(date1).num_days()
  }

  /// Create Time from UNIX timestamp
  pub fn from_unix(unix: i64) -> Self {
    let date = chrono::Utc
      .timestamp_opt(unix, 0)
      .unwrap();
    let year = date.naive_utc().year();
    let month = Month::from_num(date.naive_utc().month());
    let day = Day::from_num(date.naive_utc().day());
    Time::new(year, &month, &day)
  }
}

impl PartialEq for Time {
  fn eq(&self, other: &Self) -> bool {
    self.to_naive_date() == other.to_naive_date()
  }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
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
  pub fn to_string(&self) -> &str {
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
  /// Used to convert 'Horizon API' time response to `Month`
  pub fn from_abbrev(abbrev: &str) -> Self {
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

  pub fn from_num(num: u32) -> Self {
    match num {
      1 => Month::January,
      2 => Month::February,
      3 => Month::March,
      4 => Month::April,
      5 => Month::May,
      6 => Month::June,
      7 => Month::July,
      8 => Month::August,
      9 => Month::September,
      10 => Month::October,
      11 => Month::November,
      12 => Month::December,
      _ => panic!("Invalid month number: {}", num),
    }
  }

  pub fn to_num(&self) -> u32 {
    match self {
      Month::January => 1,
      Month::February => 2,
      Month::March => 3,
      Month::April => 4,
      Month::May => 5,
      Month::June => 6,
      Month::July => 7,
      Month::August => 8,
      Month::September => 9,
      Month::October => 10,
      Month::November => 11,
      Month::December => 12,
    }
  }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
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
  pub fn to_string(&self) -> &str {
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

  pub fn from_string(day: &str) -> Self {
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

  pub fn from_num(num: u32) -> Self {
    match num {
      1 => Day::One,
      2 => Day::Two,
      3 => Day::Three,
      4 => Day::Four,
      5 => Day::Five,
      6 => Day::Six,
      7 => Day::Seven,
      8 => Day::Eight,
      9 => Day::Nine,
      10 => Day::Ten,
      11 => Day::Eleven,
      12 => Day::Twelve,
      13 => Day::Thirteen,
      14 => Day::Fourteen,
      15 => Day::Fifteen,
      16 => Day::Sixteen,
      17 => Day::Seventeen,
      18 => Day::Eighteen,
      19 => Day::Nineteen,
      20 => Day::Twenty,
      21 => Day::TwentyOne,
      22 => Day::TwentyTwo,
      23 => Day::TwentyThree,
      24 => Day::TwentyFour,
      25 => Day::TwentyFive,
      26 => Day::TwentySix,
      27 => Day::TwentySeven,
      28 => Day::TwentyEight,
      29 => Day::TwentyNine,
      30 => Day::Thirty,
      31 => Day::ThirtyOne,
      _ => panic!("Invalid day number: {}", num),
    }
  }

  pub fn to_num(&self) -> u32 {
    match self {
      Day::One => 1,
      Day::Two => 2,
      Day::Three => 3,
      Day::Four => 4,
      Day::Five => 5,
      Day::Six => 6,
      Day::Seven => 7,
      Day::Eight => 8,
      Day::Nine => 9,
      Day::Ten => 10,
      Day::Eleven => 11,
      Day::Twelve => 12,
      Day::Thirteen => 13,
      Day::Fourteen => 14,
      Day::Fifteen => 15,
      Day::Sixteen => 16,
      Day::Seventeen => 17,
      Day::Eighteen => 18,
      Day::Nineteen => 19,
      Day::Twenty => 20,
      Day::TwentyOne => 21,
      Day::TwentyTwo => 22,
      Day::TwentyThree => 23,
      Day::TwentyFour => 24,
      Day::TwentyFive => 25,
      Day::TwentySix => 26,
      Day::TwentySeven => 27,
      Day::TwentyEight => 28,
      Day::TwentyNine => 29,
      Day::Thirty => 30,
      Day::ThirtyOne => 31,
    }
  }
}






