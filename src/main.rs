mod core;

use crate::core::assets::SCRIPTFILES;

fn main() {
  for (name, value) in SCRIPTFILES.iter() {
    println!("Found file: {} Content length: {}", name, value.len());
  }

  core::Game::new().run();
}