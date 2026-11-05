// T7: shared read-only config passed to several AST nodes.
#![allow(dead_code)]
use std::rc::Rc;

struct Config {
    tab_width: i64,
    strict: bool,
}

impl Config {
    fn is_strict(&self) -> bool {
        self.strict
    }
}

struct NumNode {
    cfg: Rc<Config>,
    value: i64,
}

impl NumNode {
    fn width(&self) -> i64 {
        self.cfg.tab_width
    }
    fn cfg_ref(&self) -> &Rc<Config> {
        &self.cfg
    }
    fn cfg_owned(&self) -> Rc<Config> {
        Rc::clone(&self.cfg)
    }
}

struct VarNode {
    cfg: Rc<Config>,
    name: String,
}

fn check_shared(c: &Rc<Config>) -> bool {
    c.is_strict()
}

fn check_plain(c: &Config) -> bool {
    c.is_strict()
}

fn main() {
    let cfg = Rc::new(Config { tab_width: 4, strict: true });
    let a = NumNode { cfg: Rc::clone(&cfg), value: 1 };
    let b = VarNode { cfg: Rc::clone(&cfg), name: "x".to_string() };
    let mut nodes = Vec::new();
    for i in 0..3 {
        nodes.push(NumNode { cfg: Rc::clone(&cfg), value: i });
    }
    let c = NumNode { cfg, value: 2 };
    let r = a.cfg_ref();
    let kept = Rc::clone(r);
    let s = check_shared(&a.cfg) && check_plain(&a.cfg);
    assert!(s);
    assert_eq!(a.width() + b.cfg.tab_width + c.value + kept.tab_width, 14);
}
