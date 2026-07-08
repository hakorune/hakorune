#!/usr/bin/env python3
"""Resolve the current latest/blocker rust_lifecycle guards without running all guards."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import tomllib
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
PHASE_DIR = ROOT / "docs/development/current/main/phases/phase-296x"
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "current-active-rust-lifecycle-guard-resolver-v0.json"

TOKEN = "MIRBUILDER-CURRENT-ACTIVE-RUST-LIFECYCLE-GUARD-RESOLVER-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def current_state() -> dict[str, Any]:
    return tomllib.loads(read(STATE))


def card_for_token(token: str, state: dict[str, Any]) -> Path | None:
    latest_path = state.get("latest_card_path")
    if state.get("latest_card") == token and latest_path:
        path = ROOT / latest_path
        if path.exists():
            return path
    matches = sorted(PHASE_DIR.glob(f"*-{token}.md"))
    return matches[0] if matches else None


def extract_guard_paths(card: Path | None) -> list[Path]:
    if card is None:
        return []
    text = read(card)
    guard_section = text.split("## Guard", 1)
    if len(guard_section) != 2:
        return []
    text = guard_section[1].split("\n## ", 1)[0]
    guards: list[Path] = []
    for match in re.findall(r"tools/checks/\s*\n\s*([A-Za-z0-9_./-]+\.sh)", text):
        path = ROOT / "tools/checks" / match
        if path.exists():
            guards.append(path)
    for match in re.findall(r"(tools/checks/[A-Za-z0-9_./-]+\.sh)", text):
        path = ROOT / match
        if path.exists() and path not in guards:
            guards.append(path)
    return guards


def resolve_token(token: str, state: dict[str, Any]) -> dict[str, Any]:
    card = card_for_token(token, state)
    guards = extract_guard_paths(card)
    status = "resolved" if guards else "pending_card_or_guard"
    return {
        "token": token,
        "card": rel(card) if card else None,
        "guard_paths": [rel(path) for path in guards],
        "guard_count": len(guards),
        "status": status,
    }


def rust_lifecycle_script_count() -> int:
    return len(list((ROOT / "tools/checks").glob("rust_lifecycle*.sh")))


def build_fixture() -> dict[str, Any]:
    state = current_state()
    latest = resolve_token(str(state["latest_card"]), state)
    blocker = resolve_token(str(state["current_blocker_token"]), state)
    runnable_guards = latest["guard_paths"] + blocker["guard_paths"]
    selected_next = str(state["current_blocker_token"])

    return {
        "schema_version": 0,
        "kind": "CurrentActiveRustLifecycleGuardResolverV1",
        "token": TOKEN,
        "source_files": {
            rel(STATE): sha256_file(STATE),
            rel(TASK_ORDER): sha256_file(TASK_ORDER),
        },
        "current_state": {
            "latest_card": state["latest_card"],
            "latest_card_path": state["latest_card_path"],
            "current_blocker_token": state["current_blocker_token"],
        },
        "resolution": {
            "latest": latest,
            "current_blocker": blocker,
            "runnable_guard_paths": runnable_guards,
            "runnable_guard_count": len(runnable_guards),
            "max_default_guard_count": 3,
            "run_all_historical_guards": False,
            "historical_rust_lifecycle_script_count": rust_lifecycle_script_count(),
        },
        "summary": {
            "current_active_guard_resolver": 1,
            "latest_guard_resolved": 1 if latest["guard_count"] > 0 else 0,
            "current_blocker_guard_resolved": 1 if blocker["guard_count"] > 0 else 0,
            "current_blocker_guard_pending": 1 if blocker["guard_count"] == 0 else 0,
            "runnable_guard_count": len(runnable_guards),
            "max_default_guard_count": 3,
            "run_all_rust_lifecycle_guards_by_default": 0,
            "selected_next_card": selected_next,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "ResolveCurrentActiveRustLifecycleGuards",
            "reason_token": "LatestCurrentScopedGuardSet",
            "selected_next_card": selected_next,
        },
        "claims": {
            "resolver_only": 1,
            "latest_current_scoped": 1,
            "all_rust_lifecycle_guards_in_ci": 0,
            "all_rust_lifecycle_guards_in_dev_gate": 0,
            "hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "source_selfhost_claim": 0,
        },
    }


def run_resolved_guards(fixture: dict[str, Any]) -> int:
    paths = fixture["resolution"]["runnable_guard_paths"]
    if len(paths) > fixture["resolution"]["max_default_guard_count"]:
        raise SystemExit("refusing to run too many active guards")
    for rel_path in paths:
        print(f"[current-active-guard-resolver] running {rel_path}")
        subprocess.run([str(ROOT / rel_path)], cwd=ROOT, check=True)
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    parser.add_argument("--run", action="store_true", help="Run resolved active guards.")
    args = parser.parse_args()

    fixture = build_fixture()
    output = stable_json(fixture)
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("current-active-rust-lifecycle-guard-resolver unchanged")
        return 0
    if args.run:
        return run_resolved_guards(fixture)

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
