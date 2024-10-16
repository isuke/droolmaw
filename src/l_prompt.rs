use std::env;
use std::process::Command;

use crate::segment::set_bg;
use crate::segment::set_fg;

use crate::segment::Color;
use crate::segment::Segment;

const SEPARATOR: &str = "";

const DATE_TIME_FORMAT: &str = "+%m/%d %H:%M:%S";

const DIR_ICON: &str = "󰉖";
const GIT_AUTHOR_ICON: &str = "󰏪";
const GIT_BRANCH_ICON: &str = "󰘬";
const GIT_UNSTAGED_ICON: &str = "";
const GIT_STAGED_ICON: &str = "";
const GIT_STASH_ICON: &str = "󰠔";
const TIME_ICON: &str = "";
const APPLE_ICON: &str = "";
const LINUX_ICON: &str = "";

pub fn run() {
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
      string: git_current_branch_and_statuses(),
      color: Color::Green,
    });
    prompt_first.push(Segment {
      string: git_remotes_and_statuses(),
      color: Color::White,
    });
  }

  prompt_second.push(Segment {
    string: date_time(),
    color: Color::White,
  });

  print!("{}", build(prompt_first));
  print!("\n");
  print!("{}", build(prompt_second));
  print!(" ");
}

fn build(segments: Vec<Segment>) -> String {
  let mut result = String::from("");

  let mut index = 1;
  let mut prev_color = &Color::Nothing;
  for segment in &segments {
    if *prev_color != Color::Nothing {
      result = format!("{}{}", result, set_fg(&set_bg(&String::from(SEPARATOR), &segment.color), prev_color))
    }

    let temp = format!(" {} ", segment.string);
    result = format!("{}{}", result, set_bg(&temp, &segment.color));

    if segments.len() <= index {
      result = format!("{}{}", result, set_fg(&String::from(SEPARATOR), &segment.color))
    }

    index += 1;
    prev_color = &segment.color;
  }

  return result;
}

fn is_inside_git_work_tree() -> bool {
  return match Command::new("git").args(["rev-parse", "--is-inside-work-tree"]).output() {
    Ok(_) => true,
    Err(_) => false,
  };
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
  return String::from(String::from_utf8_lossy(&output.stdout).trim_end());
}

fn git_current_branch_and_statuses() -> String {
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

  let stash_output = Command::new("git").args(["reflog", "refs/stash"]).output().expect("failed to execute process");
  let stash_file_num = String::from(String::from_utf8_lossy(&stash_output.stdout).trim_end())
    .split("/")
    .collect::<Vec<&str>>()
    .len()
    - 1;

  let mut status = String::from("");

  if 0 < unstaged_file_num {
    status = format!("{} {} {}", status, GIT_UNSTAGED_ICON, unstaged_file_num);
  }
  if 0 < staged_file_num {
    status = format!("{} {} {}", status, GIT_STAGED_ICON, staged_file_num);
  }
  if 0 < stash_file_num {
    status = format!("{} {} {}", status, GIT_STASH_ICON, stash_file_num);
  }

  return format!("{} {}{}", GIT_BRANCH_ICON, git_current_branch(), status);
}

fn git_remotes() -> Vec<String> {
  let output = Command::new("git").arg("remote").output().expect("failed to execute process");
  return String::from(String::from_utf8_lossy(&output.stdout))
    .split("\n")
    .map(|str| String::from(str))
    .filter(|str| *str != "")
    .collect::<Vec<String>>();
}

fn git_remotes_and_statuses() -> String {
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
