# APP-json-stream-aggregator

Status: common untyped-storage target accepted; JSON selected. Date: 2026-10-20
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Current measurement

- Fixed `real-apps-exe-boundary`: 7PASS/4FAIL, actual EXE6 + empty marker1;
  mimalloc and allocator PASS, no SKIP/timeout, duration1150.771s.
  Measured known-empty source admission `0013eb0e11` with matching quick CLI
  (build233s), quick lifecycle runtime and production Bool-equality C FFI.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --timeout 1900`, build/run limits300s.
- JSON first stop: `[freeze:contract][source-loop/v2/source Loop has an unavailable path]`.
  MIR compilation stops before artifact/link/runtime. Owner `ingest`, Loop Body(2)
  at line145, call line151 `me.ingestLine(stream.substring(start, end))`.
  Incoming/Mul gates cleared; debug: `source expression has no checked execution root`.
  Scanner also lacks line146 Local Call `end = JsonLine.find(...)`; retain its I64
  result as well as the later discarded Bool result.
- At Array transfer source admission, matching quick CLI (build210s) keeps JSON's stop;
  sentinel precheck/direct EXE both exit1 at owned-field-write/original-source:
  the I64 execution lease rejects Dynamic. No artifact; generic negative pin retained.
- Other reds remain explicit: untyped-field execution permission missing,
  binary-trees borrowed-result/source-not-i64, boxtorrent stored-child/receiver-unavailable.

## Current priority and measurement interpretation

- [Common storage D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md) is now the
  accepted target for three real apps plus the untyped sentinel. Implementation
  is open; runtime C tests alone do not prove source/production coverage.
- `mimalloc-lite` suite PASS includes a harness correction: an obsolete generic JSON
  all-Invoke refusal precheck was removed; production compiler/app behavior was unchanged.
  The canonical EXE already worked. Route/toolchain/artifact/no-replay/output/exit checks remain.
- Existing publication tests cover generic profile rejection of four object-lifecycle
  programs and positive Static I64 Invoke transport. They do not reproduce the same
  mimalloc import closure or assert every unsupported Invoke subtype. Do not describe
  this as a new runtime capability or an identical negative test transfer.
- Own unverified Loop Call source edits are saved in Git stash under
  `JSON Loop Call source connection pending common storage decision`; not built/tested.

## Next source connection

- Source now retains Dynamic layout and no-Birth known-empty field inventory,
  without writer/MIR inference. Existing fixed-Array specialization stays bounded.
- Destruction separates no-hook declaration from exact Array/object/Dynamic
  requirements. Static projections reject Dynamic even with children; focused67 PASS.
- Runtime Dynamic slots now hold explicit Empty/null/I64/Bool/Text/object/Array
  values plus an independently published complete End descriptor. No owning Clone.
  Replace atomically publishes the SAME detached old value/residence as Ready;
  End consumes it before calling outside every lock. Take leaves known Empty.
- Checked reads preserve Empty versus null and exact kind; kind-zero is a tagged
  borrowed observation for source tag tests, never an owning Dynamic call result.
  Compatibility registration/setters and direct cells cannot activate this profile;
  outer reclaim refuses nonempty Dynamic slots. No tag selects an owner or End.
- Existing kernel lifecycle-core typed_object35/fault34 PASS (test execution0.01s
  each). Archive build5.43s. Existing native checked_map_runtime_abi_test.c compiles
  with -Wall/-Wextra/-Werror, links the matching archive, passes SafeMutex and
  SingleThread; existing Map boundary is also checked on SafeMutex only.
- C fixture additions cover independent ABI/transaction/ownership boundaries:
  Empty/kind Fault, failed write retaining candidate/old slot, actual child End,
  reentry refusal and commit surviving cleanup Fault; no new test function/file
  in Rust, checker, hook or guard. Kernel398/root9263 test functions remain.
- Optional target section `.nyash.object_field_abi.v1` supplies physical sizes/
  alignment. Both final consumers must admit it and verify SAME source/slot/
  transfer/Normal/Fault correspondence before emission. Source activation is owed;
  Map placement, stored borrow/Shared and carrier forwarding remain unadmitted.
- Allocation-only fields now use SAME successful writer/RHS for I64 source reads
  of I64/Dynamic slots; untyped declaration stays absent, Empty stays unready.
  Calls exposing root/alias forget the whole root proof, including mixed I64 fields;
  alias writes clear readiness. Loop loses Dynamic-root proofs pending checked After.
- Existing source cases minimally add Dynamic kind/no-escape boundaries not covered
  by typed cases; no new test functions/files. Focused27 PASS/2 known field-read reds.
  Operand/RHS entry, writer, physical read and Lowering/Finalized retain I64 lease;
  Dynamic execution, return inference and both final consumers remain unconnected.
- Array holder retains construction outside its SAME optional element requirement.
  Bare initializer preserves selected refusals; source Normal moves the exact local
  Home into Dynamic field. The frame keeps precommit Fault Homes; exit omits Array.
  Existing sentinel/negative cases protect alias/mutation/doublemove/kind; no new tests.
  Focused38 PASS/2 baseline reds; physical acquisition/commit/End and consumers owed.
- Connect BOTH line146 Local I64 Call and line151 discarded Bool Call through the
  existing source tree/schedule and SAME Static/Instance child packet. Its substring
  argument stays inside that packet; retain original Homes and argument cleanup.
- Prior reds: count_at Loop, composition2/result2 and Unit diagnostic; earlier
  connection/normalization/filter details remain in this file's Git history.
- JSON UserStats/aggregator slots and Map remain implementation dependencies.
- Field-read baseline2: stored-child caller assertion / opaque formal ingress; unchanged.

## Retained verification obligations

- Allocator and mimalloc physical EXEs pass unchanged. Allocator's Bool Eq/Ne C admission
  retains mixed Bool/I64 and Bool-order refusals; existing matrix tail is still blocked
  by the pre-existing tagged-add negative. Earlier SSA/handoff/Array teardown/synthetic
  callee reds remain open; implementation details are retained in this file's Git history.

## Completion obligations

- Existing JSON smoke must satisfy every artifact/run/output/negative contract unchanged.
- Common untyped-slot D0 is accepted; source/layout/runtime/final-consumer implementation remains.
- Retire replaced exclusive edges when caller/validation closure holds; retain shared owners.
