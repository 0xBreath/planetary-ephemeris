
#[derive(Clone, Copy, Debug)]
pub enum Harmonic {
  Zero = 0,
  OneEighth = 45,
  OneFourth = 90,
  ThreeEighths = 135,
  OneHalf = 180,
  FiveEighths = 225,
  ThreeFourths = 270,
  SevenEighths = 315,
}
impl Harmonic {
  fn next(&self) -> Self {
    match self {
      Harmonic::Zero => Harmonic::SevenEighths,
      Harmonic::OneEighth => Harmonic::Zero,
      Harmonic::OneFourth => Harmonic::OneEighth,
      Harmonic::ThreeEighths => Harmonic::OneFourth,
      Harmonic::OneHalf => Harmonic::ThreeEighths,
      Harmonic::FiveEighths => Harmonic::OneHalf,
      Harmonic::ThreeFourths => Harmonic::FiveEighths,
      Harmonic::SevenEighths => Harmonic::ThreeFourths,
    }
  }

  fn to_num(self) -> f32 {
    match self {
      Harmonic::Zero => 0.0,
      Harmonic::OneEighth => 45.0,
      Harmonic::OneFourth => 90.0,
      Harmonic::ThreeEighths => 135.0,
      Harmonic::OneHalf => 180.0,
      Harmonic::FiveEighths => 225.0,
      Harmonic::ThreeFourths => 270.0,
      Harmonic::SevenEighths => 315.0,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Angle(pub f32);

impl Angle {
  pub fn new(angle: f32) -> Self {
    Angle(angle)
  }
  fn decrement(&mut self, value: f32) {
    self.0 -= value;
    if self.0 >= 360.0 {
      self.0 -= 360.0;
    } else if self.0 < 0.0 {
      self.0 += 360.0;
    }
  }
  fn set(&mut self, angle: f32) {
    self.0 = angle;
  }
  fn to_arc(self, arc_range: f32) -> (f32, f32) {
    // 1/3600 = one arc second
    (self.0 + arc_range - 1.0/3600.0, self.0)
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
  pub value: f64,
  pub harmonic: Option<Harmonic>,
  pub arc: Option<(f32, f32)>
}
impl Default for Point {
  fn default() -> Self {
    Self {
      value: 0.0,
      harmonic: None,
      arc: None,
    }
  }
}

#[derive(Clone, Debug)]
pub struct SquareOfNine<const N: usize> {
  /// Starting value of Square of Nine
  pub origin: f64,
  /// Matrix of coordinates that represent the 'Square of Nine'.
  /// Used for plotting numbers in a visual-friendly grid.
  pub matrix: [[Point; N]; N],
  /// One-dimensional vector of points in `Square of Nine`.
  /// Used for calculating 'Time=Price' alignments. Not intended for plotting.
  pub values: Vec<Point>
}

impl<const N: usize> SquareOfNine<N> {
  pub fn new(origin: f64) -> Self {
    if N % 2 == 0 {
      panic!("matrix size N must be odd");
    }
    let mut matrix = [[Point::default(); N]; N];
    let mut values = Vec::<Point>::new();
    let mut value = origin;
    let center = N / 2;

    let mut x = center as usize;
    let mut y = center as usize;

    // origin
    let point = Point {
      value,
      harmonic: None,
      arc: None
    };
    matrix[y][y] = point;
    values.push(point);



    // index gap between harmonics. Increases as spiral grows.
    let mut empty_harmonics = 0;
    // each new harmonic rotates/increments harmonic.
    // Starting harmonic is OneHalf and rotates counter-clockwise
    let mut harmonic = Harmonic::OneHalf;
    let mut angle = Angle::new(harmonic.to_num());
    // loop through matrix size
    // each index is used twice as the matrix spirals
    let mut index = 1;
    // size of one spiral of matrix. Increases by 8 with each new spiral.
    let mut spiral_size = 8;
    while index < N {
      let arc_per_integer = 360.0 / spiral_size as f32;
      // shift left by 1 to start next spiral
      x -= 1;
      value += 1.0;
      if index == 1 {
        angle.set(harmonic.to_num());
        let point = Point {
          value,
          harmonic: Some(harmonic),
          arc: Some(angle.to_arc(arc_per_integer))
        };
        matrix[y][x] = point;
        values.push(point);
      } else {
        angle.set(harmonic.to_num() - arc_per_integer);
        let point = Point {
          value,
          harmonic: None,
          arc: Some(angle.to_arc(arc_per_integer))
        };
        matrix[y][x] = point;
        values.push(point);
      }

      // spiral up by index
      let mut inner_index = 0;
      let mut up_empty_harmonics_length = empty_harmonics - 1;
      while inner_index < index {
        for _ in 0..up_empty_harmonics_length {
          y -= 1;
          value += 1.0;
          angle.decrement(arc_per_integer);
          let point = Point {
            value,
            harmonic: None,
            arc: Some(angle.to_arc(arc_per_integer))
          };
          matrix[y][x] = point;
          values.push(point);
          inner_index += 1;
        }

        y -= 1;
        value += 1.0;
        harmonic = harmonic.next();
        angle.set(harmonic.to_num());
        let point = Point {
          value,
          harmonic: Some(harmonic),
          arc: Some(angle.to_arc(arc_per_integer))
        };
        matrix[y][x] = point;
        values.push(point);
        inner_index += 1;
        up_empty_harmonics_length += 1;
      }
      inner_index = 0;

      // spiral right by index + 1
      while inner_index < index + 1 {
        for _ in 0..empty_harmonics {
          x += 1;
          value += 1.0;
          angle.decrement(arc_per_integer);
          let point = Point {
            value,
            harmonic: None,
            arc: Some(angle.to_arc(arc_per_integer))
          };
          matrix[y][x] = point;
          values.push(point);
          inner_index += 1;
        }

        x += 1;
        value += 1.0;
        harmonic = harmonic.next();
        angle.set(harmonic.to_num());
        let point = Point {
          value,
          harmonic: Some(harmonic),
          arc: Some(angle.to_arc(arc_per_integer))
        };
        matrix[y][x] = point;
        values.push(point);
        inner_index += 1;
      }
      inner_index = 0;

      // spiral down by index + 1
      while inner_index < index + 1 {
        for _ in 0..empty_harmonics {
          y += 1;
          value += 1.0;
          angle.decrement(arc_per_integer);
          let point = Point {
            value,
            harmonic: None,
            arc: Some(angle.to_arc(arc_per_integer))
          };
          matrix[y][x] = point;
          values.push(point);
          inner_index += 1;
        }

        y += 1;
        value += 1.0;
        harmonic = harmonic.next();
        angle.set(harmonic.to_num());
        let point = Point {
          value,
          harmonic: Some(harmonic),
          arc: Some(angle.to_arc(arc_per_integer))
        };
        matrix[y][x] = point;
        values.push(point);
        inner_index += 1;
      }
      inner_index = 0;

      while inner_index < index + 1 {
        for _ in 0..empty_harmonics {
          x -= 1;
          value += 1.0;
          angle.decrement(arc_per_integer);
          let point = Point {
            value,
            harmonic: None,
            arc: Some(angle.to_arc(arc_per_integer))
          };
          matrix[y][x] = point;
          values.push(point);
          inner_index += 1;
        }

        x -= 1;
        value += 1.0;
        harmonic = harmonic.next();
        angle.set(harmonic.to_num());
        let point = Point {
          value,
          harmonic: Some(harmonic),
          arc: Some(angle.to_arc(arc_per_integer))
        };
        matrix[y][x] = point;
        values.push(point);
        inner_index += 1;
      }

      index += 2;
      empty_harmonics += 1;
      spiral_size += 8;
    }

    Self {
      origin,
      matrix,
      values
    }
  }
}