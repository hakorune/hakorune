#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
from __future__ import annotations

from collections import Counter
from pathlib import Path
import re

PHASE_DIR = Path("docs/development/current/main/phases/phase-296x")
CURRENT_STATE = Path("docs/development/current/main/CURRENT_STATE.toml")
STATUS_PATH = PHASE_DIR / "STATUS.md"


def read_current_value(key: str) -> str:
    pattern = re.compile(rf'^{re.escape(key)}\s*=\s*"([^"]*)"')
    for line in CURRENT_STATE.read_text(encoding="utf-8").splitlines():
        match = pattern.match(line)
        if match:
            return match.group(1)
    return ""


def read_status(path: Path) -> str:
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines()[:40]:
        if line.lower().startswith("status:"):
            value = line.split(":", 1)[1].strip().lower()
            return value or "empty"
    return "missing"


def card_number(path_text: str) -> str:
    match = re.search(r"/(296x-\d+)-", path_text)
    if match:
        return match.group(1)
    return "unknown"


visible_cards = sorted(PHASE_DIR.glob("296x-*.md"))
archived_cards = sorted((PHASE_DIR / "archive").glob("*.md"))

status_counts = Counter(read_status(path) for path in visible_cards)
latest_card = read_current_value("latest_card")
latest_card_path = read_current_value("latest_card_path")
latest_number = card_number(latest_card_path)

lines: list[str] = [
    "<!-- @generated — do not edit. Regenerate: bash tools/checks/generate_phase_status.sh -->",
    "# phase-296x Status (generated)",
    "",
    "## counts",
    f"- archived (done): {len(archived_cards)}  (in archive/)",
    f"- visible: {len(visible_cards)}",
    "",
    "## visible by status",
]

for status, count in sorted(status_counts.items(), key=lambda item: (-item[1], item[0])):
    lines.append(f"{count:11d} {status}")

lines.extend(
    [
        "",
        "## in-progress (latest)",
        f"- {latest_card} ({latest_number})",
        "",
        "## rule",
        "- Select / Record / Inventory は commit message で記録し、card 化しない。",
        "- 実タスク(Implement / Write / Fix / Passes)のみ card 化する。",
        "- 詳細: phase-296x/CARD-HYGIENE-RULE.md",
        "",
    ]
)

STATUS_PATH.write_text("\n".join(lines), encoding="utf-8")
PY
