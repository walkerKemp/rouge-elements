use super::Game;
use hlua::Lua;

pub struct LuaBinder<'a> {
  pub file_location: String,
  pub game: &'a Game,
}

impl<'a> LuaBinder<'a> {
  pub fn new(file_location: &str, game: &'a Game) -> Self {
    Self {
      file_location: file_location.to_string(),
      game,
    }
  }
}