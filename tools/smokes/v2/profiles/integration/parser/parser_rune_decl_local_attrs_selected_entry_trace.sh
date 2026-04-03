#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"

BIN="${NYASH_BIN:-$NYASH_ROOT/target/release/hakorune}"
if [ ! -x "$BIN" ]; then
  BIN="$NYASH_ROOT/target/release/nyash"
fi
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
FFI_LIB="$NYASH_ROOT/target/release/libhako_llvmc_ffi.so"

if [ ! -x "$BIN" ]; then
  log_error "nyash/hakorune binary not found: $BIN"
  exit 2
fi

FEATURES="${PARSER_RUNE_FEATURES:-stage3,rune}"

TMPDIR="$(mktemp -d /tmp/parser_rune_decl_local_attrs.XXXXXX)"
cleanup() {
  rm -rf "$TMPDIR"
}
trap cleanup EXIT

SRC="$TMPDIR/rune_decl_local_attrs.hako"
STAGEB_SRC="$TMPDIR/rune_decl_local_attrs_stageb.hako"
INVALID_SRC="$TMPDIR/rune_invalid_placement.hako"
INVALID_CALLCONV_SRC="$TMPDIR/rune_invalid_callconv.hako"
INVALID_OWNERSHIP_SRC="$TMPDIR/rune_invalid_ownership.hako"
AST_LOG="$TMPDIR/ast_json.log"
AST_JSON="$TMPDIR/ast.json"
STAGEB_RAW="$TMPDIR/stageb_program_raw.log"
STAGEB_JSON="$TMPDIR/stageb_program.json"
MIR_JSON="$TMPDIR/mir.json"
MIR_JSON_MUT="$TMPDIR/mir_mut.json"
MIR_LOG="$TMPDIR/mir_json.log"
BUILD_LOG="$TMPDIR/hakorune_build.log"
TRACE_LOG="$TMPDIR/ny_llvmc_trace.log"
OUT_OBJ="$TMPDIR/out.o"

cat >"$SRC" <<'HK'
@rune Public
static box Main {
  helper() {
    return 0
  }

  @rune Hint(hot)
  @contract(no_alloc)
  @intrinsic_candidate("StringBox.length/0")
  @rune Symbol("main_sym")
  @rune CallConv("c")
  main() {
    return 0
  }
}
HK

cat >"$STAGEB_SRC" <<'HK'
static box Main {
  @rune Hint(hot)
  @contract(no_alloc)
  @intrinsic_candidate("StringBox.length/0")
  @rune Symbol("main_sym")
  @rune CallConv("c")
  main(args) {
    return 0
  }
}
HK

cat >"$INVALID_SRC" <<'HK'
static box Main {
  main() {
    @rune Public
    local x = 1
    return x
  }
}
HK

cat >"$INVALID_CALLCONV_SRC" <<'HK'
static box Main {
  @rune CallConv("sysv")
  main() {
    return 0
  }
}
HK

cat >"$INVALID_OWNERSHIP_SRC" <<'HK'
static box Main {
  @rune Ownership(unique)
  main() {
    return 0
  }
}
HK

if ! cargo build --release -q --bin hakorune >"$BUILD_LOG" 2>&1; then
  log_error "hakorune release build failed"
  tail -n 120 "$BUILD_LOG" >&2 || true
  exit 1
fi

check_direct_failfast() {
  local src_path="$1"
  local expect="$2"
  local label="$3"
  local log_path="$TMPDIR/${label}.log"

  if NYASH_FEATURES="$FEATURES" \
    "$NYASH_ROOT/tools/selfhost/run.sh" --direct --source-file "$src_path" \
    >"$log_path" 2>&1; then
    log_error ".hako direct parser route unexpectedly accepted $label"
    tail -n 120 "$log_path" >&2 || true
    exit 1
  fi

  if ! grep -Fq "$expect" "$log_path"; then
    log_error ".hako direct parser route did not emit expected rune fail-fast tag for $label"
    tail -n 120 "$log_path" >&2 || true
    exit 1
  fi
}

check_direct_failfast \
  "$INVALID_SRC" \
  '[freeze:contract][parser/rune] invalid placement on statement' \
  "invalid_placement"

check_direct_failfast \
  "$INVALID_CALLCONV_SRC" \
  '[freeze:contract][parser/rune] CallConv("c")' \
  "invalid_callconv"

check_direct_failfast \
  "$INVALID_OWNERSHIP_SRC" \
  '[freeze:contract][parser/rune] Ownership(owned|borrowed|shared)' \
  "invalid_ownership"

if ! NYASH_FEATURES="$FEATURES" \
  "$BIN" --emit-ast-json "$AST_JSON" "$SRC" \
  >"$AST_LOG" 2>&1; then
  log_error "selfhost AST JSON emit failed"
  tail -n 120 "$AST_LOG" >&2 || true
  exit 1
fi

if [ ! -s "$AST_JSON" ]; then
  log_error "selfhost AST JSON missing output"
  tail -n 120 "$AST_LOG" >&2 || true
  exit 1
fi

python3 - "$AST_JSON" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as f:
    data = json.load(f)

def walk(node):
    if isinstance(node, dict):
        yield node
        for value in node.values():
            yield from walk(value)
    elif isinstance(node, list):
        for item in node:
            yield from walk(item)

box = None
for node in walk(data):
    if node.get("kind") == "BoxDeclaration" and node.get("name") == "Main":
        box = node
        break
if not isinstance(box, dict):
    print("missing BoxDeclaration(Main)", file=sys.stderr)
    sys.exit(1)

attrs = box.get("attrs") if isinstance(box, dict) else None
runes = attrs.get("runes") if isinstance(attrs, dict) else None
if not isinstance(runes, list):
    print("box declaration missing attrs.runes", file=sys.stderr)
    sys.exit(1)

names = [entry.get("name") for entry in runes if isinstance(entry, dict)]
if names != ["Public"]:
    print(f"unexpected box rune names: {names}", file=sys.stderr)
    sys.exit(1)

methods = {}
for entry in box.get("methods", []):
    if isinstance(entry, dict):
        key = entry.get("key")
        decl = entry.get("decl")
        if isinstance(key, str) and isinstance(decl, dict):
            methods[key] = decl

main_decl = methods.get("main")
helper_decl = methods.get("helper")
if not isinstance(main_decl, dict):
    print("missing main declaration in AST JSON", file=sys.stderr)
    sys.exit(1)
if not isinstance(helper_decl, dict):
    print("missing helper declaration in AST JSON", file=sys.stderr)
    sys.exit(1)

main_attrs = main_decl.get("attrs") if isinstance(main_decl, dict) else None
main_runes = main_attrs.get("runes") if isinstance(main_attrs, dict) else None
if not isinstance(main_runes, list):
    print("main declaration missing attrs.runes", file=sys.stderr)
    sys.exit(1)

main_names = [entry.get("name") for entry in main_runes if isinstance(entry, dict)]
if main_names != ["Hint", "Contract", "IntrinsicCandidate", "Symbol", "CallConv"]:
    print(f"unexpected main rune names: {main_names}", file=sys.stderr)
    sys.exit(1)

main_args0 = main_runes[0].get("args") if isinstance(main_runes[0], dict) else None
main_args1 = main_runes[1].get("args") if isinstance(main_runes[1], dict) else None
main_args2 = main_runes[2].get("args") if isinstance(main_runes[2], dict) else None
main_args3 = main_runes[3].get("args") if isinstance(main_runes[3], dict) else None
main_args4 = main_runes[4].get("args") if isinstance(main_runes[4], dict) else None
if (
    main_args0 != ["hot"]
    or main_args1 != ["no_alloc"]
    or main_args2 != ["StringBox.length/0"]
    or main_args3 != ["main_sym"]
    or main_args4 != ["c"]
):
    print("unexpected main rune args on declaration-local attrs", file=sys.stderr)
    sys.exit(1)

helper_attrs = helper_decl.get("attrs") if isinstance(helper_decl, dict) else None
helper_runes = helper_attrs.get("runes") if isinstance(helper_attrs, dict) else None
if helper_runes != []:
    print(f"expected helper attrs.runes to be empty, got: {helper_runes}", file=sys.stderr)
    sys.exit(1)
PY

if ! NYASH_FEATURES="$FEATURES" \
  NYASH_SELFHOST_STAGEB_PROOF_ONLY=1 \
  "$NYASH_ROOT/tools/selfhost/run_stageb_compiler_vm.sh" --source-file "$STAGEB_SRC" \
  >"$STAGEB_RAW" 2>&1; then
  log_error "selfhost Stage-B program emit failed"
  tail -n 120 "$STAGEB_RAW" >&2 || true
  exit 1
fi

if ! awk '(/"version":0/ && /"kind":"Program"/){print;found=1;exit} END{exit(found?0:1)}' \
  "$STAGEB_RAW" >"$STAGEB_JSON"; then
  log_error "selfhost Stage-B program JSON missing output"
  tail -n 120 "$STAGEB_RAW" >&2 || true
  exit 1
fi

python3 - "$STAGEB_JSON" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as f:
    data = json.load(f)

if data.get("attrs") is not None:
    print("Program(JSON v0) root unexpectedly widened with attrs", file=sys.stderr)
    sys.exit(1)

defs = data.get("defs")
if not isinstance(defs, list):
    print("defs array missing from Stage-B program JSON", file=sys.stderr)
    sys.exit(1)

main_def = None
for entry in defs:
    if isinstance(entry, dict) and entry.get("box") == "Main" and entry.get("name") == "main":
        main_def = entry
        break

if not isinstance(main_def, dict):
    print("real Main.main def carrier missing from defs", file=sys.stderr)
    sys.exit(1)

if main_def.get("params") != ["args"]:
    print(f"unexpected Main.main params: {main_def.get('params')}", file=sys.stderr)
    sys.exit(1)

runes = main_def.get("attrs", {}).get("runes")
if not isinstance(runes, list):
    print("Main.main attrs.runes missing from Stage-B program JSON", file=sys.stderr)
    sys.exit(1)

names = [entry.get("name") for entry in runes if isinstance(entry, dict)]
if names != ["Hint", "Contract", "IntrinsicCandidate", "Symbol", "CallConv"]:
    print(f"unexpected Stage-B carrier rune names: {names}", file=sys.stderr)
    sys.exit(1)

args0 = runes[0].get("args") if isinstance(runes[0], dict) else None
args1 = runes[1].get("args") if isinstance(runes[1], dict) else None
args2 = runes[2].get("args") if isinstance(runes[2], dict) else None
args3 = runes[3].get("args") if isinstance(runes[3], dict) else None
args4 = runes[4].get("args") if isinstance(runes[4], dict) else None
if (
    args0 != ["hot"]
    or args1 != ["no_alloc"]
    or args2 != ["StringBox.length/0"]
    or args3 != ["main_sym"]
    or args4 != ["c"]
):
    print("unexpected Stage-B carrier rune args", file=sys.stderr)
    sys.exit(1)
PY

if ! NYASH_FEATURES="$FEATURES" \
  "$BIN" --emit-mir-json "$MIR_JSON" "$SRC" \
  >"$MIR_LOG" 2>&1; then
  log_error "selfhost MIR JSON emit failed"
  tail -n 120 "$MIR_LOG" >&2 || true
  exit 1
fi

if [ ! -s "$MIR_JSON" ]; then
  log_error "selfhost MIR JSON missing output"
  tail -n 120 "$MIR_LOG" >&2 || true
  exit 1
fi

python3 - "$MIR_JSON" "$MIR_JSON_MUT" <<'PY'
import json
import sys

src, dst = sys.argv[1:3]
with open(src, "r", encoding="utf-8") as f:
    data = json.load(f)

functions = data.get("functions")
if not isinstance(functions, list) or len(functions) < 2:
    print("missing MIR functions", file=sys.stderr)
    sys.exit(1)

helper = None
main = None
for fn in functions:
    if isinstance(fn, dict) and fn.get("name") == "main":
        main = fn

if not isinstance(main, dict):
    print("missing MIR main function", file=sys.stderr)
    sys.exit(1)

main_attrs = main.get("attrs") if isinstance(main, dict) else None
main_runes = main_attrs.get("runes") if isinstance(main_attrs, dict) else None
if not isinstance(main_runes, list):
    print("main function missing attrs.runes", file=sys.stderr)
    sys.exit(1)

names = [entry.get("name") for entry in main_runes if isinstance(entry, dict)]
if names != ["Hint", "Contract", "IntrinsicCandidate", "Symbol", "CallConv"]:
    print(f"unexpected MIR rune names: {names}", file=sys.stderr)
    sys.exit(1)

main_args0 = main_runes[0].get("args") if isinstance(main_runes[0], dict) else None
main_args1 = main_runes[1].get("args") if isinstance(main_runes[1], dict) else None
main_args2 = main_runes[2].get("args") if isinstance(main_runes[2], dict) else None
main_args3 = main_runes[3].get("args") if isinstance(main_runes[3], dict) else None
main_args4 = main_runes[4].get("args") if isinstance(main_runes[4], dict) else None
if (
    main_args0 != ["hot"]
    or main_args1 != ["no_alloc"]
    or main_args2 != ["StringBox.length/0"]
    or main_args3 != ["main_sym"]
    or main_args4 != ["c"]
):
    print("unexpected MIR rune args on main attrs", file=sys.stderr)
    sys.exit(1)

helper_entry = {
    "name": "helper",
    "params": [],
    "attrs": {
        "runes": [
            {"name": "Symbol", "args": ["helper_sym"]},
            {"name": "CallConv", "args": ["fastcc"]},
        ]
    },
    "blocks": [
        {
            "id": 0,
            "instructions": [
                {"op": "const", "dst": 1, "value": {"type": "i64", "value": 0}},
                {"op": "ret", "value": 1},
            ],
        }
    ],
}
functions.insert(0, helper_entry)

main["attrs"] = {
    "runes": [
        {"name": "Hint", "args": ["hot"]},
        {"name": "Contract", "args": ["no_alloc"]},
        {"name": "IntrinsicCandidate", "args": ["StringBox.length/0"]},
        {"name": "Symbol", "args": ["main_sym"]},
        {"name": "CallConv", "args": ["c"]},
    ]
}

with open(dst, "w", encoding="utf-8") as f:
    json.dump(data, f, separators=(",", ":"))
PY

bash "$NYASH_ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null
cargo build --release -q -p nyash-llvm-compiler --bin ny-llvmc >/dev/null

if [ ! -x "$NY_LLVM_C" ]; then
  log_error "ny-llvmc binary not found: $NY_LLVM_C"
  exit 2
fi
if [ ! -f "$FFI_LIB" ]; then
  log_error "hako_llvmc ffi lib not found: $FFI_LIB"
  exit 2
fi

NYASH_RUNE_TRACE=1 \
  "$NY_LLVM_C" --in "$MIR_JSON_MUT" --emit obj --out "$OUT_OBJ" \
  >"$TRACE_LOG" 2>&1

if [ ! -f "$OUT_OBJ" ]; then
  log_error "ny-llvmc did not produce object"
  tail -n 120 "$TRACE_LOG" >&2 || true
  exit 1
fi

if ! grep -Fq '[rune/entry] selected attrs Symbol(main_sym) CallConv(c)' "$TRACE_LOG"; then
  log_error "ny-llvmc trace did not report selected main rune attrs"
  tail -n 120 "$TRACE_LOG" >&2 || true
  exit 1
fi

if grep -Fq 'helper_sym' "$TRACE_LOG"; then
  log_error "ny-llvmc trace should not read helper entry attrs"
  tail -n 120 "$TRACE_LOG" >&2 || true
  exit 1
fi

log_success "parser_rune_decl_local_attrs_selected_entry_trace"
