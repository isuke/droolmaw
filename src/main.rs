use std::process::Command;

const PATH_MAX_LENGTH: usize = 50;

fn add_bg(string: String, color: &str) -> String {
  return format!("%K{{{}}} {} %k", color, string);
}

fn id() -> String {
  let output = Command::new("id").arg("-un").output().expect("failed to execute process");
  return String::from(String::from_utf8_lossy(&output.stdout).trim_end());
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
    return format!("{}.../{}", path_of_other_than_current_dir, current_dir);
  } else {
    return format!("{}/{}", path_of_other_than_current_dir, current_dir);
  }
}

fn main() {
  let id_prompt: String = add_bg(id(), "magenta");
  let dir_prompt: String = add_bg(dir(), "blue");
  print!("{}{} ", id_prompt, dir_prompt)
}
