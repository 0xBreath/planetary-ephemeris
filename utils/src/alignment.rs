use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub enum Alignment {
  Conjunct,
  Opposite,
  Trine,
  Square,
  Quintile,
  Sextile
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

  pub fn find_alignment(a: f32, b: f32, margin: f32) -> Option<Self> {
    let diff = (a - b).abs();
    if diff < margin {
      Some(Alignment::Conjunct)
    } else if (diff - 180.0).abs() < margin {
      Some(Alignment::Opposite)
    } else if (diff - 120.0).abs() < margin || (diff - 240.0).abs() < margin {
      Some(Alignment::Trine)
    } else if (diff - 90.0).abs() < margin || (diff - 270.0).abs() < margin {
      Some(Alignment::Square)
    } else if (diff - 72.0).abs() < margin ||
      (diff - 144.0).abs() < margin ||
      (diff - 216.0).abs() < margin ||
      (diff - 288.0).abs() < margin
    {
      Some(Alignment::Quintile)
    } else if (diff - 60.0).abs() < margin ||
      (diff - 300.0).abs() < margin
    {
      Some(Alignment::Sextile)
    } else {
      None
    }
  }
}

impl PartialEq<Self> for Alignment {
  fn eq(&self, other: &Self) -> bool {
    self.to_str() == other.to_str()
  }
}
