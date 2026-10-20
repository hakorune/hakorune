# APP-json-stream-aggregator

Status: JSON selected; common untyped-storage target accepted. Date: 2026-10-20
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Product evidence

- Fixed suite: Text Call source/publication change (base `e84df95343`),
  7PASS/4FAIL, actual EXE6 + empty marker1, no SKIP/timeout. Runner477.107s.
  Matching quick opt0 CLI35.68s, frozen quick lifecycle runtime/private C FFI;
  build/run300s, no Cargo overlap. mimalloc152.332s, allocator177.272s, typed36–37s.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --timeout 1900 --format json`.
- JSON still stops during package/MIR generation at
  `borrowed-static/text-expression-execution-pending`, before EXE/link/runtime.
  Owner26 Body3 intField argument0 line remains pending after missing Body1
  stringField Local. Body8 bytes, owner3 StringHelpers.to_i64 rebind/Loop and
  owner29 Map/report remain owed. Direct Text Return and completion ordering
  are required; original ExactString coverage alone does not close this app.
- Other product reds: untyped-field source activation, binary-trees source-not-i64,
  boxtorrent stored-child receiver. These are incomplete, not waived baselines.

## Connected source and next execution

- Self field writes retain original entry receiver, declaration, whole RHS/source
  census and prior Homes through both final scans. Unannotated slots need existing
  all-writer ClosedNumeric proof and retain absent annotations. Dynamic stays closed.
- Signed values borrow the SAME uncaptured point SUM and physical producer.
  Finite-input aliases retain SUM/read/Copy and producer forwarding; `end = start`
  closes source-use. Pure scalar aliases remain generic. Prior entry21/Static args2PASS.
- TextResidence Return retains the same packet/acquire/install/hold and original
  Home rank through recorder/pending/finishing/both final consumers/publication.
  Normal skips only that Home; Fault retains original reverse order. Plain exits
  remain Emitted. Duplicate/stale hold, replacement and borrowing remain refused.
- Ordinary Text Call retains its result below the exact argument End suffix.
  Call Fault acquires no result; argument-End Fault cancels remaining arguments,
  then result, then outer temporaries/Homes. SSA remapping and Rust/C replay retain
  these dependencies separately from actual arguments and borrow permission.
- Original ExactString Local Call joins full input, affine publication, the same
  TextResidence result and both final consumers. Entry finishing and final publication
  replay one complete original residence-exit predicate; no backend type authority.
- Existing optimized source/publication case passes. Direct canonical `--emit-exe`
  and execution print `!ab`, `Result: 0`, exit0. This small probe does not establish
  JSON acceptance or cover every Text-returning function.
- Existing cases extended only for independent held-result/argument-End Fault and
  optimized source-publication boundaries; no new test fn/file/hook/checker/token.
  Source1, projection9, finished4, Invoke42, instruction19, array-read9 and C Text
  matrix(profile1/2)PASS; initializer79PASS/2samepriorFAIL, terminal6PASS/2samepriorFAIL
  remain open. Direct test runs require16MiBstack.
- Zero-argument ExactString now reuses existing Zero input evidence/empty actuals,
  full original Text source census, same Completion/all residence exits and mandatory
  final Text callee scan. Existing I64 census stays unchanged; unselected Text stays
  passive. Worker review caught census/selection regressions; focused tests confirm
  their correction without weakening duplicate-source or old refusal boundaries.
- Matching CLI43.69s; optimized source case and direct `--emit-exe`/execution PASS:
  `ab`, `Result: 0`, exit0. Zero-input1, original input-source6, Static packet3 and
  whole expression-input1PASS; initializer79PASS/2samepriorFAIL. One existing tuple
  protects the distinct Zero-to-publication boundary; no new test fn/file/hook/guard.
  JSON smoke reconfirms the same MIR stop; suite above was not rerun for this family.
- Next: direct Text Return through existing ReturnValue plan/evaluation/terminal,
  common result contract and both final consumers. Worker reviewed same-owner
  Materialize/Core acquire → install → child cleanup → hold → original Home cleanup.
  Keep the fresh result Fault-only End and publication events joined; no fake Local.
  Unknown formal/Core callee completion ordering remains a separate required fix;
  never infer Text from names/MIR types or create another result map.
- Existing Loop Call/If and Function Return retain original reads, producers,
  Phi keys and root cleanup through both consumers. Plain scalar If requires
  whole leaf/read census, so it cannot steal Static Call/field ownership.
- Original source liveness feeds CFG finishing. Existing substring tuple protects
  Loop Return, changed Return refusal and scalar If purposes; prior related364PASS/
  36oldFAIL remain. Opaque draft stays stashed pending intentional adoption/disposal.

## Shared remaining contracts

- Common storage D0 supersedes ParkedSealed. Dynamic runtime Empty/null/I64/Bool/
  Text/object/Array slots and complete End descriptors exist; source activation,
  canonical Dynamic12 publication and both final consumers remain incomplete.
- Required: checked Integer view/replace with Fault-without-mutation, original
  signed-only SUM and source/slot/transfer, complete detached End in reverse field
  order to actual HomeRelease/Fault. Runtime support grants no source permission.
  Retain Empty/null distinction, plain-reclaim/Null refusal and legacy mixed guards.
- Map placement, stored borrow/Shared and carrier forwarding remain unadmitted.
- Array holder/contents/End, exact empty-transfer census and canonical writer must
  stay joined. Runtime empty End grants no source ownership or physical transfer.
- Earlier SSA/handoff/Array teardown/synthetic callee and field/count/composition reds
  remain open. Full history is in Git; do not create a parallel inventory.

## Measurement limits and completion

- mimalloc suite PASS includes removing an obsolete generic JSON all-Invoke refusal
  precheck; canonical EXE already worked. This was a harness correction, not new capability.
  Existing publication tests reject four object-lifecycle generic profiles; they do not
  reproduce the same mimalloc closure or every Invoke subtype. Preserve that distinction.
- JSON smoke must satisfy every artifact, execution, output/exit and negative contract.
- Membership, isolated physical execution and local green are not product completion.
  Complete common storage and both final consumers; retire replaced exclusive edges
  with caller/validation closure, retaining shared owners and finite-goal obligations.
