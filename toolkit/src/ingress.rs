use log::debug;
use ephemeris::*;
use time_series::SquareOfNine;
use crate::*;

#[derive(Debug, Clone)]
pub enum Zodiac {
  Aries,
  Taurus,
  Gemini,
  Cancer,
  Leo,
  Virgo,
  Libra,
  Scorpio,
  Sagittarius,
  Capricorn,
  Aquarius,
  Pisces
}
impl Zodiac {
  pub fn to_ingress_angle(&self) -> f32 {
    match self {
      Zodiac::Aries => 0.0,
      Zodiac::Taurus => 30.0,
      Zodiac::Gemini => 60.0,
      Zodiac::Cancer => 90.0,
      Zodiac::Leo => 120.0,
      Zodiac::Virgo => 150.0,
      Zodiac::Libra => 180.0,
      Zodiac::Scorpio => 210.0,
      Zodiac::Sagittarius => 240.0,
      Zodiac::Capricorn => 270.0,
      Zodiac::Aquarius => 300.0,
      Zodiac::Pisces => 330.0,
    }
  }

  pub fn to_str(&self) -> &str {
    match self {
      Zodiac::Aries => "Aries",
      Zodiac::Taurus => "Taurus",
      Zodiac::Gemini => "Gemini",
      Zodiac::Cancer => "Cancer",
      Zodiac::Leo => "Leo",
      Zodiac::Virgo => "Virgo",
      Zodiac::Libra => "Libra",
      Zodiac::Scorpio => "Scorpio",
      Zodiac::Sagittarius => "Sagittarius",
      Zodiac::Capricorn => "Capricorn",
      Zodiac::Aquarius => "Aquarius",
      Zodiac::Pisces => "Pisces",
    }
  }

  fn normalize(angle: f32) -> f32 {
    if angle < 0.0 {
      angle + 360.0
    } else if angle > 360.0 {
      angle - 360.0
    } else {
      angle
    }
  }

  pub fn find_zodiac(angle: f32, margin: f32) -> Option<Self> {
    let diff = Zodiac::normalize(angle);
    if diff < margin {
      Some(Zodiac::Aries)
    }
    else if Zodiac::normalize(diff - 30.0) < margin {
      Some(Zodiac::Taurus)
    }
    else if Zodiac::normalize(diff - 60.0) < margin {
      Some(Zodiac::Gemini)
    }
    else if Zodiac::normalize(diff - 90.0) < margin {
      Some(Zodiac::Cancer)
    }
    else if Zodiac::normalize(diff - 120.0) < margin {
      Some(Zodiac::Leo)
    }
    else if Zodiac::normalize(diff - 150.0) < margin {
      Some(Zodiac::Virgo)
    }
    else if Zodiac::normalize(diff - 180.0) < margin {
      Some(Zodiac::Libra)
    }
    else if Zodiac::normalize(diff - 210.0) < margin {
      Some(Zodiac::Scorpio)
    }
    else if Zodiac::normalize(diff - 240.0) < margin {
      Some(Zodiac::Sagittarius)
    }
    else if Zodiac::normalize(diff - 270.0) < margin {
      Some(Zodiac::Capricorn)
    }
    else if Zodiac::normalize(diff - 300.0) < margin {
      Some(Zodiac::Aquarius)
    }
    else if Zodiac::normalize(diff - 330.0) < margin {
      Some(Zodiac::Pisces)
    }
    else {
      None
    }
  }
}
impl PartialEq<Self> for Zodiac {
  fn eq(&self, other: &Self) -> bool {
    self.to_ingress_angle() == other.to_ingress_angle()
  }
}

#[derive(Debug, Clone)]
pub struct IngressEvent {
  pub planet: Planet,
  pub date: Time,
  pub zodiac: Zodiac,
  pub angle: f32,
}
impl IngressEvent {
  pub fn new(planet: Planet, date: Time, zodiac: Zodiac, angle: f32) -> Self {
    Self {
      planet,
      date,
      zodiac,
      angle
    }
  }
}

#[derive(Debug, Clone)]
pub struct Ingress {
  pub ingresses: Vec<IngressEvent>
}

impl Ingress {
  pub async fn new(start_date: Time, end_date: Time, error_margin_degrees: f32) -> Self {
    let planet_alignments = PlanetAlignments::new(start_date, end_date).await;

    // search for dates when a planet ingresses into a new zodiac.
    // In other words, when the planet's (angle % 30 == 0)
    let mut ingresses = Vec::<IngressEvent>::new();
    for (planet_index, planet_angles) in planet_alignments.planet_angles.into_iter().enumerate() {
      let planet = &Planet::to_vec()[planet_index];
      let mut signals = Vec::<(Time, f32, Zodiac)>::new();

      for (date, angle) in planet_angles {
        let zodiac = Zodiac::find_zodiac(angle, error_margin_degrees);
        if let Some(zodiac) = zodiac {
          signals.push((date, angle, zodiac));
        }
      }
      signals = Self::remove_duplicate_values(&mut signals);
      for (date, angle, zodiac) in signals.iter() {
        let ingress_event = IngressEvent::new(planet.clone(), *date, zodiac.clone(), *angle);
        debug!("{}\t{}\t{}", planet.to_str(), date.as_string(), angle);
        ingresses.push(ingress_event);
      };
    }

    Self { ingresses }
  }

  /// Finds duplicate target values on consecutive dates where f32 is within margin of error
  /// filters for the date with f32 closest to the actual target value
  pub fn remove_duplicate_values(vec: &mut Vec<(Time, f32, Zodiac)>) -> Vec<(Time, f32, Zodiac)> {
    if vec.is_empty() {
      return vec.to_vec();
    }
    let mut clean_values = Vec::new();
    let mut i = 0;
    while i < vec.len() - 1 {
      let mut index = i + 1;
      if vec.get(index).is_none() || vec.get(i).is_none() {
        break;
      }
      let mut closest_to_alignment_angle = (vec[i].2.to_ingress_angle() - vec[i].1).abs();
      let mut closest_to_alignment_index = i;
      while vec[index].2 == vec[i].2 {
        let angle_to_alignment = (vec[index].2.to_ingress_angle() - vec[index].1).abs();
        if angle_to_alignment < closest_to_alignment_angle {
          closest_to_alignment_angle = angle_to_alignment;
          closest_to_alignment_index = index;
        }
        index += 1;
        if vec.get(index).is_none() {
          break;
        }
      }
      i = index;
      clean_values.push(vec[closest_to_alignment_index].clone());
      if i >= vec.len() {
        break;
      }
    }
    clean_values
  }
}