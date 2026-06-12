use super::*;

#[test]
fn hako_emit_program_json_is_retired() {
    let result = build_command().try_get_matches_from([
        "hakorune",
        "--hako-emit-program-json",
        "/tmp/out.json",
    ]);
    assert!(
        result.is_err(),
        "retired hako Program(JSON) alias must not parse"
    );
}

#[test]
fn hako_emit_mir_conflicts_with_emit_mir_json() {
    let result = build_command().try_get_matches_from([
        "hakorune",
        "--hako-emit-mir-json",
        "/tmp/hako.json",
        "--emit-mir-json",
        "/tmp/rust.json",
    ]);
    assert!(result.is_err(), "conflicting emit routes must be rejected");
}

#[test]
fn emit_mir_json_minimal_parses_and_sets_output_path() {
    let matches = build_command()
        .try_get_matches_from([
            "hakorune",
            "--emit-mir-json-minimal",
            "/tmp/out.json",
            "apps/min.hako",
        ])
        .expect("minimal emit args should parse");

    let cfg = from_matches(&matches);
    assert_eq!(cfg.emit_mir_json_minimal.as_deref(), Some("/tmp/out.json"));
    assert_eq!(cfg.file.as_deref(), Some("apps/min.hako"));
}

#[test]
fn emit_mir_json_minimal_conflicts_with_emit_mir_json() {
    let result = build_command().try_get_matches_from([
        "hakorune",
        "--emit-mir-json-minimal",
        "/tmp/min.json",
        "--emit-mir-json",
        "/tmp/normal.json",
        "apps/min.hako",
    ]);
    assert!(
        result.is_err(),
        "minimal emit route must conflict with normal MIR emit"
    );
}

#[test]
fn emit_wat_route_parses_and_sets_output_path() {
    let matches = build_command()
        .try_get_matches_from(["hakorune", "--emit-wat", "/tmp/out.wat", "apps/min.hako"])
        .expect("emit-wat args should parse");

    let cfg = from_matches(&matches);
    assert_eq!(cfg.emit_wat.as_deref(), Some("/tmp/out.wat"));
    assert_eq!(cfg.file.as_deref(), Some("apps/min.hako"));
}

#[test]
fn emit_wat_conflicts_with_compile_wasm() {
    let result = build_command().try_get_matches_from([
        "hakorune",
        "--emit-wat",
        "/tmp/out.wat",
        "--compile-wasm",
        "apps/min.hako",
    ]);
    assert!(result.is_err(), "emit-wat and compile-wasm must conflict");
}

#[test]
fn allocator_hook_dry_run_cli_route_parses() {
    let matches = build_command()
        .try_get_matches_from([
            "hakorune",
            "--allocator-hook-dry-run",
            "--allocator-hook-plan",
            "/tmp/plan.toml",
            "--allocator-hook-proof",
            "/tmp/proof.toml",
        ])
        .expect("allocator hook dry-run args should parse");

    let cfg = from_matches(&matches);
    assert!(cfg.allocator_hook_dry_run);
    assert_eq!(
        cfg.allocator_hook_dry_run_plan.as_deref(),
        Some("/tmp/plan.toml")
    );
    assert_eq!(
        cfg.allocator_hook_dry_run_proof.as_deref(),
        Some("/tmp/proof.toml")
    );
}

#[test]
fn allocator_provider_manifest_cli_route_parses() {
    let matches = build_command()
        .try_get_matches_from([
            "hakorune",
            "--allocator-provider-manifest",
            "/tmp/provider.toml",
        ])
        .expect("allocator provider manifest args should parse");

    let cfg = from_matches(&matches);
    assert_eq!(
        cfg.allocator_provider_manifest.as_deref(),
        Some("/tmp/provider.toml")
    );
}

#[test]
fn allocator_provider_activation_safety_cli_route_parses() {
    let matches = build_command()
        .try_get_matches_from([
            "hakorune",
            "--allocator-provider-activation-safety-gate",
            "/tmp/safety.toml",
        ])
        .expect("allocator provider activation safety args should parse");

    let cfg = from_matches(&matches);
    assert_eq!(
        cfg.allocator_provider_activation_safety_gate.as_deref(),
        Some("/tmp/safety.toml")
    );
}

#[test]
fn allocator_provider_activation_decision_cli_route_parses() {
    let matches = build_command()
        .try_get_matches_from([
            "hakorune",
            "--allocator-provider-activation-decision",
            "/tmp/decision.toml",
        ])
        .expect("allocator provider activation decision args should parse");

    let cfg = from_matches(&matches);
    assert_eq!(
        cfg.allocator_provider_activation_decision.as_deref(),
        Some("/tmp/decision.toml")
    );
}

#[test]
fn allocator_provider_registry_snapshot_cli_route_parses() {
    let matches = build_command()
        .try_get_matches_from([
            "hakorune",
            "--allocator-provider-registry-snapshot",
            "/tmp/registry.toml",
        ])
        .expect("allocator provider registry snapshot args should parse");

    let cfg = from_matches(&matches);
    assert_eq!(
        cfg.allocator_provider_registry_snapshot.as_deref(),
        Some("/tmp/registry.toml")
    );
}

#[test]
fn allocator_provider_selection_decision_cli_route_parses() {
    let matches = build_command()
        .try_get_matches_from([
            "hakorune",
            "--allocator-provider-selection-decision",
            "/tmp/selection.toml",
        ])
        .expect("allocator provider selection decision args should parse");

    let cfg = from_matches(&matches);
    assert_eq!(
        cfg.allocator_provider_selection_decision.as_deref(),
        Some("/tmp/selection.toml")
    );
}

#[test]
fn allocator_provider_proof_bundle_consumption_cli_route_parses() {
    let matches = build_command()
        .try_get_matches_from([
            "hakorune",
            "--allocator-provider-proof-bundle-consumption",
            "/tmp/proof-bundle.toml",
        ])
        .expect("allocator provider proof bundle consumption args should parse");

    let cfg = from_matches(&matches);
    assert_eq!(
        cfg.allocator_provider_proof_bundle_consumption.as_deref(),
        Some("/tmp/proof-bundle.toml")
    );
}

#[test]
fn allocator_provider_manifest_combines_with_hook_dry_run() {
    let matches = build_command()
        .try_get_matches_from([
            "hakorune",
            "--allocator-provider-manifest",
            "/tmp/provider.toml",
            "--allocator-hook-dry-run",
            "--allocator-hook-plan",
            "/tmp/plan.toml",
            "--allocator-hook-proof",
            "/tmp/proof.toml",
        ])
        .expect("combined allocator provider/hook dry-run args should parse");

    let cfg = from_matches(&matches);
    assert!(cfg.allocator_hook_dry_run);
    assert_eq!(
        cfg.allocator_provider_manifest.as_deref(),
        Some("/tmp/provider.toml")
    );
    assert_eq!(
        cfg.allocator_hook_dry_run_plan.as_deref(),
        Some("/tmp/plan.toml")
    );
    assert_eq!(
        cfg.allocator_hook_dry_run_proof.as_deref(),
        Some("/tmp/proof.toml")
    );
}
