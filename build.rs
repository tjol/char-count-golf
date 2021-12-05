use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use unicode_normalization::char::decompose_compatible;

fn build_composition_database() -> Vec<(Vec<char>, char)> {
    let mut db = vec![];
    for c in '\u{0080}'..'\u{10FFFF}' {
        let mut s = String::new();
        decompose_compatible(c, |c2| {
            s.push(c2);
        });
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 1 {
            db.push((chars, c));
        }
    }
    db.sort();
    db
}

fn write_compdb(
    dest_path: &Path,
    compdb: &[(Vec<char>, char)],
    maxlen: usize,
) -> std::io::Result<()> {
    let mut outf = File::create(dest_path)?;

    writeln!(
        &mut outf,
        "#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]"
    )?;
    writeln!(&mut outf, "struct Composition {{")?;
    writeln!(&mut outf, "    short: char,")?;
    writeln!(&mut outf, "    len: usize,")?;
    writeln!(&mut outf, "    long: [char; {}]", maxlen)?;
    writeln!(&mut outf, "}}")?;

    writeln!(
        &mut outf,
        "const COMPOSITION_DATABASE: [Composition; {}] = [",
        compdb.len()
    )?;
    for comp in compdb {
        let mut long_extended = comp.0.clone();
        long_extended.resize(maxlen, '\0');
        writeln!(
            &mut outf,
            "    Composition {{ short: {:?}, len: {}, long: {:?} }},",
            comp.1,
            comp.0.len(),
            long_extended
        )?;
    }
    writeln!(&mut outf, "];")?;
    Ok(())
}

fn main() {
    let compdb = build_composition_database();

    // How long can decomposed sequences be?
    let maxlen = compdb.iter().map(|comp| comp.0.len()).max().unwrap();

    // Write the code to regenerate the database statically
    let out_dir = env::var_os("OUT_DIR").unwrap();

    write_compdb(&Path::new(&out_dir).join("compdb.rs"), &compdb, maxlen).unwrap();
}
