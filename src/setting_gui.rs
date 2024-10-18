use dioxus::prelude::*;

pub fn run() {
  launch(setting_gui);
}

fn setting_gui() -> Element {
  let mut count = use_signal(|| 0);

  rsx! {
    h1 { "High-Five counter: {count}" }
    button { onclick: move |_| count += 1, "Up high!" }
    button { onclick: move |_| count -= 1, "Down low!" }
  }
}
