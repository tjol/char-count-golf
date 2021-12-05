use std::io::{stdin, BufRead, Read};

use clap::{App, Arg};

use char_count_golf::*;

fn main() {
    let arg_matches =
        App::new(env!("CARGO_BIN_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .about("Shorten your tweets with broken unicode")
            .arg(Arg::with_name("match-case").long("match-case").help(
                "Only allow substitutions that preserve case (i.e., № only for No, not for no) (implies --no-punctuation)",
            ))
            .arg(Arg::with_name("no-punctuation").long("no-punctuation").help(
                "Don't use substitutions which include punctuation, such as ℁ for as or ㏘ for pm",
            ))
            .arg(Arg::with_name("eof").long("eof").short("e").help(
                "Read until EOF (default: double newline)",
            ))
            .get_matches();

    let mode = if arg_matches.is_present("match-case") {
        ShortenMode::SameCase
    } else if arg_matches.is_present("no-punctuation") {
        ShortenMode::Normal
    } else {
        ShortenMode::WithPunctuation
    };

    // Read input

    let stdin = stdin();

    let mut s = String::new();
    if arg_matches.is_present("eof") {
        stdin.lock().read_to_string(&mut s).unwrap();
    } else {
        // Read until an empty line, or eof
        for line in stdin.lock().lines().filter_map(|r| r.ok()) {
            if line.is_empty() {
                break;
            } else {
                s += &line;
                s += "\n";
            }
        }
    }

    let short = shorten_str(&s, mode);
    print!("{}", short);
}
