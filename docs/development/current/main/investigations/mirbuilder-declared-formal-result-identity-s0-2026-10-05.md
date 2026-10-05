# Declared formal result identity correction S0

Status: verified / scoped closeout pending commit
Execution row: MIRBUILDER-DECLARED-FORMAL-RESULT-IDENTITY-S0
Scope: prevent a borrowed ordinary declared formal from issuing an owned result.
Related:
- mirbuilder-declared-object-formal-contract-d1-2026-10-05.md
- mirbuilder-app-mimalloc-lite-formal-forward-result-d0-2026-10-04.md
- ../../../../reference/language/ownership.md
- ../design/mirbuilder-final-pipeline-ssot.md
- ../../../RULES.md

## Decision

Language ordinary typed formal is a noescape handle. Source contract issuer
issues DeclaredObject class membership and Handle demand, not moved-in Home.
The earlier result-class issuer incorrectly treats this as owned pass-through
and upgrades a borrowed forwarded result to NullableObject using only the actual
formal's class. Those two production mappings must be corrected.

Source authority + canonical issuer: original callable parameter contract's exact
batch slot, ordinal and BindingRef; existing result exit draft/call-actual binding
relation. Reuse NullableForwarded identity. Preserve fresh New and null rules.

Non-authority: class annotation/equality as Home acquisition or transfer; release
method name; absent take linkage; Birth Provided outside its field destination.

Fail-fast boundary: wrong/missing/rebound actual binding, mixed fresh and borrowed
returns, unknown destination and unsupported result relation remain unclaimed
before received_nullable Home publication. No scalar/backend verifier relaxation.

Production caller and replacement: ordinary_new_result_class_claim's exit fold
and ForwardFormal projection. Remove its DeclaredFormal owned-result inference
and declared actual class -> owned NullableObject substitution. No shared owner
is physically deleted and no additional registry/source scan is added.

Smallest slice: ordinary typed formal returns and composed forwarding return only
the original formal ordinal (nullable identity). Correct model/owner wording and
record the prior D0 contradiction with commit-backed evidence. Class-only borrow
ingress, Bool result and mixed realloc result ownership are later slices.

## Acceptance

- Fresh object/null result claims unchanged.
- Direct typed and opaque formal returns claim NullableForwarded identity,
  never NullableObject merely because a type name resolves.
- Composed forwarding preserves exact caller formal ordinal/binding; no new Home.
- Mixed fresh New and borrowed forward stays fail-closed, even with equal class.
- Missing/foreign/rebound/unsupported actuals cannot create owned result claims;
  existing result claim negatives remain meaningful.
- Focused result contract positives/negatives, required scope guard and pointer/
  whitespace checks. Record exact new/known/observational reds without exemption.
- Fresh unchanged app probe records actual new frontier; no .hako workaround.
- Selected sources remain below800; protect all C/S0/Array/S2 work and isolate
  source correction from the upcoming class-borrow admission.

## Entry

Numeric S1 landed/pushed9c26a6d425; fresh app9->5 exact handle drift edges, no EXE.
D1 read-only worker found no ordinary moved-in source authority; root checked
language ownership, parameter model/issuer and both incorrect result branches.
Source corrections preserve declared/opaque identity and reject equal-class mixed
fresh/borrowed results. Initial focused runs each passed18/19: the remaining
failure was this slice's census expectation, first its count then Debug spelling.
Actual source census is6 covered/2 rejected at structural Body(3), not8/0. The
latest assertion compares the resolved source path segments directly.

Worker additionally found a correctness gap: original formal BindingRef survives
reassignment, so both direct returns and forwarded actuals could misidentify a
new value as the incoming argument. Existing resolved BindingRebind facts now
supply one shared unrebound predicate for direct formal return, direct/method call
actuals and the existing sole-local initializer rule. No assignment-order analysis
or alias admission is added. Six declared/opaque direct/method/local-result
counterexamples pin rejection while the untouched give callee keeps its claim.

Focused Cargo session6116 terminal0:20/20 PASS, including six formal-rebind
counterexamples. Log:/tmp/hako-declared-formal-result-identity-S0-rebind-tests.log.
Same rebuilt binary full package:617PASS/3fail. Exact names and causes match the
pre-slice baseline (ReceiverNonEscape Capture, IncompleteOrdinaryNewCoverage,
MapLifecycleUndertaking BorrowedEntryEscape). Log:
/tmp/hako-declared-formal-result-identity-S0-package.log. No new package red.
Scope guard reaches byte-identical HEAD root test1351 debt after selected pins;
whole guard is not PASS. SHA256:
5fa8091c1e29602c85d65b0e3dfe41a3444de8c48b41138ec60de295df865467.
Log:/tmp/hako-declared-formal-result-identity-S0-scope.log.
Owner wording/old D0 correction implemented. Fresh CLI llvm-boundary build
session26430 terminal0,2m44s (/tmp/hako-declared-formal-result-identity-S0-cli.log).
Unchanged app EXE probe terminal1: named artifact-unowned-lifecycle-site before
artifact; EXE absent. Log:/tmp/hako-declared-formal-result-identity-S0-app.log.
The earlier numeric-five drift evidence is pre-correction, not the current app
frontier. Preventing an invalid received_nullable Home exposes the already
unresolved mixed-result/destination contract; app acceptance remains owed.
Plugin startup warnings match the prior probe and do not prevent this named
source/MIR boundary. Pointer/whitespace checks PASS; selected source701/test686
lines below800. Scoped semantic commit/push remain pending.
Goal and parked lanes unchanged.

## Read-only review

Worker audit_cost_oct5 confirms the common rebind proof covers direct formal
return, method actuals, direct actuals and sole local initializer. Foreign and
alias-local actuals remain conservatively unclaimed by exact original parameter
binding membership; no new alias acceptance is added.

The proposed bare-static handle forwarding positive was rejected by owner audit:
source-index ExactCallableParamAbi currently admits I64/Map, and exact header
validation requires annotated supported formals. Ordinary handle headers therefore
cannot issue a direct target in this cohort. Do not fabricate an i64 header or
expand that source index for an identity fixture. Method-call negatives exercise
the reachable forwarded-borrow path; the direct collector uses the same helper
for any future separately authorized expansion. This is not direct-call handle
production acceptance.

## Remaining production obligations

Current result contract already preserves exact exit-site terminal relations:
Construction/Home/Handle can distinguish fresh acquisition from a borrowed input.
A later composition must preserve that distinction, rather than issue an owned
NullableObject for their union. Original reallocResult additionally stores that
result into the Provided owned HakoAllocHandleResult field. A borrowed result
requires an available destination ownership contract; an exit disjunction alone
cannot create one. Read-only worker maps class-only borrow ingress first, then
exact result composition/destination. No hidden retain, source workaround or
parked ownership-lane reopening is authorized by this correction.
