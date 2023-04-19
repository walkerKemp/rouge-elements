use raylib::prelude::*;

pub struct WindowContext {
  pub title: String,
  pub width: i32,
  pub height: i32,
  pub fps_target: u32,
  pub handle: RaylibHandle,
  pub thread: RaylibThread,
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

  pub fn render_context(&mut self) -> RaylibDrawHandle {
    self.handle.begin_drawing(&self.thread)
  }
}