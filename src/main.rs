use std::env;

mod l_prompt;
mod r_prompt;
mod segment;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args[args.len() - 1] == "--right" {
    r_prompt::run()
  } else {
    l_prompt::run()
  }
}
