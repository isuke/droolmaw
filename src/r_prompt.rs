use std::process::Command;

use crate::segment::set_bg;
use crate::segment::set_fg;

use crate::segment::Color;
use crate::segment::Segment;

const SEPARATOR: &str = "";

pub fn run() {
  let mut prompt: Vec<Segment> = Vec::new();

  prompt.push(Segment {
    string: dir_path(),
    color: Color::Magenta,
  });
  if is_using_mise() {
    prompt.push(Segment {
      string: langs(),
      color: Color::Blue,
    });
  };

  print!("{}", build(prompt));
}

fn build(segments: Vec<Segment>) -> String {
  let mut result = String::from("");

  let mut prev_color = &Color::Nothing;
  for segment in &segments {
    if *prev_color == Color::Nothing {
      result = format!("{}{}", result, set_fg(&String::from(SEPARATOR), &segment.color));
    } else {
      result = format!("{}{}", result, set_fg(&set_bg(&String::from(SEPARATOR), prev_color), &segment.color))
    }

    let temp = format!(" {} ", segment.string);
    result = format!("{}{}", result, set_bg(&temp, &segment.color));

    prev_color = &segment.color;
  }

  return result;
}

fn is_using_mise() -> bool {
  return match Command::new("mise").arg("--version").output() {
    Ok(_) => true,
    Err(_) => false,
  };
}

fn langs() -> String {
  return format!("TODO");
}

fn dir_path() -> String {
  let output = Command::new("pwd").output().expect("failed to execute process");

  return format!("{}", String::from_utf8_lossy(&output.stdout).trim_end());
}
