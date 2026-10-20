# APP-json-stream-aggregator

Status: JSON selected; common untyped-storage target accepted. Date: 2026-10-20
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Product evidence

- Last fixed suite at `6189d252de`: 7PASS/4FAIL, actual EXE6 + empty marker1, no SKIP/timeout,
  runner499.338s. mimalloc156.737s, allocator179.746s, typed38–43s.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --jobs 1 --timeout 1900 --format json`.
- Current source-normalization CLI34.13s; focused build58.52s/run0.53s.
  Unchanged quick runtime/private C FFI, no overlapping Cargo. No fixed-suite rerun
  for this source-proof normalization; Dynamic physical permission remains closed.
- Matching JSON smoke advances beyond borrowed-static Text-expression pending,
  now stops in MIR at `owned-field-write/original-source`, before EXE/link/runtime.
  Exact stop: UserStats.add, app line87 `me.ok_count = me.ok_count + 1`, owner22,
  Body1/IfThen0, canonical object0/field2. Premise is now `execution-storage`:
  SAME signed RHS/read proof passes; Dynamic read/replace/End execution is still owed.
  Body8 bytes, StringHelpers.to_i64 Loop, Map/report and full-app composition remain.
- Other product reds: untyped-field source activation, binary-trees source-not-i64,
  boxtorrent stored-child receiver. These are incomplete, not waived baselines.

## Connected source and next execution

- FieldWrite RHS now reuses the existing ClosedNumeric/read recheck and exact plan;
  no second proof/index is issued. Signed-only closure preserves absent annotations.
  Original Lowering/Finalized context is retained through every caller and final scan.
  Dynamic target permission stays closed until transaction/End and both consumers join.
- Existing RHS2/field7/write4/entry1/receiver2/brand1/source-mutation2PASS. No new test
  case/fn/file/hook/checker/token. Canonical typed Self update EXE prints1/Result0/exit0.
  Read-claim7/2FAIL and composition9/2FAIL reproduce on prior frozen binary.
- Signed values borrow the SAME uncaptured point SUM and physical producer.
  Finite-input aliases retain SUM/read/Copy and producer forwarding; `end = start`
  closes source-use. Pure scalar aliases remain generic. Prior entry21/Static args2PASS.
- TextResidence Return retains the same packet/acquire/install/hold and original
  Home rank through recorder/pending/finishing/both final consumers/publication.
  Normal skips only that Home; Fault retains original reverse order. Plain exits
  remain Emitted. Duplicate/stale hold, replacement and borrowing remain refused.
- Ordinary Text Call holds its result below the exact argument End suffix.
  Call Fault acquires nothing; argument-End Fault cancels remaining args, result,
  then outer cleanup. SSA/Rust/C retain lifecycle dependencies apart from arguments.
- Text Local Call retains full original input, affine publication, result residence,
  install/hold and both final consumers. Canonical prior EXEs `!ab`/`ab`/exit0 passed.
  Zero-input Text keeps existing Zero evidence; old I64 census/refusals stay intact.
- Fresh Text Return uses original ReturnValue/shared expression qualification;
  literal Materialize and Core acquisition join child cleanup, hold, original Home
  cleanup and Fault cancellation. No fake Local or second result registry.
- Original Static input dependencies now order required Text callees before callers
  in one walk. Identity/slot/brand stay original; cycles grant no unfinished result.
  Callers borrow the same completed Text contract even when old catalog is unknown.
- Input finishing retains the same Completion/terminal-map Rc in existing evidence.
  Same-package publication borrows that contract before completed_context install;
  every Text packet rechecks it with whole input and affine handoff identity.
  Passive argument observation does not depend on result classification or grant ABI.
  Replaced declaration-only publication entry has no production caller and is test-only.
- Existing optimized source tuple covers forward declaration plus unknown-formal
  Text return through full publication; missing install/hold and changed Return
  remain rejected. Canonical forward `make(text){return text.substring(0,1)}`
  EXE builds/runs `a`, Result:0/exit0. No new test fn/file/hook/checker/guard/token.
- Focused: result96/Core11/input6/packet3/dispatch2/received4/Invoke60/final projection4/
  argument-port13PASS. Initializer79/2FAIL and terminal6/2FAIL remain unchanged;
  callback0/1FAIL reproduces identically on prior frozen binary. Plan5/1FAIL and
  admission2/2FAIL previously reproduced on clean HEAD; all remain owed, not waived.
- Next: checked Self read copies use the same ExactLexicalRead, not a received Call.
  Join Dynamic FieldGet/Set prepare/commit/old-End with both final consumers;
  retain precommit Fault/no mutation and postcommit Fault/no rollback. No gate-only activation.
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
