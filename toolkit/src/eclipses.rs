use std::fs::File;
use std::path::PathBuf;
use log::debug;
use ephemeris::*;
use crate::*;


#[derive(Clone, Debug)]
pub struct PlanetEquatorCrossTwoEclipses {
  pub planet: Planet,
  pub first_eclipse: EclipseEvent,
  pub second_eclipse: EclipseEvent,
}
impl PlanetEquatorCrossTwoEclipses {
  pub fn new(
    planet: Planet,
    first_eclipse: EclipseEvent,
    second_eclipse: EclipseEvent,
  ) -> Self {
    Self {
      planet,
      first_eclipse,
      second_eclipse,
    }
  }
}

#[derive(Clone, Debug)]
pub struct PlanetSelfAlignmentTwoEclipses {
  pub planet: Planet,
  pub first_eclipse: EclipseEvent,
  pub second_eclipse: EclipseEvent,
  pub alignment: Alignment,
}
impl PlanetSelfAlignmentTwoEclipses {
  pub fn new(
    planet: Planet,
    first_eclipse: EclipseEvent,
    second_eclipse: EclipseEvent,
    alignment: Alignment,
  ) -> Self {
    Self {
      planet,
      first_eclipse,
      second_eclipse,
      alignment,
    }
  }
}

#[derive(Clone, Debug)]
pub struct PlanetPairAlignmentOnEclipse {
  pub planet_pair: (Planet, Planet),
  pub alignment: Alignment,
  pub date: Time,
  pub eclipse: EclipseEvent,
}
impl PlanetPairAlignmentOnEclipse {
  pub fn new(
    planet_a: Planet,
    planet_b: Planet,
    alignment: Alignment,
    date: Time,
    eclipse: EclipseEvent,
  ) -> Self {
    Self {
      planet_pair: (planet_a, planet_b),
      alignment,
      date,
      eclipse,
    }
  }
}


#[derive(Debug, Clone)]
pub enum EclipseClass {
  Solar,
  Lunar,
}

#[derive(Debug, Clone)]
pub enum EclipseType {
  TotalSolar,
  AnnularSolar,
  PartialSolar,
  PenumbralLunar,
  PartialLunar,
  TotalLunar,
}

impl EclipseType {
  /// ## Lunar Eclipse Types
  /// N = penumbral eclipse: Moon traverses Earth's penumbra, but misses umbra.
  ///
  /// P = partial eclipse: Moon traverses Earth's penumbra and umbra, but does not enter umbra entirely.
  ///
  /// T = total eclipse: Moon passes completely into Earth's umbra.
  ///
  /// Read lunar eclipse CSV into Vec<(Time, EclipseType, EclipseRank)>
  ///
  /// ## Solar Eclipse Types
  /// P = partial eclipse: Earth traverses Moon’s penumbra, but misses umbra.
  ///
  /// A = annular eclipse: Earth enters Moon's antumbra, but does not completely cover the Sun.
  ///
  /// T = total eclipse: Earth completely enters Moon’s umbra.
  ///
  /// H = hybrid eclipse: Earth traverses Moon's umbra and antumbra, so annular and total eclipses are visible in different locations on Earth.
  ///
  pub fn from_symbol(symbol: &str, kind: EclipseClass) -> Self {
    match kind {
      EclipseClass::Lunar => {
        match symbol {
          "N" => EclipseType::PenumbralLunar,
          "Nx" => EclipseType::PenumbralLunar,
          "Ne" => EclipseType::PenumbralLunar,
          "Nb" => EclipseType::PenumbralLunar,
          "P" => EclipseType::PartialLunar,
          "T" => EclipseType::TotalLunar,
          "T+" => EclipseType::TotalLunar,
          "T-" => EclipseType::TotalLunar,
          _ => panic!("Invalid lunar eclipse symbol"),
        }
      },
      EclipseClass::Solar => {
        match symbol {
          "P" => EclipseType::PartialSolar,
          "Pe" => EclipseType::PartialSolar,
          "Pb" => EclipseType::PartialSolar,
          "A" => EclipseType::AnnularSolar,
          "A+" => EclipseType::AnnularSolar,
          "A-" => EclipseType::AnnularSolar,
          "Am" => EclipseType::AnnularSolar,
          "An" => EclipseType::AnnularSolar,
          "As" => EclipseType::AnnularSolar,
          "T" => EclipseType::TotalSolar,
          "T+" => EclipseType::TotalSolar,
          "T-" => EclipseType::TotalSolar,
          "Tm" => EclipseType::TotalSolar,
          "Tn" => EclipseType::TotalSolar,
          "Ts" => EclipseType::TotalSolar,
          "H" => EclipseType::TotalSolar,
          "Hm" => EclipseType::TotalSolar,
          "H2" => EclipseType::TotalSolar,
          "H3" => EclipseType::TotalSolar,
          _ => panic!("Invalid solar eclipse symbol"),
        }
      },
    }
  }

  /// ## Eclipse Importance Ranking Greatest To Least:
  ///
  /// 1) Total Solar Eclipse (TSE)
  ///
  /// 2) Annular Solar Eclipse (ASE) & Penumbral Lunar Eclipse (PenLE)
  ///
  /// 3) Partial Solar Eclipse (PSE) & Partial Lunar Eclipse (PLE)
  ///
  /// 4) Total Lunar Eclipse (TLE)
  pub fn to_rank(&self) -> u8 {
    match self {
      EclipseType::TotalSolar => 1,
      EclipseType::AnnularSolar => 2,
      EclipseType::PenumbralLunar => 2,
      EclipseType::PartialSolar => 3,
      EclipseType::PartialLunar => 3,
      EclipseType::TotalLunar => 4,
    }
  }
}

#[derive(Debug, Clone)]
pub struct EclipseEvent {
  pub date: Time,
  pub kind: EclipseType,
}

#[derive(Debug, Clone)]
pub struct Eclipses {
    pub events: Vec<EclipseEvent>
}
impl EclipseEvent {
  pub fn new(date: Time, kind: EclipseType) -> Self {
    Self {
      date,
      kind,
    }
  }
}

impl Eclipses {
  pub fn new(solar_eclipse_csv: &PathBuf, lunar_eclipse_csv: &PathBuf) -> Self {
    //
    // read Solar Eclipse CSV and load into Vec<EclipseEvent>
    let solar_buffer = File::open(solar_eclipse_csv).unwrap();
    let mut solar = csv::Reader::from_reader(solar_buffer);
    let mut solar_events = Vec::<EclipseEvent>::new();
    for record in solar.records().flatten() {
      let date_unformatted = &record[1];
      let date = Time::from_eclipse_date_format(date_unformatted);
      debug!("SOLAR\t{}", date.as_string());
      let eclipse_type = &record[6];
      debug!("SOLAR\t{:?}", eclipse_type);
      let eclipse_phase = EclipseType::from_symbol(eclipse_type, EclipseClass::Solar);
      debug!("SOLAR\t{:?}", eclipse_phase);
      solar_events.push(EclipseEvent::new(date, eclipse_phase));
    }
    //
    // read Lunar Eclipse CSV and load into Vec<EclipseEvent>
    let lunar_buffer = File::open(lunar_eclipse_csv).unwrap();
    let mut lunar = csv::Reader::from_reader(lunar_buffer);
    let mut lunar_events = Vec::<EclipseEvent>::new();
    for record in lunar.records().flatten() {
      let date_unformatted = &record[1];
      let date = Time::from_eclipse_date_format(date_unformatted);
      debug!("LUNAR\t{}", date.as_string());
      let eclipse_type = &record[6];
      debug!("LUNAR\t{:?}", eclipse_type);
      let eclipse_phase = EclipseType::from_symbol(eclipse_type, EclipseClass::Lunar);
      debug!("LUNAR\t{:?}", eclipse_phase);
      lunar_events.push(EclipseEvent::new(date, eclipse_phase));
    }
    //
    // concatenate solar and lunar events into one Vec<(Time, EclipseType)>
    let events = [solar_events, lunar_events].concat();
    Self { events }
  }

  /// Compare previous eclipses and determine if the same planet
  /// will cross the celestial equator (zero declination cross) during both eclipses.
  pub async fn find_planet_equator_cross_in_two_eclipses(
    &self,
    start_time: Time,
    stop_time: Time,
    error_margin_days: i64,
  ) -> Vec<PlanetEquatorCrossTwoEclipses> {
    let planet_equator_crosses = PlanetEquatorCrosses::new(start_time, stop_time).await;

    // iterate planet declinations and find when
    // the same planet crosses zero declination on a second eclipse
    // return those events as Vec<PlanetEquatorCrossTwoEclipses>
    let mut signals = Vec::<PlanetEquatorCrossTwoEclipses>::new();
    let mut planet_equator_crosses_two_eclipses: Vec<Option<PlanetEquatorCrossTwoEclipses>> = vec![None; Planet::to_vec().len()];

    for equator_cross in planet_equator_crosses.equator_crosses.into_iter() {
      let planet_index = equator_cross.planet.to_index();
      for event in self.events.iter() {
        let range_start = event.date.delta_date(-(error_margin_days));
        let range_end = event.date.delta_date(error_margin_days);
        if equator_cross.date.within_range(range_start, range_end) {
          if planet_equator_crosses_two_eclipses[planet_index].is_none() {
            planet_equator_crosses_two_eclipses[planet_index] = Some(
              PlanetEquatorCrossTwoEclipses::new(equator_cross.planet.clone(), event.clone(), event.clone())
            );
          } else {
            let mut planet_equator_cross_two_eclipses = planet_equator_crosses_two_eclipses[planet_index].take().unwrap();
            planet_equator_cross_two_eclipses.second_eclipse = event.clone();

            println!(
              "{}\t{}\t{}",
              &planet_equator_cross_two_eclipses.planet.to_str(),
              &planet_equator_cross_two_eclipses.first_eclipse.date.as_string(),
              &planet_equator_cross_two_eclipses.second_eclipse.date.as_string(),
            );

            signals.push(planet_equator_cross_two_eclipses.clone());
            planet_equator_crosses_two_eclipses[planet_index] = Some(
              PlanetEquatorCrossTwoEclipses::new(
                equator_cross.planet.clone(),
                planet_equator_cross_two_eclipses.second_eclipse.clone(),
                planet_equator_cross_two_eclipses.second_eclipse.clone()
              )
            );
          }
        }
      }
    }
    signals
  }

  /// Compare previous eclipses and determine if the same planet
  /// is in `Alignment` with itself during a previous eclipse.
  pub async fn find_planet_self_alignment_in_two_eclipses(
    &self,
    start_time: Time,
    stop_time: Time,
    error_margin_days: i64,
    error_margin_degrees: f32,
  ) -> Vec<PlanetSelfAlignmentTwoEclipses> {
    let planet_alignments = PlanetAlignments::new(start_time, stop_time).await;

    let mut signals = Vec::<PlanetSelfAlignmentTwoEclipses>::new();
    for (index, event) in self.events.iter().enumerate() {
      // find planet self alignments relative to EclipseEvent date
      // returns Ok if relative date is >= start date of planetary ephemerides
      if let Ok(relative_alignments) = planet_alignments
        .self_relative_alignments(event.date, error_margin_degrees) {
        // loop thru the rest of the events to find a relative alignment on another eclipse
        for inner_index in index+1..self.events.len() {
          let second_event = &self.events[inner_index];
          let range_start = second_event.date.delta_date(-(error_margin_days));
          let range_end = second_event.date.delta_date(error_margin_days);

          for alignment in relative_alignments.alignments.iter() {
            // a planet is in Alignment with itself on the second eclipse
            // relative to its position on the first eclipse.
            // If planet self relative alignment occurred within error margin of days of second eclipse
            // then we have a signal.
            if alignment.date.within_range(range_start, range_end) {
              let signal = PlanetSelfAlignmentTwoEclipses::new(
                alignment.planet.clone(),
                event.clone(),
                second_event.clone(),
                alignment.alignment.clone(),
              );
              println!(
                "{}\t{}\t{}\t{:?}",
                signal.planet.to_str(),
                signal.first_eclipse.date.as_string(),
                signal.second_eclipse.date.as_string(),
                signal.alignment.to_str(),
              );
              signals.push(signal);
            }
          }
        }
      }
    }
    signals
  }


  /// Find confluence between PlanetMatrix and EclipseEvents.
  pub async fn find_planet_matrix_alignments_on_eclipse(
    &self,
    start_time: Time,
    end_time: Time,
    error_margin_degrees: f32
  ) -> Vec<PlanetPairAlignmentOnEclipse> {
    let mut planet_matrix = PlanetMatrix::new(
      Origin::Geocentric,
      &start_time,
      &end_time,
      error_margin_degrees
    ).await;

    let mut signals = Vec::<PlanetPairAlignmentOnEclipse>::new();
    for event in self.events.iter() {
      let planet_pair_alignments: Vec<PlanetPairAlignment> = planet_matrix.alignments_on_date(&event.date);
      for alignment in planet_pair_alignments.iter() {
        let signal = PlanetPairAlignmentOnEclipse {
          planet_pair: alignment.planet_pair.clone(),
          eclipse: event.clone(),
          alignment: alignment.alignment.clone(),
          date: alignment.date,
        };
        println!(
          "{}-{}\t{}\t{}\t{:?}",
          signal.planet_pair.0.to_str(),
          signal.planet_pair.1.to_str(),
          signal.eclipse.date.as_string(),
          signal.date.as_string(),
          signal.alignment.to_str(),
        );
        signals.push(signal);
      }
    }
    signals
  }

}

