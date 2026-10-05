# Declared object formal class and lifetime contract D1

Status: decision accepted / corrective source slice selected
Execution row: MIRBUILDER-DECLARED-OBJECT-FORMAL-CONTRACT-D1
Scope: five remaining mimalloc-lite handle call edges; reconcile class, borrow,
and transfer before selecting a bounded production implementation.
Related:
- mirbuilder-exact-usize-entry-projection-s1-2026-10-05.md (landed 9c26a6d425)
- mirbuilder-call-argument-drift-diagnostic-t0-2026-10-05.md
- mirbuilder-app-mimalloc-lite-formal-forward-result-d0-2026-10-04.md
- ../../../../reference/language/ownership.md
- ../design/mirbuilder-final-pipeline-ssot.md
- ../../../RULES.md

## Verified entry

Fresh quick llvm-boundary CLI passes. Unchanged mimalloc-lite EXE probe rc1,
no EXE: four usize errors eliminated; remaining five are source handle arguments,
all ExistingCallableI64. Numeric source/finalization4/4, usize24/24, borrowed19/19,
parameter-entry7/7 pass. Scope guard retains byte-identical root test1351 debt.
No Cargo live. Protected C/S0/Array ABI/S2 work remains uncommitted.

## Exact edges

- realloc -> isLiveHandle (bb369 / value1)
- realloc -> release (bb392 / value88 and bb389 / value1)
- reallocResult -> isLiveHandle (bb406 / value1)
- reallocResult -> realloc (bb416 / value1; declared HakoAllocHandle formal)

The first four need a class-proven object actual at an opaque borrowed formal.
The last crosses two declared-object formals; equality of class or i64-shaped
carrier alone cannot establish its lifetime contract.

## Decision brief

Decision: separate declared class authority from ownership/transfer authority.
Retain independent final-MIR and physical gates while resolving the original
contract mismatch. Do not weaken scalar verification or borrow tagged loans
without exact original source coverage.

Source authority + canonical issuer: declared class membership is already issued
by callable_parameter_contract/issuer from the resolved ordinary box catalog.
Ordinary formal lifetime semantics belong to language ownership.md; exact
destination/return/cleanup contracts must come from their existing source owners.
Read-only worker audit_cost_oct5 maps those owners before implementation.

Non-authority: class annotation by itself as evidence of a Home transfer; method
name release as evidence of descriptor disposal; Box class equality; C defaults;
raw MIR observations; missing/empty source loan; dormant HomeV1 syntax.

Fail-fast boundary: missing transfer/destination/return support or borrowed full-
use coverage remains unsupported before effect/artifact. No duplicate Home,
untracked consume, or implicit runtime reclamation is permitted.

Smallest next slice: determine the original source receipt for declared-formal
class borrowing and separately the last edge's destination/return relation.
Choose one responsibility, exact callers, replacement and positive/negative gates
after the mapping is fixed. No ownership lane or parked task is reopened here.

Non-claims: no arbitrary object ABI, HomeV1 implementation, take/share grammar,
global Box-to-i64 admission, complete allocator or whole-goal success.

## Contradictions requiring resolution

Original formal-forward-result D0 describes DeclaredObject as moved-in/out;
model.rs repeats owned-transfer language. Language ownership.md ordinary typed
parameters are noescape handles, while source take linkage is inactive. Check
whether an existing exact bounded destination receipt supplies this exception;
if absent, correct the earlier inferred contract rather than inventing one.

Page.release only changes block_used/free_stack allocation policy; Heap.release
forwards that borrowed input. It does not dispose the HakoAllocHandle runtime
descriptor. The original D0 residual naming release consumption is therefore
not sufficient ownership authority. Descriptor cleanup and allocator policy
updates must remain distinct in the selected plan.

## Acceptance to select with the implementation

Class-only borrow must retain original binding/owner/entry ValueId and closed
callee use vocabulary, with foreign class/binding/source-loan and escape negatives.
Any transfer must additionally prove the source destination, consume once, and
Normal/Fault/returned-forwarded cleanup. Missing authority must not mint an owned
result. Keep original app and fixtures unchanged and record its actual frontier.

## Accepted reconciliation

Root inspection and read-only worker agree: no source-issued moved-in destination
exists for ordinary DeclaredObject. Parameter issuer rejects non-Ordinary transfer;
home_demand is Handle; actual Home ABI issuance only covers Trivial/Unit. Birth
Provided has a separate exact field destination and cannot authorize realloc.
No class-annotation ownership exception will be introduced.

Plain declared object formals therefore retain the ordinary borrowed input law.
They may lend class evidence but cannot mint a fresh result Home. Correct the
earlier result issuer's DeclaredFormal owned pass-through and forwarded-actual
class-to-NullableObject conversion first. Preserve original parameter identity
through NullableForwarded; a mixed borrowed-forward/fresh-New result remains
unclaimed until an exact result ownership relation exists. This is a correction
to the current mapping, not reopening parked HomeV1 or changing language syntax.

Next responsibility: MIRBUILDER-DECLARED-FORMAL-RESULT-IDENTITY-S0. After that,
class-only typed formal -> opaque isLiveHandle borrow can reuse the existing
ingress/actual/entry owner. release Bool result and the mixed realloc result
remain separately named obligations; no promise that all five edges pass at once.
