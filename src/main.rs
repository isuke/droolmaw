use std::env;

mod l_prompt;
mod r_prompt;
mod segment;
mod setting_gui;

fn main() {
  let args: Vec<String> = env::args().collect();

  match args[args.len() - 1].as_str() {
    "--right" => r_prompt::run(),
    "--setting-gui" => setting_gui::run().expect("failed to execute process"),
    _ => l_prompt::run(),
  }
}
