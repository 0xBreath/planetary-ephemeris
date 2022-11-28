use ephemeris::Time;
use log::debug;

#[derive(Clone, Copy, Debug)]
pub enum Input {
  Date(Time),
  Angle(f32),
}
impl Input {
  pub fn increment(input: Self) -> Self {
    match input {
      Input::Date(date) => Input::Date(date.delta_date(1)),
      Input::Angle(angle) => Input::Angle(angle + 1.0),
    }
  }
}

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
      Harmonic::Zero => Harmonic::OneEighth,
      Harmonic::OneEighth => Harmonic::OneFourth,
      Harmonic::OneFourth => Harmonic::ThreeEighths,
      Harmonic::ThreeEighths => Harmonic::OneHalf,
      Harmonic::OneHalf => Harmonic::FiveEighths,
      Harmonic::FiveEighths => Harmonic::ThreeFourths,
      Harmonic::ThreeFourths => Harmonic::SevenEighths,
      Harmonic::SevenEighths => Harmonic::Zero,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
  pub value: Input,
  pub harmonic: Option<Harmonic>
}
impl Default for Point {
  fn default() -> Self {
    Self {
      value: Input::Angle(0.0),
      harmonic: None
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct SquareOfNine<const N: usize> {
  /// Starting value of Square of Nine
  pub origin: Input,
  /// Matrix of coordinates that represent the 'Square of Nine'
  pub matrix: [[Point; N]; N],
}

impl<const N: usize> SquareOfNine<N> {
  pub fn new(origin: Input) -> Self {
    if N % 2 == 0 {
      panic!("matrix size (N) must be odd");
    }
    let mut matrix = [[Point::default(); N]; N];
    let mut value = origin;
    let center = N / 2;

    let mut x = center as usize;
    let mut y = center as usize;

    // origin
    matrix[y][x] = Point {
      value,
      harmonic: None
    };
    debug!("ORIGIN [{},{}]", x, y);


    // index gap between harmonics. Increases as spiral grows.
    let mut empty_harmonics = 0;
    // each new harmonic rotates/increments harmonic.
    // Starting harmonic is Zero, so start on SevenEighths to increment to Zero in first iteration.
    let mut last_harmonic = Harmonic::ThreeFourths;
    // loop through matrix size
    // each index is used twice as the matrix spirals
    let mut index = 1;
    while index < N {
      // shift left by 1 to start next spiral
      x -= 1;
      value = Input::increment(value);
      debug!("START LEFT, {:?} [{},{}]", matrix[y][x].value, x, y);
      if index == 1 {
        matrix[y][x] = Point {
          value,
          harmonic: Some(last_harmonic)
        };
      } else {
        matrix[y][x] = Point {
          value,
          harmonic: None
        };
      }

      // spiral up by index
      let mut inner_index = 0;
      let mut up_empty_harmonics_length = empty_harmonics - 1;
      while inner_index < index {
        for _ in 0..up_empty_harmonics_length {
          y -= 1;
          value = Input::increment(value);
          matrix[y][x] = Point {
            value,
            harmonic: None
          };
          debug!("UP EMPTY, {:?} [{},{}]", matrix[y][x].value, x, y);
          inner_index += 1;
        }

        y -= 1;
        value = Input::increment(value);
        last_harmonic = last_harmonic.next();
        matrix[y][x] = Point {
          value,
          harmonic: Some(last_harmonic)
        };
        inner_index += 1;
        up_empty_harmonics_length += 1;
        debug!("UP, {:?} [{},{}]", matrix[y][x].value, x, y);
      }
      inner_index = 0;

      // spiral right by index + 1
      while inner_index < index + 1 {
        for _ in 0..empty_harmonics {
          x += 1;
          value = Input::increment(value);
          matrix[y][x] = Point {
            value,
            harmonic: None
          };
          inner_index += 1;
          debug!("RIGHT EMPTY, {:?} [{},{}]", matrix[y][x].value, x, y);
        }

        x += 1;
        value = Input::increment(value);
        last_harmonic = last_harmonic.next();
        matrix[y][x] = Point {
          value,
          harmonic: Some(last_harmonic)
        };
        inner_index += 1;
        debug!("RIGHT, {:?} [{},{}]", matrix[y][x].value, x, y);
      }
      inner_index = 0;

      // spiral down by index + 1
      while inner_index < index + 1 {
        for _ in 0..empty_harmonics {
          y += 1;
          value = Input::increment(value);
          matrix[y][x] = Point {
            value,
            harmonic: None
          };
          inner_index += 1;
          debug!("DOWN EMPTY, {:?} [{},{}]", matrix[y][x].value, x, y);
        }

        y += 1;
        value = Input::increment(value);
        last_harmonic = last_harmonic.next();
        matrix[y][x] = Point {
          value,
          harmonic: Some(last_harmonic)
        };
        inner_index += 1;
        debug!("DOWN, {:?} [{},{}]", matrix[y][x].value, x, y);
      }
      inner_index = 0;

      while inner_index < index + 1 {
        for _ in 0..empty_harmonics {
          x -= 1;
          value = Input::increment(value);
          matrix[y][x] = Point {
            value,
            harmonic: None
          };
          inner_index += 1;
          debug!("LEFT EMPTY, {:?} [{},{}]", matrix[y][x].value, x, y);
        }

        x -= 1;
        value = Input::increment(value);
        last_harmonic = last_harmonic.next();
        matrix[y][x] = Point {
          value,
          harmonic: Some(last_harmonic)
        };
        inner_index += 1;
        debug!("LEFT, {:?} [{},{}]", matrix[y][x].value, x, y);
      }

      index += 2;
      empty_harmonics +=1;
    }

    Self {
      origin,
      matrix,
    }
  }
}