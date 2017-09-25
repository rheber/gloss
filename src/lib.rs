use std::env;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub fn getWord(mut args: env::Args) -> Result<String, &'static str> {
  args.next(); // Discard program name.

  args.next().ok_or("Missing headword argument")
}

pub fn run(word: String) {
  println!("{}", word);
}
