// T3: symbol table as a Map. Update one entry for a short section while recording into a
// local error list or counter.
#![allow(dead_code)]
use std::collections::HashMap;

struct Symbol {
    name: String,
    is_const: bool,
    uses: i64,
    writes: i64,
}

struct Checker {
    symbols: HashMap<String, Symbol>,
    errors: Vec<String>,
}

impl Checker {
    fn note_write(&mut self, name: &str, line: u32) -> i64 {
        let mut bad = 0;
        match self.symbols.get_mut(name) {
            None => {
                self.errors.push(format!("line {line}: undefined {name}"));
                bad += 1;
            }
            Some(s) => {
                s.writes += 1;
                if s.is_const {
                    self.errors.push(format!("line {line}: write to const {}", s.name));
                    bad += 1;
                }
            }
        }
        bad
    }
}

fn main() {
    let mut c = Checker { symbols: HashMap::new(), errors: Vec::new() };
    c.symbols.insert(
        "k".to_string(),
        Symbol { name: "k".to_string(), is_const: true, uses: 0, writes: 0 },
    );
    assert_eq!(c.note_write("k", 1), 1);
    assert_eq!(c.note_write("x", 2), 1);
    assert_eq!(c.symbols["k"].writes, 1);
    assert_eq!(c.errors.len(), 2);
}
