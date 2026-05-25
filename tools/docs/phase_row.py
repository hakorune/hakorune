#!/usr/bin/env python3
"""Small helper for current phase row boilerplate.

The tool is intentionally narrow: it creates a phase card, updates the compact
current-state pointer, and can update the short taskboard queue. It does not
regenerate historical ledgers or rewrite phase history.
"""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
INDEX = ROOT / "docs/tools/check-scripts-index.md"


@dataclass(frozen=True)
class State:
    taskboard: Path
    latest_card: str
    latest_card_path: Path
    current_blocker_token: str


def repo_path(path: str) -> Path:
    p = Path(path)
    if p.is_absolute():
        raise SystemExit(f"path must be repo-relative: {path}")
    return ROOT / p


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def write_text(path: Path, text: str, *, dry_run: bool) -> None:
    if dry_run:
        print(f"[phase-row] would write {path.relative_to(ROOT)}")
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")
    print(f"[phase-row] wrote {path.relative_to(ROOT)}")


def scalar(text: str, key: str) -> str:
    m = re.search(rf'^{re.escape(key)}\s*=\s*"([^"]*)"\s*$', text, re.M)
    if not m:
        raise SystemExit(f"CURRENT_STATE.toml missing scalar: {key}")
    return m.group(1)


def load_state() -> State:
    text = read_text(STATE)
    return State(
        taskboard=repo_path(scalar(text, "taskboard")),
        latest_card=scalar(text, "latest_card"),
        latest_card_path=repo_path(scalar(text, "latest_card_path")),
        current_blocker_token=scalar(text, "current_blocker_token"),
    )


def replace_scalar(text: str, key: str, value: str) -> str:
    pattern = rf'^{re.escape(key)}\s*=\s*"[^"]*"\s*$'
    replacement = f'{key} = "{value}"'
    new, count = re.subn(pattern, replacement, text, count=1, flags=re.M)
    if count != 1:
        raise SystemExit(f"failed to update CURRENT_STATE scalar: {key}")
    return new


def prepend_landed_tail(text: str, summary: str, limit: int) -> str:
    marker = "landed_tail = ["
    start = text.find(marker)
    if start < 0:
        raise SystemExit("CURRENT_STATE.toml missing landed_tail")
    body_start = text.find("\n", start)
    end = text.find("\n]", body_start)
    if body_start < 0 or end < 0:
        raise SystemExit("CURRENT_STATE.toml has malformed landed_tail")

    lines = [line for line in text[body_start + 1 : end].splitlines() if line.strip()]
    entry = f'  "{summary}",'
    lines = [entry] + [line for line in lines if line.strip() != entry.strip()]
    if limit > 0:
        lines = lines[:limit]
    return text[: body_start + 1] + "\n".join(lines) + text[end:]


def card_text(args: argparse.Namespace, card_rel: str) -> str:
    related = [args.previous_card] if args.previous_card else []
    if args.guard:
        related.append(args.guard)
    related_block = "\n".join(f"  - {item}" for item in related)
    selected = ""
    if args.selected_row:
        selected = f"""
## Selected Row

Select:

```text
{args.selected_row}
```
"""
    guard_line = f"\nbash {args.guard}" if args.guard else ""
    return f"""---
Status: {args.status}
Date: {args.date}
Scope: {args.scope}
Blocker: {args.blocker}
Related:
{related_block}
---

# {args.row} {args.title}

## Decision

Close:

```text
{args.blocker}
```

{args.summary}
{selected}
## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash{guard_line}
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
"""


def update_state(args: argparse.Namespace, card_rel: str, dry_run: bool) -> None:
    text = read_text(STATE)
    text = replace_scalar(text, "latest_card", f"{args.row}-{args.slug}")
    text = replace_scalar(text, "latest_card_path", card_rel)
    text = replace_scalar(text, "latest_card_summary", args.summary)
    text = replace_scalar(text, "current_blocker_token", args.blocker)
    text = prepend_landed_tail(text, f"{args.row} {args.summary}", args.landed_tail_limit)
    write_text(STATE, text, dry_run=dry_run)


def update_taskboard(args: argparse.Namespace, state: State, dry_run: bool) -> None:
    if not args.queue_boundary:
        return
    text = read_text(state.taskboard)
    queue_has_row = re.search(rf"^\| {re.escape(args.row_number)} \|", text, flags=re.M) is not None
    blocker_block = (
        "## Current Blocker\n\n```text\n"
        f"{args.blocker}:\n  {args.queue_boundary}\n"
        "```"
    )
    text, count = re.subn(
        r"## Current Blocker\n\n```text\n.*?\n```",
        blocker_block,
        text,
        count=1,
        flags=re.S,
    )
    if count != 1:
        raise SystemExit("failed to update taskboard Current Blocker block")

    if args.land_row:
        row_re = re.compile(rf"^\| {re.escape(args.land_row)} \| .*? \| Current \|", re.M)
        text, count = row_re.subn(lambda m: m.group(0).replace("| Current |", "| Landed |"), text)
        if count == 0:
            print(f"[phase-row] warning: did not find Current queue row {args.land_row}", file=sys.stderr)

    row_line = f"| {args.row_number} | `{args.blocker}` | {args.status} | {args.queue_boundary} |"
    if not queue_has_row:
        queue_match = re.search(r"(## Queue\n\n\|.*?\n)(\n## Full Queue)", text, flags=re.S)
        if not queue_match:
            raise SystemExit("failed to locate taskboard Queue table")
        queue = queue_match.group(1).rstrip() + "\n" + row_line + "\n"
        text = text[: queue_match.start(1)] + queue + text[queue_match.end(1) :]
    write_text(state.taskboard, text, dry_run=dry_run)


def update_index(args: argparse.Namespace, dry_run: bool) -> None:
    if not args.guard or not args.guard_description:
        return
    text = read_text(INDEX)
    if args.guard in text:
        print(f"[phase-row] index already lists {args.guard}")
        return
    line = f"| `{args.guard}` | {args.guard_description} |"
    marker = "| `tools/allocator/"
    pos = text.find(marker)
    if pos < 0:
        text = text.rstrip() + "\n" + line + "\n"
    else:
        text = text[:pos] + line + "\n" + text[pos:]
    write_text(INDEX, text, dry_run=dry_run)


def cmd_create(args: argparse.Namespace) -> int:
    state = load_state()
    phase_dir = state.taskboard.parent
    card_rel = f"{phase_dir.relative_to(ROOT)}/{args.row}-{args.slug}.md"
    card_path = repo_path(card_rel)
    if card_path.exists() and not args.force:
        raise SystemExit(f"card already exists: {card_rel}")

    dry_run = not args.write
    write_text(card_path, card_text(args, card_rel), dry_run=dry_run)
    update_state(args, card_rel, dry_run)
    update_taskboard(args, state, dry_run)
    update_index(args, dry_run)
    if dry_run:
        print("[phase-row] dry-run only; pass --write to modify files")
    return 0


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(description=__doc__)
    sub = p.add_subparsers(dest="cmd", required=True)

    c = sub.add_parser("create", help="create a phase row card and update compact pointers")
    c.add_argument("--write", action="store_true", help="write changes; default is dry-run")
    c.add_argument("--force", action="store_true", help="overwrite an existing card")
    c.add_argument("--date", default="2026-05-25")
    c.add_argument("--row", required=True, help="row id, e.g. 295x-200")
    c.add_argument("--row-number", required=True, help="numeric taskboard row, e.g. 200")
    c.add_argument("--slug", required=True, help="card slug after the row id")
    c.add_argument("--title", required=True)
    c.add_argument("--scope", required=True)
    c.add_argument("--blocker", required=True)
    c.add_argument("--summary", required=True)
    c.add_argument("--status", default="Current")
    c.add_argument("--previous-card")
    c.add_argument("--selected-row")
    c.add_argument("--guard")
    c.add_argument("--guard-description")
    c.add_argument("--queue-boundary")
    c.add_argument("--land-row", help="existing numeric queue row to mark Landed")
    c.add_argument("--landed-tail-limit", type=int, default=12)
    c.set_defaults(func=cmd_create)
    return p


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
