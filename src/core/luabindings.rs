use super::assets::SCRIPTFILES;
use hlua::Lua;

pub fn parse_build_file(file_name: &str) -> (String, i32, i32, i32) {
  println!("Loading Lua Build Settings");

  let mut lua = Lua::new();
  lua.openlibs();

  let file_contents = SCRIPTFILES.get(file_name).expect("Unable to find build file in binary");
  lua.execute::<()>(&file_contents).unwrap();

  let title = lua.get("title").unwrap();
  let width = lua.get("width").unwrap();
  let height = lua.get("height").unwrap();
  let fps_target = lua.get("fps_target").unwrap();

  (title, width, height, fps_target)
}