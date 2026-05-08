use crate::mir::MirCompiler;
use crate::parser::NyashParser;
use std::sync::{Mutex, MutexGuard, OnceLock};

fn ensure_ring0_initialized() {
    use crate::runtime::ring0::{default_ring0, init_global_ring0};
    let _ = std::panic::catch_unwind(|| {
        init_global_ring0(default_ring0());
    });
}

fn env_guard() -> &'static Mutex<()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(()))
}

struct FeatureOverrideGuard {
    prev: Option<String>,
    _lock: MutexGuard<'static, ()>,
}

impl FeatureOverrideGuard {
    fn new(features: &str) -> Self {
        let lock = match env_guard().lock() {
            Ok(lock) => lock,
            Err(poisoned) => poisoned.into_inner(),
        };
        let prev = std::env::var("NYASH_FEATURES").ok();
        std::env::set_var("NYASH_FEATURES", features);
        Self { prev, _lock: lock }
    }
}

impl Drop for FeatureOverrideGuard {
    fn drop(&mut self) {
        match &self.prev {
            Some(value) => std::env::set_var("NYASH_FEATURES", value),
            None => std::env::remove_var("NYASH_FEATURES"),
        }
    }
}

#[test]
fn mir_preserves_rune_contracts_as_effect_plan_metadata() {
    ensure_ring0_initialized();
    let _guard = FeatureOverrideGuard::new("rune");
    let ast = NyashParser::parse_from_string(
        r#"
static box Main {
  @rune Contract(no_alloc)
  @rune Contract(no_safepoint)
  main() {
    return 0
  }
}
"#,
    )
    .expect("parse effect plan contracts");

    let mut compiler = MirCompiler::with_options(false);
    let module = compiler.compile(ast).expect("compile effect plan").module;
    let main = module
        .functions
        .values()
        .find(|function| function.signature.name.contains("main"))
        .expect("main function");

    assert_eq!(main.metadata.effect_plans.len(), 1);
    let plan = &main.metadata.effect_plans[0];
    assert_eq!(plan.function, main.signature.name);
    assert_eq!(
        plan.requires
            .iter()
            .map(|requirement| requirement.as_str())
            .collect::<Vec<_>>(),
        vec!["no_alloc", "no_safepoint"]
    );
    assert!(!plan.verified);
    assert_eq!(plan.source, "rune_contract");
    assert!(main.metadata.capability_plans.is_empty());
}
