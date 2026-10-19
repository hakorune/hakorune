# APP-allocator-stress

Status: selected; final MIR correspondence passes; LLVM V4 admission remains open.
Date: 2026-10-19
Scope: unchanged allocator-stress, canonical physical EXE, exact output/exit and handle safety.
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[retained-handle obligation, row 7](mirbuilder-exe-acceptance-suite-red-disposition-d21-2026-09-27.md),
[parked untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md).

## Selection and measured stop

- User selected allocator after parking untyped-field. Allocator reuses mimalloc's heap;
  binary-trees/JSON need unresolved untyped slots, JSON additional Text/Loop connections.
- Official fixed suite at `b6195de60f`: 6PASS/5FAIL, actual EXE5 + empty marker1,
  mimalloc PASS, no SKIP/timeout; outer1900/build/run300s, total704.9s.
  Later holder/group-tree changes are outside this measurement.
- Matching quick CLI passes computed Object arguments, Loop, If Phi, Bool Get condition,
  Main scalar-source/argument handoff, final holder and borrowed Call incoming coverage.
  The physical program reaches LLVM C API V4, which refuses unsupported-cohort during
  flow admission before library emission. Program has 30 functions/2819 blocks; no EXE.
- Next: locate the exact V4 instruction/type refusal and connect its existing owner contract.
  Preserve exact requested_bytes=454, rejects=4, output/exit0, route and no-replay checks.

## Connected owners

- Original named-Array source selects holder/Get/Loop Append before Text issuance.
  Same Home observation, checked Loop action, Recipe slot and child group reach the sole
  writer; no duplicate effects, AST/source rewrite or MIR-based semantic inference.
- Append transfer/Fault cancellation and holder teardown retain independent checks.
  Original Phi inputs feed existing liveness; backedges require stable resource state and
  agreement between cached/actual CFG. Termination is not inferred.
- Computed arguments retain complete signed source SUM and original purpose frames.
  I64 keeps ScalarCompleted/NormalInteger; Text keeps OwnedTextDeferred/OwnedString.
  Same Call/ordinal, method/plan/loan and evaluation-time Homes reach final validation.
- Bool Eq/Ne requires Bool/Bool, with the existing Compare writer and matcher.
  Original Call walk now lends SAME Array Get Fault inventory through array_read_records_v1;
  value-producer lending and original Read/Fault/final checks remain unchanged.
- IfForm snapshots/restores each arm and merge values/provenance. BranchNormal retains
  complete original ThenNormal/ElseNormal inputs and the same immutable flow coordinate.
  Original condition/exits, same indexed Phi and every input producer reach final checks.
  Loop coverage alone grants no physical edge correspondence. No additional SSA writer.
- Actual sorted final Phi is registered by its sole successful writer. Unborrowed final
  patch replaces its indexed observation; identity/append loans freeze it. Provisional
  Phi still completes once. Both use the same existing SSA index.
- Object candidate arguments use the SAME Taken Object packet/original Array recipe and
  existing borrowed site/arguments/Homes view; no synthetic local or Home is constructed.
  Existing receiver consumer handles nested argument Calls; candidate is consumed once.
- Scalar/Object purposes share one argument cleanup graph and independent final validator.
  Call Fault: argument Ends then prior Homes, no nonexistent candidate cancellation.
  Normal argument-End Fault: remaining Ends, SAME candidate cancellation, prior Homes.
  Success retains candidate for Append; Append Fault cancels candidate then unwinds Homes.
- Append Home normalization is committed separately: acquisition order is shared with
  Call/expression frames, existing unwind reverses once. Initializer order is unchanged.
- Zero-input scalar demands use the existing original incoming census for result and
  argument rows. Exact target, empty actuals, ScalarReturnSource identity, all Returns,
  grounding/dependencies and executable corroboration remain mandatory. No ABI default.
- Final Compare clone may omit its dead original only with SAME finisher-certified omission,
  exact Append opcode/ordered operands and mandatory surviving target. Identity cannot omit.
  Root/child final validation now seals the captured physical projection before its source
  consumers; all validators still run before the final state is published.

- Holder final inventory now uses the same finisher's definition/structural-Jump/metadata/
  mandatory-effect correspondence, retaining original Read Call and reverse Array census.
  Only value-free Jumps contract; argument-carrying Jumps stay mandatory effects.
- Final lexical visitor walks the existing per-owner group tree, including Loop Append
  child packets. Root aggregate instruction census is unchanged; original candidate/args,
  exact producer/copy correspondence and duplicate source/coordinate refusals stay intact.

## Verification and remaining evidence

- Root quick-test-fast and matching quick CLI builds PASS. Existing test list9263 unchanged.
  Focused SSA10, physical-boundary22, root2 and handoff5 PASS. Root2/handoff1 reds reproduce
  with the pre-order binary; result-protocol4 reds also reproduce with pre-source binary.
  These remain unresolved boundaries, not accepted product success.
- Result/actual and If/initializer/Object input focused results are retained with the build;
  previous connection checks remain in this card's Git history. Formal suite is recorded above.
- Latest holder root/CLI builds PASS, Array/physical focused36 PASS and 1 known teardown red.
  Visitor focused32 PASS and 1 known synthetic-callee red; test names9263 unchanged.
- Builder dependency/CallTarget guards and diff checks PASS. Worker reviewed source family,
  full cleanup graph, Phi identity and final Compare/order connection, read-only/no Cargo.
- Existing test adds final-Phi patch/borrow mutation coverage absent from provisional checks.
  Existing Compare matrix adds clone-source omission versus mandatory Identity/target checks;
  these independent boundaries require no new test function/file, hook, checker or registry.
- No runtime/C ABI change or app workaround. Manual Array writer test adapts its callback only.

## Completion obligations

- Existing allocator_stress_exe.sh must pass every artifact/run/output/negative obligation.
- Retire replaced exclusive edges after caller/validation closure; preserve shared owners.
- Untyped-slot D0 remains ParkedSealed until explicit acceptance. Full goal is incomplete.
- Completed implementation details and prior selection remain in this path's Git history.
