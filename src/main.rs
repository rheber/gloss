extern crate gloss;

use std::env;
use std::process;

fn main() {
  let word = gloss::getWord(env::args()).unwrap_or_else(|err| {
    eprintln!("Error: {}", err);
    process::exit(1);
  });
  gloss::run(word);
}
