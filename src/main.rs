use core::fmt;
use std::env;
use std::process::Command;

const PATH_MAX_LENGTH: usize = 50;
const DATE_TIME_FORMAT: &str = "+%m/%d %H:%M:%S";

const DIR_ICON: &str = "󰉖";
const TIME_ICON: &str = "";
const APPLE_ICON: &str = "";
const LINUX_ICON: &str = "";

enum Color {
  _Nothing,
  _Black,
  Blue,
  _Cyan,
  _Green,
  Magenta,
  _Red,
  White,
  _Yellow,
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Color::_Nothing => panic!("Color::_Nothing is can not convert to string."),
      Color::_Black => write!(f, "black"),
      Color::Blue => write!(f, "blue"),
      Color::_Cyan => write!(f, "cyan"),
      Color::_Green => write!(f, "green"),
      Color::Magenta => write!(f, "magenta"),
      Color::_Red => write!(f, "red"),
      Color::White => write!(f, "white"),
      Color::_Yellow => write!(f, "yellow"),
    }
  }
}

fn add_bg(string: String, color: Color) -> String {
  return format!("%K{{{}}} {} %k", color, string);
}

fn id() -> String {
  let os_icon = match env::consts::OS {
    "macos" => APPLE_ICON,
    "linux" => LINUX_ICON,
    _ => "",
  };

  let output = Command::new("id").arg("-un").output().expect("failed to execute process");

  return format!("{} {}", os_icon, String::from_utf8_lossy(&output.stdout).trim_end());
}

fn dir() -> String {
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
    return format!("{} {}.../{}", DIR_ICON, path_of_other_than_current_dir, current_dir);
  } else {
    return format!("{} {}/{}", DIR_ICON, path_of_other_than_current_dir, current_dir);
  }
}

fn date_time() -> String {
  let output = Command::new("date").arg(DATE_TIME_FORMAT).output().expect("failed to execute process");

  return format!("{} {}", TIME_ICON, String::from_utf8_lossy(&output.stdout).trim_end());
}

fn main() {
  let id_prompt = add_bg(id(), Color::Magenta);
  let dir_prompt = add_bg(dir(), Color::Blue);
  let date_time_prompt = add_bg(date_time(), Color::White);

  print!("{}{}\n{} ", id_prompt, dir_prompt, date_time_prompt)
}
