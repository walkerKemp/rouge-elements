use std::error::Error;

use super::window::WindowContextBuilder;
use super::assets::SCRIPTFILES;
use hlua::Lua;

pub fn parse_build_file(file_name: &str, window_builder: &mut WindowContextBuilder) -> Result<(), String> {
  let mut lua = Lua::new();

  let file_contents = SCRIPTFILES.get(file_name).expect("Unable to find build file in binary");

  window_builder.title = lua.get("title");
  window_builder.width = lua.get("width");
  window_builder.height = lua.get("height");
  window_builder.fps_target = lua.get("fps_target");

  lua.execute::<()>(file_contents).unwrap();

  Ok(())
}