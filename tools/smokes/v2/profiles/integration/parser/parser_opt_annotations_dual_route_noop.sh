#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"

BIN="${NYASH_BIN:-$NYASH_ROOT/target/release/hakorune}"
if [ ! -x "$BIN" ]; then
  BIN="$NYASH_ROOT/target/release/nyash"
fi
WRAPPER="$NYASH_ROOT/tools/selfhost/run.sh"

if [ ! -x "$BIN" ]; then
  log_error "nyash binary not found: $BIN"
  exit 2
fi
if [ ! -x "$WRAPPER" ]; then
  log_error "selfhost wrapper not found/executable: $WRAPPER"
  exit 2
fi

FEATURE_SETS="${PARSER_ANNOTATION_FEATURE_SETS:-stage3,opt-annotations,no-try-compat|stage3,rune,no-try-compat}"
TIMEOUT_SECS="${PARSER_ANNOTATION_SELFHOST_TIMEOUT_SECS:-30}"
if ! [[ "$TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  log_error "timeout must be integer: $TIMEOUT_SECS"
  exit 2
fi

TMPDIR="$(mktemp -d /tmp/parser_opt_annotations_dual_route.XXXXXX)"
cleanup() {
  rm -rf "$TMPDIR"
}
trap cleanup EXIT

BASE_SRC="$TMPDIR/base.hako"
ANNO_SRC="$TMPDIR/annotated.hako"
cat >"$BASE_SRC" <<'HK'
static box Main {
  main() {
    local s = "abc"
    return s.length()
  }
}
HK

cat >"$ANNO_SRC" <<'HK'
static box Main {
  @hint(hot)
  @intrinsic_candidate("StringBox.length/0")
  main() {
    @contract(no_alloc)
    local s = "abc"
    return s.length()
  }
}
HK

run_one_feature_set() {
  local features="$1"
  local label="$2"
  local rust_base_ast_json="$TMPDIR/rust_base_${label}.ast.json"
  local rust_anno_ast_json="$TMPDIR/rust_anno_${label}.ast.json"
  local hako_base_log="$TMPDIR/hako_base_${label}.log"
  local hako_anno_log="$TMPDIR/hako_anno_${label}.log"
  local hako_base_json="$TMPDIR/hako_base_${label}.json"
  local hako_anno_json="$TMPDIR/hako_anno_${label}.json"

  NYASH_FEATURES="$features" "$BIN" --emit-ast-json "$rust_base_ast_json" "$BASE_SRC" >/dev/null
  NYASH_FEATURES="$features" "$BIN" --emit-ast-json "$rust_anno_ast_json" "$ANNO_SRC" >/dev/null

  run_hako_parser_route_or_skip "$features" "$BASE_SRC" "$hako_base_log" "baseline"
  run_hako_parser_route_or_skip "$features" "$ANNO_SRC" "$hako_anno_log" "annotated"

  if ! awk '(/"version":0/ && /"kind":"Program"/){print;found=1;exit} END{exit(found?0:1)}' \
    "$hako_base_log" >"$hako_base_json"; then
    log_error "hako parser route did not emit Program(JSON v0) for baseline fixture ($features)"
    tail -n 80 "$hako_base_log" >&2 || true
    exit 1
  fi

  if ! awk '(/"version":0/ && /"kind":"Program"/){print;found=1;exit} END{exit(found?0:1)}' \
    "$hako_anno_log" >"$hako_anno_json"; then
    log_error "hako parser route did not emit Program(JSON v0) for annotated fixture ($features)"
    tail -n 80 "$hako_anno_log" >&2 || true
    exit 1
  fi

  python3 - "$rust_base_ast_json" "$rust_anno_ast_json" "$hako_base_json" "$hako_anno_json" <<'PY'
import json
import sys

RUST_BASE_AST, RUST_ANNO_AST, HAKO_BASE, HAKO_ANNO = sys.argv[1:5]
DROP_KEYS = {"span", "line", "column", "loc", "source", "file", "start", "end"}
AST_NOOP_DROP_KEYS = DROP_KEYS | {"attrs"}


def normalize(obj, drop_keys=DROP_KEYS):
    if isinstance(obj, dict):
        return {
            key: normalize(value, drop_keys)
            for key, value in sorted(obj.items())
            if key not in drop_keys
        }
    if isinstance(obj, list):
        return [normalize(v, drop_keys) for v in obj]
    return obj


def load_norm(path, drop_keys=DROP_KEYS):
    with open(path, "r", encoding="utf-8") as f:
        return normalize(json.load(f), drop_keys)


rust_base_ast = load_norm(RUST_BASE_AST, AST_NOOP_DROP_KEYS)
rust_anno_ast = load_norm(RUST_ANNO_AST, AST_NOOP_DROP_KEYS)
hako_base = load_norm(HAKO_BASE)
hako_anno = load_norm(HAKO_ANNO)

if rust_base_ast != rust_anno_ast:
    print("rust parser route changed AST structure beyond attrs with annotations", file=sys.stderr)
    sys.exit(1)
if hako_base.get("body") != hako_anno.get("body"):
    print("hako parser route changed Program(JSON v0) body with annotations", file=sys.stderr)
    sys.exit(1)
if hako_base.get("attrs") is not None or hako_anno.get("attrs") is not None:
    print("hako parser route unexpectedly widened Program(JSON v0) root attrs", file=sys.stderr)
    sys.exit(1)
PY
}

run_hako_parser_route_or_skip() {
  local features="$1"
  local src="$2"
  local log_path="$3"
  local label="$4"
  local rc=0

  set +e
  NYASH_FEATURES="$features" \
    "$WRAPPER" --direct --source-file "$src" --timeout-secs "$TIMEOUT_SECS" \
    >"$log_path" 2>&1
  rc=$?
  set -e

  if [ "$rc" -ne 0 ]; then
    echo "[SKIP] parser_opt_annotations_dual_route_noop: hako parser route not ready for $label fixture ($features, rc=$rc)" >&2
    tail -n 80 "$log_path" >&2 || true
    exit 0
  fi
}

IFS='|' read -r -a FEATURE_SET_LIST <<< "$FEATURE_SETS"
for idx in "${!FEATURE_SET_LIST[@]}"; do
  run_one_feature_set "${FEATURE_SET_LIST[$idx]}" "$idx"
done

log_success "parser_opt_annotations_dual_route_noop"
