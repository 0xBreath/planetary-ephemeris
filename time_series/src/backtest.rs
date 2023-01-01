use ephemeris::Planet;

#[derive(Debug, Clone)]
pub enum AlignmentObjectType {
  Price(f64),
  Planet(Planet),
}

#[derive(Debug, Clone)]
pub struct Backtest {
  object_1: Option<AlignmentObjectType>,
  object_2: Option<AlignmentObjectType>,
  harmonic: Option<f32>,
  win_count: u64,
  total_count: u64,
  win_rate: f64,
}
impl Backtest {
  pub fn default() -> Backtest {
    Backtest {
      object_1: None,
      object_2: None,
      harmonic: None,
      win_count: 0,
      total_count: 0,
      win_rate: 0.0,
    }
  }
  pub fn get_win_count(&self) -> u64 {
    self.win_count
  }
  pub fn increment_win_count(&mut self) {
    self.win_count += 1;
    self.set_win_rate();
  }
  pub fn get_total_count(&self) -> u64 {
    self.total_count
  }
  pub fn increment_total_count(&mut self) {
    self.total_count += 1;
    self.set_win_rate();
  }
  fn set_win_rate(&mut self) {
    self.win_rate = self.win_count as f64 / self.total_count as f64;
  }
  pub fn get_win_rate(&self) -> f64 {
    self.win_rate
  }
  pub fn get_harmonic(&self) -> Option<f32> {
    self.harmonic
  }
  pub fn set_harmonic(&mut self, harmonic: f32) {
    self.harmonic = Some(harmonic);
  }
  pub fn get_object_1(&self) -> Option<AlignmentObjectType> {
    self.object_1.clone()
  }
  pub fn set_object_1(&mut self, object_1: AlignmentObjectType) {
    self.object_1 = Some(object_1);
  }
  pub fn get_object_2(&self) -> Option<AlignmentObjectType> {
    self.object_2.clone()
  }
  pub fn set_object_2(&mut self, object_2: AlignmentObjectType) {
    self.object_2 = Some(object_2);
  }
}