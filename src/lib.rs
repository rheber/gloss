use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub fn get_word(mut args: env::Args) -> Result<String, &'static str> {
  args.next(); // Discard program name.

  args.next().ok_or("Missing headword argument")
}

fn read_file() -> Result<String, io::Error> {
  let mut file = File::open("keys.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

fn parse_keys<'a>(key_string: &'a str) -> Vec<Vec<&'a str>> {
  let pairs = key_string.lines();

  pairs.map(|line| {
    line.split('=').collect()
  }).collect()
}

pub fn run(word: &str) -> Result<(), io::Error> {
  println!("{}", word);
  let key_string = read_file()?;
  let key_pairs = parse_keys(&key_string[..]);
  println!("{:?}", key_pairs);

  Ok(())
}
