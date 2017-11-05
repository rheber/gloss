extern crate gloss;

use std::process;

fn main() {
  let matches = gloss::new_app();
  let word = matches.value_of("headword").unwrap();

  gloss::run(&word[..]).unwrap_or_else(|err| {
    eprintln!("Error: {}", err);
    process::exit(1);
  });
}
