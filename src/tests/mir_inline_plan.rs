use crate::mir::{Callee, MirCompiler, MirInstruction};
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
fn mir_preserves_rune_hint_inline_as_inline_plan_metadata() {
    ensure_ring0_initialized();
    let _guard = FeatureOverrideGuard::new("rune");
    let ast = NyashParser::parse_from_string(
        r#"
static box Main {
  @rune Hint(inline)
  main() {
    return 0
  }
}
"#,
    )
    .expect("parse inline hint");

    let mut compiler = MirCompiler::with_options(false);
    let module = compiler.compile(ast).expect("compile inline hint").module;
    let main = module
        .functions
        .values()
        .find(|function| function.signature.name.contains("main"))
        .expect("main function");

    assert_eq!(main.metadata.inline_plans.len(), 1);
    let plan = &main.metadata.inline_plans[0];
    assert_eq!(plan.function, main.signature.name);
    assert_eq!(plan.request.as_str(), "prefer");
    assert_eq!(plan.hotness, None);
    assert_eq!(plan.max_ir, None);
    assert!(plan.requires.is_empty());
    assert!(!plan.verified);
    assert_eq!(plan.fallback, "keep_call");
    assert_eq!(plan.source, "rune_hint");
}

#[test]
fn mir_preserves_rune_hint_hot_as_inline_plan_metadata_without_inline_request() {
    ensure_ring0_initialized();
    let _guard = FeatureOverrideGuard::new("rune");
    let ast = NyashParser::parse_from_string(
        r#"
static box Main {
  @rune Hint(hot)
  main() {
    return 0
  }
}
"#,
    )
    .expect("parse hot hint");

    let mut compiler = MirCompiler::with_options(false);
    let module = compiler.compile(ast).expect("compile hot hint").module;
    let main = module
        .functions
        .values()
        .find(|function| function.signature.name.contains("main"))
        .expect("main function");

    assert_eq!(main.metadata.inline_plans.len(), 1);
    let plan = &main.metadata.inline_plans[0];
    assert_eq!(plan.request.as_str(), "none");
    assert_eq!(
        plan.hotness
            .as_ref()
            .map(crate::mir::inline_plan::InlineHotness::as_str),
        Some("hot")
    );
    assert_eq!(plan.fallback, "keep_call");
    assert!(!plan.verified);
}

#[test]
fn mir_preserves_rune_lowering_inline_required_as_inline_plan_metadata() {
    ensure_ring0_initialized();
    let _guard = FeatureOverrideGuard::new("rune");
    let ast = NyashParser::parse_from_string(
        r#"
static box Main {
  @rune Lowering(inline_required)
  main() {
    return 0
  }
}
"#,
    )
    .expect("parse required inline lowering");

    let mut compiler = MirCompiler::with_options(false);
    let module = compiler
        .compile(ast)
        .expect("compile required inline lowering")
        .module;
    let main = module
        .functions
        .values()
        .find(|function| function.signature.name.contains("main"))
        .expect("main function");

    assert_eq!(main.metadata.inline_plans.len(), 1);
    let plan = &main.metadata.inline_plans[0];
    assert_eq!(plan.function, main.signature.name);
    assert_eq!(plan.request.as_str(), "required");
    assert_eq!(plan.hotness, None);
    assert_eq!(plan.max_ir, None);
    assert_eq!(
        plan.requires,
        vec!["no_alloc".to_string(), "no_safepoint".to_string()]
    );
    assert!(!plan.verified);
    assert_eq!(plan.fallback, "fail_fast");
    assert_eq!(plan.source, "rune_lowering");
}

#[test]
fn mir_optimizer_consumes_verified_profile_allocator_fast_required_inline() {
    ensure_ring0_initialized();
    let _guard = FeatureOverrideGuard::new("rune");
    let ast = NyashParser::parse_from_string(
        r#"
static box AllocFastProof {
  @rune Profile(allocator.fast)
  size_to_bin(size) {
    local adjusted = size + 7
    return adjusted / 8
  }
}

static box Main {
  main() {
    return AllocFastProof.size_to_bin(17)
  }
}
"#,
    )
    .expect("parse allocator fast profile");

    let mut compiler = MirCompiler::with_options(true);
    let module = compiler
        .compile(ast)
        .expect("compile allocator fast profile")
        .module;

    let callee = module
        .functions
        .get("AllocFastProof.size_to_bin/1")
        .expect("allocator fast helper");
    assert!(callee.metadata.inline_plans.iter().any(|plan| {
        plan.request.as_str() == "required"
            && plan.source == "rune_profile:allocator.fast"
            && plan.verified
    }));

    let main = module.functions.get("main").expect("main function");
    let has_helper_call = main.blocks.values().any(|block| {
        block.all_instructions().any(|inst| {
            matches!(
                inst,
                MirInstruction::Call {
                    callee: Some(Callee::Global(name)),
                    ..
                } if name == "AllocFastProof.size_to_bin/1"
            )
        })
    });
    assert!(!has_helper_call);
}
