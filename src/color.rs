use core::fmt;

#[derive(Debug, PartialEq)]
pub enum Color {
  Nothing,
  _Black,
  Blue,
  _Cyan,
  Green,
  Magenta,
  _Red,
  White,
  Yellow,
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Color::Nothing => panic!("Color::Nothing is can not convert to string."),
      Color::_Black => write!(f, "black"),
      Color::Blue => write!(f, "blue"),
      Color::_Cyan => write!(f, "cyan"),
      Color::Green => write!(f, "green"),
      Color::Magenta => write!(f, "magenta"),
      Color::_Red => write!(f, "red"),
      Color::White => write!(f, "white"),
      Color::Yellow => write!(f, "yellow"),
    }
  }
}
