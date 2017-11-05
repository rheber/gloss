extern crate clap;
use clap::{App, Arg, ArgMatches};

extern crate reqwest;
use reqwest::{Client, Response, Url};

#[macro_use] extern crate hyper;
use hyper::header::Headers;

extern crate serde_json;

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub fn new_app<'a>() -> ArgMatches<'a> {
  App::new("gloss").
       version("0.1").
       arg(Arg::with_name("headword").
                required(true).
                takes_value(true).
                index(1).
                help("word to define")).
       get_matches()
}

#[derive(Debug)]
struct GlossError {
  err_string: String
}

impl fmt::Display for GlossError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "GlossError")
  }
}

impl Error for GlossError {
  fn description(&self) -> &str {
    &self.err_string[..]
  }
}

fn read_file(filename: &str) -> Result<String, io::Error> {
  let mut file = File::open(filename)?;
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

// Submit API request and return response.
fn get_response(word: &str) -> Result<Response, Box<Error>> {
  let key_string = read_file("keys.txt")?;
  let keys = parse_keys(&key_string[..])?;

  let base_url = keys.get("base_url=").ok_or("Missing base_url key")?;
  let app_key = keys.get("app_key=").ok_or("Missing app_key key")?;
  let app_id = keys.get("app_id=").ok_or("Missing app_id key")?;
  let url = (String::from(*base_url) + word).parse::<Url>().unwrap();

  header! { (Appkey, "app_key") => [String] }
  header! { (Appid, "app_id") => [String] }
  let mut heads = Headers::new();
  heads.set(Appkey(String::from(*app_key)));
  heads.set(Appid(String::from(*app_id)));

  let resp = Client::new()?.
    get(url)?.
    headers(heads).
    send();

  match resp {
    Err(e) => Err(Box::new(e)),
    Ok(a) => Ok(a),
  }
}

// Read map of glosses from serialised string.
fn read_glosses(text: &str) -> Result<HashMap<&str, String>, Box<Error>> {
  serde_json::from_str(text).or({
    Ok(HashMap::new())
  })
}

// Write map of glosses to file.
fn save_glosses(glosses: HashMap<&str, String>) -> Result<(), Box<Error>> {
  let serial = serde_json::to_string(&glosses)?;
  let mut gloss_file = File::create("glosses")?;
  gloss_file.write_all(serial.as_bytes())?;

  Ok(())
}

// Retrieve gloss from map, requesting and inserting it if absent.
fn get_gloss<'a, 'b>(word: &'b str, glosses: &'a mut HashMap<&'b str, String>) ->
  Result<&'a String, Box<Error>> {
  // Used even though we know a gloss exists to satisfy types.
  let impossible_error =
    Box::new(GlossError {err_string: String::from("Expected gloss in map.")});

  if glosses.contains_key(word) {
    return glosses.get(word).ok_or(impossible_error);
  }
  let mut resp = get_response(word)?;
  let mut content = String::new();
  resp.read_to_string(&mut content)?;
  glosses.insert(word, content);

  glosses.get(word).ok_or(impossible_error)
}

pub fn run(word: &str) -> Result<(), Box<Error>> {
  let glosses_result: Result<String, &'static str> = read_file("glosses").or({
    File::create("glosses")?;
    Ok(String::new())
  });
  let glosses_unwrapped = glosses_result.unwrap();
  let mut glossmap = read_glosses(&glosses_unwrapped[..])?;
{
  let resp = get_gloss(word, &mut glossmap)?;
  println!("{}", resp);
}
  save_glosses(glossmap)?;

  Ok(())
}
