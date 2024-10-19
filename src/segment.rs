use core::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug)]
pub struct Segment {
  pub string: String,
  pub color: Color,
}

pub fn set_bg(string: &String, color: &Color) -> String {
  return format!("%K{{{}}}{}%k", color, string);
}

pub fn set_fg(string: &String, color: &Color) -> String {
  return format!("%F{{{}}}{}%f", color, string);
}
