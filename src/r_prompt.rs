use std::process::Command;

use crate::segment::set_bg;
use crate::segment::set_fg;

use crate::segment::Color;
use crate::segment::Segment;

const SEPARATOR: &str = "";

const PATH_MAX_LENGTH: usize = 50;
const LANGS: [&str; 2] = ["ruby", "node"];

pub fn run() {
  let mut prompt: Vec<Segment> = Vec::new();

  prompt.push(Segment {
    string: dir_path(),
    color: Color::Magenta,
  });
  if LANGS.len() > 0 && is_using_mise() {
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
  let mut result = String::from("");

  for lang in LANGS {
    let output = Command::new("mise")
      .args(["ls", "--current", "--no-header", lang])
      .output()
      .expect("failed to execute process");

    let version: String = String::from(String::from_utf8_lossy(&output.stdout).trim_end().split_whitespace().collect::<Vec<&str>>()[1]);

    result = format!("{} {} {}", result, lang, version);
  }

  return result;
}

fn dir_path() -> String {
  let output = Command::new("pwd").output().expect("failed to execute process");
  let stdout = String::from(String::from_utf8_lossy(&output.stdout).trim_end());

  let collection: Vec<&str> = stdout.split("/").collect();
  let current_dir = String::from(*collection.last().unwrap());
  let path_of_other_than_current_dir = &collection[..collection.len()].join("/");

  if path_of_other_than_current_dir.len() > PATH_MAX_LENGTH {
    let path_of_other_than_current_dir = match path_of_other_than_current_dir.get(..PATH_MAX_LENGTH.min(path_of_other_than_current_dir.len())) {
      Some(str) => str,
      None => path_of_other_than_current_dir,
    };
    return format!("{}.../{}", path_of_other_than_current_dir, current_dir);
  } else {
    return format!("{}/{}", path_of_other_than_current_dir, current_dir);
  }
}
