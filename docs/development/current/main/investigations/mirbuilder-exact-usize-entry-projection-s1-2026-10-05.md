# Exact usize source entry projection S1

Status: verified / scoped closeout
Execution row: MIRBUILDER-EXACT-USIZE-ENTRY-PROJECTION-S1
Scope: source-backed parameter representation only; no usize backend admission.
Related:
- mirbuilder-call-argument-drift-diagnostic-t0-2026-10-05.md (landed cbd5ec43c7)
- mirbuilder-exe-acceptance-exact-usize-d0-2026-09-25.md
- mirbuilder-exe-acceptance-exact-usize-s0-2026-09-25.md
- ../design/mirbuilder-final-pipeline-ssot.md
- ../../../RULES.md

## Decision

Read-only worker audit_cost_oct5 and source inspection identify four app errors:
SizeClassBox *_usize wrappers pass their usize entry to an unannotated scalar
callee. Shared builder_metadata projection deliberately remains i64-only and
therefore records Box(usize). Existing ExactTrivialParameterAbiV1::USIZE already
projects Integer, but its verified source row is not joined to the entry value.

Source authority + canonical issuer: callable_parameter_contract/issuer issues
ExactTrivial(USIZE) from the resolved declaration/binding. The selected input
lends that original row to its existing request-local callable entry state.
Join it against that state's original parameter binding and the existing
PreparedCallableEntryValuesV1, declaration name/type, formal ordinal and ValueId.
Project only those exact rows to Integer before body lowering. Keep source
usize identity and ExactNumeric entry runtime checks; no proof elision.

Non-authority: Box("usize") alone, scalar/return/literal classifiers, callee
names, C defaults, generic metadata or second source scan. No new registry,
semantic issuer, ValueId, physical lane, or backend permission.

Fail-fast boundary: foreign/duplicate source binding, ordinal/name/type drift,
receiver collision, missing or mismatched formal value/type reject before the
entry commit. Existing final entry-contract verification and source verification
remain independent. Shared metadata and return projection stay unchanged.

Production caller: with_selected_source_scope -> existing callable entry adoption.
Replacement: exact selected usize entry's Box representation only. Unselected
legacy/reference callers retain their existing behavior. Backend i64-only gate
continues to reject unsupported usize execution; this is not whole-app success.

Smallest next slice: private exact-parameter entry projection child, mechanically
retained source rows beside existing entry state, preflight then infallible
signature/value-type commit in the existing adoption owner.

## Acceptance

- Real source package/selected input/entry adoption projects exact usize to
  Integer while keeping declared usize and the original ValueId/binding.
- i64, opaque, user object, return and unselected metadata behavior unchanged.
- Foreign binding, declaration/name/type/ordinal drift and repeated entry
  adoption reject without committing the projection.
- ExactNumeric parameter entry row remains runtime_check_required=true,
  proof_elision_allowed=false; missing/drift rows still reject independently.
- Existing borrowed entry scope positives/negatives remain green.
- Fresh unchanged app probe records exact remaining frontier, not a PASS claim.
- Pointer/diff and required scope guard; known unchanged root test1351 debt
  is classified, never counted as whole guard PASS.

## Other obligations

Five handle edges are separate: reallocResult -> realloc is DeclaredObject owned
transfer; four isLiveHandle/release edges borrow an owned binding into opaque
formals. Source release updates allocator policy, not descriptor Home disposal;
its name is not consuming authority. Existing formal-
forward-result D0 records owned-formal cleanup and consuming-actual enforcement
as residual. Class equality or a borrowed-tagged exception cannot close them.
Protected C/S1/S0/Array/S2 WIP and required semantic landing remain owed.
Previous goal turn was prompt review, no product progress; this turn pushed the
verified diagnostic commit and selected the next source-backed construction.

## Verification checkpoint

Original source rows are mechanically lent to existing entry state. Private
preflight checks binding/ordinal/declaration/ValueId/type/carrier before the
existing entry owner commits. Shared classifier, returns and backend gates
remain unchanged. Read-only review found carrier-column gap; corrected before
final verification. Test harness compile errors were fixed, not baseline.

Focused source tests4/4 PASS (latest session53581 terminal0,
/tmp/hako-exact-usize-entry-S1-final-tests.log). Includes source-issued foreign
owner/duplicate/ordinal rejection with subsequent valid staging (no partial
commit), unselected Box(usize) preservation, name/type and missing/short/Map/
bits/tagged carrier negatives, repeat adoption, missing exact check-row rejection,
plus real-source wrapper finalization both optimization modes. Final signature
and metadata type Integer, original usize Param fact/ValueId and required runtime
check/no proof elision all retained.

Latest same test binary: borrowed_entry19/19, usize24/24, parameter_entry7/7 PASS
(*-borrowed-regression.log, *-usize-regression.log, *-entry-regression.log).
Pointer/diff checks pass. Numeric source scope pins pass; whole guard stops at
root test1351 byte-identical HEAD SHA256
5fa8091c1e29602c85d65b0e3dfe41a3444de8c48b41138ec60de295df865467;
NOT whole guard PASS (*-scope.log). New private source101/test314 lines.

Fresh CLI quick llvm-boundary build PASS (session78181 terminal0, 2m56s,
*-cli.log). Unchanged mimalloc-lite --emit-exe rc1 (*-app.log), no EXE:
all four SizeClassBox usize drift edges disappear; five HakoAllocHandle drift
edges remain exactly as the T0 census. No verifier relaxation or source mutation.
Plugin startup missing Integer/Python messages are unchanged informational
observations; the compiler reaches the named MIR boundary. Scoped semantic
commit/push and next owner selection follow this verified checkpoint.
Isolated README/scope patch prepared at /tmp/hako-exact-usize-entry-S1-owner-docs.patch;
no protected C/S1/S0/Array/S2 differences will be staged. No whole-app or goal PASS.
