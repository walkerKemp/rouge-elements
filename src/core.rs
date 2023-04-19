pub mod window;
pub mod scene;
pub mod game_object;
pub mod assets;
pub mod luabindings;

use game_object::GameObject;
use window::WindowContext;
use raylib::prelude::*;

pub struct Game {
  pub window_context: WindowContext,
}

impl Game {
  pub fn new() -> Self {
    let window_context = WindowContext::new("Alchemy", 1280, 720, 144);

    Self {
      window_context
    }
  }

  fn on_update(&mut self, delta_time: f32) {
  }

  fn on_render(&mut self) {
    let mut context = self.window_context.render_context();
    context.clear_background(Color::BLACK);
    context.draw_fps(4, 4);
  }

  pub fn run(&mut self) {
    while !self.window_context.handle.window_should_close() {
      self.on_update(self.window_context.handle.get_frame_time());
      self.on_render();
    }
  }
}
