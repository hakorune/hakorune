#!/usr/bin/env python3
"""Source-backed llvmlite ingress census and G1 route fence."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
TAG = "llvm-llvmlite-production-census-guard"
MANIFEST = ROOT / "docs/development/current/main/investigations/llvmlite-production-ingress-census-v0.json"


def fail(message: str) -> None:
    print(f"[{TAG}] FAIL: {message}", file=sys.stderr)
    raise SystemExit(1)


def source(path: str) -> str:
    target = ROOT / path
    if not target.exists():
        fail(f"missing owner path: {path}")
    return target.read_text(encoding="utf-8")


def need(path: str, needle: str, label: str) -> None:
    if needle not in source(path):
        fail(f"{label}: {path} does not contain {needle!r}")


def need_ordered(path: str, first: str, second: str, label: str) -> None:
    text = source(path)
    first_at = text.find(first)
    second_at = text.find(second)
    if first_at < 0 or second_at < 0 or first_at >= second_at:
        fail(f"{label}: expected {first!r} before {second!r} in {path}")


ROW_EVIDENCE = {
    "ny-llvmc-default-boundary": (
        ("crates/nyash-llvm-compiler/src/main.rs", "default_value_t = DriverKind::Boundary"),
        ("crates/nyash-llvm-compiler/src/main.rs", "DriverKind::Harness"),
    ),
    "env-codegen-ordinary-boundary": (
        ("src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs", "CodegenRouteRequestV1::BoundaryPureFirst"),
        ("src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs", "validate_ordinary_ambient_replay"),
    ),
    "route-default-legacy-ambient": (
        ("src/host_providers/llvm_codegen/defaults.rs", "CodegenRouteRequestV1::BoundaryPureFirst"),
        ("src/host_providers/llvm_codegen/defaults.rs", 'compile_recipe: Some("pure-first".to_string())'),
        ("src/host_providers/llvm_codegen/defaults.rs", 'compat_replay: Some("none".to_string())'),
        ("src/host_providers/llvm_codegen/route.rs", "CodegenRouteRequestV1::LegacyAmbientKeep => Ok(())"),
    ),
    "hako-aot-generic": (
        ("lang/c-abi/shims/hako_aot_shared_impl.inc", "hako_aot_reject_ambient_harness_replay"),
        ("lang/c-abi/shims/hako_aot_shared_impl.inc", "aot-compat-admission-required"),
    ),
    "hako-aot-named-compat": (
        ("lang/c-abi/include/hako_aot.h", "hako_aot_compile_json_compat_harness"),
        ("lang/c-abi/shims/hako_aot_shared_impl.inc", "hako_aot_reject_ambient_harness_replay"),
    ),
    "hako-llvmc-named-compat": (
        ("lang/c-abi/shims/hako_llvmc_ffi_route.inc", "hako_llvmc_compile_json_compat_harness"),
        ("lang/c-abi/shims/hako_llvmc_ffi_route.inc", "hako_llvmc_reject_ambient_harness_replay"),
    ),
    "provider-llvmlite-keep": (
        ("src/host_providers/llvm_codegen/provider_keep.rs", "mir_json_to_object_llvmlite"),
        ("src/host_providers/llvm_codegen/route.rs", "CodegenRouteRequestV1::ExplicitHarnessCompat"),
    ),
    "ny-llvmc-harness-driver": (
        ("crates/nyash-llvm-compiler/src/harness_driver.rs", 'Command::new("python3")'),
        ("crates/nyash-llvm-compiler/src/main.rs", "default_value_t = DriverKind::Boundary"),
    ),
    "ny-mir-builder-tool": (
        ("tools/ny_mir_builder.sh", "NYASH_LLVM_BACKEND=llvmlite"),
        ("tools/ny_mir_builder.sh", "explicit compat/debug keep only"),
    ),
    "llvmlite-harness-script": (
        ("tools/llvmlite_harness.py", "NYASH_LLVM_USE_HARNESS"),
        ("src/llvm_py/README.md", "explicit compat/probe keep"),
    ),
    "runner-llvmlite-helper": (
        ("src/runner/modes/common_util/exec.rs", "fn llvmlite_emit_obj_lib"),
        ("src/runner/modes/common_util/exec.rs", "pub fn llvmlite_emit_obj_lib"),
    ),
    "runner-non-python-fallback": (
        ("src/runner/product/llvm/mod.rs", "FallbackExecutorBox::execute"),
        ("src/runner/product/llvm/fallback_executor.rs", "LLVM harness requested"),
    ),
    "runner-selected-boundary": (
        ("src/runner/product/llvm/harness_executor.rs", "try_execute_selected_dynamic"),
        ("src/runner/product/llvm/mod.rs", "selected_dynamic_aot_metadata_present"),
    ),
    "test-shlib-llvmlite-keep": (
        ("tools/test/lib/shlib.sh", "--harness"),
        ("tools/test/lib/shlib.sh", "tools/llvmlite_harness.py"),
    ),
    "smoke-v2-harness-helper": (
        ("tools/smokes/v2/lib/test_runner_llvm_helpers.sh", 'NYASH_LLVM_USE_HARNESS="${NYASH_LLVM_USE_HARNESS:-0}"'),
        ("tools/smokes/v2/lib/test_runner_llvm_helpers.sh", "run_nyash_llvm"),
    ),
    "smoke-v2-result-checker": (
        ("tools/smokes/v2/lib/result_checker.sh", 'local llvm_harness="${NYASH_LLVM_USE_HARNESS:-0}"'),
        ("tools/smokes/v2/lib/result_checker.sh", "llvm_output"),
    ),
    "debug-phi-llvmlite": (
        ("tools/debug/phi/phi_trace_run.sh", "NYASH_LLVM_USE_HARNESS=1"),
        ("tools/debug/phi/phi_trace_run.sh", "phi_trace_check.py"),
    ),
    "curated-llvm-keep": (
        ("tools/smokes/curated_llvm.sh", "NYASH_LLVM_USE_HARNESS=1"),
        ("tools/smokes/curated_llvm.sh", "Curated LLVM smoke runner"),
    ),
    "cache-llvmlite-keep": (
        ("tools/cache/phase29x_l2_object_cache.sh", "NYASH_LLVM_USE_HARNESS=1"),
        ("tools/cache/phase29x_l2_object_cache.sh", "phase29x_l2_object_cache"),
    ),
    "debug-phi-bridge-keep": (
        ("tools/debug/phi/phi_trace_bridge_try.sh", "--harness"),
        ("tools/debug/phi/phi_trace_bridge_try.sh", "tools/llvmlite_harness.py"),
    ),
    "phase29ck-identity-keep": (
        ("tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_llvmlite_keep_identity_min.sh", "python3 \"$HARNESS\""),
        ("tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_llvmlite_keep_identity_min.sh", "compat/probe keep harness"),
    ),
    "perf-manual-harness": (
        ("tools/perf/microbench.sh", 'NYASH_LLVM_USE_HARNESS="${NYASH_LLVM_USE_HARNESS:-0}"'),
        ("tools/perf/microbench.sh", "explicit =1 keeps the frozen llvmlite oracle"),
        ("tools/perf/lib/aot_helpers.sh", "perf AOT route must not use NYASH_LLVM_USE_HARNESS=1"),
    ),
    "smoke-compat-monitor": (
        ("tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/_llvmlite_provider_stopline_common.sh", "HAKO_LLVM_EMIT_PROVIDER=llvmlite"),
        ("tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/README.md", "monitor-only keep"),
    ),
    "ci-manual-compat": (
        (".github/workflows/fast-smoke.yml", "llvmlite"),
        (".github/workflows/fast-smoke.yml", "workflow_dispatch"),
    ),
    "ny-llvmc-harness-input": (
        ("crates/nyash-llvm-compiler/src/compile_input.rs", "resolve_harness_path"),
        ("crates/nyash-llvm-compiler/src/main.rs", "default_value_t = DriverKind::Boundary"),
    ),
    "ny-mir-builder-rust-tool": (
        ("src/bin/ny_mir_builder.rs", "NYASH_LLVM_USE_HARNESS"),
        ("src/bin/ny_mir_builder.rs", 'std::env::remove_var("NYASH_LLVM_USE_HARNESS")'),
    ),
    "run-llvm-harness-tool": (
        ("tools/run_llvm_harness.sh", "NYASH_LLVM_USE_HARNESS=1"),
        ("tools/run_llvm_harness.sh", "themselves select `ny-llvmc --driver harness`"),
    ),
    "perf-llvmlite-opt-in": (
        ("tools/perf/run_all.sh", "NYASH_LLVM_RUN_LLVMLITE"),
        ("tools/perf/run_all.sh", "llvmlite (opt-in)"),
    ),
    "smoke-default-harness-env": (
        ("tools/smokes/v2/lib/env.sh", "NYASH_LLVM_USE_HARNESS"),
        ("tools/smokes/v2/lib/env.sh", "Smoke Environment Configuration"),
    ),
    "smoke-static-config": (
        ("tools/smokes/v2/configs/llvm_static.conf", 'NYASH_LLVM_USE_HARNESS="${NYASH_LLVM_USE_HARNESS:-0}"'),
        ("tools/smokes/v2/configs/llvm_static.conf", "llvm_static.conf"),
    ),
    "smoke-matrix-config": (
        ("tools/smokes/v2/configs/matrix.conf", "export NYASH_LLVM_USE_HARNESS=1"),
        ("tools/smokes/v2/configs/matrix.conf", "matrix.conf"),
    ),
    "smoke-llvmlite-probe": (
        ("tools/smokes/v2/profiles/integration/core/phase2100/run_probe_llvmlite.sh", "NYASH_LLVM_RUN_LLVMLITE"),
        ("tools/smokes/v2/profiles/integration/core/phase2100/run_probe_llvmlite.sh", "deprecated by default"),
    ),
    "llvm-py-keep-root": (
        ("src/llvm_py/README.md", "explicit compat/probe keep"),
        ("src/llvm_py/README.md", "current daily owner ではなく"),
    ),
    "hako-compat-provider-box": (
        ("lang/src/compat/codegen/llvm_emit_compat_box.hako", "does not own the llvmlite lane"),
        ("lang/src/compat/codegen/llvm_emit_compat_box.hako", "llvmlite is not routed through LLVMEmitBox"),
    ),
    "legacy-pyvm-reference": (
        ("src/runner/modes/common_util/legacy/pyvm.rs", "run_pyvm_harness_lib"),
        ("src/runner/modes/common_util/legacy/pyvm.rs", "tools/historical/pyvm/pyvm_runner.py"),
    ),
    "selfhost-python-reference": (
        ("src/runner/selfhost.rs", "NYASH_NY_COMPILER_USE_PY"),
        ("src/runner/selfhost.rs", "Python MVP (optional)"),
    ),
    "llvm-build-stale-label": (
        ("tools/build/build_llvm.sh", "llvmlite"),
        ("tools/build/build_llvm.sh", "cargo build"),
    ),
}


def main() -> int:
    if not MANIFEST.is_file():
        fail("census manifest is missing")
    data = json.loads(MANIFEST.read_text(encoding="utf-8"))
    if data.get("schema") != "llvmlite-production-ingress-census-v0":
        fail("manifest schema drifted")
    if data.get("status") != "g0-source-census" or data.get("production_claim"):
        fail("manifest must remain a non-production G0 census")
    observation = data.get("child_observation")
    if observation != {
        "mode": "opt_in_strace",
        "ordinary": 0,
        "named_compat": 1,
        "inherited_replay": 0,
        "guard_env": "LLVM_ROUTE_IDENTITY_CHILD_OBSERVATION=1",
    }:
        fail("child observation contract drifted")
    rows = data.get("rows")
    if not isinstance(rows, list) or not rows:
        fail("manifest rows are empty")

    ids = [row.get("id") for row in rows]
    if len(ids) != len(set(ids)) or any(not item for item in ids):
        fail("manifest row ids must be unique and non-empty")
    required = {
        "ny-llvmc-default-boundary",
        "env-codegen-ordinary-boundary",
        "hako-aot-generic",
        "route-default-legacy-ambient",
        "hako-aot-named-compat",
        "hako-llvmc-named-compat",
        "provider-llvmlite-keep",
        "ny-llvmc-harness-driver",
        "ny-mir-builder-tool",
        "runner-llvmlite-helper",
        "runner-non-python-fallback",
        "runner-selected-boundary",
        "test-shlib-llvmlite-keep",
        "smoke-v2-harness-helper",
        "smoke-v2-result-checker",
        "debug-phi-llvmlite",
        "curated-llvm-keep",
        "cache-llvmlite-keep",
        "debug-phi-bridge-keep",
        "phase29ck-identity-keep",
        "ny-llvmc-harness-input",
        "ny-mir-builder-rust-tool",
        "run-llvm-harness-tool",
        "perf-llvmlite-opt-in",
        "smoke-default-harness-env",
        "smoke-static-config",
        "smoke-matrix-config",
        "smoke-llvmlite-probe",
        "llvm-py-keep-root",
        "hako-compat-provider-box",
        "legacy-pyvm-reference",
        "selfhost-python-reference",
        "llvm-build-stale-label",
    }
    missing = required.difference(ids)
    if missing:
        fail(f"required census rows missing: {sorted(missing)}")

    production = [row for row in rows if row.get("automatic_production")]
    if any(row.get("python_child") != "zero" for row in production):
        fail("automatic production row reaches Python/llvmlite")
    if any(row.get("native_retry") for row in rows):
        fail("native failure retry is still classified in the census")
    for row in rows:
        for key in ("owner", "selector", "positive_evidence", "negative_evidence"):
            if not isinstance(row.get(key), str) or not row[key].strip():
                fail(f"row {row.get('id')} lacks {key}")
        target = ROOT / row["owner"]
        if not target.exists():
            fail(f"row {row['id']} owner is missing: {row['owner']}")

    if set(ROW_EVIDENCE) != set(ids):
        fail("row evidence map and manifest rows have drifted")
    for row_id, evidence in ROW_EVIDENCE.items():
        for path, needle in evidence:
            need(path, needle, f"row {row_id} source evidence")

    # Source-backed selectors and child boundaries. These are deliberately
    # exact strings; labels and environment names alone cannot satisfy G0.
    need("crates/nyash-llvm-compiler/src/main.rs", "default_value_t = DriverKind::Boundary", "Boundary default")
    need("src/host_providers/llvm_codegen/defaults.rs", "CodegenRouteRequestV1::BoundaryPureFirst", "Boundary object default route")
    need("src/host_providers/llvm_codegen/defaults.rs", 'compile_recipe: Some("pure-first".to_string())', "Boundary object pure-first recipe")
    need("src/host_providers/llvm_codegen/defaults.rs", 'compat_replay: Some("none".to_string())', "Boundary object replay fence")
    need("crates/nyash_kernel/src/plugin/module_string_dispatch/compat/llvm_backend_surrogate.rs", "select_explicit_harness_compat", "named Stage1 compat admission")
    need("src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs", "CodegenRouteRequestV1::BoundaryPureFirst", "ordinary env.codegen route")
    need("src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs", "CodegenRouteRequestV1::ExplicitHarnessCompat", "named env.codegen keep")
    need("lang/c-abi/shims/hako_aot_shared_impl.inc", "hako_aot_reject_ambient_harness_replay", "generic AOT replay fence")
    need("lang/c-abi/shims/hako_aot_shared_impl.inc", "hako_aot_compile_json_compat_harness", "named AOT keep")
    need("lang/c-abi/shims/hako_llvmc_ffi_route.inc", "hako_llvmc_compile_json_compat_harness", "named C FFI keep")
    need("src/host_providers/llvm_codegen/provider_keep.rs", "mir_json_to_object_llvmlite", "provider keep owner")
    need("crates/nyash-llvm-compiler/src/harness_driver.rs", 'Command::new("python3")', "explicit harness child")
    need("tools/ny_mir_builder.sh", "NYASH_LLVM_BACKEND=llvmlite", "explicit tool keep")
    need("src/runner/product/llvm/mod.rs", "FallbackExecutorBox::execute", "non-Python fallback classification")
    need("src/runner/product/llvm/mod.rs", "execute_via_harness_or_fallback", "runner orchestration owner")
    need("src/runner/product/llvm/harness_executor.rs", "try_execute_selected_dynamic", "selected Boundary executor")
    need("src/runner/product/llvm/mod.rs", "selected Dynamic candidate is Boundary-only; VM execution is rejected", "selected VM fence")
    need("src/runner/product/llvm/mod.rs", "selected Dynamic object emission is not a live Boundary artifact route", "selected object fence")
    need("tools/perf/lib/aot_helpers.sh", "perf AOT route must not use NYASH_LLVM_USE_HARNESS=1", "perf Boundary fence")

    runner = source("src/runner/product/llvm/mod.rs")
    selected_marker = "if selected_dynamic {\n        let code = harness_executor::HarnessExecutorBox::try_execute_selected_dynamic(module)?;"
    ordinary_marker = "match harness_executor::HarnessExecutorBox::try_execute(module)"
    need_ordered(
        "src/runner/product/llvm/mod.rs",
        selected_marker,
        ordinary_marker,
        "selected Boundary dominance",
    )
    selected_at = runner.find(selected_marker)
    ordinary_at = runner.find(ordinary_marker, selected_at + len(selected_marker))
    if selected_at < 0 or ordinary_at < 0 or selected_at >= ordinary_at:
        fail("selected Boundary executor must dominate ordinary harness/fallback dispatch")
    selected_region = runner[selected_at:ordinary_at]
    if "FallbackExecutorBox::execute" in selected_region:
        fail("selected Boundary branch must not contain mock fallback")
    if "try_execute_selected_dynamic(module)?" not in selected_region:
        fail("selected Boundary branch lost its sole executor")

    # The helper is a deletion candidate, not a production claim. Its exact
    # symbol must have no caller outside its defining source file.
    result = subprocess.run(
        ["rg", "-l", "llvmlite_emit_obj_lib", "src", "crates", "tools", "lang"],
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode not in (0, 1):
        fail("caller census command failed")
    callers = {
        line.strip()
        for line in result.stdout.splitlines()
        if line.strip() and line.strip() != "tools/checks/llvm_llvmlite_production_census_guard.py"
    }
    if callers != {"src/runner/modes/common_util/exec.rs"}:
        fail(f"zero-consumer helper has unexpected callers: {sorted(callers)}")

    print(f"[{TAG}] ok (rows={len(rows)}, automatic_python_ingress=0, native_retry=0, keep_roots={sum(row['class'] in {'explicit_keep', 'manual_keep'} for row in rows)})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
