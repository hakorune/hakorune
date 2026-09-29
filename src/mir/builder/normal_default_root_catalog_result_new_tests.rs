use super::normal_default_root_catalog_lifecycle_tests::{callable_source, session};
use crate::mir::builder::{CallableMainMaterializationPolicyV1, NormalRuntimeInputSnapshotV1};
use crate::parser::ParserBuildConfig;

#[test]
fn artifact_child_accepts_return_position_new_under_its_result_claim() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        "box Point {
  x: i64
  y: i64
  birth(x, y) {
    me.x = x
    me.y = y
  }
}
static box Work {
  make(args) {
    return new Point(1, 2)
  }
}
static box Main {
  main() {
    return 30
  }
}",
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("return-position `new` must lower to a module");
    let (_, module, validate) = completed.into_artifact_parts();
    validate(&module).expect("result claim owns the emitted lifecycle sites");
}

#[test]
fn artifact_root_return_position_new_reaches_the_truthful_next_boundary() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    // Root `return new <plain-i64 box>` is admitted by the claim lane — the
    // lifecycle sites are owned, so the observed terminal is the next
    // truthful boundary (the root/entry result ABI has no handle arm yet),
    // never `artifact-unowned-lifecycle-site`.
    let source = callable_source(
        "box Point {
  x: i64
  y: i64
  birth(x, y) {
    me.x = x
    me.y = y
  }
}
static box Main {
  main() {
    return new Point(1, 2)
  }
}",
        ParserBuildConfig::default(),
    );
    match session().complete_normal_default_program_root_catalog_lifecycle(
        source,
        CallableMainMaterializationPolicyV1::Omitted,
        NormalRuntimeInputSnapshotV1::empty(),
    ) {
        Ok(completed) => {
            let (_, module, validate) = completed.into_artifact_parts();
            match validate(&module) {
                Ok(_) => {}
                Err(error) => {
                    assert!(
                        !error.contains("artifact-unowned-lifecycle-site"),
                        "the claim owns the lifecycle sites; got {error}"
                    );
                }
            }
        }
        Err(rejected) => {
            let message = format!("{rejected:?}");
            assert!(
                !message.contains("artifact-unowned-lifecycle-site"),
                "the claim owns the lifecycle sites; got {message}"
            );
            rejected.discard();
        }
    }
}

#[test]
fn artifact_child_rejects_retained_unavailable_commit_before_lifecycle_coverage() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for holder_birth in [false, true] {
        let holder = if holder_birth {
            "box Holder {
  init { items }
  birth() {
    local items = new ArrayBox()
    me.items = items
  }
}"
        } else {
            "box Holder {
  init { items }
}"
        };
        let source = callable_source(
            &format!(
                "{holder}
box Worker {{
  run(): i64 {{
    local h = new Holder()
    return 0
  }}
}}
static box Main {{
  main() {{
    local w = new Worker()
    return w.run()
  }}
}}"
            ),
            ParserBuildConfig::default(),
        );
        let completed = session()
            .complete_normal_default_program_root_catalog_lifecycle(
                source,
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            )
            .expect("fixture must lower to a module");
        let (_, module, validate) = completed.into_artifact_parts();
        let error = validate(&module).unwrap_err();
        assert!(
            error.contains("artifact-source-unavailable"),
            "holder_birth={holder_birth}: {error}"
        );
    }
}
