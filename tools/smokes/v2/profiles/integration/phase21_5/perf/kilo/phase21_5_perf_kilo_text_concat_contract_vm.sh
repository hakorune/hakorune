#!/bin/bash
# phase21_5_perf_kilo_text_concat_contract_vm.sh
#
# Contract pin (LLVM-HOT-20 structural hotspot):
# - kilo text loop must preserve string concat route in nested append path.
# - main IR should keep text helper density (`concat_hs` / `concat_hh` / `concat3_hhh` / `insert_hsi`).
# - data set route must consume concat result without falling back to literal 0.
# - insert-mid collapse must not leave dead `from_i8_string_const` boxer calls behind.
# - this contract uses the direct emit route as the canonical source owner for
#   `bench_kilo_kernel_small`; helper/mainline Stage1 emit is out of scope here.

set -euo pipefail

source "$(dirname "$0")/../../../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_kilo_text_concat_contract_vm"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
EMIT_TIMEOUT_SECS="${EMIT_TIMEOUT_SECS:-30}"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"
BENCH="$NYASH_ROOT/benchmarks/bench_kilo_kernel_small.hako"

if [ ! -x "$EMIT_ROUTE" ]; then
  test_fail "$SMOKE_NAME: emit route helper missing/executable: $EMIT_ROUTE"
  exit 2
fi
require_smoke_path "$SMOKE_NAME" "MIR builder" "$MIR_BUILDER" || exit 2
require_smoke_path "$SMOKE_NAME" "benchmark" "$BENCH" || exit 2

tmp_mir="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.mir.json")"
tmp_ir="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.ll")"
tmp_exe="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.exe")"
tmp_log="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.log")"
tmp_main="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.main.ll")"

cleanup() {
  rm -f "$tmp_mir" "$tmp_ir" "$tmp_exe" "$tmp_log" "$tmp_main" >/dev/null 2>&1 || true
}
trap cleanup EXIT

set +e
"$EMIT_ROUTE" --route direct --timeout-secs "$EMIT_TIMEOUT_SECS" --out "$tmp_mir" --input "$BENCH" >"$tmp_log" 2>&1
emit_rc=$?
set -e
if [ "$emit_rc" -ne 0 ]; then
  tail -n 60 "$tmp_log" || true
  test_fail "$SMOKE_NAME: MIR emit failed rc=$emit_rc"
  exit 1
fi

set +e
NYASH_LLVM_FAST=1 \
NYASH_LLVM_FAST_INT="${NYASH_LLVM_FAST_INT:-1}" \
NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
NYASH_LLVM_AUTO_SAFEPOINT=0 \
NYASH_LLVM_HOT_TRACE=1 \
NYASH_LLVM_DUMP_IR="$tmp_ir" \
bash "$MIR_BUILDER" --in "$tmp_mir" --emit exe -o "$tmp_exe" --quiet >>"$tmp_log" 2>&1
build_rc=$?
set -e
if [ "$build_rc" -ne 0 ]; then
  tail -n 80 "$tmp_log" || true
  test_fail "$SMOKE_NAME: AOT build failed rc=$build_rc"
  exit 1
fi

if [ ! -s "$tmp_ir" ]; then
  test_fail "$SMOKE_NAME: expected IR dump is empty"
  exit 1
fi

require_ir_entry_function "$SMOKE_NAME" "$tmp_ir" "$tmp_main" || exit 1

concat_hs_count="$(count_fixed_pattern_in_file "$tmp_main" 'nyash.string.concat_hs')"
concat_hh_count="$(count_fixed_pattern_in_file "$tmp_main" 'nyash.string.concat_hh')"
concat3_count="$(count_fixed_pattern_in_file "$tmp_main" 'nyash.string.concat3_hhh')"
insert_hsi_count="$(count_fixed_pattern_in_file "$tmp_main" 'nyash.string.insert_hsi')"
substring_count="$(count_fixed_pattern_in_file "$tmp_main" 'nyash.string.substring_hii')"
concat_total_count=$((concat_hs_count + concat_hh_count + concat3_count + insert_hsi_count))
if [ "${concat_total_count}" -lt 2 ]; then
  test_fail "$SMOKE_NAME: text helper density too low in main (expected >=2, got total=${concat_total_count}, concat_hs=${concat_hs_count}, concat_hh=${concat_hh_count}, concat3_hhh=${concat3_count}, insert_hsi=${insert_hsi_count})"
  exit 1
fi
if [ "$substring_count" -ne 0 ]; then
  test_fail "$SMOKE_NAME: main still contains substring_hii after insert-mid collapse (count=${substring_count})"
  exit 1
fi

unused_const_box_regs="$(python3 - "$tmp_main" <<'PY'
import re
import sys

lines = open(sys.argv[1], encoding="utf-8").read().splitlines()
inside = False
main = []
defs = []

for line in lines:
    if re.match(r'define i64 @"?(main|ny_main)"?\(\)', line):
        inside = True
    if inside:
        main.append(line)
        if line.strip() == "}":
            break

for idx, line in enumerate(main):
    m = re.search(r'(%r\d+)\s*=\s*call i64 @"?nyash\.box\.from_i8_string_const"?', line)
    if m:
        defs.append((idx, m.group(1)))

unused = []
for idx, reg in defs:
    pat = re.compile(r'(?<![0-9A-Za-z_])%s(?![0-9A-Za-z_])' % re.escape(reg))
    used = any(pat.search(line) for line in main[idx + 1:])
    if not used:
        unused.append(reg)

print(" ".join(unused))
PY
)"
if [ -n "$unused_const_box_regs" ]; then
  test_fail "$SMOKE_NAME: dead boxed string const remains in main (${unused_const_box_regs})"
  exit 1
fi

if grep -Eq ' = add i64 0, %r[0-9]+' "$tmp_main"; then
  test_fail "$SMOKE_NAME: main still contains copy-style add i64 0, %rN noise"
  exit 1
fi
if grep -Eq ' = or i1 %r[0-9]+, false' "$tmp_main"; then
  test_fail "$SMOKE_NAME: main still contains copy-style i1 passthrough noise"
  exit 1
fi

array_indexof_count="$(count_fixed_pattern_in_file "$tmp_main" 'nyash.array.string_indexof_hih')"
if [ "$array_indexof_count" -lt 1 ]; then
  test_fail "$SMOKE_NAME: main missing array.string_indexof_hih call"
  exit 1
fi

if grep -q 'nyash.any.length_h' "$tmp_main"; then
  test_fail "$SMOKE_NAME: main still contains generic any.length_h route"
  exit 1
fi

ir_attr_check="$(python3 - "$tmp_ir" <<'PY'
import re
import sys

path = sys.argv[1]
lines = open(path, encoding="utf-8").read().splitlines()
attrs = {}
decls = {}

for line in lines:
    m = re.match(r'^(?:attributes\s+)?#(\d+)\s*=\s*\{([^}]*)\}', line)
    if m:
        attrs[m.group(1)] = m.group(2)
    m = re.match(r'^declare\s+\S+\s+@("?)(nyash\.array\.(string_indexof_hih|slot_load_hi|set_his))\1\([^)]*\)(?:\s+#(\d+))?.*$', line)
    if m:
        decls[m.group(2)] = (line, m.group(4))

def merged_text(name: str) -> str:
    line, attr_id = decls[name]
    if attr_id and attr_id in attrs:
        return f"{line} {attrs[attr_id]}"
    return line

required = ("nyash.array.string_indexof_hih", "nyash.array.slot_load_hi")
for name in required:
    text = merged_text(name)
    if "nounwind" not in text or "willreturn" not in text or ("readonly" not in text and "memory(read)" not in text):
        print(f"missing-readonly {name}")
        raise SystemExit(0)

text = merged_text("nyash.array.set_his")
if "readonly" in text or "memory(read)" in text:
    print("readonly-mutating nyash.array.set_his")
    raise SystemExit(0)

print("ok")
PY
)"

case "$ir_attr_check" in
  missing-readonly\ *)
    test_fail "$SMOKE_NAME: ${ir_attr_check#missing-readonly } declaration is missing readonly attrs"
    exit 1
    ;;
  readonly-mutating\ *)
    test_fail "$SMOKE_NAME: mutating ${ir_attr_check#readonly-mutating } must not be stamped readonly"
    exit 1
    ;;
esac

set_consumer_stats="$(python3 - "$tmp_main" <<'PY'
import re
import sys

text = open(sys.argv[1], encoding="utf-8").read().splitlines()
concat_regs = set()
set_consume = 0
set_zero = 0

for line in text:
    m = re.search(r'(%r\d+)\s*=\s*call i64 @"?(nyash\.string\.concat_hs|nyash\.string\.concat_hh|nyash\.string\.concat3_hhh|nyash\.string\.insert_hsi)"?', line)
    if m:
        concat_regs.add(m.group(1))
        continue
    m = re.search(r'call i64 @"?(nyash\.array\.set_his|nyash\.array\.set_hih|nyash\.array\.set_hii)"?\((.*)\)$', line)
    if not m:
        continue
    if re.search(r', i64 0\)$', line):
        set_zero += 1
        continue
    for reg in concat_regs:
        if re.search(r', i64 %s\)$' % re.escape(reg), line):
            set_consume += 1
            break

print(f"{set_consume} {set_zero}")
PY
)"

set_consume_count="$(echo "$set_consumer_stats" | awk '{print $1}')"
set_zero_count="$(echo "$set_consumer_stats" | awk '{print $2}')"
if [ "$set_consume_count" -lt 1 ]; then
  test_fail "$SMOKE_NAME: set route does not consume concat result"
  exit 1
fi
if [ "$set_zero_count" -ne 0 ]; then
  test_fail "$SMOKE_NAME: set route fallback to literal 0 detected"
  exit 1
fi

test_pass "$SMOKE_NAME"
