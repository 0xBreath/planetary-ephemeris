use ephemeris::Planet;

#[derive(Debug, Clone)]
pub struct Backtest {
  planet: Option<Planet>,
  harmonic: Option<f32>,
  win_count: u64,
  total_count: u64,
  win_rate: f64,
}
impl Backtest {
  pub fn default() -> Backtest {
    Backtest {
      planet: None,
      harmonic: None,
      win_count: 0,
      total_count: 0,
      win_rate: 0.0,
    }
  }
  pub fn increment_win_count(&mut self) {
    self.win_count += 1;
    self.set_win_rate();
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
  pub fn get_planet(&self) -> Option<Planet> {
    self.planet.clone()
  }
  pub fn set_planet(&mut self, planet: Planet) {
    self.planet = Some(planet);
  }
  pub fn get_harmonic(&self) -> Option<f32> {
    self.harmonic
  }
  pub fn set_harmonic(&mut self, harmonic: f32) {
    self.harmonic = Some(harmonic);
  }
}