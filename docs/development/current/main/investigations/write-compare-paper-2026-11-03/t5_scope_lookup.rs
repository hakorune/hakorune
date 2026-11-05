// T5: scope-chain lookup returning a borrowed symbol from either the local or the parent scope.
#![allow(dead_code)]
use std::collections::HashMap;

// === COMMON ===
struct Symbol {
    name: String,
    id: i64,
}

#[derive(Default)]
struct Table {
    entries: HashMap<String, Symbol>,
}

impl Table {
    fn get(&self, name: &str) -> Option<&Symbol> {
        self.entries.get(name)
    }
}

// === V1 two supports ===
fn find<'a>(inner: &'a Table, outer: &'a Table, name: &str) -> Option<&'a Symbol> {
    inner.get(name).or_else(|| outer.get(name))
}

fn resolve_id(inner: &Table, outer: &Table, name: &str) -> i64 {
    match find(inner, outer, name) {
        Some(s) => s.id,
        None => -1,
    }
}

// === V2 owner chain ===
#[derive(Default)]
struct Scope {
    table: Table,
    parent: Option<Box<Scope>>,
}

impl Scope {
    fn lookup(&self, name: &str) -> Option<&Symbol> {
        if let Some(hit) = self.table.get(name) {
            return Some(hit);
        }
        self.parent.as_ref()?.lookup(name)
    }
}

// Required current scope, written without Option. Uses mem::take because Scope: Default.
#[derive(Default)]
struct Resolver {
    cur: Scope,
}

impl Resolver {
    fn enter(&mut self) {
        let old = std::mem::take(&mut self.cur);
        self.cur.parent = Some(Box::new(old));
    }
    fn leave(&mut self) {
        if let Some(p) = self.cur.parent.take() {
            self.cur = *p;
        }
    }
    fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.cur.lookup(name)
    }
    fn current_mut(&mut self) -> &mut Table {
        &mut self.cur.table
    }
    fn define(&mut self, name: &str, sym: Symbol) {
        self.current_mut().entries.insert(name.to_string(), sym);
    }
}

// === V2b borrowed parent chain (first choice in Rust, not writable in Hakorune) ===
struct RefScope<'p> {
    table: Table,
    parent: Option<&'p RefScope<'p>>,
}

impl<'p> RefScope<'p> {
    fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.table.get(name).or_else(|| self.parent?.lookup(name))
    }
}

// === caller ===
fn main() {
    let mut outer = Table::default();
    outer.entries.insert("x".to_string(), Symbol { name: "x".to_string(), id: 7 });
    let inner = Table::default();
    assert_eq!(resolve_id(&inner, &outer, "x"), 7);

    let mut r = Resolver::default();
    r.define("y", Symbol { name: "y".to_string(), id: 3 });
    r.enter();
    assert_eq!(r.lookup("y").map(|s| s.id), Some(3));
    r.leave();

    let root = RefScope { table: outer, parent: None };
    let child = RefScope { table: inner, parent: Some(&root) };
    assert_eq!(child.lookup("x").map(|s| s.id), Some(7));
}
