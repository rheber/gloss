# README

Gloss is a command-line program for fetching and caching definitions from
the Oxford Dictionary API.

## Preparation

Go to https://developer.oxforddictionaries.com and make an account.
Then create a file called "keys.txt" looking like this:

```
base_url=https://od-api.oxforddictionaries.com/api/v1/entries/en/
app_id=<your app_id>
app_key=<your app_key>
```

## Building

Run `cargo build`.

## Usage

* `cargo run -- -d foo` will print definitions of the word "foo", storing
them if necessary.

* `cargo run -- -e foo` will print etymologies of the word "foo", storing
them if necessary.

* `cargo run -- -l` will list all stored words with definitions.

* `cargo run -- -l -c 3` will list all stored, defined words in 3 columns.

* `cargo run -- -n` will list all stored words without definitions.

* `cargo run -- -f wordfile.txt` will look up each word in wordfile.txt, which
is assumed to have one word per line.

* `cargo run -- -r foo` will erase any data stored for the word "foo".
