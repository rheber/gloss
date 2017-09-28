extern crate reqwest;
use reqwest::{Client, Request, Url};

use std::collections::HashMap;
use std::env;
use std::error::Error;
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

fn parse_key_line(key_line: &str) -> Result<(&str, &str), &'static str> {
  let idx = key_line.find('=').ok_or("Failed to parse keys file")?;

  Ok(key_line.split_at(idx+1))
}

fn parse_keys(key_string: &str) -> Result<HashMap<&str, &str>, &'static str> {
  let pairs = key_string.lines();
  let results: Vec<Result<(&str, &str), _>> = pairs.map(parse_key_line).collect();
  let mut keys = HashMap::new();

  for r in results {
    match r {
      Err(e) => return Err(e),
      Ok((k, v)) => keys.insert(k, v),
    };
  };

  Ok(keys)
}

fn construct_request(word: &str, keys: HashMap<&str, &str>) ->
  Result<Request, Box<Error>> {
  let base_url = keys.get("base_url=").ok_or("Missing base_url key")?;
  let app_key = keys.get("app_key=").ok_or("Missing app_key key")?;
  let app_id = keys.get("app_id=").ok_or("Missing app_id key")?;
  let url = (String::from(*base_url) + word).parse::<Url>().unwrap();

  let req = Client::new()?.
    post(url)?.
    form(&[("app_key", *app_key), ("app_id", *app_id)])?.
    build();
  Ok(req)
}

pub fn run(word: &str) -> Result<(), Box<Error>> {
  println!("{}", word);
  let key_string = read_file()?;
  let key_pairs = parse_keys(&key_string[..])?;
  let req = construct_request(word, key_pairs);
  println!("{:?}", req);

  Ok(())
}
