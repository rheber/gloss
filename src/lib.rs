use std::env;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub fn run(mut args: env::Args) {
  args.next(); // Discard program name.
  let word = args.next().expect("Missing argument.");
  println!("{}", word);
}
