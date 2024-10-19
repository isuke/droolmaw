use dioxus::prelude::*;

use crate::segment::Color;

#[derive(Debug, Clone, Copy)]
struct Separator<'a> {
  id: &'a str,
  l: &'a str,
  r: &'a str,
}

#[derive(Debug, Clone, Copy)]
struct Component<'a> {
  id: &'a str,
  sample: &'a str,
}

#[derive(Debug, Clone, Copy)]
struct Segment<'a> {
  component: Component<'a>,
  color: Color,
}

pub fn run() {
  launch(setting_gui);
}

fn setting_gui() -> Element {
  //// CONST ////
  let mut separator_list: Vec<Separator> = Vec::new();
  separator_list.push(Separator {
    id: "hard_divider",
    l: "",
    r: "",
  });
  separator_list.push(Separator {
    id: "triangle1",
    l: "",
    r: "",
  });
  separator_list.push(Separator {
    id: "triangle2",
    l: "",
    r: "",
  });
  separator_list.push(Separator {
    id: "half_circle_thick",
    l: "",
    r: "",
  });
  separator_list.push(Separator {
    id: "flame_thick",
    l: "",
    r: "",
  });
  separator_list.push(Separator {
    id: "ice_waveform",
    l: "",
    r: "",
  });

  let mut component_list = Vec::<Component>::new();
  component_list.push(Component { id: "id", sample: " id" });
  component_list.push(Component {
    id: "date_time",
    sample: "2024/01/13 12:31",
  });
  component_list.push(Component {
    id: "dir",
    sample: "󰉖 current_directory",
  });
  component_list.push(Component {
    id: "dir_path",
    sample: "󰉋 current_path",
  });
  component_list.push(Component {
    id: "git_name",
    sample: "󰏪 git author",
  });
  component_list.push(Component {
    id: "git_current_branch",
    sample: "󰘬 git-branch",
  });
  component_list.push(Component {
    id: "git_current_branch_and_statuses",
    sample: "󰲁 origin",
  });
  component_list.push(Component {
    id: "langs",
    sample: "ruby 3.3.0 node 20.11.0",
  });
  //// CONST END ////

  let mut current_separator = use_signal(|| separator_list[0].clone());

  let mut is_active_separator_select = use_signal(|| false);

  let mut current_l1_segments = use_signal(|| Vec::<Segment>::new());
  current_l1_segments.push(Segment {
    component: component_list[0],
    color: Color::Magenta,
  });
  current_l1_segments.push(Segment {
    component: component_list[1],
    color: Color::Blue,
  });
  current_l1_segments.push(Segment {
    component: component_list[2],
    color: Color::Yellow,
  });

  let mut current_r_segments = use_signal(|| Vec::<Segment>::new());
  current_r_segments.push(Segment {
    component: component_list[2],
    color: Color::Magenta,
  });
  current_r_segments.push(Segment {
    component: component_list[6],
    color: Color::Blue,
  });

  rsx! {
    link { rel: "stylesheet", href: "destyle.css" }
    link { rel: "stylesheet", href: "setting_gui.css" }

    div {
      class: "Main",

      h1 { class: "title", "Droolmaw Setting" }

      form {
        class: "separator_select",
        h2 { class: "description", "Select Separator" }
        label {
          class: "current_select",
          onclick: move |_| { is_active_separator_select.set((*is_active_separator_select)() != true) },
          LComponent { separator: "{(*current_separator)().l}", string: "sample", color: Color::Magenta },
          RComponent { separator: "{(*current_separator)().r}", string: "sample", color: Color::Magenta },
        }

        if (*is_active_separator_select)() {
          ul {
            class: "options",
            for separator in separator_list {
              li {
                class: "option",
                onclick: move |_| { current_separator.set(separator); is_active_separator_select.set(false) },
                LComponent { separator: "{separator.l}", string: "sample", color: Color::Magenta },
                RComponent { separator: "{separator.r}", string: "sample", color: Color::Magenta },
              }
            }
          }
        }
      }

      div {
        class: "component_select",
        h2 { class: "description", "Edit Segments" }

        div {
          class: "segments",
          ul {
            class: "left_segments",
            for segment in current_l1_segments.iter() {
              li {
                class: "segment",
                LComponent { separator: (*current_separator)().l, string: segment.component.sample, color: segment.color },
                button {
                  class: "edit_button",
                  "Edit"
                }
                button {
                  class: "remove_button",
                  "Remove"
                }
              }
            }
            li {
              class: "add",
              button {
                class: "add_button",
                "Add"
              }
            }
          },
          ul {
            class: "right_segments",
            for segment in current_r_segments.iter() {
              li {
                class: "segment",
                RComponent { separator: (*current_separator)().r, string: segment.component.sample, color: segment.color },
                button {
                  class: "edit_button",
                  "Edit"
                }
                button {
                  class: "remove_button",
                  "Remove"
                }
              }
            }
            li {
              class: "add",
              button {
                class: "add_button",
                "Add"
              }
            }
          },
        }
      }

      div {
        class: "result",
        ul {
          class: "l_prompt",
          for segment in current_l1_segments.iter() {
            li {
              class: "segment",
              LComponent { separator: (*current_separator)().l, string: segment.component.sample, color: segment.color },
            }
          }
        }
      }
    }
  }
}

#[component]
fn LComponent(separator: String, string: String, color: Color) -> Element {
  let ft_color = match color {
    Color::_Black => "white",
    Color::Blue => "white",
    Color::_Cyan => "white",
    Color::Green => "white",
    Color::Magenta => "white",
    Color::_Red => "white",
    Color::White => "black",
    Color::Yellow => "black",
    _ => panic!("Color::Nothing can not display."),
  };

  rsx! {
    span {
      class: "LComponent",
      span { class: "before_separator", background_color: format!("{}", color), color: "#2c2c2c", "{separator}" }
      span { class: "text", background_color: format!("{}", color), color: "{ft_color}", " {string} " }
      span { class: "after_separator", background_color: "#2c2c2c", color: format!("{}", color), "{separator}" }
    }
  }
}

#[component]
fn RComponent(separator: String, string: String, color: Color) -> Element {
  let ft_color = match color {
    Color::_Black => "white",
    Color::Blue => "white",
    Color::_Cyan => "white",
    Color::Green => "white",
    Color::Magenta => "white",
    Color::_Red => "white",
    Color::White => "black",
    Color::Yellow => "black",
    _ => panic!("Color::Nothing can not display."),
  };

  rsx! {
    span {
      class: "RComponent",
      span { class: "before_separator", background_color: "#2c2c2c", color: format!("{}", color), "{separator}" }
      span { class: "text", background_color: format!("{}", color),  color: "{ft_color}", " {string} " }
      span { class: "after_separator", background_color: format!("{}", color), color: "#2c2c2c", "{separator}" }    }
  }
}
