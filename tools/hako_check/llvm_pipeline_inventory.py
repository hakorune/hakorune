#!/usr/bin/env python3
"""Inventory LLVM runner pipeline seams without executing them."""

from __future__ import annotations

import argparse
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]


def read_text(root: Path, rel: str) -> str:
    path = root / rel
    if not path.is_file():
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def bit(value: bool) -> int:
    return 1 if value else 0


def emit_kv(rows: dict[str, str | int]) -> None:
    for key, value in rows.items():
        print(f"{key}={value}")


def build_inventory(root: Path) -> dict[str, str | int]:
    llvm_mod = read_text(root, "src/runner/product/llvm/mod.rs")
    mir_compiler = read_text(root, "src/runner/product/llvm/mir_compiler.rs")
    mir_flags = read_text(root, "src/config/env/mir_flags.rs")
    normalize = read_text(root, "src/mir/optimizer_passes/normalize.rs")
    method_wrapper = read_text(root, "src/runner/product/llvm/method_id_injector.rs")
    method_pass = read_text(root, "src/mir/passes/method_id_inject.rs")
    joinir = read_text(root, "src/runner/product/llvm/joinir_experiment.rs")
    joinir_flags = read_text(root, "src/config/env/joinir_flags.rs")
    pyvm = read_text(root, "src/runner/product/llvm/pyvm_executor.rs")
    legacy_pyvm = read_text(root, "src/runner/modes/common_util/legacy/pyvm.rs")
    harness = read_text(root, "src/runner/product/llvm/harness_executor.rs")
    fallback = read_text(root, "src/runner/product/llvm/fallback_executor.rs")
    vm_flags = read_text(root, "src/config/env/vm_backend_flags.rs")
    pyvm_retreat = read_text(
        root,
        "docs/development/current/main/design/archive/pyvm-retreat-ssot.md",
    )
    llvm_harness_doc = read_text(root, "docs/reference/architecture/llvm-harness.md")

    future_forced = (
        '"NYASH_REWRITE_FUTURE"' in mir_compiler
        and 'EnvVarRestore::set("NYASH_REWRITE_FUTURE", "1")' in mir_compiler
    )
    future_restore = (
        "EnvVarRestore" in mir_compiler
        and "impl Drop for EnvVarRestore" in mir_compiler
        and "remove_var(self.key)" in mir_compiler
    )
    future_consumed = "rewrite_future()" in normalize and "env.future" in normalize

    method_called = "MethodIdInjectorBox::inject" in llvm_mod
    method_wrapper_calls_pass = "inject_method_ids(module)" in method_wrapper
    method_pass_noop = "nothing to inject" in method_pass and "0" in method_pass

    joinir_called = "JoinIrExperimentBox::apply" in llvm_mod
    joinir_feature_gated = '#[cfg(feature = "llvm-harness")]' in joinir
    joinir_env_gated = (
        "joinir_experiment_enabled()" in joinir
        and "joinir_llvm_experiment_enabled()" in joinir
        and "llvm_use_harness()" in joinir
        and "NYASH_JOINIR_EXPERIMENT" in joinir_flags
        and "NYASH_JOINIR_LLVM_EXPERIMENT" in joinir_flags
    )
    joinir_original_fallback = (
        "return module;" in joinir
        and "Falling back to original MIR" in joinir
        and "using original MIR" in joinir
    )

    pyvm_stage = "PyVmExecutorBox::try_execute" in llvm_mod
    pyvm_reachable = "SMOKES_USE_PYVM" in pyvm and "run_pyvm_harness_lib" in legacy_pyvm
    pyvm_not_daily = "historical/diagnostic" in pyvm_retreat or "diagnostic" in pyvm_retreat

    obj_stage = "NYASH_LLVM_OBJ_OUT" in llvm_mod
    harness_stage = "HarnessExecutorBox::try_execute" in llvm_mod
    harness_feature_gate = '#[cfg(feature = "llvm-harness")]' in harness
    harness_default_on = "true // legacy default remains ON" in vm_flags
    harness_keep_lane = "explicit keep lane" in llvm_harness_doc or "keep lane" in llvm_harness_doc

    mock_stage = "FallbackExecutorBox::execute" in llvm_mod
    mock_reachable = "Mock LLVM Backend Execution" in fallback
    mock_blocked_when_harness_explicit = (
        "NYASH_LLVM_USE_HARNESS" in fallback
        and "do not silently fall back to mock" in fallback
    )

    checks = {
        "mir_future_rewrite_forced": future_forced,
        "mir_future_rewrite_env_restore_guard": future_restore,
        "mir_future_rewrite_consumed_by_normalize": future_consumed,
        "method_id_injector_called": method_called,
        "method_id_injector_wrapper_calls_pass": method_wrapper_calls_pass,
        "method_id_injector_noop_stub": method_pass_noop,
        "joinir_experiment_hook_called": joinir_called,
        "joinir_experiment_feature_gated": joinir_feature_gated,
        "joinir_experiment_env_gated": joinir_env_gated,
        "joinir_experiment_original_mir_fallback": joinir_original_fallback,
        "pyvm_executor_stage_present": pyvm_stage,
        "pyvm_reachable": pyvm_reachable,
        "pyvm_daily_route": not pyvm_not_daily,
        "llvm_obj_out_stage_present": obj_stage,
        "llvm_harness_stage_present": harness_stage,
        "llvm_harness_feature_gated": harness_feature_gate,
        "llvm_harness_default_enabled": harness_default_on,
        "mock_fallback_stage_present": mock_stage,
        "mock_fallback_reachable": mock_reachable,
        "mock_fallback_blocked_when_harness_explicit": mock_blocked_when_harness_explicit,
    }

    failures = [
        key
        for key, ok in checks.items()
        if not ok and key not in {"pyvm_daily_route"}
    ]

    rows: dict[str, str | int] = {
        "output_contract": "hako-check-llvm-pipeline-inventory-v0",
        "tool_surface": "hako_check_llvm_pipeline_inventory",
        "observation_only": 1,
        "rewrite_executed": 0,
        "source_rewrite_executed": 0,
        "benchmark_run_executed": 0,
        "behavior_change": 0,
        "mir_future_rewrite_forced": bit(future_forced),
        "mir_future_rewrite_env_key": "NYASH_REWRITE_FUTURE",
        "mir_future_rewrite_env_restore_guard": bit(future_restore),
        "mir_future_rewrite_consumed_by_normalize": bit(future_consumed),
        "mir_future_rewrite_route": "FutureNew/FutureSet/Await->env.future",
        "method_id_injector_stage_present": bit(method_called),
        "method_id_injector_called": bit(method_called),
        "method_id_injector_wrapper_calls_pass": bit(method_wrapper_calls_pass),
        "method_id_injector_noop_stub": bit(method_pass_noop),
        "method_id_injector_mutation_count": 0,
        "joinir_experiment_hook_called": bit(joinir_called),
        "joinir_experiment_feature_gate": "llvm-harness",
        "joinir_experiment_feature_gated": bit(joinir_feature_gated),
        "joinir_experiment_env_gate": (
            "NYASH_JOINIR_EXPERIMENT+NYASH_JOINIR_LLVM_EXPERIMENT+NYASH_LLVM_USE_HARNESS"
        ),
        "joinir_experiment_env_gated": bit(joinir_env_gated),
        "joinir_experiment_fallback_policy": "original_mir",
        "joinir_experiment_original_mir_fallback": bit(joinir_original_fallback),
        "pyvm_executor_stage_present": bit(pyvm_stage),
        "pyvm_reachable": bit(pyvm_reachable),
        "pyvm_gate": "SMOKES_USE_PYVM",
        "pyvm_daily_route": 0,
        "pyvm_withdrawn_policy": "diagnostic_only",
        "pyvm_retreat_doc_present": bit(pyvm_retreat != ""),
        "llvm_obj_out_stage_present": bit(obj_stage),
        "llvm_harness_stage_present": bit(harness_stage),
        "llvm_harness_feature_gate": "llvm-harness",
        "llvm_harness_feature_gated": bit(harness_feature_gate),
        "llvm_harness_default_enabled": bit(harness_default_on),
        "llvm_harness_keep_lane": bit(harness_keep_lane),
        "llvmlite_daily_owner": 0,
        "mock_fallback_stage_present": bit(mock_stage),
        "mock_fallback_reachable": bit(mock_reachable),
        "mock_fallback_blocked_when_harness_explicit": bit(
            mock_blocked_when_harness_explicit
        ),
        "execution_backend_order": "pyvm,obj_out,ny_llvmc_exe,mock",
        "execution_backend_runtime_sample": 0,
        "llvm_fallback_used": 0,
        "llvm_fallback_reason": "static_inventory_only",
        "type_abi_hot_lookup_count": 0,
        "provider_abi_hot_dispatch_count": 0,
        "product_activation": 0,
        "hook_installed": 0,
        "global_allocator_product_claim": 0,
        "winner_claim": 0,
        "failure_count": len(failures),
    }
    for idx, failure in enumerate(failures):
        rows[f"failure_{idx}_reason"] = failure
    rows["summary"] = "ok" if not failures else "failed"
    return rows


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=REPO_ROOT)
    parser.add_argument("--format", choices=("kv", "summary"), default="kv")
    args = parser.parse_args()

    rows = build_inventory(args.repo_root.resolve())
    if args.format == "summary":
        print(
            "LLVM pipeline inventory: "
            f"future_rewrite={rows['mir_future_rewrite_forced']} "
            f"method_id_mutations={rows['method_id_injector_mutation_count']} "
            f"pyvm_daily={rows['pyvm_daily_route']} "
            f"mock_fallback={rows['mock_fallback_reachable']} "
            f"summary={rows['summary']}"
        )
    else:
        emit_kv(rows)
    return 0 if rows["summary"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
