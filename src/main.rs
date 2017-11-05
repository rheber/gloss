extern crate gloss;

use std::process;

fn main() {
  let matches = gloss::new_app();
  let maybe_word : Option<&str> = matches.value_of("headword");

  maybe_word.and_then::<Option<()>, _>(|word| {
    gloss::define_one(&word[..]).unwrap_or_else(|err| {
      eprintln!("Error: {}", err);
      process::exit(1);
    });
    // At this point we have successfully defined a word.
    process::exit(0);
  });

  // Did not define anything.
  eprintln!("{}", matches.usage());
  process::exit(1);
}
