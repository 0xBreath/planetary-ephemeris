
#[derive(Clone, Debug)]
pub enum Reversal {
  RangeHigh,
  RangeLow
}

#[derive(Clone, Debug)]
pub struct ReversalInfo {
  /// Start of movement at range high or low?
  pub reversal_type: Reversal,
  /// Angle at start of movement
  pub reversal_angle: f32,
  /// Alignment to search for
  pub alignment: Alignment,
}

#[derive(Clone, Debug)]
pub enum Alignment {
  Conjunct,
  Opposite,
  Trine120,
  Trine240,
  Square90,
  Square270,
  Quintile72,
  Quintile144,
  Quintile216,
  Quintile288,
  Sextile60,
  Sextile300,
  Octile45,
  Octile135,
  Octile225,
  Octile315,
}

impl Alignment {
  pub fn to_str(&self) -> &'static str {
    match *self {
      Alignment::Conjunct => "Conjunct",
      Alignment::Opposite => "Opposite",
      Alignment::Trine120 => "Trine120",
      Alignment::Trine240 => "Trine240",
      Alignment::Square90 => "Square90",
      Alignment::Square270 => "Square270",
      Alignment::Quintile72 => "Quintile72",
      Alignment::Quintile144 => "Quintile144",
      Alignment::Quintile216 => "Quintile216",
      Alignment::Quintile288 => "Quintile288",
      Alignment::Sextile60 => "Sextile60",
      Alignment::Sextile300 => "Sextile300",
      Alignment::Octile45 => "Octile45",
      Alignment::Octile135 => "Octile135",
      Alignment::Octile225 => "Octile225",
      Alignment::Octile315 => "Octile315",
    }
  }
  
  pub fn to_num(&self) -> f32 {
    match *self {
      Alignment::Conjunct => 0.0,
      Alignment::Opposite => 180.0,
      Alignment::Trine120 => 120.0,
      Alignment::Trine240 => 240.0,
      Alignment::Square90 => 90.0,
      Alignment::Square270 => 270.0,
      Alignment::Quintile72 => 72.0,
      Alignment::Quintile144 => 144.0,
      Alignment::Quintile216 => 216.0,
      Alignment::Quintile288 => 288.0,
      Alignment::Sextile60 => 60.0,
      Alignment::Sextile300 => 300.0,
      Alignment::Octile45 => 45.0,
      Alignment::Octile135 => 135.0,
      Alignment::Octile225 => 225.0,
      Alignment::Octile315 => 315.0,
    }
  }

  pub fn normalize(angle: f32) -> f32 {
    if angle < 0.0 {
      angle + 360.0
    } else if angle > 360.0 {
      angle - 360.0
    } else {
      angle
    }
  }

  pub fn find_alignment(a: f32, b: f32, margin: f32) -> Option<Self> {
    let diff = Alignment::normalize(a - b);
    if diff < margin {
      Some(Alignment::Conjunct)
    }
    else if Alignment::normalize(diff - 180.0) < margin {
      Some(Alignment::Opposite)
    }
    else if Alignment::normalize(diff - 120.0) < margin {
      Some(Alignment::Trine120)
    }
    else if Alignment::normalize(diff - 240.0) < margin {
      Some(Alignment::Trine240)
    }
    else if Alignment::normalize(diff - 90.0) < margin {
      Some(Alignment::Square90)
    }
    else if Alignment::normalize(diff - 270.0) < margin {
      Some(Alignment::Square270)
    }
    else if Alignment::normalize(diff - 72.0) < margin {
      Some(Alignment::Quintile72)
    }
    else if Alignment::normalize(diff - 144.0) < margin {
      Some(Alignment::Quintile144)
    }
    else if Alignment::normalize(diff - 216.0) < margin {
      Some(Alignment::Quintile216)
    }
    else if Alignment::normalize(diff - 288.0) < margin {
      Some(Alignment::Quintile288)
    }
    else if Alignment::normalize(diff - 60.0) < margin {
      Some(Alignment::Sextile60)
    }
    else if Alignment::normalize(diff - 300.0) < margin {
       Some(Alignment::Sextile300)
    }
    else if Alignment::normalize(diff - 45.0) < margin {
      Some(Alignment::Octile45)
    }
    else if Alignment::normalize(diff - 135.0) < margin {
      Some(Alignment::Octile135)
    }
    else if Alignment::normalize(diff - 225.0) < margin {
      Some(Alignment::Octile225)
    }
    else if Alignment::normalize(diff - 315.0) < margin {
      Some(Alignment::Octile315)
    }
    else {
      None
    }
  }

  pub fn to_vec() -> Vec<Alignment> {
    vec![
      Alignment::Conjunct,
      Alignment::Opposite,
      Alignment::Trine120,
      Alignment::Trine240,
      Alignment::Square90,
      Alignment::Square270,
      Alignment::Quintile72,
      Alignment::Quintile144,
      Alignment::Quintile216,
      Alignment::Quintile288,
      Alignment::Sextile60,
      Alignment::Sextile300,
      Alignment::Octile45,
      Alignment::Octile135,
      Alignment::Octile225,
      Alignment::Octile315,
    ]
  }
}

impl PartialEq<Self> for Alignment {
  fn eq(&self, other: &Self) -> bool {
    self.to_str() == other.to_str()
  }
}
