use raylib::prelude::RaylibDrawHandle;

type EntityId = usize;

pub trait GameEvent {
  fn on_create(&mut self) {}
  fn on_enter(&mut self);
  fn on_update(&mut self, delta_time: f32);
  fn on_render(&self, render_context: &mut RaylibDrawHandle);
  fn on_exit(&mut self);
}


pub struct ObjectManager {
  pub objects: Vec<Box<dyn GameEvent>>,
}

impl ObjectManager {
  pub fn new() -> Self {
    Self {
      objects: Vec::new(),
    }
  }

  pub fn insert_object(&mut self, mut object: Box<dyn GameEvent>) -> EntityId {
    object.on_create();
    let id = self.objects.len();
    self.objects.push(object);
    id
  }
}