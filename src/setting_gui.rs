use iced::widget::{column, text, Column};
use iced::Task;

#[derive(Debug)]
enum OriginalTask {}

#[derive(Debug, Clone)]
struct SettingGUI(String);

impl Default for SettingGUI {
  fn default() -> Self {
    Self("Setting Droolmaw".to_string())
  }
}

pub fn run() -> iced::Result {
  iced::run("Setting Droolmaw", update, view)
}

fn update(_setting_gui: &mut SettingGUI, _task: OriginalTask) -> impl Into<Task<OriginalTask>> {
  Task::none()
}

fn view(_setting_gui: &SettingGUI) -> Column<OriginalTask> {
  column![text(_setting_gui.0.clone())]
}
