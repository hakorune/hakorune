#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/current-active-rust-lifecycle-guard-resolver-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/current_active_rust_lifecycle_guard_resolver.py"
RUNNER="$ROOT/tools/checks/current_active_rust_lifecycle_guard_resolver.sh"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3345-MIRBUILDER-CURRENT-ACTIVE-RUST-LIFECYCLE-GUARD-RESOLVER-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-current-active-rust-lifecycle-guard-resolver"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$RUNNER" "$CARD" "$TASK_ORDER" "$STATE"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$STATE" <<'PY'
import json
import sys
from pathlib import Path
import tomllib

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CURRENT-ACTIVE-RUST-LIFECYCLE-GUARD-RESOLVER-001"
latest_token = state.get("latest_card")
next_card = state.get("current_blocker_token")

need(fixture.get("kind") == "CurrentActiveRustLifecycleGuardResolverV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(next_card, "CURRENT_STATE blocker missing")

resolution = fixture.get("resolution") or {}
latest = resolution.get("latest") or {}
blocker = resolution.get("current_blocker") or {}
need(latest.get("token") == latest_token, "latest token drift")
need(latest.get("guard_count") == 1, "latest guard count drift")
need(latest.get("status") == "resolved", "latest guard unresolved")
need(blocker.get("token") == next_card, "blocker token drift")
need(blocker.get("guard_count") <= 1, "blocker guard count drift")
need(blocker.get("status") == "pending_card_or_guard", "blocker status drift")
need(resolution.get("runnable_guard_count") == 1, "runnable guard count drift")
need(resolution.get("max_default_guard_count") == 3, "max guard count drift")
need(resolution.get("run_all_historical_guards") is False, "run-all drift")
need(resolution.get("historical_rust_lifecycle_script_count", 0) >= 700, "historical guard count drift")

summary = fixture.get("summary") or {}
for key in ["current_active_guard_resolver", "latest_guard_resolved", "current_blocker_guard_pending"]:
    need(summary.get(key) == 1, f"missing summary claim: {key}")
for key in ["current_blocker_guard_resolved", "run_all_rust_lifecycle_guards_by_default", "source_selfhost_claim"]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")
need(summary.get("runnable_guard_count") == 1, "summary runnable guard count drift")
need(summary.get("selected_next_card") == next_card, "summary next drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "ResolveCurrentActiveRustLifecycleGuards", "decision drift")
need(decision.get("selected_next_card") == next_card, "decision next drift")

claims = fixture.get("claims") or {}
need(claims.get("resolver_only") == 1, "resolver claim drift")
need(claims.get("latest_current_scoped") == 1, "scope claim drift")
for key in [
    "all_rust_lifecycle_guards_in_ci",
    "all_rust_lifecycle_guards_in_dev_gate",
    "hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=current-active-rust-lifecycle-guard-resolver")
print("latest_guard_resolved=1")
print("current_blocker_guard_pending=1")
print("runnable_guard_count=1")
print("run_all_rust_lifecycle_guards_by_default=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY
