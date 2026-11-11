#!/bin/bash
# Canonical app EXE cases
#
# Contract pin, for every case:
# - the app enters the selected physical lifecycle V4 owner through the LLVM C
#   API, with no compat replay and no unsupported pure shape;
# - the EXE exits with the case's expected code (and prints its expected line).
# Cases: the typed-object seeds (allocation-only New, explicit i64 fields,
# birth, birth with a parameter, method; exit 30) and loop-basics (twelve
# everyday loop shapes compared with hand-computed results; exit 0).
#
# Usage: canonical_app_exe_cases.sh [case]   (no case runs all of them)

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="canonical_app_exe_cases"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"
# case|app directory|expected exit|expected stdout line|extra compile flag|storage selection
CASES=(
  "typed_object_newbox_min|typed-object-newbox-min|30||--grammar-profile canonical"
  "typed_object_birth_min|typed-object-birth-min|30||"
  "typed_object_method_min|typed-object-method-min|30||"
  "typed_object_birth_param_min|typed-object-birth-param-min|30||"
  "ownership_move_min|ownership-move-min|30||"
  "ownership_move_direct_min|ownership-move-min|30|||direct_v0:Pair"
  "ownership_copy_min|ownership-copy-min|7||"
  "ownership_shared_min|ownership-shared-min|10||"
  "ownership_typed_null_min|ownership-typed-null-min|5||"
  "ownership_tree_typed_min|ownership-tree-typed-min|3||"
  "ownership_tree_direct_min|ownership-tree-typed-min|3|||direct_v0:TreeNode"
  "ownership_tree_bench_typed|ownership-tree-bench-typed|0|summary=ok|"
  "ownership_tree_bench_direct|ownership-tree-bench-typed|0|summary=ok||direct_v0:TreeNode"
  "ownership_child_typed_min|ownership-child-typed-min|10||"
  "ownership_child_direct_min|ownership-child-typed-min|10|||direct_v0:Leaf,Parent"
  "loop_basics|loop-basics|0|loop-basics failures=0|"
)
ONLY_CASE="${1:-}"

resolve_lifecycle_runtime() {
  if [ -n "${HAKO_LIFECYCLE_NYRT:-}" ]; then
    printf '%s' "$HAKO_LIFECYCLE_NYRT"
  elif [ -n "${NYASH_EMIT_EXE_NYRT:-}" ] && [ -f "$NYASH_EMIT_EXE_NYRT/libnyash_lifecycle_kernel.a" ]; then
    printf '%s' "$NYASH_EMIT_EXE_NYRT"
  else
    printf '%s' "$HAKO_ROOT/target/lifecycle-kernel/release"
  fi
}

NYRT_DIR="$(resolve_lifecycle_runtime)"
if [ ! -f "$NYRT_DIR/libnyash_lifecycle_kernel.a" ]; then
  test_skip "$SMOKE_NAME: lifecycle runtime archive missing: $NYRT_DIR/libnyash_lifecycle_kernel.a"
  exit 0
fi

TMP_ROOT="${TMPDIR:-/tmp}/hakorune_canonical_app_exe_$$"
cleanup() {
  rm -f "$TMP_ROOT".* 2>/dev/null || true
}
trap cleanup EXIT

# The first actual can be a moved local or a fresh construction. The second
# actual may Fault before entry, and the callee may Fault after taking the Home.
for take_case in ownership_take_entry ownership_take_fresh_entry; do
  if [ -n "$ONLY_CASE" ] && [ "$ONLY_CASE" != "$take_case" ]; then
    continue
  fi
  take_setup='local p = new Payload()'
  take_actual='move p'
  if [ "$take_case" = ownership_take_fresh_entry ]; then
    take_setup=''
    take_actual='new Payload()'
  fi
  cat > "$TMP_ROOT.$take_case.hako" <<HAKO
box Payload {
  value: i64
  birth() { me.value = 7 }
}
static box Main {
  later(): i64 {
    local scratch = new Payload()
    return 1
  }
  consume(take p: Payload, marker: i64): i64 {
    local scratch = new Payload()
    return 7
  }
  main() {
    $take_setup
    return me.consume($take_actual, me.later())
  }
}
HAKO
  CASES+=("$take_case|$TMP_ROOT.$take_case.hako|7||")
done

# `birth(take x: T)`: the moved Leaf is Ended once on Normal, on a Fault before the
# constructor is entered (the caller Ends it), and on a Fault after entry (the constructor
# unwinding Ends it and the caller does not End it again).
# A Fault from a later birth argument (`new Node(move leaf, me.later())`) cannot be written
# yet: that argument is rejected as `argument-source-unavailable` (or `static-execution/
# local-contract`). The allocation Fault of a fresh `new` stands in for it: it also
# precedes entry to the constructor, so the caller Ends the moved actual exactly once.
if [ -z "$ONLY_CASE" ] || [ "$ONLY_CASE" = ownership_birth_take ]; then
  cat > "$TMP_ROOT.birth-take.hako" <<'HAKO'
box Leaf {
  value: i64
  birth(value) { me.value = value }
}
box Node {
  next: Leaf
  v: i64
  birth(take next: Leaf, v: i64) { me.next = move next  me.v = v }
}
static box Main { main() { local leaf = new Leaf(1)  local n = new Node(move leaf, 5)  return n.v } }
HAKO
  CASES+=("ownership_birth_take|$TMP_ROOT.birth-take.hako|5||")
fi

# Explicit read return borrows the same local Home; only its scalar snapshot escapes.
if [ -z "$ONLY_CASE" ] || [ "$ONLY_CASE" = ownership_from_read ]; then
  cat > "$TMP_ROOT.from.hako" <<'HAKO'
box Leaf {
  value: i64
  birth() { me.value = 7 }
  identity(p: Leaf): Leaf from p { return p }
}
static box Main {
  main() { local p = new Leaf() local q = p.identity(p) return q.value }
}
HAKO
  CASES+=("ownership_from_read|$TMP_ROOT.from.hako|7||")
fi

if [ -z "$ONLY_CASE" ] || [ "$ONLY_CASE" = ownership_from_receiver ]; then
  cat > "$TMP_ROOT.from-receiver.hako" <<'HAKO'
box Leaf {
  value: i64
  birth() { me.value = 7 }
  identity(): Leaf from me { return me }
  forward(): Leaf from me { local q = me.identity() return q }
}
static box Main {
  main() { local p = new Leaf() local q = p.forward() return q.value }
}
HAKO
  CASES+=("ownership_from_receiver|$TMP_ROOT.from-receiver.hako|7||")
fi

if [ -z "$ONLY_CASE" ] || [ "$ONLY_CASE" = ownership_from_field ]; then
  cat > "$TMP_ROOT.from-field.hako" <<'HAKO'
box Leaf { value: i64 birth() { me.value = 7 } }
box Parent {
  child: Leaf
  marker: i64
  birth(child) { me.child = child me.marker = 0 }
  get(): Leaf from me { return me.child }
  forward(): Leaf from me { local q = me.get() return q }
}
static box Main {
  main() { local child = new Leaf() local parent = new Parent(child) local q = parent.forward() return q.value }
}
HAKO
  CASES+=("ownership_from_field|$TMP_ROOT.from-field.hako|7||")
fi

if [ -z "$ONLY_CASE" ] || [ "$ONLY_CASE" = ownership_from_forward ]; then
  cat > "$TMP_ROOT.from-forward.hako" <<'HAKO'
box Leaf {
  value: i64
  birth() { me.value = 7 }
  identity(p: Leaf): Leaf from p { return p }
  forward(n: i64, p: Leaf): Leaf from p { local q = me.identity(p) return q }
}
static box Main {
  main() { local p = new Leaf() local q = p.forward(0, p) return q.value }
}
HAKO
  CASES+=("ownership_from_forward|$TMP_ROOT.from-forward.hako|7||")
fi

if [ -z "$ONLY_CASE" ] || [ "$ONLY_CASE" = ownership_from_nullable ]; then
  cat > "$TMP_ROOT.from-nullable.hako" <<'HAKO'
box Leaf {
  value: i64
  birth() { me.value = 7 }
  maybe(n: i64, p: Leaf): Leaf from p { if n == 0 { return null } return p }
}
static box Main {
  main() {
    local p = new Leaf()
    local absent = p.maybe(0, p)
    local present = p.maybe(1, p)
    return 7
  }
}
HAKO
  CASES+=("ownership_from_nullable|$TMP_ROOT.from-nullable.hako|7||")
fi

for null_guard_input in 0 1; do
  if [ -z "$ONLY_CASE" ] || [ "$ONLY_CASE" = "ownership_from_null_guard_$null_guard_input" ]; then
    cat > "$TMP_ROOT.from-null-guard-$null_guard_input.hako" <<HAKO
box Leaf {
  value: i64
  birth() { me.value = 7 }
  maybe(n: i64, p: Leaf): Leaf from p { if n == 0 { return null } return p }
}
static box Main {
  main() {
    local p = new Leaf()
    local q = p.maybe($null_guard_input, p)
    if q == null { return 0 }
    return q.value
  }
}
HAKO
    null_guard_expected=0
    if [ "$null_guard_input" = 1 ]; then null_guard_expected=7; fi
    CASES+=("ownership_from_null_guard_$null_guard_input|$TMP_ROOT.from-null-guard-$null_guard_input.hako|$null_guard_expected||")
  fi
done

run_case() {
  local name="$1" app="$HAKO_ROOT/apps/$2/main.hako" expected_rc="$3" expected_line="$4" flag="$5" profile="${6:-}"
  if [[ "$2" = /* ]]; then app="$2"; fi
  local extra_link=()
  if [ "$name" = ownership_take_entry ] || [ "$name" = ownership_take_fresh_entry ] || [ "$name" = ownership_birth_take ]; then
    local probe="$TMP_ROOT.take-probe.o"
    "${CC:-cc}" -DHAKO_TEST_OWNED_CALL_RESULT -c \
      "$HAKO_ROOT/lang/c-abi/tests/published_lifecycle_v4_runtime_probe.c" -o "$probe" || {
      test_fail "$SMOKE_NAME/$name: runtime probe compilation failed"; return 1;
    }
    local libs="-Wl,--start-group $probe $NYRT_DIR/libnyash_lifecycle_kernel.a -Wl,--end-group" operation
    for operation in fault.frame_init fault.frame_dispose fault.report_final object.checked_field_set \
      object.checked_residence_set object.home_release_plain_i64 object.reclaim_unpublished object.checked_new; do
      libs+=" -Wl,--wrap=nyash.${operation}_v1"
    done
    extra_link=("--emit-exe-libs=$libs")
  fi
  local exe="$TMP_ROOT.$name.exe" build_log="$TMP_ROOT.$name.build.log" run_log="$TMP_ROOT.$name.run.log"
  if [ ! -f "$app" ]; then
    test_fail "$SMOKE_NAME/$name: app missing: $app"
    return 1
  fi
  set +e
  # shellcheck disable=SC2086
  HAKO_TYPED_OBJECT_STORE="${profile:-${HAKO_TYPED_OBJECT_STORE:-}}" \
    NYASH_DISABLE_PLUGINS=1 \
    NYASH_LLVM_ROUTE_TRACE=1 \
    HAKO_BACKEND_COMPILE_RECIPE=pure-first \
    HAKO_BACKEND_COMPAT_REPLAY=none \
    timeout "$RUN_TIMEOUT_SECS" \
    "$NYASH_BIN" $flag --backend mir --emit-exe "$exe" --emit-exe-nyrt "$NYRT_DIR" "${extra_link[@]}" "$app" \
    >"$build_log" 2>&1
  local build_rc=$?
  set -e
  local problem=""
  if [ "$build_rc" -ne 0 ]; then
    problem="physical EXE emit failed rc=$build_rc"
  elif ! grep -Fq 'stage=lifecycle-v4-measure result=ok' "$build_log"; then
    problem="lifecycle V4 route trace missing"
  elif ! grep -Fq 'toolchain=llvm-c-api' "$build_log"; then
    problem="LLVM C API physical route missing"
  elif [[ "$profile" == direct_v0:* ]] && ! grep -Fq 'profile=3;' "$build_log"; then
    problem="explicit direct placement route missing"
  elif grep -Fq "unsupported pure shape" "$build_log" || grep -Fq "compat_replay=harness" "$build_log"; then
    problem="fallback or unsupported pure shape was used"
  elif ! grep -Fq "EXE written:" "$build_log" || [ ! -x "$exe" ]; then
    problem="physical EXE artifact missing"
  fi
  if [ -n "$problem" ]; then
    echo "[INFO] $name EXE build output tail:"
    tail -n 160 "$build_log" || true
    test_fail "$SMOKE_NAME/$name: $problem"
    return 1
  fi
  local executions=("||$expected_rc|$expected_line||")
  if [ "$name" = ownership_birth_take ]; then
    executions=("||5||OWNED 2 2 2|COUNTS 1 3 2 0 0 1"
      "2||70||OWNED 2 1 1|COUNTS 1 1 1 0 1 1"
      "|2|70||OWNED 2 2 2|COUNTS 1 2 1 1 1 1")
  fi
  if [ "$name" = ownership_take_entry ] || [ "$name" = ownership_take_fresh_entry ]; then
    executions=("||7||OWNED 3 3 3|COUNTS 1 3 3 0 0 1"
      "2||70||OWNED 2 1 1|COUNTS 1 1 1 0 1 1"
      "3||70||OWNED 3 2 2|COUNTS 1 2 2 0 1 1")
  fi
  local execution fault store_fault expected_owned expected_counts
  for execution in "${executions[@]}"; do
    IFS='|' read -r fault store_fault expected_rc expected_line expected_owned expected_counts <<<"$execution"
    set +e
    env -u V4_PROBE_HOME_FAULT_AT V4_PROBE_FAULT_AT="$store_fault" \
      V4_PROBE_NEW_FAULT_AT="$fault" V4_PROBE_MODE=normal NYASH_DISABLE_PLUGINS=1 \
      timeout "$RUN_TIMEOUT_SECS" "$exe" >"$run_log" 2>&1
    local exe_rc=$?
    set -e
    if [ "$exe_rc" -ne "$expected_rc" ] ||
      { [ -n "$expected_line" ] && ! grep -Fxq "$expected_line" "$run_log"; } ||
      { [ -n "$expected_owned" ] && ! grep -Fxq "$expected_owned" "$run_log"; } ||
      { [ -n "$expected_counts" ] && ! grep -Fxq "$expected_counts" "$run_log"; }; then
      echo "[INFO] $name EXE stdout/stderr:"
      cat "$run_log" || true
      test_fail "$SMOKE_NAME/$name: Fault=${fault:-Normal}, expected exit $expected_rc and original cleanup counts, got $exe_rc"
      return 1
    fi
  done
}

ran=0
for row in "${CASES[@]}"; do
  IFS='|' read -r name app expected_rc expected_line flag profile <<<"$row"
  if [ -n "$ONLY_CASE" ] && [ "$name" != "$ONLY_CASE" ]; then
    continue
  fi
  run_case "$name" "$app" "$expected_rc" "$expected_line" "$flag" "$profile" || exit 1
  ran=$((ran + 1))
done
if [ "$ran" -eq 0 ]; then
  test_fail "$SMOKE_NAME: unknown case: $ONLY_CASE"
  exit 2
fi

test_pass "$SMOKE_NAME"
