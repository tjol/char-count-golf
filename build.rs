use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use superslice::Ext;
use unicode_normalization::char::decompose_compatible;

const BANNED: &'static [char] = &['\u{1F14D}'];
const MAGIC_PUNCUATION: &'static [char] = &['.', ',', '/'];
const DIGITS: &'static [char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn build_composition_database(lowercase: bool) -> Vec<(Vec<char>, char)> {
    let mut db = vec![];
    for c in '\u{0080}'..'\u{10FFFF}' {
        if BANNED.contains(&c) {
            continue;
        }
        let mut s = String::new();
        decompose_compatible(c, |c2| {
            s.push(c2);
        });
        let chars: Vec<char> = if lowercase {
            s.to_lowercase().chars().collect()
        } else {
            s.chars().collect()
        };
        if chars.len() != 1 {
            db.push((chars, c));
        }
    }
    db.sort();
    db.dedup_by_key(|item| item.0.clone());

    db
}

fn ignore_punctuation(db: &mut Vec<(Vec<char>, char)>) {
    let mut new_entries = HashMap::new();

    // looping over the db multiple times in order to prioritize CO. over c/o
    // there are probably smarter ways of doing this.
    for c in MAGIC_PUNCUATION {
        for item in db.iter() {
            let seq = &item.0;
            if !DIGITS.contains(&seq[0]) && seq.contains(c) {
                let without_punct: Vec<char> = seq
                    .clone()
                    .into_iter()
                    .filter(|c| !MAGIC_PUNCUATION.contains(c))
                    .collect();
                if without_punct.len() < seq.len() {
                    if !new_entries.contains_key(&without_punct) {
                        new_entries.insert(without_punct, item.1);
                    }
                }
            }
        }
    }

    let new_entries: Vec<_> = new_entries
        .into_iter()
        .filter(|item| db.lower_bound_by_key(&item.0, |item| item.0.clone()) < db.len())
        .collect();

    db.extend_from_slice(&new_entries);
    db.sort();
    db.dedup_by_key(|item| item.0.clone());
}

fn write_struct<W: Write>(mut outf: W, maxlen: usize) -> std::io::Result<()> {
    writeln!(
        outf,
        "#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]"
    )?;
    writeln!(outf, "struct Composition {{")?;
    writeln!(outf, "    short: char,")?;
    writeln!(outf, "    len: usize,")?;
    writeln!(outf, "    long: [char; {}]", maxlen)?;
    writeln!(outf, "}}")?;
    Ok(())
}

fn write_compdb<W: Write>(
    mut outf: W,
    name: &str,
    compdb: &[(Vec<char>, char)],
    maxlen: usize,
) -> std::io::Result<()> {
    writeln!(outf, "const {}: [Composition; {}] = [", name, compdb.len())?;
    for comp in compdb {
        let mut long_extended = comp.0.clone();
        long_extended.resize(maxlen, '\0');
        writeln!(
            outf,
            "    Composition {{ short: {:?}, len: {}, long: {:?} }},",
            comp.1,
            comp.0.len(),
            long_extended
        )?;
    }
    writeln!(outf, "];")?;
    Ok(())
}

fn main() {
    // let compdb_pedantic = build_composition_database(false);
    let compdb_lowercase = build_composition_database(true);
    let mut compdb_depunctuated = compdb_lowercase.clone();
    ignore_punctuation(&mut compdb_depunctuated);

    // How long can decomposed sequences be?
    let maxlen = compdb_lowercase
        .iter()
        .map(|comp| comp.0.len())
        .max()
        .unwrap();

    // Write the code to regenerate the database statically
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let mut outf = File::create(Path::new(&out_dir).join("compdb.rs")).unwrap();

    write_struct(&mut outf, maxlen).unwrap();
    // write_compdb(&mut outf, "COMPOSITION_DATABASE", &compdb_lowercase, maxlen).unwrap();
    // write_compdb(
    //     &mut outf,
    //     "COMPOSITION_DATABASE_PEDANTIC",
    //     &compdb_pedantic,
    //     maxlen,
    // )
    // .unwrap();
    write_compdb(
        &mut outf,
        "COMPOSITION_DATABASE_DEPUNCTUATED",
        &compdb_depunctuated,
        maxlen,
    )
    .unwrap();
}
