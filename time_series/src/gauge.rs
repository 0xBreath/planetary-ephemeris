use ephemeris::Time;


pub type GaugeRatio = f64;
pub const GAUGE_RATIOS_OVER_1: &[GaugeRatio; 12] = &[
  // integer fractions
  2.0, // 2/1
  1.75, // 7/4
  1.5, // 3/2
  1.33, // 4/3
  1.25, // 5/4
  1.2, // 6/5
  1.17, // 7/6
  // fibonacci fractions
  1.618, // golden ratio (phi)
  1.434610113, // (1.618).powi(3.0 / 4.0)
  1.378221471, // (1.618).powi(2.0 / 3.0)
  1.127832563, // (1.618).powi(1.0 / 2.0)
  1.272006289, // (1.618).powi(1.0 / 2.0)
];

pub const GAUGE_RATIOS_BELOW_1: &[GaugeRatio; 14] = &[
  // integer fractions
  0.875, // 7/8
  0.75, // 3/4
  0.675, // 5/8
  0.66666, // 2/3
  0.5, // 1/2
  0.375, // 3/8
  0.33, // 1/3
  0.25, // 1/4
  0.125, // 1/8
  // fibonacci fractions
  0.8518, // 1 / (1.618).powi(1/3)
  0.78615, // 1 / (1.618).powi(1/2)
  0.618, // 1 / 1.618
  0.38198, // 1 / 1.618.powi(2)
  0.23608, // 1 / 1.618.powi(3)
];

#[derive(Clone, Debug)]
pub enum Operation {
  Multiply,
  Divide,
}

#[derive(Clone, Debug)]
pub struct Gauge {
  pub price: Option<f64>,
  pub price_ratio: Option<GaugeRatio>,
  pub date: Option<Time>,
  pub date_ratio: Option<GaugeRatio>,
}

impl Gauge {
  /// Create a new Gauge.
  /// Functions as a point of reference for price/date predictions to stem from.
  pub fn new(
    price: Option<f64>,
    price_ratio: Option<GaugeRatio>,
    date: Option<Time>,
    date_ratio: Option<GaugeRatio>,
  ) -> Self {
    Self {
      price,
      price_ratio,
      date,
      date_ratio,
    }
  }

  pub fn compute_price(&self, ratio: GaugeRatio, op: &Operation) -> Option<f64> {
    match self.price {
      Some(price) => {
        match op {
          Operation::Multiply => {
            let root_price = price.sqrt();
            let root_ratio = root_price * ratio;
            Some(root_ratio.powi(2))
          },
          Operation::Divide => {
            let root_price = price.sqrt();
            let root_ratio = root_price / ratio;
            Some(root_ratio.powi(2))
          }
        }
      },
      None => None,
    }
  }

  fn gauge_prices_inner(&self, op: &Operation) -> Vec<Gauge> {
    let mut gauge_vec = Vec::new();
    for ratio in GAUGE_RATIOS_OVER_1.iter() {
      let ratio_price = self.compute_price(*ratio, op);
      let gauge = Gauge::new(
        ratio_price,
        Some(*ratio),
        self.date,
        self.date_ratio,
      );
      gauge_vec.push(gauge);
    }
    gauge_vec
  }

  /// Find price points for all significant multiples >1 from self.
  pub fn bullish_price_gauges(&self) -> Vec<Gauge> {
    self.gauge_prices_inner(&Operation::Multiply)
  }

  /// Find price points for all significant multiples <1 from self.
  pub fn bearish_price_gauges(&self) -> Vec<Gauge> {
    self.gauge_prices_inner(&Operation::Divide)
  }

  /// Find dates for all significant multiples from self.
  pub fn gauge_dates(&self, period: u32) -> Option<Vec<Gauge>> {
    match self.date {
      Some(date) => {
        let mut gauge_vec = Vec::new();
        for ratio in GAUGE_RATIOS_OVER_1.iter() {
          let ratio_period = (period as f64 * ratio) as i64;
          let ratio_date = date.delta_date(ratio_period);
          let gauge = Gauge::new(
            self.price,
            self.price_ratio,
            Some(ratio_date),
            Some(*ratio),
          );
          gauge_vec.push(gauge);
        }
        Some(gauge_vec)
      },
      None => None
    }
  }

  // TODO: gauge price and date in matrix of ratios for a reference Gauge/self
}