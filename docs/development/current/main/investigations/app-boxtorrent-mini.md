# APP-boxtorrent-mini

Status: selected app; canonical EXE incomplete. Date: 2026-10-27
Scope: unchanged boxtorrent-mini through canonical EXE generation, execution, and expected output/exit.
Refs: [RULES](../../../RULES.md), [CURRENT_STATE](../CURRENT_STATE.toml), [Array owner](../../../../../src/mir/normal_callable_semantic_package/core_method_source/README.md).

## Current stop
- Binary/runtime: `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick`; canonical LLVM C API/pure-first, no compatibility replay.
- Latest smoke fails `borrowed-actual/unsupported-or-unavailable` at `apps/boxtorrent-mini/main.hako:108`: BoxTorrentStore.release, `me.allocator.release(chunk.alloc_handle)`, argument0 (owner54 Body3/IfThen0/IfCondition/Argument0). No EXE. Diagnostic source changes were removed; the same stop remains with the final binary.
- `chunk.alloc_handle` now retains the exact local read → sole Map get → all-set payload class → canonical field's complete writer census. Every field node is reserved before actual/writer expansion; nullable nominal results from exact original calls join the same SUM. PendingStored lends only its retained reference and checked result roots, never a selected receiver or Home.
- Original source ingress now passes the earlier Main Body10 and ingest addChunk stops. The computed i64 `allocate(data.length())` argument receives its original loan/frame, so its previous frame-missing stop is gone. This does not establish completion of every later Home/return/cleanup consumer.
- Last full suite (`run.sh --profile integration --owner-profile integration --suite real-apps-exe-boundary --timeout 420`)9/11 in576.456s:8realEXEs+marker; JSON/boxtorrent compilation failures, no timeout/SKIP. Binary-trees/mimalloc/allocator retain canonical EXE/run success. CLI post-diagnostic-removal build38.64s PASS (1,877 warnings); current source has no diagnostic additions.
- Prior source-series focused checks51PASS/4FAIL: SUM solver1, entry21, incoming-kind1, declared6/2, result claims22/2. The same four failures were established before this series: two source-only declared-cohort cases, PageHeap required-integer-source, JSON callee-exit. Earlier physical/Text and terminal reds remain open; no refusal was disabled.

- Current interval change: CLI `cargo quick-build-fast -j 4`39.30s PASS; selected smoke retains the same compilation refusal. Existing interval test demonstrates red-before/green-after for checked same-field reread→early child End. Descriptor and returned-wrapper publication checks also PASS (3PASS total). An additional returned-field source-input test refuses `borrowed-object/forward-source-identity` during package issuance, before the changed physical interval; it remains unresolved, not waived.


## Implementation boundary
- Exact local New class lending checks complete flow read, original acquisition, the same New census and singleton formal agreement. New-origin points join the existing SUM graph; this grants no live Home. Unknown object observations stay pending without qualified/current ownership evidence.
- Original field Object projection is solved after SUM and refreshed after result joins, retaining unchanged domain identity. All original incoming rows participate; unknown/mixed-class inputs mint no new view. The source field-class callback retains independently valid selected-source evidence and may also borrow the complete original view; when both exist their class and canonical object must agree. Selected transport, guarded-formal null exclusion and independent physical checks remain separate.
- Birth field RHSs reuse the existing stable-constructor-formal proof; nominal result sets retain every Fresh/Null root across later scalar re-solves. Primitive result projection lives with the existing result owner; touched production files remain below800 lines.
- Mixed nominal-object/Text/scalar cohorts select the existing Uses owner before finite entry publication. Object views, complete incoming, actual class correspondence and Normal/Fault checks remain independent. Plain finite-input bindings keep their exact source-read proof and qualified-expression requirement.
- Opaque signed Core MethodCall arguments now use the existing original-Core/site/result checks and i64 source-loan classification. Text results remain distinct; source loan creation itself is nonexecuting.
- Entry selection/aliases and argument lending/binding/completion were mechanically separated with unchanged caller API/private scope, keeping touched owners below800 lines. The unused append lending wrapper/export was retired after caller-zero source search; observed append lending remains.
- Dynamic ArrayGet retains its carrier class and independent End; bounds/append SUM may prove Text only for a separate snapshot. Materialize/final validation retain exact operands and Normal/Fault cleanup. Direct initializer/deferred-argument capability refusals remain where not connected.
- `.hako`, fixtures and checker entrances are unchanged; no test function/file was added. One existing interval case was extended because the prior ObjectFieldGet matrix did not cover checked ObjectFieldBorrow→Normal aliasing or DynamicCarrierEnd. Source diagnostic snapshots are retired at landing.
- Field argument liveness now includes exact checked same-field Normal results in the alias cohort, validates their Invoke/Normal correspondence, and rejects aliasing carrier End. Terminating Fault cleanup remains distinct from paths rejoining the argument use.



## Next
- Connect the exact terminal Map-derived Object field to the existing ReadActual field issuer: retain MapGet frame/nonnull exclusion and use nullable ObjectFieldBorrow, without forging ReturnedField provenance. Normalize BorrowedObject value evidence behind a private-origin wrapper (existing child relation / complete all-writer SUM); teardown pairing permission stays on the original child-relation type.
- Extend the existing checked read/field-argument consumer through Normal/Fault and final interval/End validation, including the parent anchor. Parent MapGet nonnull does not prove child nonnull. Passive SUM/class completion grants no executable read.
- Then finish local-result live transfer, remaining field/array reads, publication and actual EXE behavior. Original local-New source class is not proof of an executable return leaf.
- JSON, required source/production connections, R6/R7, Loop and finite-product closeout remain incomplete.
