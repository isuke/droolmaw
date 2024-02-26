use core::fmt;
use std::env;
use std::process::Command;

const SEGMENT_L_SEPARATOR: &str = "";
const _SEGMENT_R_SEPARATOR: &str = "";

const PATH_MAX_LENGTH: usize = 50;
const DATE_TIME_FORMAT: &str = "+%m/%d %H:%M:%S";

const DIR_ICON: &str = "󰉖";
const GIT_AUTHOR_ICON: &str = "󰏪";
const GIT_BRANCH_ICON: &str = "󰘬";
const GIT_STASH_ICON: &str = "󰠔";
const TIME_ICON: &str = "";
const APPLE_ICON: &str = "";
const LINUX_ICON: &str = "";

#[derive(Debug, PartialEq)]
enum Color {
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
struct Segment {
  string: String,
  color: Color,
}

fn is_inside_git_work_tree() -> bool {
  return match Command::new("git").args(["rev-parse", "--is-inside-work-tree"]).output() {
    Ok(_) => true,
    Err(_) => false,
  };
}

fn set_bg(string: &String, color: &Color) -> String {
  return format!("%K{{{}}}{}%k", color, string);
}

fn set_fg(string: &String, color: &Color) -> String {
  return format!("%F{{{}}}{}%f", color, string);
}

fn build_l_prompt(segments: Vec<Segment>) -> String {
  let mut result = String::from("");

  let mut index = 1;
  let mut prev_color = &Color::Nothing;
  for segment in &segments {
    if *prev_color != Color::Nothing {
      result = format!("{}{}", result, set_fg(&set_bg(&String::from(SEGMENT_L_SEPARATOR), &segment.color), prev_color))
    }

    let temp = format!(" {} ", segment.string);
    result = format!("{}{}", result, set_bg(&temp, &segment.color));

    if segments.len() <= index {
      result = format!("{}{}", result, set_fg(&String::from(SEGMENT_L_SEPARATOR), &segment.color))
    }

    index += 1;
    prev_color = &segment.color;
  }

  return result;
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

fn git_name() -> String {
  let output = Command::new("git")
    .args(["config", "--get", "user.name"])
    .output()
    .expect("failed to execute process");

  return format!("{} {}", GIT_AUTHOR_ICON, String::from_utf8_lossy(&output.stdout).trim_end());
}

fn git_current_branch() -> String {
  let output = Command::new("git")
    .args(["branch", "--show-current"])
    .output()
    .expect("failed to execute process");

  return format!("{} {}", GIT_BRANCH_ICON, String::from_utf8_lossy(&output.stdout).trim_end());
}

fn git_stash() -> String {
  let output = Command::new("git").args(["reflog", "refs/stash"]).output().expect("failed to execute process");

  return format!(
    "{} stash +{}",
    GIT_STASH_ICON,
    (String::from_utf8_lossy(&output.stdout).trim_end().split("/").collect::<Vec<&str>>().len() - 1)
  );
}

fn main() {
  let mut prompt_first: Vec<Segment> = Vec::new();
  let mut prompt_second: Vec<Segment> = Vec::new();

  prompt_first.push(Segment {
    string: id(),
    color: Color::Magenta,
  });
  prompt_first.push(Segment {
    string: dir(),
    color: Color::Blue,
  });
  if is_inside_git_work_tree() {
    prompt_first.push(Segment {
      string: git_name(),
      color: Color::Yellow,
    });
    prompt_first.push(Segment {
      string: git_current_branch(),
      color: Color::Green,
    });
    prompt_first.push(Segment {
      string: git_stash(),
      color: Color::White,
    });
  }

  prompt_second.push(Segment {
    string: date_time(),
    color: Color::White,
  });

  print!("{}", build_l_prompt(prompt_first));
  print!("\n");
  print!("{}", build_l_prompt(prompt_second));
  print!(" ");
}
