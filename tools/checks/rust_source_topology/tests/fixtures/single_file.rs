#[cfg(feature = "demo")]
fn top_level() {
    let café = 1;
    crate::support::prepare(café);
}

trait WorkerContract {
    fn default_step() {
        Self::helper();
    }

    const DEFAULT: usize = create_default();
}

impl Worker {
    const START: usize = seed();

    #[cfg_attr(test, allow(dead_code))]
    fn execute(&self) {
        self.step();
        (factory())();
        let closure = || crate::inside();
        closure();
        async { crate::later(); };
        wrapper!();
        include!("generated.rs");
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
mod nested {
    #[cfg(not(feature = "off"))]
    fn child() {
        crate::nested_call();
    }
}

mod external;

#[path = "alternate.rs"]
mod alternate;

const TOP: usize = build_top();
static GLOBAL: usize = make_global();
