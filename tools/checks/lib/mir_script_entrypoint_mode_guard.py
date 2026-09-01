#!/usr/bin/env python3
"""Guard the small, explicit executable surface for direct tool entrypoints.

Only documented front doors are executable.  Shell helpers and smoke bodies
are intentionally invoked through ``bash`` so a clean checkout does not rely
on incidental filesystem modes.
"""

from __future__ import annotations

from pathlib import Path
import stat
import subprocess
import tomllib


TAG = "mir-script-entrypoint-mode"
ROW = "MIR-TOOLS-CANONICAL-ENTRYPOINT-MODE-I0"
CARD_REL = Path(
    "docs/development/current/main/investigations/"
    "mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml"
)
STATE_REL = Path("docs/development/current/main/CURRENT_STATE.toml")
EXPECTED_ENTRYPOINTS = (
    "tools/selfhost/run.sh",
    "tools/selfhost/selfhost_build.sh",
    "tools/smokes/v2/run.sh",
    "tools/smokes/v2/lib/emit_mir_route.sh",
)


def fail(message: str) -> None:
    raise SystemExit(f"[{TAG}] {message}")


def load_toml(path: Path) -> dict:
    try:
        with path.open("rb") as stream:
            value = tomllib.load(stream)
    except (OSError, tomllib.TOMLDecodeError) as exc:
        fail(f"cannot load {path}: {exc}")
    if not isinstance(value, dict):
        fail(f"{path} is not a TOML table")
    return value


def tracked_mode(root: Path, rel: str) -> str:
    result = subprocess.run(
        ["git", "ls-files", "-s", "--", rel],
        cwd=root,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        fail(f"git index lookup failed for {rel}: {result.stderr.strip()}")
    records = [line for line in result.stdout.splitlines() if line.strip()]
    if len(records) != 1:
        fail(f"canonical entrypoint must have one tracked index record: {rel}")
    return records[0].split(maxsplit=1)[0]


def check_entrypoint(root: Path, rel: str) -> None:
    path = root / rel
    if not path.is_file():
        fail(f"canonical entrypoint is missing: {rel}")
    if tracked_mode(root, rel) != "100755":
        fail(f"canonical entrypoint is not tracked as 100755: {rel}")
    if stat.S_IMODE(path.stat().st_mode) & 0o111 == 0:
        fail(f"canonical entrypoint is not executable in the checkout: {rel}")


def check(state: dict, card: dict, root: Path) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("canonical entrypoint mode row requires fast or closeout work_mode")
    if state.get("current_execution_row") != ROW:
        fail("canonical entrypoint mode row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("canonical entrypoint mode row must clear current_design_stop")
    if state.get("next_execution_card") != ROW:
        fail("canonical entrypoint mode row pointer drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("canonical entrypoint mode card pointer drifted")

    rows = [
        value
        for value in card.values()
        if isinstance(value, dict) and value.get("task_id") == ROW
    ]
    if len(rows) != 1:
        fail(f"canonical entrypoint mode manifest row must be unique: {ROW}")
    row = rows[0]
    status = row.get("status")
    if status not in {"fast_open", "landed"}:
        fail("canonical entrypoint mode status is not finite")
    if row.get("implementation_permission") is not (status == "fast_open"):
        fail("canonical entrypoint mode permission/status drifted")

    for rel in EXPECTED_ENTRYPOINTS:
        check_entrypoint(root, rel)

    policy_files = (
        root / "tools/checks/hakorune_emit_mir_direct_caller_guard.sh",
        root / "tools/selfhost/README.md",
        root / "docs/tools/README.md",
    )
    for path in policy_files:
        if not path.is_file():
            fail(f"entrypoint mode policy owner is missing: {path}")
    guard_text = policy_files[0].read_text(encoding="utf-8")
    for rel in EXPECTED_ENTRYPOINTS:
        if rel not in guard_text:
            fail(f"caller guard does not own executable entrypoint: {rel}")
    selfhost_text = policy_files[1].read_text(encoding="utf-8")
    tools_text = policy_files[2].read_text(encoding="utf-8")
    for text, label in ((selfhost_text, "selfhost README"), (tools_text, "tools README")):
        if "noncanonical" not in text or "bash" not in text:
            fail(f"{label} does not document bash invocation for helpers")

    if status == "landed":
        allowed = row.get("allowed_files")
        if not isinstance(allowed, list) or not all(isinstance(item, str) for item in allowed):
            fail("canonical entrypoint mode allowed_files is malformed")
        base = row.get("base_commit")
        if not isinstance(base, str) or not base.strip():
            fail("canonical entrypoint mode landed row lacks base_commit")
        result = subprocess.run(
            ["git", "diff", "--name-only", f"{base}..HEAD"],
            cwd=root,
            capture_output=True,
            text=True,
            check=False,
        )
        if result.returncode != 0:
            fail(f"git diff failed: {result.stderr.strip()}")
        changed = {line for line in result.stdout.splitlines() if line.strip()}
        if not changed.issubset(set(allowed)):
            fail(
                "canonical entrypoint mode changed paths exceed allowed boundary: "
                + repr(sorted(changed - set(allowed)))
            )


if __name__ == "__main__":
    root = Path.cwd()
    check(load_toml(root / STATE_REL), load_toml(root / CARD_REL), root)
    print(f"[{TAG}] row={ROW} ok")
