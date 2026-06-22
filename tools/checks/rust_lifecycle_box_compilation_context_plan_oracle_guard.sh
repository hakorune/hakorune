#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/extract_box_compilation_context_facts.py --check-reference

python3 - <<'PY'
import json
from pathlib import Path

plan = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-plan-v0.json").read_text())
oracle = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/box-compilation-context-oracle-v0.json").read_text())

assert plan["kind"] == "HakoLifecyclePlan"
assert plan["subject"] == "hakorune_mir_builder::context::BoxCompilationContext"
plan_ids = {row["id"] for row in plan["plans"]}
for expected in [
    "BoxCompilationContext",
    "BoxCompilationContext.variable_map",
    "BoxCompilationContext.value_origin_newbox",
    "BoxCompilationContext.value_types",
    "BoxCompilationContext::new",
    "BoxCompilationContext::is_empty",
]:
    assert expected in plan_ids
assert plan["denied"][0]["id"] == "BoxCompilationContext.size_info"

assert oracle["kind"] == "RustOracleVectors"
assert oracle["subject"] == "hakorune_mir_builder::context::BoxCompilationContext"
assert oracle["vectors"][0]["id"] == "box_compilation_context_basic"
assert oracle["drop_oracle"]["required_fact"] == "BoxCompilationContext.drop_fact=TrivialMemory"
assert "size_info" in oracle["excluded_vectors"]
PY

cat <<'REPORT'
output_contract=rustc-semir-box-compilation-context-pilot-v0
box_compilation_context_facts_extraction_green=1
box_compilation_context_plan_green=1
box_compilation_context_oracle_green=1
size_info_excluded=1
summary=ok
REPORT
