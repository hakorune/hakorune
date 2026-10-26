# APP-boxtorrent-mini

Status: selected app; canonical EXE incomplete. Date: 2026-10-26
Scope: unchanged boxtorrent-mini through canonical EXE generation, execution, and expected output/exit.
Refs: [RULES](../../../RULES.md), [CURRENT_STATE](../CURRENT_STATE.toml), [Array owner](../../../../../src/mir/normal_callable_semantic_package/core_method_source/README.md).

## Current stop
- Binary/runtime: `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick`; canonical LLVM C API/pure-first, no compatibility replay.
- Latest smoke fails `object-input/callee-exit` in HakoAllocHeap.release(handle), owner38 Body1/IfThen0, TerminalNotCovered. The earlier condition `handle.page_id == 0` is not admitted: Body0's null guard walks, Body1 has a known Prefix, but its then branch is not walked. No EXE.
- Exact source-column comparison: release's handle formal retains its own origin, but selected Object view, original Object view and SUM are all absent. isLiveHandle's corresponding formal has HakoAllocHandle in both views. Replacing one view with another would not close this proof.
- Original source ingress now passes the earlier Main Body10 and ingest addChunk stops. The computed i64 `allocate(data.length())` argument receives its original loan/frame, so its previous frame-missing stop is gone. This does not establish completion of every later Home/return/cleanup consumer.
- Fixed suite (`run.sh --profile integration --owner-profile integration --suite real-apps-exe-boundary --timeout 420`)9/11 in551.806s:8realEXEs+marker; JSON/boxtorrent compilation failures, no timeout/SKIP. Binary-trees/mimalloc/allocator retain canonical EXE/run success. CLI post-diagnostic-removal build37.76s PASS (1,877 warnings); current source has no diagnostic additions.
- Focused source/entry/result checks54PASS/4FAIL. The four failures reproduce on pre-series HEAD: two source-only declared-cohort cases, PageHeap required-integer-source, JSON callee-exit. Earlier physical/Text and terminal baseline reds remain open. Extended Core test PASS, including opaque signed MethodCall and rejection/identity cases. No refusal was weakened.

## Implementation boundary
- Exact local New class lending checks complete flow read, original acquisition, the same New census and singleton formal agreement. New-origin points join the existing SUM graph; this grants no live Home. Unknown object observations stay pending without qualified/current ownership evidence.
- Mixed nominal-object/Text/scalar cohorts select the existing Uses owner before finite entry publication. Object views, complete incoming, actual class correspondence and Normal/Fault checks remain independent. Plain finite-input bindings keep their exact source-read proof and qualified-expression requirement.
- Opaque signed Core MethodCall arguments now use the existing original-Core/site/result checks and i64 source-loan classification. Text results remain distinct; source loan creation itself is nonexecuting.
- Entry selection/aliases and argument lending/binding/completion were mechanically separated with unchanged caller API/private scope, keeping touched owners below800 lines. The unused append lending wrapper/export was retired after caller-zero source search; observed append lending remains.
- Dynamic ArrayGet retains its carrier class and independent End; bounds/append SUM may prove Text only for a separate snapshot. Materialize/final validation retain exact operands and Normal/Fault cleanup. Direct initializer/deferred-argument capability refusals remain where not connected.
- `.hako`, fixtures and checker entrances are unchanged; no test function/file was added. Existing tests minimally cover exact New read versus foreign binding and signed Core MethodCall versus Text/nonexecuting source. The obsolete unsupported-ArrayGet expression pin now asserts the accepted generic Dynamic class, preserving nominal/i64 rejection checks. Temporary diagnostic source code is removed.

## Next
- First missing producer: `chunk.alloc_handle` has a Core Map-get receiver initializer, outside the ordinary-call-only ReturnedField source walk; the general actual classifier has no corresponding FieldAccess seed. Connect the existing all-set `MapFieldPayloadSourceV1` to exact construction-child/all-writer field evidence (`dynamic_stored_child_field_v1`). Preserve nullable alternatives and existing Map get/nonnull execution checks; do not infer class from field names or use a SUM fallback.
- Then finish local-result live transfer, remaining field/array reads, publication and actual EXE behavior. Original local-New source class is not proof of an executable return leaf.
- JSON, required source/production connections, R6/R7, Loop and finite-product closeout remain incomplete.
