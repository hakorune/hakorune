#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-root-facade-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

MIR_ROOT="$ROOT_DIR/src/mir/mod.rs"
ALLOWLIST="$ROOT_DIR/tools/checks/mir_root_facade_allowlist.txt"
CONTRACT="$ROOT_DIR/docs/development/current/main/design/mir-root-facade-contract-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$MIR_ROOT" "$ALLOWLIST" "$CONTRACT"

echo "[$TAG] checking MIR root facade export allowlist"

python3 - "$ROOT_DIR" "$MIR_ROOT" "$ALLOWLIST" <<'PY'
import pathlib
import re
import sys

tag = "mir-root-facade-guard"
root = pathlib.Path(sys.argv[1]).resolve()
mir_root = pathlib.Path(sys.argv[2]).resolve()
allowlist_path = pathlib.Path(sys.argv[3]).resolve()


def fail(message: str) -> None:
    print(f"[{tag}] ERROR: {message}", file=sys.stderr)
    sys.exit(1)


def load_allowlist() -> set[str]:
    symbols: set[str] = set()
    for lineno, raw in enumerate(allowlist_path.read_text(encoding="utf-8").splitlines(), 1):
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        if not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", line):
            fail(f"bad allowlist symbol at line {lineno}: {line}")
        if line in symbols:
            fail(f"duplicate allowlist symbol at line {lineno}: {line}")
        symbols.add(line)
    if not symbols:
        fail("allowlist is empty")
    return symbols


def exported_symbol(part: str) -> str:
    item = part.strip()
    if not item:
        return ""
    if item == "*":
        fail("wildcard pub use is forbidden in src/mir/mod.rs")
    if " as " in item:
        item = item.split(" as ", 1)[1].strip()
    return item.split("::")[-1].strip()


def parse_pub_exports() -> set[str]:
    text = mir_root.read_text(encoding="utf-8")
    exports: set[str] = set()
    pos = 0
    while True:
        match = re.search(r"(?m)^pub use ", text[pos:])
        if not match:
            break
        start = pos + match.start()
        end = text.find(";", start)
        if end < 0:
            fail("unterminated pub use in src/mir/mod.rs")
        statement = text[start : end + 1]
        pos = end + 1

        statement = re.sub(r"//.*", "", statement)
        rest = statement[len("pub use ") : -1].strip()
        if "*" in rest:
            fail(f"wildcard pub use is forbidden: {rest}")
        if "{" in rest:
            inner = rest.split("{", 1)[1].rsplit("}", 1)[0]
            parts = inner.split(",")
        else:
            parts = [rest]
        for part in parts:
            symbol = exported_symbol(part)
            if not symbol:
                continue
            if symbol in exports:
                fail(f"duplicate root export symbol: {symbol}")
            exports.add(symbol)
    return exports


allowlist = load_allowlist()
actual = parse_pub_exports()
extra = sorted(actual - allowlist)
missing = sorted(allowlist - actual)

if extra:
    for symbol in extra:
        print(
            f"[{tag}] ERROR: root export not in allowlist: {symbol}",
            file=sys.stderr,
        )
if missing:
    for symbol in missing:
        print(
            f"[{tag}] ERROR: allowlist symbol missing from root exports: {symbol}",
            file=sys.stderr,
        )
if extra or missing:
    rel_allowlist = allowlist_path.relative_to(root).as_posix()
    rel_contract = "docs/development/current/main/design/mir-root-facade-contract-ssot.md"
    fail(
        "MIR root facade drift detected; update "
        f"{rel_allowlist} only with a phase card and {rel_contract} gate rationale"
    )

print(f"[{tag}] ok exports={len(actual)}")
PY
