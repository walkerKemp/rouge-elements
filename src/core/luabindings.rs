use super::assets::SCRIPTFILES;
use hlua::Lua;

pub fn parse_build_file(file_name: &str) -> (String, i32, i32, i32) {
  println!("Loading {}", &file_name);

  let mut lua = Lua::new();
  lua.openlibs();

  let file_contents = SCRIPTFILES.get(file_name).expect("Unable to find build file in binary");
  lua.execute::<()>(&file_contents).unwrap();

  let title = lua.get("title").expect("Unable to find title");
  let width = lua.get("width").expect("Unable to find width");
  let height = lua.get("height").expect("Unable to find height");
  let fps_target = lua.get("fps_target").expect("Unable to find fps_target");

  (title, width, height, fps_target)
}
