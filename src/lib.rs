use std::ops::Range;

use hashbrown::HashMap;
use superslice::Ext;

// Defines struct Composition and const COMPOSITION_DATABASE: [Composition; _]
include!(concat!(env!("OUT_DIR"), "/compdb.rs"));

impl Composition {
    fn long(&self) -> &[char] {
        &self.long[..self.len]
    }

    fn short(&self) -> char {
        self.short
    }
}

fn find_range_starting(s: &[char], compdb: &[Composition]) -> Range<usize> {
    let len = s.len();
    let s_lower = s
        .iter()
        .flat_map(|c| c.to_lowercase())
        .collect::<Vec<char>>();
    let slice: &[char] = &s_lower;
    compdb.equal_range_by_key(&slice, |comp| {
        if comp.long().len() >= len {
            &comp.long()[..len]
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

            if compdb_range.len() >= 1 && partial_compdb[0].long() == &s[..prefix_len] {
                candidate.push(partial_compdb[0].short());
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

fn shorten_str_with_db(s: &str, compdb: &[Composition]) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut cache = HashMap::new();
    shorten(&chars, compdb, &mut cache).into_iter().collect()
}

pub enum ShortenMode {
    Normal,
    WithPunctuation,
    SameCase,
}

pub fn shorten_str(s: &str, mode: ShortenMode) -> String {
    match mode {
        ShortenMode::Normal => shorten_str_with_db(s, &COMPOSITION_DATABASE),
        ShortenMode::WithPunctuation => shorten_str_with_db(s, &COMPOSITION_DATABASE_DEPUNCTUATED),
        ShortenMode::SameCase => shorten_str_with_db(s, &COMPOSITION_DATABASE_PEDANTIC),
    }
}
