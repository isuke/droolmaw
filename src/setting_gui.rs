use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

pub fn run() {
  launch(setting_gui);
}

#[derive(Debug, Clone, Copy)]
struct Separator<'a> {
  id: &'a str,
  l: &'a str,
  r: &'a str,
}

fn setting_gui() -> Element {
  let mut separator_list: Vec<Separator> = Vec::new();
  separator_list.push(Separator {
    id: "hard_divider",
    l: "",
    r: "",
  });
  separator_list.push(Separator {
    id: "id-hard_divider",
    l: "",
    r: "",
  });
  separator_list.push(Separator {
    id: "id-triangle1",
    l: "",
    r: "",
  });
  separator_list.push(Separator {
    id: "id-triangle2",
    l: "",
    r: "",
  });
  separator_list.push(Separator {
    id: "id-half_circle_thick",
    l: "",
    r: "",
  });
  separator_list.push(Separator {
    id: "id-flame_thick",
    l: "",
    r: "",
  });
  separator_list.push(Separator {
    id: "id-ice_waveform",
    l: "",
    r: "",
  });

  let mut current_separator = use_signal(|| separator_list[0].clone());

  let mut is_active_separator_select = use_signal(|| false);

  rsx! {
    link { rel: "stylesheet", href: "destyle.css" }
    link { rel: "stylesheet", href: "setting_gui.css" }

    div {
      class: "main",

      h1 { class: "title", "Droolmaw Setting" }

      form {
        class: "separator_select",
        p { class: "description", "Select Separator" }
        label {
          class: "current_select",
          onclick: move |_| { is_active_separator_select.set((*is_active_separator_select)() != true) },
          LComponent { separator: "{(*current_separator)().l}", string: "sample" },
          RComponent { separator: "{(*current_separator)().r}", string: "sample" },
        }

        if (*is_active_separator_select)() {
          ul {
            class: "options",
            for separator in separator_list {
              li {
                class: "option",
                onclick: move |_| { current_separator.set(separator); is_active_separator_select.set(false) },
                LComponent { separator: "{separator.l}", string: "sample" },
                RComponent { separator: "{separator.r}", string: "sample" },
              }
            }
          }
        }
      }

      p { class: "description", "Drag and Drop" }
      // ul {
      //   class: "components_list",
      //   li { class: "component", LComponent { separator: current_separator.l, string: " id" } }
      //   li { class: "component", LComponent { separator: current_separator.l, string: "2024/01/13" } }
      //   li { class: "component", LComponent { separator: current_separator.l, string: "󰉖 current_directory" } }
      //   li { class: "component", LComponent { separator: current_separator.l, string: "current_path" } }
      //   li { class: "component", LComponent { separator: current_separator.l, string: "󰏪 git author" } }
      //   li { class: "component", LComponent { separator: current_separator.l, string: "󰘬 git-branch" } }
      //   li { class: "component", LComponent { separator: current_separator.l, string: "origin" } }
      //   li { class: "component", LComponent { separator: current_separator.l, string: "ruby 3.3.0 node 20.11.0" } }
      // }
    }
  }
}

// #[component]
// fn SeparatorSelectOption(id: String, l_separator: String, r_separator: String, onclick: EventHandler<MouseEvent>) -> Element {
// fn SeparatorSelectOption(id: String, l_separator: String, r_separator: String) -> Element {
//   rsx! {
//     li {
//       class: "item",
//       input { class: "radio", r#type: "radio", name: "separator_select", id: "{id}", onclick: move |event|  }
//       label {
//         class: "label",
//         r#for: "{id}",
//         LComponent { separator: "{l_separator}", string: "sample" }
//         RComponent { separator: "{r_separator}", string: "sample" }
//         }
//     }
//   }
// }

#[component]
fn LComponent(separator: String, string: String) -> Element {
  rsx! {
    span {
      class: "LComponent",
      span { class: "before_separator", "{separator}" }
      span { class: "text", " {string} " }
      span { class: "after_separator", "{separator}" }
    }
  }
}

#[component]
fn RComponent(separator: String, string: String) -> Element {
  rsx! {
    span {
      class: "RComponent",
      span { class: "before_separator", "{separator}" }
      span { class: "text", " {string} " }
      span { class: "after_separator", "{separator}" }
    }
  }
}
