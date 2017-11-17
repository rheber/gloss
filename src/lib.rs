extern crate clap;
use clap::{App, Arg, ArgMatches};

extern crate reqwest;
use reqwest::{Client, Response, Url};

#[macro_use] extern crate hyper;
use hyper::header::Headers;

extern crate serde_json;

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::str;
use std::vec::Vec;

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
                takes_value(true).
                index(1).
                help("word to define")).
       arg(Arg::with_name("columns").
                short("c").
                long("columns").
                default_value("1").
                help("amount of columns when listing lexemes")).
       arg(Arg::with_name("definitions").
                short("d").
                long("definitions").
                help("just show the definitions part of the JSON")).
       arg(Arg::with_name("etym").
                short("e").
                long("etym").
                help("just show the etymologies part of the JSON")).
       arg(Arg::with_name("file").
                short("f").
                long("file").
                takes_value(true).
                help("file containing words to define, one word per line")).
       arg(Arg::with_name("lexemes").
                short("l").
                long("lexemes").
                help("list stored words which have definitions")).
       arg(Arg::with_name("nonlexemes").
                short("n").
                long("nonlexemes").
                help("list stored words which do not have definitions")).
       arg(Arg::with_name("remove").
                takes_value(true).
                short("r").
                long("remove").
                help("erase any data stored for a word")).
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

fn parse_keys(key_string: &str) -> Result<BTreeMap<&str, &str>, &'static str> {
  let pairs = key_string.lines();
  let results: Vec<Result<(&str, &str), _>> = pairs.map(parse_key_line).collect();
  let mut keys = BTreeMap::new();

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
fn read_glosses(text: &str) ->
  Result<BTreeMap<String, Option<String>>, Box<Error>> {
  serde_json::from_str(text).or({
    Ok(BTreeMap::new())
  })
}

// Write map of glosses to file.
fn save_glosses(glosses: BTreeMap<String, Option<String>>) ->
  Result<(), Box<Error>> {
  let serial = serde_json::to_string(&glosses)?;
  let mut gloss_file = File::create("glosses")?;
  gloss_file.write_all(serial.as_bytes())?;

  Ok(())
}

// Request gloss and insert into map.
fn get_new_gloss<'a>(word: String,
  glosses: &'a mut BTreeMap<String, Option<String>>) ->
  Result<String, Box<Error>> {
  // Used even though we know a gloss exists to satisfy types.
  let impossible_error =
    Box::new(GlossError {err_string: String::from("Expected gloss in map.")});

  let mut resp = get_response(&word[..])?;
  if resp.status().is_success() {
    let mut content = String::new();
    resp.read_to_string(&mut content)?;
    let new_entry = Some(content);
    glosses.insert(word, new_entry.clone());

    new_entry.ok_or(impossible_error)
  } else {
    glosses.insert(word, None);

    Ok(String::from("Not defined"))
  }
}

fn potentially_create_glossfile() -> Result<String, Box<Error>> {
  read_file("glosses").or({
    OpenOptions::new().append(true).create(true).open("glosses")?;
    Ok(String::new())
  })
}

// If non is true then print undefined words.
pub fn list_lexemes(non: bool, amt_columns: Option<&str>) ->
  Result<(), Box<Error>> {
  let glosses_result = potentially_create_glossfile();
  let glosses_unwrapped = glosses_result.unwrap();
  let glossmap = read_glosses(&glosses_unwrapped[..])?;

  let amt_str = amt_columns.unwrap_or("1");
  let amt_int : usize = amt_str.trim().parse()?;
  let columns = if amt_int > 1 {amt_int} else {1};
  let mut i = 0;

  if non {
    for (word, def) in glossmap {
      if let None = def {
        print!("{:15}\t", word);
        i = i + 1;
        if i % columns == 0 {
          println!();
        }
      }
    }
  } else {
    for (word, def) in glossmap {
      if let Some(_) = def {
        print!("{:15}\t", word);
        i = i + 1;
        if i % columns == 0 {
          println!();
        }
      }
    }
  }

  Ok(())
}

pub fn remove_lexeme(word: &str) -> Result<(), Box<Error>> {
  let glosses_result = potentially_create_glossfile();
  let glosses_unwrapped = glosses_result.unwrap();
  let mut glossmap = read_glosses(&glosses_unwrapped[..])?;

  glossmap.remove(word).ok_or("No data was stored for that word.")?;
  save_glosses(glossmap)?;

  Ok(())
}

fn list_from_json<'a>(j: &'a serde_json::Value, needle: String) ->
  Result<String, &'static str> {
  let lex_entries = j.pointer("/results/0/lexicalEntries").
    ok_or("Not defined")?;
  let empty_vec = Vec::new();
  let entries = (*lex_entries).as_array().unwrap_or(&empty_vec);
  let mut s = String::from("");

  if needle == "etymologies" {
    for entry in entries {
      let etym_maybe = entry.pointer("/entries/0/etymologies");
      if let Some(arr) = etym_maybe {
        for etym in (*arr).as_array().unwrap_or(&empty_vec) {
          let etym_formed = format!("* {}\n", etym.as_str().unwrap_or(""));
          s.push_str(&etym_formed[..]);
        }
      }
    }
    return Ok(s);
  }

  for entry in entries {
    let def_maybe = entry.pointer("/entries/0/senses/0/definitions");
    if let Some(defs) = def_maybe {
      for def in (*defs).as_array().unwrap_or(&empty_vec) {
        let def_formed = format!("* {}\n", def.as_str().unwrap_or(""));
        s.push_str(&def_formed[..]);
      }
    }
  }

  Ok(s)
}

pub fn define_one(word: &str, matches: &ArgMatches) -> Result<(), Box<Error>> {
  let glosses_result = potentially_create_glossfile();
  let glosses_unwrapped = glosses_result.unwrap();
  let mut glossmap = read_glosses(&glosses_unwrapped[..])?;
  let cloned = glossmap.clone();
{
  let resp: String = match cloned.get(word) {
    Some(entry) => match entry {
      &Some(ref def) => def.clone(),
      &None => String::from("Not defined.")
    },
    None => get_new_gloss(word.to_string(), &mut glossmap)?
  };
  if matches.is_present("definitions") {
    let j = serde_json::from_str(&resp[..]).or(Err("Not defined."))?;
    let definitions = list_from_json(&j, String::from("definitions"))?;
    println!("{}", definitions);
  } else if matches.is_present("etym") {
    let j = serde_json::from_str(&resp[..]).or(Err("Not defined."))?;
    let etym = list_from_json(&j, String::from("etymologies"))?;
    println!("{}", etym);
  } else {
    println!("{}", resp);
  }
}
  save_glosses(glossmap)?;

  Ok(())
}

pub fn define_list(filename: &str) -> Result<(), Box<Error>> {
  let glosses_result = potentially_create_glossfile();
  let glosses_unwrapped = glosses_result.unwrap();
  let mut glossmap = read_glosses(&glosses_unwrapped[..])?;
  let cloned = glossmap.clone();

  let wordfile = read_file(filename)?;
  let wordlist : str::Lines = wordfile.lines();

  for word in wordlist {
    let def_opt = cloned.get(word);
    match def_opt {
      Some(_) => String::from("Already defined"),
      None => get_new_gloss(word.to_string(), &mut glossmap)?
    };
  }
  save_glosses(glossmap)?;

  Ok(())
}
