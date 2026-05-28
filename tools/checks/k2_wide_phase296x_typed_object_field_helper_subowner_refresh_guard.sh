#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-213-TYPED-OBJECT-FIELD-HELPER-SUBOWNER-REFRESH.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-212-SELECTED-METHOD-ARRAY-SLOT-DIRECT-OP-MEASUREMENT.md"
TOOL="$ROOT_DIR/tools/allocator/typed_object_field_helper_subowner_refresh.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row213_field_subowner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

PERF_REPORT="$TMP_DIR/perf.report"
GET_ANNOTATE="$TMP_DIR/field_get.annotate"
SET_ANNOTATE="$TMP_DIR/field_set.annotate"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row213-field-subowner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-field-helper-subowner-refresh-v0"
require_line "$DOC" "perf_field_helper_pct=59.42"
require_line "$DOC" "dominant_field_helper_subowner=control_validation_branch"
require_line "$DOC" "recommended_next=typed_object_exact_slot_direct_helper_selection"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"
require_line "$DOC" "selected_owner_family=typed_object_exact_slot_direct_helper"
require_line "$DOC" "next_row=typed_object_exact_slot_direct_helper_selection"

cat >"$PERF_REPORT" <<'REPORT'
    23.70%  app.exe  app.exe               [.] nyash.object.field_set_hii
    15.73%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_store_i64
    14.17%  app.exe  app.exe               [.] nyash.object.field_set_u64_hiu
    11.92%  app.exe  app.exe               [.] nyash.object.field_get_hii
    10.34%  app.exe  app.exe               [.] core::hash::BuildHasher::hash_one
     9.63%  app.exe  app.exe               [.] nyash.object.field_get_u64_hii
     2.40%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64
     2.35%  app.exe  app.exe               [.] HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
REPORT

cat >"$GET_ANNOTATE" <<'REPORT'
 Percent | Source code & Disassembly
 26.27 : 427a82: ja     427a95 <nyash.object.field_get_hii+0x255>
 13.89 : 427840: push   %rbp
 13.19 : 427847: push   %rbx
  6.90 : 427845: push   %r12
  6.67 : 427a87: mov    0x8(%rax),%rax
  6.64 : 427843: push   %r15
  6.64 : 427a8b: mov    %rcx,(%rbx)
  6.63 : 427871: jne    4278be <nyash.object.field_get_hii+0x7e>
  6.60 : 427a5e: jbe    427a90 <nyash.object.field_get_hii+0x250>
  6.57 : 42789f: cmp    $0x1,%eax
REPORT

cat >"$SET_ANNOTATE" <<'REPORT'
 Percent | Source code & Disassembly
 19.97 : 427b45: jne    427d96 <nyash.object.field_set_hii+0x276>
 16.46 : 427d65: mov    0x8(%rcx),%rdx
 10.00 : 427d9b: pop    %r14
  9.97 : 427d82: mov    %rdx,(%rcx)
  9.94 : 427b36: sets   %al
  6.87 : 427d7a: ja     427d91 <nyash.object.field_set_hii+0x271>
  6.65 : 427b29: movabs $0x7fffffffffffffff,%rbx
  6.63 : 427b59: cmpb   $0x1,0xfe0b4(%rip)
  3.53 : 427b68: mov    %rdx,%r12
  3.33 : 427b40: setbe  %cl
  3.33 : 427d60: jae    427d91 <nyash.object.field_set_hii+0x271>
  3.31 : 427b85: cmp    $0x1,%eax
REPORT

"$TOOL" \
  --perf-report "$PERF_REPORT" \
  --field-get-annotate "$GET_ANNOTATE" \
  --field-set-annotate "$SET_ANNOTATE" \
  --out "$REPORT"

require_line "$REPORT" "output_contract=typed-object-field-helper-subowner-refresh-v0"
require_line "$REPORT" "input_contract=selected-method-array-slot-direct-op-post-fusion-owner-refresh-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "perf_field_helper_pct=59.42"
require_line "$REPORT" "annotate_local_pct.control_validation_branch=99.47"
require_line "$REPORT" "annotate_local_pct.direct_vec_field_access=46.37"
require_line "$REPORT" "dominant_field_helper_subowner=control_validation_branch"
require_line "$REPORT" "recommended_next=typed_object_exact_slot_direct_helper_selection"
require_line "$REPORT" "rejected_owner=array_slot_backend_handle_map_hash"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
