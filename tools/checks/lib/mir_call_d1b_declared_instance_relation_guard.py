#!/usr/bin/env python3
"""Structural guard for the resolver-only DeclaredInstance relation I0."""

from __future__ import annotations

from pathlib import Path
import subprocess
import sys
import tomllib


TAG = "mir-call-d1b-declared-instance-relation"
STATE_REL = Path("docs/development/current/main/CURRENT_STATE.toml")
CARD_REL = Path(
    "docs/development/current/main/investigations/"
    "mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml"
)
ROW = "MIR-CALL-ME-DECLARED-INSTANCE-RESOLVER-RELATION-I0"
KEY = "mir_call_me_declared_instance_resolver_relation_i0_2026_08_31"
RELATION = "src/mir/resolved_semantics/declared_instance_call_relation.rs"
ISSUER = "src/mir/callable_semantic_batch/issuer.rs"
MODEL = "src/mir/callable_semantic_batch/model.rs"
MOD = "src/mir/resolved_semantics/mod.rs"
BATCH_MOD = "src/mir/callable_semantic_batch/mod.rs"
TESTS = "src/mir/callable_semantic_batch/declared_instance_call_relation_tests.rs"


def fail(message: str) -> None:
    raise SystemExit(f"[{TAG}] {message}")


def load(path: Path) -> dict:
    try:
        with path.open("rb") as stream:
            return tomllib.load(stream)
    except (OSError, tomllib.TOMLDecodeError) as exc:
        fail(f"cannot load {path}: {exc}")


def text(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        fail(f"missing owner: {relative}")
    if sum(1 for _ in path.open(encoding="utf-8")) >= 760:
        fail(f"owner reached 760-line boundary: {relative}")
    return path.read_text(encoding="utf-8")


def changed_paths(root: Path, base: str) -> set[str]:
    result = subprocess.run(
        ["git", "diff", "--name-only", f"{base}..HEAD"],
        cwd=root,
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode:
        fail(f"cannot inspect implementation diff: {result.stderr.strip()}")
    return {line for line in result.stdout.splitlines() if line.strip()}


def check_pointer(state: dict, card: dict, root: Path) -> dict:
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("I0 requires fast or closeout work_mode")
    if state.get("current_execution_row") != ROW:
        fail("I0 is not the selected execution row")
    if state.get("current_design_stop") != "none":
        fail("I0 must clear current_design_stop")
    if state.get("next_execution_card") != ROW:
        fail("I0 next_execution_card drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("I0 next_execution_card_path drifted")
    row = card.get(KEY)
    if not isinstance(row, dict) or row.get("task_id") != ROW:
        fail(f"{KEY} is missing")
    status = row.get("status")
    if status not in {"fast_open", "landed"}:
        fail(f"unexpected I0 status: {status!r}")
    if row.get("implementation_permission") is not (status == "fast_open"):
        fail("I0 permission/status mismatch")
    allowed = set(row.get("allowed_files", []))
    required = {
        RELATION,
        ISSUER,
        MODEL,
        MOD,
        BATCH_MOD,
        TESTS,
        "src/mir/resolved_semantics/instance_method_declaration.rs",
        "src/mir/resolved_semantics/declared_instance_contract.rs",
        "src/mir/resolved_semantics/README.md",
        "src/mir/callable_semantic_batch/README.md",
        str(Path("tools/checks/lib/mir_call_d1b_declared_instance_relation_guard.py")),
        str(STATE_REL),
        str(CARD_REL),
    }
    if not required <= allowed:
        fail(f"I0 allowed_files omits {sorted(required - allowed)}")
    if status == "landed":
        base = row.get("base_commit")
        if not isinstance(base, str) or not base:
            fail("landed I0 needs base_commit")
        changed = changed_paths(root, base)
        if not changed <= allowed:
            fail(f"I0 changed paths escaped: {sorted(changed - allowed)}")
    return row


def check_structure(root: Path, row: dict) -> None:
    relation = text(root, RELATION)
    issuer = text(root, ISSUER)
    model = text(root, MODEL)
    resolved_mod = text(root, MOD)
    batch_mod = text(root, BATCH_MOD)
    tests = text(root, TESTS)

    required_relation = (
        "DeclaredInstanceCallSourceDispositionV1",
        "NoRootDeclaredInstanceCall",
        "Published(VerifiedDeclaredInstanceCallRelationCatalogV1)",
        "DeclaredInstanceCallRelationIssuerV1",
        "TargetArityMismatch",
        "RootReceiverBindingMismatch",
    )
    for token in required_relation:
        if token not in relation:
            fail(f"relation lacks structural token: {token}")
    # Comments are allowed to describe the forbidden downstream products.
    code = "\n".join(line for line in relation.splitlines() if not line.lstrip().startswith("//"))
    for token in ("Callee", "ValueId", "MirInstruction", "resolve_call_target", "record_direct_call"):
        if token in code:
            fail(f"resolver relation acquired forbidden downstream token: {token}")
    if "#[derive(Debug, Clone)]\npub(crate) enum DeclaredInstanceCallSourceDispositionV1" in relation:
        fail("source disposition must stay non-Clone")
    if "#[derive(Debug, Clone)]\npub(crate) struct VerifiedDeclaredInstanceCallRelationCatalogV1" in relation:
        fail("relation catalog must stay non-Clone")
    if issuer.count("DeclaredInstanceCallRelationIssuerV1::issue(") != 1:
        fail("relation issuer must be called exactly once")
    if "with_callable_semantic_syntax(|loan|" not in issuer:
        fail("relation is not inside the final syntax HRTB")
    if "declared_instance_call_source: DeclaredInstanceCallSourceDispositionV1" not in model:
        fail("batch lost its explicit relation disposition")
    if "fn declared_instance_call_source(&self)" not in model:
        fail("batch relation accessor is missing")
    if "declared_instance_call_relation" not in resolved_mod or "declared_instance_call_relation_tests" not in batch_mod:
        fail("relation module/test sibling is not wired")
    expected_tests = (
        "declared_instance_relation_publishes_one_exact_me_call",
        "same_method_name_on_different_boxes_keeps_nominal_relations_separate",
        "static_current_owner_me_call_is_outside_declared_instance_relation",
        "declared_instance_relation_rejects_target_arity_mismatch_before_lowering",
    )
    for name in expected_tests:
        if f"fn {name}(" not in tests:
            fail(f"focused relation test missing: {name}")
    listed = row.get("focused_test_filter")
    if not isinstance(listed, str) or not listed:
        fail("focused_test_filter is missing")


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: mir_call_d1b_declared_instance_relation_guard.py ROOT")
    root = Path(sys.argv[1]).resolve()
    state = load(root / STATE_REL)
    card = load(root / CARD_REL)
    row = check_pointer(state, card, root)
    check_structure(root, row)
    print(f"[{TAG}] row={ROW} ok")


if __name__ == "__main__":
    main()
