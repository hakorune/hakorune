#!/usr/bin/env python3
"""Fail-closed structural guard for the B1 typed Global carrier cutover.

This guard is intentionally small: it proves that the two core carrier
surfaces agree and that the transitional String constructor did not return.
It does not claim that D1B source issuance, Call schema retirement, or the
legacy compatibility routes are complete.
"""

from __future__ import annotations

from pathlib import Path
import sys
import tomllib


TAG = "mir-call-global-target-b1-cutover"
CARD_REL = Path(
    "docs/development/current/main/investigations/"
    "mir-call-d1b-direct-call-source-inventory-coseal-d0-2026-08-26.toml"
)
STATE_REL = Path("docs/development/current/main/CURRENT_STATE.toml")
REGISTRY_REL = Path("tools/checks/guard_rows.toml")
CORE_FILES = (
    Path("crates/hakorune_mir_defs/src/call_unified.rs"),
    Path("src/mir/builder/calls/call_target.rs"),
)
LINE_LIMIT_FILES = (
    Path("crates/hakorune_mir_defs/src/global_target.rs"),
    Path("crates/hakorune_mir_defs/src/call_unified.rs"),
    Path("src/mir/builder/calls/call_target.rs"),
    Path("src/mir/builder/calls/emit.rs"),
    Path("src/mir/builder/calls/resolver.rs"),
    Path("src/mir/builder/calls/unified_emitter.rs"),
    Path("src/mir/builder/calls/function_call_preflight_route.rs"),
    Path("src/mir/canonical_direct_call.rs"),
)
COMPATIBILITY_ADAPTER_USERS = {
    Path("src/mir/builder/calls/call_target.rs"),
    Path("src/mir/builder/calls/build.rs"),
    Path("src/mir/builder/calls/static_resolution.rs"),
    Path("src/mir/builder/normal_module_transaction/physical_thunk.rs"),
    Path("src/mir/builder/ops/arithmetic.rs"),
    Path("src/mir/builder/ops/comparison.rs"),
    Path("src/mir/builder/ops/unary.rs"),
    Path("src/mir/builder/control_flow/plan/lowerer/effect_emission.rs"),
    Path("src/mir/builder/rewrite/known.rs"),
}
ROW_ID = "mir-call-global-target-b1-cutover"


def fail(message: str) -> None:
    raise SystemExit(f"[{TAG}] ERROR: {message}")


def load(path: Path) -> dict:
    try:
        with path.open("rb") as stream:
            value = tomllib.load(stream)
    except tomllib.TOMLDecodeError as exc:
        fail(f"TOML parse failed: {path}: {exc}")
    if not isinstance(value, dict):
        fail(f"TOML root must be a table: {path}")
    return value


def require_text(root: Path, relative: Path) -> str:
    path = root / relative
    if not path.is_file():
        fail(f"missing source owner: {relative}")
    return path.read_text(encoding="utf-8")


def rust_files(root: Path) -> list[Path]:
    result: list[Path] = []
    for prefix in (root / "src", root / "crates"):
        if not prefix.is_dir():
            continue
        result.extend(path for path in prefix.rglob("*.rs") if path.is_file())
    return sorted(result)


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    card = load(root / CARD_REL)
    state = load(root / STATE_REL)
    registry = load(root / REGISTRY_REL)
    b1 = card.get("b1_cutover")
    if not isinstance(b1, dict):
        fail("active card is missing [b1_cutover]")
    if b1.get("task_id") != "MIR-CALL-GLOBAL-TARGET-B1-CUTOVER":
        fail("active card does not select the B1 cutover")
    if b1.get("status") not in {"fast_open", "landed", "closeout"}:
        fail(f"B1 cutover status is not closed: {b1.get('status')!r}")
    if b1.get("status") == "fast_open" and b1.get("implementation_permission") is not True:
        fail("fast B1 cutover does not have scoped implementation permission")
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("CURRENT_STATE must remain fast or closeout for B1")
    if state.get("current_execution_row") != b1["task_id"]:
        fail("CURRENT_STATE current_execution_row does not select B1")

    rows = registry.get("rows")
    if not isinstance(rows, list):
        fail("guard_rows.toml rows table is missing")
    matches = [row for row in rows if isinstance(row, dict) and row.get("id") == ROW_ID]
    if len(matches) != 1:
        fail(f"expected exactly one registry row for {ROW_ID}, found {len(matches)}")
    row = matches[0]
    expected_cmd = ["python3", "tools/checks/lib/mir_call_global_target_b1_cutover_guard.py", "."]
    if row.get("profiles") != ["pilot", "quick-static"] or row.get("cmd") != expected_cmd:
        fail("B1 cutover guard registry row drifted")

    call_defs = require_text(root, CORE_FILES[0])
    call_target = require_text(root, CORE_FILES[1])
    carrier = require_text(root, Path("crates/hakorune_mir_defs/src/global_target.rs"))
    if "Global(CanonicalGlobalTargetV1)" not in call_defs:
        fail("Callee::Global is not the canonical typed carrier")
    if "Global(CanonicalGlobalTargetV1)" not in call_target:
        fail("CallTarget::Global is not the canonical typed carrier")
    for label, text in (("Callee", call_defs), ("CallTarget", call_target)):
        if "Global(String)" in text:
            fail(f"{label} still exposes Global(String)")
    if "pub fn global(\n        dst: Option<ValueId>,\n        target: CanonicalGlobalTargetV1," not in call_defs:
        fail("MirCall::global does not accept the typed carrier")
    if "CallTarget::Global(target) => Ok(Callee::Global(target))" not in require_text(
        root, Path("src/mir/builder/calls/resolver.rs")
    ):
        fail("resolver does not forward the typed Global carrier")

    for relative in LINE_LIMIT_FILES:
        line_count = len(require_text(root, relative).splitlines())
        if line_count >= 760:
            fail(f"B1 owner reached the 760-line split threshold: {relative} ({line_count})")

    all_sources = rust_files(root)
    direct_string_literals: list[str] = []
    for path in all_sources:
        text = path.read_text(encoding="utf-8")
        if 'Callee::Global("' in text or 'CallTarget::Global("' in text:
            direct_string_literals.append(path.relative_to(root).as_posix())
        if "Global(String)" in text:
            fail(f"legacy Global(String) surface remains: {path.relative_to(root)}")
    if direct_string_literals:
        fail("direct String Global constructors remain: " + ", ".join(direct_string_literals))

    adapter = "typed_global_target_from_selected_symbol"
    if adapter not in call_target:
        fail("legacy selected-symbol projection is not explicitly named")
    users = {
        path.relative_to(root)
        for path in all_sources
        if adapter in path.read_text(encoding="utf-8")
    }
    if users != COMPATIBILITY_ADAPTER_USERS:
        unexpected = sorted((users - COMPATIBILITY_ADAPTER_USERS), key=str)
        missing = sorted((COMPATIBILITY_ADAPTER_USERS - users), key=str)
        fail(f"compatibility adapter user set drifted: unexpected={unexpected}, missing={missing}")

    for forbidden in ("impl Display", "impl std::fmt::Display", "impl From<String>", "serde"):
        if forbidden in carrier:
            fail(f"carrier exposes forbidden API: {forbidden}")
    if "CanonicalGlobalTargetV1::new_static_box_method" not in require_text(
        root, Path("src/mir/builder/callable_declaration_catalog/key.rs")
    ):
        fail("catalog key owner does not expose the typed static-method projection")
    if "direct-call/gc-global-retired" not in require_text(
        root, Path("src/mir/builder/calls/function_call_preflight_route.rs")
    ):
        fail("retired GC Global terminal is not present")

    print(
        f"[{TAG}] typed Callee/CallTarget carrier, legacy String absence, "
        "compatibility boundary, and owner limits ok"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
