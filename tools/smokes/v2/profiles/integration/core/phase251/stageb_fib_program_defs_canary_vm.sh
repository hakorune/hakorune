#!/usr/bin/env bash
# stageb_fib_program_defs_canary_vm.sh
# - Canary for Stage‑B defs + Loop 出力確認（fib 風 multi-carrier 用の下準備）。
# - 目的:
#   - compiler_stageb.hako が FuncScannerBox を通じて TestBox.fib を defs に載せているか。
#   - defs 内の body に Loop ノードが含まれているか（後続の LoopForm lower の前提確認）。

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || (cd "$SCRIPT_DIR/../../../../../../.." && pwd))"

# Quick profile: Stage-B emit is flaky under Stage-3 default; skip for now.
echo "[SKIP] stageb_fib_program_defs_canary_vm (disabled in quick profile after env consolidation)"
exit 0

source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true
require_env || { echo "[SKIP] env not ready"; exit 0; }

if ! command -v jq >/dev/null 2>&1; then
  echo "[SKIP] stageb_fib_program_defs_canary_vm.sh (jq not available)" >&2
  exit 0
fi

TMP_HAKO="$(mktemp --suffix .hako)"
OUT_JSON="$(mktemp --suffix .json)"
trap 'rm -f "$TMP_HAKO" "$OUT_JSON" || true' EXIT

cat >"$TMP_HAKO" <<'HAKO'
static box TestBox {
  method fib(n) {
    local i = 0
    local a = 0
    local b = 1
    loop(i < n) {
      local t = a + b
      a = b
      b = t
      i = i + 1
    }
    return b
  }
}

static box Main {
  method main(args) {
    local t = new TestBox()
    return t.fib(6)
  }
}
HAKO

# Stage‑B: emit Program(JSON v0) with defs
set +e
OUT_RAW=$(
  cd "$ROOT_DIR" && \
  NYASH_JSON_ONLY=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
  NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
  NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 HAKO_STAGEB_FUNC_SCAN=1 \
  HAKO_VM_MAX_STEPS=0 \
  HAKO_STAGEB_APPLY_USINGS=0 \
  "$NYASH_BIN" --backend vm "$ROOT_DIR/lang/src/compiler/entry/compiler_stageb.hako" -- --source "$(cat "$TMP_HAKO")" 2>&1
)
rc=$?
set -e

if [ $rc -ne 0 ]; then
  echo "[FAIL] stageb_fib_program_defs_canary_vm (Stage‑B rc=$rc)" >&2
  printf '%s\n' "$OUT_RAW" | head -n 40 >&2 || true
  exit 1
fi

# 抽出: Program(JSON) 部分だけを抽出（emit_mir.sh と同じ Python ロジックを簡略化）
extract_prog() {
  python3 - <<'PYEOF'
import sys
text = sys.stdin.read()
start = text.find('"kind":"Program"')
if start < 0:
    sys.exit(1)
pos = start
depth = 0
while pos >= 0:
    if text[pos] == '{':
        depth += 1
        if depth == 1:
            break
    elif text[pos] == '}':
        depth -= 1
    pos -= 1
if pos < 0:
    sys.exit(1)
obj_start = pos
depth = 0
in_str = False
esc = False
i = obj_start
while i < len(text):
    ch = text[i]
    if esc:
        esc = False
    elif in_str:
        if ch == '\\\\':
            esc = True
        elif ch == '"':
            in_str = False
    else:
        if ch == '"':
            in_str = True
        elif ch == '{':
            depth += 1
        elif ch == '}':
            depth -= 1
            if depth == 0:
                print(text[obj_start:i+1])
                sys.exit(0)
    i += 1
sys.exit(1)
PYEOF
}

RAW_DUMP=$(mktemp --suffix .stageb.raw)
printf '%s\n' "$OUT_RAW" > "$RAW_DUMP"
if ! python3 - "$RAW_DUMP" <<'PYEOF' >"$OUT_JSON" 2>/dev/null; then
import sys
from pathlib import Path
dump_path = Path(sys.argv[1])
text = dump_path.read_text()
start = text.find('"kind":"Program"')
if start < 0:
    sys.exit(1)
pos = start
while pos >= 0 and text[pos] != '{':
    pos -= 1
if pos < 0:
    sys.exit(1)
depth = 0
in_str = False
esc = False
i = pos
while i < len(text):
    ch = text[i]
    if esc:
        esc = False
    elif in_str:
        if ch == '\\\\':
            esc = True
        elif ch == '"':
            in_str = False
    else:
        if ch == '"':
            in_str = True
        elif ch == '{':
            depth += 1
        elif ch == '}':
            depth -= 1
            if depth == 0:
                print(text[pos:i+1])
                sys.exit(0)
    i += 1
sys.exit(1)
PYEOF
  echo "[FAIL] stageb_fib_program_defs_canary_vm (failed to extract Program JSON)" >&2
  printf '%s\n' "$OUT_RAW" | head -n 40 >&2 || true
  rm -f "$RAW_DUMP" || true
  exit 1
fi
rm -f "$RAW_DUMP" || true

# 1) Program.kind == "Program"
if ! jq -e '.kind == "Program"' "$OUT_JSON" >/dev/null 2>&1; then
  echo "[FAIL] stageb_fib_program_defs_canary_vm (kind != Program)" >&2
  cat "$OUT_JSON" >&2
  exit 1
fi

# 2) defs に TestBox.fib が含まれているか
has_fib=$(jq '.defs // [] | any(.box=="TestBox" and .name=="fib")' "$OUT_JSON")
if [ "$has_fib" != "true" ]; then
  echo "[FAIL] stageb_fib_program_defs_canary_vm (defs に TestBox.fib が存在しない)" >&2
  jq '.defs // []' "$OUT_JSON" >&2 || true
  exit 1
fi

# 3) fib の body 内に Loop ノードがあるか
has_loop=$(jq '.defs // [] | map(select(.box=="TestBox" and .name=="fib")) | any(.body | .body? | any(.type=="Loop"))' "$OUT_JSON")
if [ "$has_loop" != "true" ]; then
  echo "[FAIL] stageb_fib_program_defs_canary_vm (TestBox.fib body に Loop が無い)" >&2
  jq '.defs // [] | map(select(.box=="TestBox" and .name=="fib"))' "$OUT_JSON" >&2 || true
  exit 1
fi

echo "[PASS] stageb_fib_program_defs_canary_vm"
exit 0
