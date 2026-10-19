# APP-json-stream-aggregator

Status: common untyped-storage target accepted; JSON selected. Date: 2026-10-20
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Current measurement

- Latest fixed suite with linear ABI extraction/per-invocation grounding reuse:
  6PASS/5FAIL, actual EXE5 + empty marker1, no SKIP, duration657.431s.
  Matching quick opt0 CLI42.89s, quick runtime and C FFI; no Cargo overlap.
- mimalloc PASS205.577s; typed-newbox196.124s →37.816s. Allocator still timeout300s.
  Prior pre-extraction suite: 5PASS/6FAIL/1392.243s; Array-transfer suite: 7PASS/4FAIL.
- Same allocator smoke: prior build-timeout300s → input-census sharing267.063s PASS
  → cohort/selection folding160.247s PASS (-40% total). Matching CLI43.24s; no Cargo/profiler overlap.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --timeout 1900`, build/run limits300s.
- Matching quick opt0 CLI after source-family ordering fix (build41s), existing JSON smoke: MIR compilation fails
  at `ordinary-new/object-input/callee-exit: PrefixNotCovered(Body(2))`; no artifact/link/runtime.
  Existing debug identifies `ingest` Loop line145 and `source control If has no checked condition`
  at line147 `if end < 0`. This is the original implicit Unit input's source refusal.
  Line146 Local I64 and line151 discarded Bool now retain existing Call packets;
  combined source/publication verification passes, but unchanged JSON EXE is still red.
- Other reds: untyped-field execution permission, binary-trees source-not-i64, boxtorrent receiver-unavailable.

## Current priority and measurement interpretation

- [Common storage D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md) is now the
  accepted target for three real apps plus the untyped sentinel; ParkedSealed is
  superseded. Shared critical-path implementation is open; it does not fix every stop.
- `mimalloc-lite` suite PASS includes a harness correction: an obsolete generic JSON
  all-Invoke refusal precheck was removed; production compiler/app behavior was unchanged.
  The canonical EXE already worked. Route/toolchain/artifact/no-replay/output/exit checks remain.
- Existing publication tests cover generic profile rejection of four object-lifecycle
  programs and positive Static I64 Invoke transport. They do not reproduce the same
  mimalloc import closure or assert every unsupported Invoke subtype. Do not describe
  this as a new runtime capability or an identical negative test transfer.
- Loop Call source/SSA, full finite incoming/uses, readonly Text materialization and
  implicit Unit inputs connect to both final consumers; focused176 plus original-root refusal PASS, publication PASS.
  Unit value-use and before/after-take guard-loss refusals PASS; object packet26 PASS.
  Fixtures retain real-EXE profile/field vetoes; final-entry refusal replaces an internal partition pin.

## Next source connection

- Source now retains Dynamic layout and no-Birth known-empty field inventory,
  without writer/MIR inference. Existing fixed-Array specialization stays bounded.
- Destruction separates no-hook declaration from exact Array/object/Dynamic; focused67 PASS.
- Runtime Dynamic slots now hold explicit Empty/null/I64/Bool/Text/object/Array
  values plus an independently published complete End descriptor. No owning Clone.
  Replace atomically publishes the SAME detached old value/residence as Ready;
  End consumes it before calling outside every lock. Take leaves known Empty.
- Checked reads preserve Empty versus null and exact kind; kind-zero is a tagged
  borrowed observation for source tag tests, never an owning Dynamic call result.
  Compatibility registration/setters and direct cells cannot activate this profile;
  outer reclaim refuses nonempty Dynamic slots. No tag selects an owner or End.
- Lifecycle-core checked_array9 PASS (0.01s), archive build31.54s; existing native Map C
  -Wall/-Wextra/-Werror PASS SafeMutex/SingleThread with empty-Array End; Map SafeMutex only.
- Existing C cases cover Empty/kind Fault, transactions, End, reentry and cleanup Fault; no new test files/hooks/guards.
- Optional target section `.nyash.object_field_abi.v1` supplies physical sizes/
  alignment. Both final consumers must admit it and verify SAME source/slot/
  transfer/Normal/Fault correspondence before emission. Source activation is owed;
  Map placement, stored borrow/Shared and carrier forwarding remain unadmitted.
- Allocation-only fields now use SAME successful writer/RHS for I64 source reads
  of I64/Dynamic slots; untyped declaration stays absent, Empty stays unready.
  Calls exposing root/alias forget the whole root proof, including mixed I64 fields;
  alias writes clear readiness. Loop loses Dynamic-root proofs pending checked After.
- Existing cases cover Dynamic kind/no-escape without new tests/files; focused27 PASS/2 baseline reds.
  Operand/RHS entry, writer, physical read and Lowering/Finalized retain I64 lease;
  Dynamic execution, return inference and both final consumers remain unconnected.
- Array lifecycle recipe anchors holder/Completion separately from element operations.
  Bare holder retains whole-source empty-transfer coverage: sole original Plain RHS,
  no candidate/parent capture, escape, rebind or other access to the destination field.
  SAME canonical Dynamic writer/live Homes join that fact before source Normal moves
  the local Home into the field; precommit Fault retains it, local exit omits it.
  Existing cases minimally cover independent content/End and capture boundaries;
  focused12 PASS, no new tests/files. Physical acquisition/commit/End and consumers owed.
- Runtime checked empty End retires unclaimed Boxed(empty), releases outside locks and
  rejects other/retired states. Both consumers still require source contents/End proof.
- Connect BOTH line146 Local I64 Call and line151 discarded Bool Call through the
  existing source/schedule and SAME child packet; original Core demand selects all entry stages.
  Local receipt feeds SAME SSA index; Call receipt separates new operand writes from cached loans.
- One publication case covers Local I64/discard Bool/substring cleanup and implicit Unit, matching JSON roles.
  Both explicit/implicit Unit exits use the SAME final pool; borrowed Copy closure starts at source entry roots.
  Static qualified input uses the checked complete census and existing Invoke writer; no new Plain Call admission.
  C Text matrix reuses Unit/Bool leaf parameters to cover independent Normal/Fault/End/escape boundaries.
  Shared Append outer Call now belongs to its existing effect packet; retain full census.
  Entry membership is passive; result loan issuer retains full anchor/grounding/dependency proof.
  Original numeric forwarding guard loss stays terminal; no nullable owner retry.
  If/SSA stays isolated; cohort/selection folds reuse same-operation immutable checks, no persistent cache.
  Archive framing cases reuse one existing test fn to protect ABI refusals after retiring per-member subprocesses.
- Prior reds: field-read2, count_at Loop, composition2/result2, Unit remain open.
- Previous Array-transfer measurement passed allocator/mimalloc EXEs unchanged;
  current fixed suite restores mimalloc; later allocator standalone PASS160.247s. Bool Eq/Ne C admission
  retains mixed Bool/I64 and Bool-order refusals; existing matrix tail is still blocked
  by the pre-existing tagged-add negative. Earlier SSA/handoff/Array teardown/synthetic
  callee reds remain open. Array provider3/handoff1 reproduce identically at pre-change HEAD.

## Completion obligations
- Existing JSON smoke must satisfy every artifact/run/output/negative contract unchanged.
- Common untyped-slot D0 is accepted; source/layout/runtime/final-consumer implementation remains.
- Retire replaced exclusive edges when caller/validation closure holds; retain shared owners.
