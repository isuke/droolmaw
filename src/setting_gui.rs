use egui::{vec2, Frame, Label};
use egui_flex::{item, Flex, FlexAlignContent};

use eframe::egui::{self};

const WINDOW_WIDTH: f32 = 320.0;
const WINDOW_HEIGHT: f32 = 240.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Location {}

#[derive(Default)]
struct SettingGUI {}

pub fn run() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
    ..Default::default()
  };
  eframe::run_native("Setting droolmaw", options, Box::new(|_cc| Ok(Box::<SettingGUI>::default())))
}

impl eframe::App for SettingGUI {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      let ui_available_size = ui.available_size();

      ui.heading("Setting droolmaw");
      ui.label("drag and drop");

      let frame = Frame::default().inner_margin(4.0);

      Flex::horizontal().show(ui, |flex| {
        flex.add(item(), Label::new("id"));
        flex.add(item(), Label::new("current_directory_name"));
        flex.add(item(), Label::new("current/directory/path"));
        flex.add(item(), Label::new("git_branch"));
        flex.add(item(), Label::new("origin +1 -1"));
        flex.add(item(), Label::new("ruby 3.3.0 node 20.11.0"));
      });

      let (_, dropped_payload) = ui.dnd_drop_zone::<Location, ()>(frame, |dnd_drop_zone_ui| {
        dnd_drop_zone_ui.set_min_size(ui_available_size);
      });
    });
  }
}
