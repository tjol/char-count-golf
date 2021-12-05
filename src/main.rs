use std::io::{stdin, Read};
use std::ops::Range;

use hashbrown::HashMap;
use superslice::Ext;
use unicode_normalization::char::decompose_compatible;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Composition {
    long: Vec<char>,
    short: char,
}

fn build_composition_database() -> Vec<Composition> {
    let mut db = vec![];
    for c in '\u{0080}'..'\u{10FFFF}' {
        let mut s = String::new();
        decompose_compatible(c, |c2| {
            s.push(c2);
        });
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 1 {
            db.push(Composition {
                long: chars,
                short: c,
            });
        }
    }
    db.sort();
    db
}

fn find_range_starting(s: &[char], compdb: &[Composition]) -> Range<usize> {
    let len = s.len();
    compdb.equal_range_by_key(&s, |comp| {
        if comp.long.len() >= len {
            &comp.long[..len]
        } else {
            &[]
        }
    })
}

fn shorten(
    s: &[char],
    compdb: &[Composition],
    cache: &mut HashMap<Vec<char>, Vec<char>>,
) -> Vec<char> {
    if let Some(cached) = cache.get(s) {
        return cached.clone();
    } else if s.is_empty() {
        vec![]
    } else {
        let mut candidates = vec![];
        let mut prefix_len = 1;

        let mut partial_compdb: &[Composition] = compdb;
        loop {
            // Try to shorten the tail
            let tail = &s[prefix_len..];
            let short_tail = shorten(tail, compdb, cache);

            let compdb_range = find_range_starting(&s[..prefix_len], &partial_compdb);

            partial_compdb = &partial_compdb[compdb_range.clone()];

            let mut candidate = vec![];

            if compdb_range.len() >= 1 && partial_compdb[0].long == &s[..prefix_len] {
                candidate.push(partial_compdb[0].short);
            } else {
                // Cannot shorten with this prefix, but there may be longer matches
                candidate.extend_from_slice(&s[..prefix_len]);
            }

            candidate.extend_from_slice(&short_tail);
            candidates.push(candidate);

            if compdb_range.len() == 0 {
                break;
            } else {
                prefix_len += 1;
            }
        }
        let result = candidates
            .into_iter()
            .min_by_key(|cand| cand.len())
            .expect("Should always find one possible string");
        cache.insert(s.to_vec(), result.clone());
        result
    }
}

fn shorten_str(s: &str, compdb: &[Composition]) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut cache = HashMap::new();
    shorten(&chars, compdb, &mut cache).into_iter().collect()
}

fn main() {
    let compdb = build_composition_database();

    let mut s = String::new();
    stdin().lock().read_to_string(&mut s).unwrap();

    let short = shorten_str(&s, &compdb);
    print!("{}", short);
}
