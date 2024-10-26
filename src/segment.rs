use std::env;
use std::process::Command;

use core::fmt;

const DATE_TIME_FORMAT: &str = "+%m/%d %H:%M:%S";
const DEFAULT_PATH_MAX_LENGTH: usize = 50;

const DIR_ICON: &str = "󰉖";
const GIT_AUTHOR_ICON: &str = "󰏪";
const GIT_BRANCH_ICON: &str = "󰘬";
const GIT_UNSTAGED_ICON: &str = "";
const GIT_STAGED_ICON: &str = "";
const GIT_STASH_ICON: &str = "󰠔";
const TIME_ICON: &str = "";
const APPLE_ICON: &str = "";
const LINUX_ICON: &str = "";

#[derive(Debug, PartialEq, Deserialize)]
pub enum Color {
  Nothing,
  Black,
  Blue,
  Cyan,
  Green,
  Magenta,
  Red,
  White,
  Yellow,
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Color::Nothing => panic!("Color::Nothing is can not convert to string."),
      Color::Black => write!(f, "black"),
      Color::Blue => write!(f, "blue"),
      Color::Cyan => write!(f, "cyan"),
      Color::Green => write!(f, "green"),
      Color::Magenta => write!(f, "magenta"),
      Color::Red => write!(f, "red"),
      Color::White => write!(f, "white"),
      Color::Yellow => write!(f, "yellow"),
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct Droolmaw {
  pub l_separator: String,
  pub r_separator: String,
  pub l_components_first: Vec<Component>,
  pub l_components_second: Vec<Component>,
  pub r_components: Vec<Component>,
}

#[derive(Debug, Deserialize)]
pub struct Component {
  pub name: String,
  pub color: Color,
  pub langs: Option<Vec<String>>,
  pub max_length: Option<usize>,
}

#[derive(Debug)]
pub struct Segment {
  pub string: String,
  pub color: Color,
}

pub fn create_segments(components: Vec<Component>) -> Vec<Segment> {
  let mut segments: Vec<Segment> = Vec::new();

  for component in components {
    match component.name.as_str() {
      "id" => segments.push(Segment {
        string: id(),
        color: component.color,
      }),
      "dir" => segments.push(Segment {
        string: dir(),
        color: component.color,
      }),
      "dir_path" => segments.push(Segment {
        string: dir_path(component.max_length),
        color: component.color,
      }),
      "date_time" => segments.push(Segment {
        string: date_time(),
        color: component.color,
      }),
      "git_name" => {
        if is_inside_git_work_tree() {
          segments.push(Segment {
            string: git_name(),
            color: component.color,
          })
        }
      }
      "git_current_branch_and_statuses" => {
        if is_inside_git_work_tree() {
          segments.push(Segment {
            string: git_current_branch_and_statuses(),
            color: component.color,
          })
        }
      }
      "git_remotes_and_statuses" => {
        if is_inside_git_work_tree() {
          segments.push(Segment {
            string: git_remotes_and_statuses(),
            color: component.color,
          })
        }
      }
      "git_stash" => {
        if is_inside_git_work_tree() {
          segments.push(Segment {
            string: git_stash(),
            color: component.color,
          })
        }
      }
      "langs" => match component.langs {
        None => panic!("can not find lang parameter."),
        Some(l) => {
          if is_using_mise() {
            segments.push(Segment {
              string: langs(l.iter().map(String::as_str).collect()),
              color: component.color,
            })
          }
        }
      },
      _ => panic!("can not find name '{}'.", component.name),
    }
  }

  return segments;
}

pub fn set_bg(string: &String, color: &Color) -> String {
  return format!("%K{{{}}}{}%k", color, string);
}

pub fn set_fg(string: &String, color: &Color) -> String {
  return format!("%F{{{}}}{}%f", color, string);
}

pub fn is_inside_git_work_tree() -> bool {
  return match Command::new("git").args(["rev-parse", "--is-inside-work-tree"]).output() {
    Ok(_) => true,
    Err(_) => false,
  };
}

pub fn id() -> String {
  let os_icon = match env::consts::OS {
    "macos" => APPLE_ICON,
    "linux" => LINUX_ICON,
    _ => "",
  };

  let output = Command::new("id").arg("-un").output().expect("failed to execute process");

  return format!("{} {}", os_icon, String::from_utf8_lossy(&output.stdout).trim_end());
}

pub fn dir() -> String {
  let output = Command::new("pwd").output().expect("failed to execute process");
  let stdout = String::from(
    *String::from_utf8_lossy(&output.stdout)
      .trim_end()
      .split("/")
      .collect::<Vec<&str>>()
      .last()
      .unwrap(),
  );

  return format!("{} {}", DIR_ICON, stdout);
}

pub fn dir_path(max_length: Option<usize>) -> String {
  let output = Command::new("pwd").output().expect("failed to execute process");
  let stdout = String::from(String::from_utf8_lossy(&output.stdout).trim_end());

  let collection: Vec<&str> = stdout.split("/").collect();
  let current_dir = String::from(*collection.last().unwrap());
  let path_of_other_than_current_dir = &collection[..collection.len()].join("/");

  let max_length = match max_length {
    None => DEFAULT_PATH_MAX_LENGTH,
    Some(ml) => ml,
  };

  if path_of_other_than_current_dir.len() > max_length {
    let path_of_other_than_current_dir = match path_of_other_than_current_dir.get(..max_length.min(path_of_other_than_current_dir.len())) {
      Some(str) => str,
      None => path_of_other_than_current_dir,
    };
    return format!("{}.../{}", path_of_other_than_current_dir, current_dir);
  } else {
    return format!("{}/{}", path_of_other_than_current_dir, current_dir);
  }
}

pub fn date_time() -> String {
  let output = Command::new("date").arg(DATE_TIME_FORMAT).output().expect("failed to execute process");

  return format!("{} {}", TIME_ICON, String::from_utf8_lossy(&output.stdout).trim_end());
}

pub fn git_name() -> String {
  let output = Command::new("git")
    .args(["config", "--get", "user.name"])
    .output()
    .expect("failed to execute process");

  return format!("{} {}", GIT_AUTHOR_ICON, String::from_utf8_lossy(&output.stdout).trim_end());
}

pub fn git_current_branch_and_statuses() -> String {
  let unstaged_file_output = Command::new("git").args(["diff", "--name-only"]).output().expect("failed to execute process");
  let unstaged_file_num = String::from(String::from_utf8_lossy(&unstaged_file_output.stdout))
    .split("\n")
    .filter(|str| *str != "")
    .collect::<Vec<&str>>()
    .len();

  let staged_file_output = Command::new("git")
    .args(["diff", "--name-only", "--cached"])
    .output()
    .expect("failed to execute process");
  let staged_file_num = String::from(String::from_utf8_lossy(&staged_file_output.stdout))
    .split("\n")
    .filter(|str| *str != "")
    .collect::<Vec<&str>>()
    .len();

  let mut status = String::from("");

  if 0 < unstaged_file_num {
    status = format!("{} {} {}", status, GIT_UNSTAGED_ICON, unstaged_file_num);
  }
  if 0 < staged_file_num {
    status = format!("{} {} {}", status, GIT_STAGED_ICON, staged_file_num);
  }

  return format!("{} {}{}", GIT_BRANCH_ICON, git_current_branch(), status);
}

pub fn git_remotes_and_statuses() -> String {
  let current_branch = git_current_branch();
  let remotes = git_remotes();

  let mut result = String::from("");
  for remote in &remotes {
    let remote_path = match Command::new("git")
      .args(["rev-parse", "--verify", "--symbolic-full-name", &format!("remotes/{}/{}", remote, current_branch)])
      .output()
    {
      Ok(remote_path_output) => String::from(String::from_utf8_lossy(&remote_path_output.stdout).trim_end()),
      Err(_) => String::from(""),
    };

    let status: String = if remote_path != "" {
      let ahead_output = Command::new("git")
        .args(["rev-list", &format!("{}..HEAD", remote_path)])
        .output()
        .expect("failed to execute process");
      let ahead_num = format!("{}", String::from_utf8_lossy(&ahead_output.stdout))
        .split("\n")
        .filter(|str| *str != "")
        .collect::<Vec<&str>>()
        .len();

      let behind_output = Command::new("git")
        .args(["rev-list", &format!("HEAD..{}", remote_path)])
        .output()
        .expect("failed to execute process");
      let behind_num = format!("{}", String::from_utf8_lossy(&behind_output.stdout))
        .split("\n")
        .filter(|str| *str != "")
        .collect::<Vec<&str>>()
        .len();

      if ahead_num == 0 && behind_num == 0 {
        String::from("")
      } else {
        format!("+{} -{}", ahead_num, behind_num)
      }
    } else {
      String::from("")
    };

    // NOTE: add a space to end for nerd icon.
    result = format!("{}{} {} ", result, remote, status)
  }

  return result;
}

pub fn git_stash() -> String {
  let stash_output = Command::new("git").args(["reflog", "refs/stash"]).output().expect("failed to execute process");
  let stash_file_num = String::from(String::from_utf8_lossy(&stash_output.stdout).trim_end())
    .split("/")
    .collect::<Vec<&str>>()
    .len()
    - 1;

  return format!("{} {}", GIT_STASH_ICON, stash_file_num);
}

pub fn langs(langs: Vec<&str>) -> String {
  let mut result = String::from("");

  for lang in langs {
    let output = Command::new("mise")
      .args(["ls", "--current", "--no-header", lang])
      .output()
      .expect("failed to execute process");

    let version: String = String::from(String::from_utf8_lossy(&output.stdout).trim_end().split_whitespace().collect::<Vec<&str>>()[1]);

    result = format!("{} {} {}", result, lang, version);
  }

  return result;
}

pub fn is_using_mise() -> bool {
  return match Command::new("mise").arg("--version").output() {
    Ok(_) => true,
    Err(_) => false,
  };
}

fn git_current_branch() -> String {
  let output = Command::new("git")
    .args(["branch", "--show-current"])
    .output()
    .expect("failed to execute process");
  return String::from(String::from_utf8_lossy(&output.stdout).trim_end());
}

fn git_remotes() -> Vec<String> {
  let output = Command::new("git").arg("remote").output().expect("failed to execute process");
  return String::from(String::from_utf8_lossy(&output.stdout))
    .split("\n")
    .map(|str| String::from(str))
    .filter(|str| *str != "")
    .collect::<Vec<String>>();
}
