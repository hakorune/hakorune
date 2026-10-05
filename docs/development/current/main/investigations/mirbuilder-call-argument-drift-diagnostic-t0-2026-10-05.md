# Cataloged call-argument drift diagnostics T0

Status: verified / scoped diagnostic closeout
Execution row: MIRBUILDER-CALL-ARGUMENT-DRIFT-DIAGNOSTIC-T0
Scope: identify exact existing verifier rejection without changing acceptance.
Related:
- mirbuilder-app-mimalloc-lite-heap-construction-d0-2026-10-04.md (protected source work; S2 evidence retained locally)
- ../design/mirbuilder-final-pipeline-ssot.md
- ../../../RULES.md

## Decision

Existing check_call_edge has two call-argument-type-drift boundaries: arity and
scalar/Map domain disagreement. Its block-only message hides caller/callee and
actual/formal types, so nine real app blocks cannot distinguish these causes.
Append those already-borrowed facts to the existing error value, preserving the
exact freeze token, rejection predicate, order and count. No new logger, source
scan, receipt, authority, acceptance or publication path.

Owner src/mir/verification/invoke.rs (652 lines). Production caller is the existing
cataloged edge verifier. Replaces only its two context-free drift error constructors.
This is BoxShape diagnostic work, separate from the later semantic repair. Read-only
worker audit_cost_oct5 is reviewing source/carrier/policy ordering in parallel;
its result cannot authorize verifier weakening before the exact edges are identified.

## Acceptance / completion

- Existing invoke Call positive/negative tests remain accepted/rejected.
- Arity error reports caller/callee/actual/formal counts; domain error reports
  caller/callee/argument ordinal/value/recorded type/formal/carrier.
- Preserve token [freeze:contract][mir/invoke/call-argument-type-drift].
- Fresh unchanged mimalloc-lite CLI probe identifies the nine failing edges.
- No .hako or positive JSON mutation, fallback or global non-i64 exception.
- Pointer/diff checks and owner source below800; classify failures honestly.
- Scope guard retains known root test1351 debt, not PASS.
- Existing C/S1/S0/Array/S2 WIP protected; scoped landing remains owed.

## Current evidence

Entry: S2 provider10/10, Array18/18, wire70 negatives, JSON29/29, C3 and child
runtime pass. App stops at nine block-only errors; no Cargo live. Goal unchanged.

## Verification and exact next frontier

T0 predicates/early returns unchanged; source687 lines, no new unconditional log.
Focused Call tests12/12 PASS (/tmp/hako-call-argument-drift-T0-tests.log), including
arity, non-scalar, carrier/name drift and unchanged document policy. Fresh quick
llvm-boundary CLI build PASS (2m28s; *-cli.log). App rc1, no EXE (*-app.log),
now names all9 existing scalar-domain errors, no arity failures:

| Caller | Callee | Block / value | Actual | Formal / carrier |
| --- | --- | --- | --- | --- |
| Heap.realloc/2 | Heap.isLiveHandle/1 |369 /1 | Box(HakoAllocHandle) | Unknown / ExistingCallableI64 |
| Heap.realloc/2 | Heap.release/1 |392 /88;389 /1 | Box(HakoAllocHandle) | Unknown / ExistingCallableI64 |
| Heap.reallocResult/2 | Heap.realloc/2 |416 /1 | Box(HakoAllocHandle) | Box(HakoAllocHandle) / ExistingCallableI64 |
| Heap.reallocResult/2 | Heap.isLiveHandle/1 |406 /1 | Box(HakoAllocHandle) | Unknown / ExistingCallableI64 |
| SizeClassBox.accepts_usize/1 | accepts/1 |534 /2 | Box(usize) | Unknown / ExistingCallableI64 |
| SizeClassBox.bin_size_usize/1 | bin_size/1 |549 /2 | Box(usize) | Unknown / ExistingCallableI64 |
| SizeClassBox.good_size_usize/1 | good_size/1 |554 /2 | Box(usize) | Unknown / ExistingCallableI64 |
| SizeClassBox.size_to_bin_usize/1 | size_to_bin/1 |569 /2 | Box(usize) | Unknown / ExistingCallableI64 |

Heap abbreviates HakoAllocHeap; full symbols are in the log. Evidence is the
protected current C/S1/S0/ABI worktree, not a claim that those semantic changes
are landed. Two separate obligations remain: exact numeric representation and
user handle transport; this diagnostic slice grants neither. Worker review confirms
Birth calls are outside check_call_edge, Unit is not an exception, and original
source visitors already include Discard calls. Early source-less gate remains a
candidate to audit after exact carrier authority is established.

Pointer/diff checks pass; scope guard stops at root test1351 byte-identical HEAD
(*-scope.log), NOT PASS. Corrected S2 diagnostic-log note: --dump-mir actually
stops at named-array/retained-source-required, not the EXE type verifier token.
Next integrate worker's numeric-vs-handle authority Decision into bounded cards;
no global Box-to-i64/tagged exception. No whole-app or finite-goal completion.
