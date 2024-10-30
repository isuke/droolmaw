use std::env;

use crate::component::*;

pub fn run(separator: &str, components: Vec<Component>) {
  print!("{}", build(separator, create_segments(components)));
  print!(" ");
}

fn build(separator: &str, segments: Vec<Segment>) -> String {
  let mut result = String::from("");

  let mut index = 1;
  let mut prev_color = &Color::Nothing;
  for segment in &segments {
    let background_color = match segment.ng_color {
      Some(val) if String::from(env::var("DROOLMAW_RETVAL").unwrap()) != "0" => val,
      _ => segment.color,
    };
    let font_color = get_font_color(&background_color);

    if *prev_color != Color::Nothing {
      result = format!("{}{}", result, set_fg(&set_bg(&String::from(separator), &background_color), prev_color))
    }

    let temp = format!(" {} ", segment.string);
    result = format!("{}{}", result, set_fg(&set_bg(&temp, &background_color), &font_color));

    if segments.len() <= index {
      result = format!("{}{}", result, set_fg(&String::from(separator), &background_color))
    }

    index += 1;
    prev_color = &segment.color;
  }

  return result;
}
