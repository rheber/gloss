extern crate gloss;

use std::error::Error;
use std::process;

fn err_exit(err_msg: Box<Error>) {
  eprintln!("Error: {}", err_msg);
  process::exit(1);
}

fn main() {
  let matches = gloss::new_app();

  let maybe_word : Option<&str> = matches.value_of("headword");
  maybe_word.and_then::<Option<()>, _>(|word| {
    gloss::define_one(&word[..], &matches).unwrap_or_else(|err| {
      err_exit(err);
    });
    // At this point we have successfully defined a word.
    process::exit(0);
  });

  let maybe_file : Option<&str> = matches.value_of("file");
  maybe_file.and_then::<Option<()>, _>(|filename| {
    gloss::define_list(&filename[..]).unwrap_or_else(|err| {
      err_exit(err);
    });
    // At this point we have successfully defined each word.
    process::exit(0);
  });

  let amt_columns : Option<&str> = matches.value_of("columns");
  if matches.is_present("lexemes") {
    gloss::list_lexemes(false, amt_columns).unwrap_or_else(|err| {
      err_exit(err);
    });
    process::exit(0);
  }
  if matches.is_present("nonlexemes") {
    gloss::list_lexemes(true, amt_columns).unwrap_or_else(|err| {
      err_exit(err);
    });
    process::exit(0);
  }

  let maybe_rem : Option<&str> = matches.value_of("remove");
  maybe_rem.and_then::<Option<()>, _>(|word| {
    gloss::remove_lexeme(&word[..]).unwrap_or_else(|err| {
      err_exit(err);
    });
    // At this point we have successfully removed the word.
    process::exit(0);
  });


  // Did not perform any action.
  eprintln!("{}", matches.usage());
  process::exit(1);
}
