
#[derive(Clone, Debug)]
pub enum Alignment {
  Conjunct,
  Opposite,
  Trine,
  Square,
  Quintile,
  Sextile
}

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

impl Alignment {
  pub fn to_str(&self) -> &'static str {
    match *self {
      Alignment::Conjunct => "Conjunct",
      Alignment::Opposite => "Opposite",
      Alignment::Trine => "Trine",
      Alignment::Square => "Square",
      Alignment::Quintile => "Quintile",
      Alignment::Sextile => "Sextile",
    }
  }
  
  pub fn to_num(&self) -> f32 {
    match *self {
      Alignment::Conjunct => 0.0,
      Alignment::Opposite => 180.0,
      Alignment::Trine => 120.0,
      Alignment::Square => 90.0,
      Alignment::Quintile => 72.0,
      Alignment::Sextile => 60.0,
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
    } else if Alignment::normalize(diff - 180.0) < margin {
      Some(Alignment::Opposite)
    } else if Alignment::normalize(diff - 120.0) < margin || Alignment::normalize(diff - 240.0) < margin {
      Some(Alignment::Trine)
    } else if Alignment::normalize(diff - 90.0) < margin || Alignment::normalize(diff - 270.0) < margin {
      Some(Alignment::Square)
    } else if Alignment::normalize(diff - 72.0) < margin ||
      Alignment::normalize(diff - 144.0) < margin ||
      Alignment::normalize(diff - 216.0) < margin ||
      Alignment::normalize(diff - 288.0) < margin
    {
      Some(Alignment::Quintile)
    } else if Alignment::normalize(diff - 60.0) < margin ||
      Alignment::normalize(diff - 300.0) < margin
    {
      Some(Alignment::Sextile)
    } else {
      None
    }
  }

  pub fn square_of_nine_compute(info: ReversalInfo) -> f32 {
     match info.alignment {
        Alignment::Conjunct => {
          match info.reversal_type {
            Reversal::RangeHigh => {
              Alignment::normalize((info.reversal_angle.sqrt() - 2.0).powi(2))
            },
            Reversal::RangeLow => {
              Alignment::normalize((info.reversal_angle.sqrt() + 2.0).powi(2))
            }
          }
        },
        Alignment::Opposite => {
          match info.reversal_type {
            Reversal::RangeHigh => {
              Alignment::normalize((info.reversal_angle.sqrt() - 1.0).powi(2))
            },
            Reversal::RangeLow => {
              Alignment::normalize((info.reversal_angle.sqrt() + 1.0).powi(2))
            }
          }
        },
        Alignment::Trine => {
          match info.reversal_type {
            Reversal::RangeHigh => {
              Alignment::normalize((info.reversal_angle.sqrt() - 2.0/3.0).powi(2))
            },
            Reversal::RangeLow => {
              Alignment::normalize((info.reversal_angle.sqrt() + 2.0/3.0).powi(2))
            }
          }
        },
        Alignment::Square => {
          match info.reversal_type {
            Reversal::RangeHigh => {
              Alignment::normalize((info.reversal_angle.sqrt() - 0.5).powi(2))
            },
            Reversal::RangeLow => {
              Alignment::normalize((info.reversal_angle.sqrt() + 0.5).powi(2))
            }
          }
        },
        Alignment::Quintile => {
          match info.reversal_type {
            Reversal::RangeHigh => {
              Alignment::normalize((info.reversal_angle.sqrt() - 2.0/5.0).powi(2))
            },
            Reversal::RangeLow => {
              Alignment::normalize((info.reversal_angle.sqrt() + 2.0/5.0).powi(2))
            }
          }
        },
        Alignment::Sextile => {
          match info.reversal_type {
            Reversal::RangeHigh => {
              Alignment::normalize((info.reversal_angle.sqrt() - 2.0/6.0).powi(2))
            },
            Reversal::RangeLow => {
              Alignment::normalize((info.reversal_angle.sqrt() + 2.0/6.0).powi(2))
            }
          }
        }
     }
  }
}

impl PartialEq<Self> for Alignment {
  fn eq(&self, other: &Self) -> bool {
    self.to_str() == other.to_str()
  }
}
