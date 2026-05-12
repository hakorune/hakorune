#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-record-construction-read"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-215-C205B-ALLOCATOR-RECORD-CONSTRUCTION-READ-LOWERING.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_allocator_record_construction_read_guard.sh"

echo "[$TAG] checking C205b allocator record construction/read lowering"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$PHASE_README" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "src/mir/builder/record_values.rs" \
  "src/mir/builder/builder_build.rs" \
  "src/mir/builder/fields.rs" \
  "src/mir/builder/stmts/variable_stmt.rs" \
  "src/mir/builder/compilation_context.rs" \
  "lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako" \
  "lang/src/hako_alloc/memory/huge_page_model_box.hako"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C205b card must be complete"
guard_expect_in_file "$TAG" 'C205b status:' "$PLAN" "mimalloc plan must record C205b status"
guard_expect_in_file "$TAG" '`C205b` is complete as `293x-215`' "$RECORD_SSOT" "record SSOT must mark C205b complete"
guard_expect_in_file "$TAG" '`293x-215`' "$PHASE_README" "phase README must list C205b row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C205b guard"

guard_expect_in_file "$TAG" 'RecordValueScalarizationBox' "src/mir/builder/record_values.rs" "record scalarization owner must be explicit"
guard_expect_in_file "$TAG" 'build_record_constructor_value' "src/mir/builder/record_values.rs" "record constructor helper must exist"
guard_expect_in_file "$TAG" 'try_lower_record_field_read_from_ast' "src/mir/builder/record_values.rs" "record field read helper must exist"
guard_expect_in_file "$TAG" '\[record-construction/escape\]' "src/mir/builder/builder_build.rs" "record construction must not silently fall through to NewBox"
guard_expect_in_file "$TAG" '\[record-value/escape\]' "src/mir/builder/record_values.rs" "record value escape must fail fast"
guard_expect_in_file "$TAG" '\[record-field-read/unknown-field\]' "src/mir/builder/record_values.rs" "unknown record field must fail fast"
guard_expect_in_file "$TAG" 'record_local_values' "src/mir/builder/compilation_context.rs" "builder-local record values must be tracked separately"

guard_expect_in_file "$TAG" 'meta_ptrs: ArrayBox = new ArrayBox\(\)' "lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako" "M178 scalar ptr metadata must remain runtime truth"
guard_expect_in_file "$TAG" 'page_ids: ArrayBox = new ArrayBox\(\)' "lang/src/hako_alloc/memory/huge_page_model_box.hako" "M180 scalar huge-page metadata must remain runtime truth"

if rg -n 'new HakoAllocAlignedSmallMeta|new HakoAllocHugePageMeta|ArrayStorage::InlineRecord|InlineRecord' \
  lang/src/hako_alloc -g'*.hako' >/tmp/"$TAG".hako_alloc 2>&1; then
  echo "[$TAG] ERROR: C205b must not migrate live hako_alloc metadata storage yet" >&2
  cat /tmp/"$TAG".hako_alloc >&2
  rm -f /tmp/"$TAG".hako_alloc
  exit 1
fi
rm -f /tmp/"$TAG".hako_alloc

if rg -n 'RecordValueScalarizationBox|record-construction|record-field-read|HakoAllocAlignedSmallMeta|HakoAllocHugePageMeta' \
  lang/c-abi/shims src/llvm_py/instructions >/tmp/"$TAG".backend 2>&1; then
  echo "[$TAG] ERROR: C205b record scalarization leaked into backend lowering surfaces" >&2
  cat /tmp/"$TAG".backend >&2
  rm -f /tmp/"$TAG".backend
  exit 1
fi
rm -f /tmp/"$TAG".backend

ok_src="$(mktemp /tmp/hakorune_c205b_record_ok.XXXXXX.hako)"
ok_json="$(mktemp /tmp/hakorune_c205b_record_ok.XXXXXX.json)"
escape_src="$(mktemp /tmp/hakorune_c205b_record_escape.XXXXXX.hako)"
ctor_escape_src="$(mktemp /tmp/hakorune_c205b_record_ctor_escape.XXXXXX.hako)"
unknown_src="$(mktemp /tmp/hakorune_c205b_record_unknown.XXXXXX.hako)"
trap 'rm -f "$ok_src" "$ok_json" "$escape_src" "$ctor_escape_src" "$unknown_src"' EXIT

cat >"$ok_src" <<'HAKO'
record ProbeMeta {
    ptr: i64
    alignment: i64
}

static box Main {
    main(args) {
        local meta = new ProbeMeta(41, 8)
        local value = meta.ptr
        return value
    }
}
HAKO

NYASH_DISABLE_PLUGINS=1 cargo run -q --bin hakorune -- --emit-mir-json "$ok_json" "$ok_src" \
  >/tmp/"$TAG".ok.out 2>/tmp/"$TAG".ok.err

python3 - "$ok_json" <<'PY'
import json
import sys

path = sys.argv[1]
data = json.load(open(path))
functions = data.get("functions", [])
main = next((f for f in functions if f.get("name") == "main"), None)
if main is None:
    raise SystemExit("main function not found")

def walk(value):
    if isinstance(value, dict):
        yield value
        for child in value.values():
            yield from walk(child)
    elif isinstance(value, list):
        for child in value:
            yield from walk(child)

nodes = list(walk(main))
if any(node.get("op") == "newbox" and node.get("type") == "ProbeMeta" for node in nodes):
    raise SystemExit("record construction leaked to NewBox in main")
if any(node.get("op") == "field_get" and node.get("field") == "ptr" for node in nodes):
    raise SystemExit("record field read leaked to FieldGet in main")
if not any(node.get("op") == "const" and node.get("value", {}).get("value") == 41 for node in nodes):
    raise SystemExit("scalarized record field value not found in main")
PY

cat >"$escape_src" <<'HAKO'
record ProbeMeta {
    ptr: i64
}

static box Main {
    main(args) {
        local meta = new ProbeMeta(41)
        return meta
    }
}
HAKO

if NYASH_DISABLE_PLUGINS=1 cargo run -q --bin hakorune -- --emit-mir-json /tmp/"$TAG".escape.json "$escape_src" \
  >/tmp/"$TAG".escape.out 2>/tmp/"$TAG".escape.err; then
  echo "[$TAG] ERROR: escaped record value unexpectedly compiled" >&2
  exit 1
fi
guard_expect_in_file "$TAG" '\[record-value/escape\]' /tmp/"$TAG".escape.err "escaped record variable must fail fast"

cat >"$ctor_escape_src" <<'HAKO'
record ProbeMeta {
    ptr: i64
}

static box Main {
    main(args) {
        return new ProbeMeta(41)
    }
}
HAKO

if NYASH_DISABLE_PLUGINS=1 cargo run -q --bin hakorune -- --emit-mir-json /tmp/"$TAG".ctor_escape.json "$ctor_escape_src" \
  >/tmp/"$TAG".ctor_escape.out 2>/tmp/"$TAG".ctor_escape.err; then
  echo "[$TAG] ERROR: escaped record constructor unexpectedly compiled" >&2
  exit 1
fi
guard_expect_in_file "$TAG" '\[record-construction/escape\]' /tmp/"$TAG".ctor_escape.err "escaped record constructor must fail fast"

cat >"$unknown_src" <<'HAKO'
record ProbeMeta {
    ptr: i64
}

static box Main {
    main(args) {
        local meta = new ProbeMeta(41)
        return meta.missing
    }
}
HAKO

if NYASH_DISABLE_PLUGINS=1 cargo run -q --bin hakorune -- --emit-mir-json /tmp/"$TAG".unknown.json "$unknown_src" \
  >/tmp/"$TAG".unknown.out 2>/tmp/"$TAG".unknown.err; then
  echo "[$TAG] ERROR: unknown record field unexpectedly compiled" >&2
  exit 1
fi
guard_expect_in_file "$TAG" '\[record-field-read/unknown-field\]' /tmp/"$TAG".unknown.err "unknown record field must fail fast"

echo "[$TAG] ok"
