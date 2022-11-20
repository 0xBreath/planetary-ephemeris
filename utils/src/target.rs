
pub struct Target {
  pub value: String,
}
#[allow(dead_code)]
impl Target {
  pub fn from_earth(planet: &Planet) -> Self {
    Self {
      value: format!("&COMMAND='{}'", planet.to_earth_center()),
    }
  }

  pub fn from_planet_center(planet: &Planet) -> Self {
    Self {
      value: format!("&COMMAND='{}'", planet.to_planet_center()),
    }
  }

  pub fn from_planet_barycenter(planet: &Planet) -> Self {
    Self {
      value: format!("&COMMAND='{}'", planet.to_earth_planet_barycenter()),
    }
  }
}
#[allow(dead_code)]
pub enum Planet {
  Mercury,
  Venus,
  Moon,
  Mars,
  Jupiter,
  Saturn,
  Uranus,
  Neptune,
  Pluto,
}
#[allow(dead_code)]
impl Planet {
  /// API mapping for object relative to earth center
  fn to_earth_center(&self) -> &str {
    match self {
      Planet::Mercury => "199",
      Planet::Venus => "299",
      Planet::Moon => "399",
      Planet::Mars => "499",
      Planet::Jupiter => "599",
      Planet::Saturn => "699",
      Planet::Uranus => "799",
      Planet::Neptune => "899",
      Planet::Pluto => "999",
    }
  }
  /// API mapping for object relative to object center
  fn to_planet_center(&self) -> &str {
    match self {
      Planet::Mercury => "101",
      Planet::Venus => "201",
      Planet::Moon => "301",
      Planet::Mars => "401",
      Planet::Jupiter => "501",
      Planet::Saturn => "601",
      Planet::Uranus => "701",
      Planet::Neptune => "801",
      Planet::Pluto => "901",
    }
  }
  /// API mapping for object relative to earth-object barycenter
  fn to_earth_planet_barycenter(&self) -> &str {
    match self {
      Planet::Mercury => "1",
      Planet::Venus => "2",
      Planet::Moon => "3",
      Planet::Mars => "4",
      Planet::Jupiter => "5",
      Planet::Saturn => "6",
      Planet::Uranus => "7",
      Planet::Neptune => "8",
      Planet::Pluto => "9",
    }
  }
}