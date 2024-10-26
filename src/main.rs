#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::env;
use std::fs::File;
use std::io::Read;

use crate::segment::Droolmaw;

mod l_prompt;
mod l_prompt2;
mod r_prompt;
mod segment;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
  let path = match dirs::home_dir() {
    Some(path_buf) => format!("{}/.droolmaw.toml", path_buf.display()),
    None => panic!("can not found '.droolmaw.toml' in home directory."),
  };
  let mut file = File::open(path).expect("file not found");
  let mut file_contents = String::new();
  file.read_to_string(&mut file_contents).expect("something went wrong reading the file");

  let toml: Result<Droolmaw, toml::de::Error> = toml::from_str(file_contents.as_str());

  match toml {
    Ok(content) => {
      let args: Vec<String> = env::args().collect();

      match args[args.len() - 1].as_str() {
        "--2" => l_prompt2::run(content.l_separator.as_str(), content.l_components2),
        "--right" => r_prompt::run(content.r_separator.as_str(), content.r_components),
        "--version" => println!("{}", VERSION),
        _ => l_prompt::run(content.l_separator.as_str(), content.l_components_first, content.l_components_second),
      }
    }
    Err(e) => panic!("fail to parse toml: {}", e),
  }
}
