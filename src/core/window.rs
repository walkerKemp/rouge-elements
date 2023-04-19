use raylib::prelude::*;
use super::luabindings;

const DEFAULTTITLE: &'static str = "Title";

pub struct WindowContext {
  pub title: String,
  pub width: i32,
  pub height: i32,
  pub fps_target: u32,
  pub handle: RaylibHandle,
  pub thread: RaylibThread,
}

pub struct WindowContextBuilder {
  pub title: Option<String>,
  pub width: Option<i32>,
  pub height: Option<i32>,
  pub fps_target: Option<u32>,
}

impl WindowContextBuilder {
  fn build(&mut self) -> WindowContext {

    let title = self.title.unwrap_or(String::from("Default"));
    let width = self.width.unwrap_or(640);
    let height = self.height.unwrap_or(480);
    let fps_target = self.fps_target.unwrap_or(30);

    WindowContext::new(title.as_str(), width, height, fps_target)
  }
}

impl WindowContext {
  pub fn new(title: &str, width: i32, height: i32, fps_target: u32) -> Self {
    let (mut handle, thread) = raylib::init()
      .size(width, height)
      .title(title)
      .build();

    handle.set_target_fps(fps_target);

    Self {
      title: title.to_string(),
      width, height, fps_target, handle, thread
    }
  }

  pub fn from_build_file() -> Self {
    let mut wc_builder = WindowContextBuilder { 
      title: None,
      width: None,
      height: None,
      fps_target: None,
    };

    luabindings::parse_build_file("build_settings.lua", &mut wc_builder);

    wc_builder.build()
  }

  pub fn render_context(&mut self) -> RaylibDrawHandle {
    self.handle.begin_drawing(&self.thread)
  }
}