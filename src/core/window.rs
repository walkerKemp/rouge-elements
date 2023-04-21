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

impl WindowContext {
  pub fn new() -> Self {
    let (title, width, height, fps_target) = luabindings::parse_build_file("build_settings.lua", );

    let (mut handle, thread) = raylib::init()
      .size(width, height)
      .title(&title.as_str())
      .build();

    handle.set_target_fps(fps_target as u32);

    Self {
      title: title.to_string(),
      width, height, fps_target: fps_target as u32, handle, thread
    }
  }


  pub fn render_context(&mut self) -> RaylibDrawHandle {
    self.handle.begin_drawing(&self.thread)
  }
}