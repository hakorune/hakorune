# Declared object entry and physical tagged borrow S1

Status: Complete / landed fe0c9ad278
Execution row: MIRBUILDER-DECLARED-OBJECT-TAGGED-BORROW-PHYSICAL-S1
Scope: source-admitted DeclaredObject entry/header projection through the existing
BorrowedTaggedValue, final object view publication and C class/null transport.
Related:
- mirbuilder-declared-object-tagged-borrow-source-s0-2026-10-05.md
- mirbuilder-declared-object-formal-contract-d1-2026-10-05.md
- ../design/mirbuilder-final-pipeline-ssot.md
- ../../../RULES.md

## Decision

Source authority + canonical issuer: original parameter contract and S0's exact
borrowed source loan, owner/ordinal/binding/class, canonical object membership and
existing entry ValueIds. Preserve the ordinary borrowed lifetime: no moved-in Home.
Existing entry ledger retains source correspondence; do not issue a second class
registry or infer class from MirType. Loan the class from the original source at
preflight, or mechanically carry it in the same staged entry row if needed.

Selected production consumers: source_scope stages the same original source entry;
borrowed_entry preflights exact declared name/type/header/ordinal/value/carrier;
binding_materialization performs all preflight before entry installation, then
commits Integer physical representation and BorrowedTaggedValue on the existing
column. The physical Integer never creates numeric or ownership authority.
Original source kind and declared parameter metadata remain intact.

Finalized source projection lends its existing canonical object views unchanged.
Final object_view_values publishes every proven class/null view with the same
exact root/duplicate verification, including ignored declared and forwarding-only
opaque formals. is_declared authorizes Box header projection only, not inferred
opaque views. Existing field coverage remains independent and exact.
The ABI layout reference set joins those published param views by existing
function index/param ValueId so null-only ignored domains retain their layout.

Wire: every borrowed param has exactly value/representation/object_view. The view
is uint canonical object for class/null, explicit null for unrestricted opaque.
Other param representations keep the existing two-key schema. No new carrier,
domain discriminator, compatibility retry or second serializer.

C schema requires that borrowed key, validates null/unique-layout uint, rejects
scalar incoming when uint, and checks known New/receiver canonical class plus
Forwarded original Copy-root/view. Nullable acquired results keep their existing
LIVE proof; their index has no producer-class row, so default object_id0 cannot
prove class. The prologue restricts uint views to tag0/payload0 or tag3 with exact
runtime class, using existing nyash.object.type_id_h and layout.runtime_type_id.
Borrowed class checking adds no retain/release and precedes callee effects.

Non-authority: Box whitelist, payload bits, runtime liveness alone, names as
target resolvers, missing optional evidence, inferred Home, fixture exceptions.
Missing/foreign source class, header, entry value, class view or carrier fails
before installation/publication/artifact at its existing owner.

External C cannot distinguish a source-inconsistent uint->null edit from a new
unconstrained function without the original AST. That correspondence negative
belongs to the source-backed final Rust boundary; C missing-key rejection is a
different obligation. Do not claim compile-time nullable producer class evidence
or recovery of erased source semantics.

## Acceptance and work order

1. Exact-source entry positives for declared object and null, alongside opaque
   regressions. Reject foreign header name/class, ordinal/binding/value/collision,
   implicit receiver, physical carrier and original type-context drift. All
   preflight errors leave entry and metadata uncommitted.
2. Same declared-original/copy -> opaque guarded field-reader through final MIR
   and JSON. Publish the ignored declared formal's uint view. The source issuer
   rejects missing/conflicting declared class constraints; final publication
   must preserve the exact canonical source class, including null-only domains.
   Header/carrier/incoming correspondence negatives remain mandatory.
3. C rejects missing/invalid view, scalar input to uint view, known foreign class,
   wrong Forwarded root/view and foreign Copy. Class checking uses runtime type
   ID, never object ordinal. Keep nullable received-result positives and borrowed
   release negatives; no relaxation of acquired-LIVE transport.
4. Both optimization settings compile/run null/object and Normal/Fault witnesses;
   caller cleanup remains sole. Update only the handwritten borrowed-ingress
   helper to explicit object_view:null and regenerate Rust-issued witnesses.
5. Focused tests, physical boundary/JSON/C affected families, package baseline
   classification, selected pins, pointer/whitespace and source size checks.
6. Fresh quick llvm-boundary CLI and unchanged mimalloc-lite frontier probe.
   Record actual terminal; do not equate fixture success with app completion.

Replacement/deletion set: source-admitted typed entry exclusion, field-only view
publication filter, optional borrowed view spelling and unrestricted
class-domain prologue for uint views. Shared generic opaque/numeric/nullable
owners stay; no backend fallback or unrelated old-edge retirement.

## Entry evidence and obligations

S0 landed/pushed c0a8bf05bd: source5/5, entry/copy18/18, package622/known3;
exact known names/causes and HEAD-identical root-test1351 guard debt are in S0.
Pointer/whitespace PASS. HEAD equals remote; no Cargo live at S1 selection.
Protected C/S0/Array/S2 WIP remains unstaged and must be preserved.

Read-only audit_cost_oct5 reviewed owner mapping, mandatory null|uint wire,
nullable class provenance gap and runtime ID distinction. Main owner integrates
the decision here; no worker editing or Cargo.

Unchanged actual app last stops artifact-unowned-lifecycle-site. Whole-definition
<= vocabulary, borrowed return/mutation, call-valued result and exact owning
Provided destination remain later mandatory obligations. This S1 does not reopen
parked ownership, rewrite production .hako or close the finite pipeline goal.

## Construction

Previous goal turn was progress: S0 source switch/negative correction landed at
c0a8bf05bd, S1 selected at da66420124; both remote hashes verified. Current-first
and original attachment re-read, pointer guard PASS. No Cargo/rustc live at entry.
Protected mixed source/C diff captured in `/tmp/hako-declared-tagged-S1-before/`
before any S1 editing; never stage whole protected paths.

Worker confirmed the existing ledger's pre-entry correspondence is sufficient
for an on-demand declared-class loan. Source_scope tuples remain unchanged;
validated owner/ordinal/binding rows alone lend is_declared=true class strings.
No additional snapshot or registry. Carrier preflight now returns the existing
carrier column plus a temporary list of exact Integer projection coordinates.
Header/name/class/value/typectx/carrier checks precede install; the existing
materialization port commits the projection and column only after installation.

Entry positive/negative tests are under the existing borrowed_entry test owner.
Cargo51502 terminal101: new fixture import/private-helper errors were this-slice;
fixed to MirParamDecl's existing module and the existing entry test parser/issuer.
Cargo35806 terminal0, focused entry21/21 PASS. New declared root/null checks,
nine header/value drift cases and inferred-opaque misuse negative pass.
Log `/tmp/hako-declared-tagged-S1-entry-tests.log`. No S1 PASS or activation claim.
JSON borrowed params always publish null|uint view. C schema/incoming/prologue edits implemented using
the same carrier and existing runtime type-id accessor. C shim rebuild41663 PASS.
Old `/tmp/hako-issued-param-field-handle.json` has only four layout keys, without
the already-required owned_residences. Old-wire borrowed/field probes stop at
abi-layout, informational stale-input observations, not S1 regression evidence.
Regenerate source-issued inputs from the rebuilt current binary before C checks.
Regenerated source24/24 PASS on that rebuilt binary. Package622/3 retains exact
same known names/causes as S0; no new package reds (full package NOT green).
C borrowed-ingress71358 terminal0 PASS; guarded field2 programs/seven forged rows
PASS on freshly issued inputs. These predate the final forwarding-only filter
removal and new declared-source witness; rerun affected acceptance after rebuild.

Worker confirmed a concrete forwarding-only opaque class proof loss: its view
was filtered out while the destination uint domain requires that view. Selected
mapping corrected to publish all original proven class/null views, preserving
exact field-use coverage and C Copy-root/class checks. Removed the attempted
extra declared-flag final map transport; no new semantic product or registry.
Worker also confirmed null-only declared view lacks New/get/release references:
join the existing param view into the same canonical ABI object inventory.
Physical definition/layout validation remains the original owner.
Cargo94953 terminal101: first source witness included return-of-local-call-result,
which hits the existing borrowed-result/source-not-i64 boundary. This is the
explicit later call-valued result obligation, not a typed transport proof. The
S1 witness now executes the forwarded field call with its existing literal-I64
result contract; production source is unchanged and result expansion remains owed.
Added opaque original/Copy forwarding with both object and null incoming edges,
and explicit null-only ignored assertions that no Item New/read operation supplies
the referenced layout. Cargo8394 live for the rebuilt source witness;
log `/tmp/hako-declared-tagged-S1-source-tests.log`. S1 source/C/runtime acceptance
and selected guard/doc closeout are not yet complete.
New C execution harness is written but not run: both opts, exact caller Normal
cleanup and every selected Birth-store Fault edge, plus missing/foreign domain,
layout/forward-root and scalar negatives. C nullable spelling cannot erase a
known New/Copy class; unknown acquired nullable still uses the runtime guard.

## Continuation evidence

The intervening prompt-writing turn changed no repository state. Re-read the
original goal/current-first files; pointer PASS. Poll8394 terminal101, no Cargo
left live: declared/null/optfalse stops `check-uncovered-function` because the
unused `Item.birth(value)` carries a numeric runtime-check contract outside the
selected physical functions. This is not borrowed ingress authority. The S1
fixture now uses literal Birth stores and omits the unused opaque reader for
ignored shapes; no production .hako or verifier change.

Cargo55231 terminal101: declared/null/optfalse publishes, opttrue stops
`ordinary-terminal-field-return/physical-drift`. The direct field-result
optimizer correspondence needs its owner audit; it is not waived or classified
as a passing S1 result. The selected borrowed-use witness now reads the field in
an already-admitted comparison with literal I64 exits, isolating class transport
from direct field-result identity. Both optimization settings remain required.
Cargo68845 live; source log remains `/tmp/hako-declared-tagged-S1-source-tests.log`.
Whole-goal result/optimizer obligations remain open; no completion claim.

C negative harness now covers foreign class under nullable spelling, receiver
Copy masquerading as the original formal, and borrowed-formal release. A private
linker-wrap probe perturbs only the real runtime type query, keeping source-issued
layouts intact; object invalid-before-body and null/no-query control are separate
from Normal/Fault cleanup acceptance. Python/C syntax checks PASS; runtime
acceptance is pending the complete newly issued source inputs.

Selected owner README contracts and scope pins updated. Guard reaches the same
HEAD-identical root-test1351 debt; full guard is NOT PASS. No unrelated guard
debt repaired, no commit/staging and no production app probe yet.

Cargo68845 terminal0: source witness PASS, all20 null/object/shape/opt inputs
issued. Full source family25/25 PASS. C execution85078 terminal0: all20 Normal
and every Birth-store Fault edge PASS;15 forged-domain/Copy/release/scalar inputs
reject before artifact, preserving the destination marker. Runtime query drift
stops with Invalid rc70 before bridge's local construction; null control makes
no query and returns11. This is Invalid evidence, not Fault cleanup evidence.
C generic borrowed72527 PASS; guarded field2 programs/seven negatives PASS;
nested-call/receiver-identity/physical-parser C tests3/3 PASS.

Final source boundary11/11 and JSON29/29 PASS. Entry filter `borrowed_entry_tests`
selected0 tests and is not PASS; corrected to the actual module path
`normal_callable_semantic_lowering_state::borrowed_entry::tests`,21/21 PASS.
Package622 PASS/exact known3: ReceiverNonEscape Capture,
IncompleteOrdinaryNewCoverage, MapLifecycleUndertaking BorrowedEntryEscape;
same named failures/causes as S0, whole package remains NOT green.

Read-only worker traces direct field-result drift to original block-ID lookup
in `ordinary_new_terminal_field_return.rs`: CFG sole-predecessor merge can move
the exact read. Existing PhysicalBoundary/FinishedBindings owns the mapping;
the field/result/Return checks must remain exact. This is an owed separate
result correspondence repair, not a borrowed transport exception. Final opttrue
coordinates for the failing direct-result case are not yet measured.
Source-final missing/foreign view acceptance is under owner audit; required CLI
rebuild and unchanged app probe, scoped landing and S1 closeout remain owed.

Latest execution: CLI quick llvm-boundary build39562 is confirmed live by its
handle and cargo/rustc processes; log `/tmp/hako-declared-tagged-S1-cli-build.log`.
Poll this handle to terminal, do not start another Cargo or edit compiled Rust
while it runs. C execution6961 re-run after pinning runtime type ID distinct from
canonical ordinal is terminal0 PASS. Pointer/whitespace PASS. Worker
audit_cost_oct5 is reviewing the remaining source-final view requirement
read-only; no new source authority or duplicate registry is authorized.
No staging/commit/push, S1 and finite goal remain active.

## Final source-view Decision clarification

Previous turn was progress: source/C both-opt execution and negative evidence
changed the next action. Current-first/pointer PASS. Build39562 terminal0 quick
llvm-boundary PASS. Fresh unchanged app EXE probe rc1 at the same
`ordinary-new/local-commit/artifact-unowned-lifecycle-site`; no EXE generated.
Log `/tmp/hako-declared-tagged-S1-app.log`.

Read-only worker found no production path that mutates the immutable source
view map. Its issuer seeds the original DeclaredObject contract and rejects
missing/conflicting classes; final source lending and entry projection preserve
all views and exact roots. Final entry alone does not independently reject a
synthetic deletion of that private map, nor compare all-null actuals to a class.
The earlier acceptance wording asserted such a gate incorrectly. Corrected it
to the actual issuer/publication boundary; no new registry, weakening of incoming
contracts or waiver of external-input checks. The source witness now compares
the published view to the original canonical Item definition index, independent
of transported views/layouts, and retains erased-carrier/changed-producer
negatives. Cargo46691 live for this strengthened source test.

## Closeout

Cargo46691 terminal0 PASS: exact canonical class/null projection for all20
source-issued cases, both opts, plus erased-carrier and changed-producer
negatives. C75396 terminal0 PASS on these regenerated inputs:20 Normal/Fault
programs, runtime query drift/null control and15 preartifact negatives.
Source25/25, entry21/21, boundary11/11, JSON29/29 and affected C families are
recorded above; package622/exact known3 and full-guard root1351 debt remain
classified unchanged baseline, never claimed green. Selected scope pins pass
before that HEAD-identical debt. Original .hako files are unchanged.

Source/header projection, all-view publication, mandatory borrowed key and uint
class-domain C checking are the sole selected paths. The field-only exclusion
and optional borrowed-view serializer spelling are removed at their original
owners; generic opaque/nullable/numeric owners remain. No fallback or second
registry. Owner README contracts updated. Rustfmt audit leaves an unchanged
pre-existing `nonnull_successors` formatting difference only; selected changed
function formatting corrected. Pointer/whitespace PASS; selected source files
remain below800. Pre-S1 snapshots isolate16 tracked hunk paths from protected
work. Scoped source/tests/C/docs landing only; older C/Array/S2 WIP stays.

S1 entry/physical transport acceptance is complete. Production app still stops
artifact-unowned-lifecycle-site, with no EXE; direct field-result optimizer
correspondence is a separately identified required repair. Neither app nor the
finite pipeline goal is complete. Next select that exact existing physical
projection owner, then continue result/Provided obligations and pending scoped
C/Array/S2 landing according to the same goal.

Scoped S1 landed/pushed `fe0c9ad278`; remote branch hash verified equal to HEAD.
Protected C/Array/S2 diff remains unstaged. Next selected existing final field
read projection repair; full finite pipeline remains active.
