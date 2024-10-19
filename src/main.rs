use std::env;

mod l_prompt;
mod r_prompt;
mod segment;
mod setting_gui;

fn main() {
  let args: Vec<String> = env::args().collect();

  match args[args.len() - 1].as_str() {
    "--left" => l_prompt::run(),
    "--right" => r_prompt::run(),
    _ => setting_gui::run(),
  }
}
