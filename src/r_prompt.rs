use crate::segment::*;

pub fn run(separator: &str, components: Vec<Component>) {
  print!("{}", build(separator, create_segments(components)));
}

fn build(separator: &str, segments: Vec<Segment>) -> String {
  let mut result = String::from("");

  let mut prev_color = &Color::Nothing;
  for segment in &segments {
    let font_color = get_font_color(&segment.color);

    if *prev_color == Color::Nothing {
      result = format!("{}{}", result, set_fg(&String::from(separator), &segment.color));
    } else {
      result = format!("{}{}", result, set_fg(&set_bg(&String::from(separator), prev_color), &segment.color))
    }

    let temp = format!(" {} ", segment.string);
    result = format!("{}{}", result, set_fg(&set_bg(&temp, &segment.color), &font_color));

    prev_color = &segment.color;
  }

  return result;
}
