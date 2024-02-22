use std::process::Command;

fn add_bg(string: String, color: &str) -> String {
  return format!("%K{{{}}} {} %k", color, string);
}

fn id() -> String {
  let output = Command::new("id").arg("-un").output().expect("failed to execute process");
  return String::from(String::from_utf8_lossy(&output.stdout).trim_end());
}

fn main() {
  let id_prompt: String = add_bg(id(), "magenta");
  print!("{}", id_prompt)
}
