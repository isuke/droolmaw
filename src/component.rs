use std::process::Command;

use core::fmt;

use crate::chip::*;

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
  pub l_components2: Vec<Component>,
  pub r_components: Option<Vec<Component>>,
}

#[derive(Debug, Deserialize)]
pub struct Component {
  pub name: String,
  pub color: Color,
  pub langs: Option<Vec<String>>,
  pub max_length: Option<usize>,
  pub ok_text: Option<String>,
  pub ng_text: Option<String>,
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
      "None" => segments.push(Segment {
        string: String::new(),
        color: component.color,
      }),
      "Id" => segments.push(Segment {
        string: id(),
        color: component.color,
      }),
      "Dir" => segments.push(Segment {
        string: dir(),
        color: component.color,
      }),
      "DirPath" => segments.push(Segment {
        string: dir_path(component.max_length),
        color: component.color,
      }),
      "DateTime" => segments.push(Segment {
        string: date_time(),
        color: component.color,
      }),
      "GitName" => {
        if is_inside_git_work_tree() {
          segments.push(Segment {
            string: git_name(),
            color: component.color,
          })
        }
      }
      "GitCurrentBranchAndStatuses" => {
        if is_inside_git_work_tree() {
          segments.push(Segment {
            string: git_current_branch_and_statuses(),
            color: component.color,
          })
        }
      }
      "GitRemotesAndStatuses" => {
        if is_inside_git_work_tree() {
          segments.push(Segment {
            string: git_remotes_and_statuses(),
            color: component.color,
          })
        }
      }
      "GitStash" => {
        if is_inside_git_work_tree() {
          segments.push(Segment {
            string: git_stash(),
            color: component.color,
          })
        }
      }
      "Langs" => match component.langs {
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
      "ResultText" => segments.push(Segment {
        string: result_text(component.ok_text, component.ng_text),
        color: component.color,
      }),
      _ => panic!("can not find name '{}'.", component.name),
    }
  }

  return segments;
}

pub fn get_font_color(background_color: &Color) -> Color {
  match background_color {
    Color::Nothing => panic!("Color::Nothing has no corresponding color."),
    Color::Black => Color::White,
    Color::Blue => Color::White,
    Color::Cyan => Color::Black,
    Color::Green => Color::Black,
    Color::Magenta => Color::White,
    Color::Red => Color::White,
    Color::White => Color::Black,
    Color::Yellow => Color::Black,
  }
}

pub fn set_bg(string: &String, color: &Color) -> String {
  return format!("%K{{{}}}{}%k", color, string);
}

pub fn set_fg(string: &String, color: &Color) -> String {
  return format!("%F{{{}}}{}%f", color, string);
}

fn is_inside_git_work_tree() -> bool {
  return match Command::new("git").args(["rev-parse", "--is-inside-work-tree"]).output() {
    Ok(result) => String::from(String::from_utf8_lossy(&result.stdout)).trim_end() == "true",
    Err(_) => false,
  };
}

fn is_using_mise() -> bool {
  return match Command::new("mise").arg("--version").output() {
    Ok(_) => true,
    Err(_) => false,
  };
}
