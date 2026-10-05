# Declared object joins the existing tagged borrow source cohort S0

Status: source acceptance verified / scoped closeout
Execution row: MIRBUILDER-DECLARED-OBJECT-TAGGED-BORROW-SOURCE-S0
Scope: original declared class/null formals join the existing ordinary borrowed
source draft, incoming/forward graph, class fixpoint and actual join.
Related:
- mirbuilder-declared-object-formal-contract-d1-2026-10-05.md
- mirbuilder-declared-formal-result-identity-s0-2026-10-05.md
- ../design/mirbuilder-final-pipeline-ssot.md
- ../../../../reference/language/ownership.md
- ../../../RULES.md

## Decision

Reuse BorrowedTaggedValue for ordinary DeclaredObject. This is the same borrowed
lifetime with a declared class/null constraint. Preserve source DeclaredObject;
never turn the annotation into Home transfer. Do not introduce a second raw-i64
nullable-formal ABI, carrier, actual registry or implicit retain.

Source authority + issuer: original callable parameter contract's exact owner,
batch slot, ordinal, binding and DeclaredObject class. Existing use draft proves
all tracked uses, rebind/capture exclusion and copy origin. Existing source
incoming/forward graph covers all callers. Existing object view fixpoint resolves
canonical object membership; declared class is a constraint even without a field
read. Existing actual preparation corroborates live-prefix root and incoming
class/null. Dynamic opaque formals keep their original numeric vocabulary;
declared object roots cannot supply NormalInteger/new/array numeric authority.

Non-authority: MirType/Box whitelist, class equality as ownership, payload bits,
optional missing proof, method names, derived physical observations or old fallback.

Fail-fast: wrong class or scalar actual, missing canonical object, incompatible
forward view, original binding/ordinal/domain drift, capture, rebind, unsupported
use or outside selected graph cannot activate this declared borrowing contract.

Production replacement: source selection/use/actual owners replace opaque-only
exclusion for admitted original typed formals. The existing source graph and
Forwarded actual remain sole; no new source classification pass is introduced.

## Bounded sequence

S0 owns source draft/actual/class admission and meaningful package-level positives
and negatives. It does not claim physical activation. Next S1 must preflight the
same original class/header/entry ValueId and project the existing tagged carrier;
final source/physical owner publishes declared object views even for ignored
formals. C must corroborate the class/null restriction per incoming ordinal and
prologue, using existing tagged transport and object_view, with no new carrier.
S1 must prove source->final MIR->JSON->C runtime both opts, null/object, Normal/
Fault and no extra release before this series is called a production cutover.

Required production targets remain realloc/reallocResult -> isLiveHandle and
unchanged mimalloc-lite acceptance. Original callers currently stay outside the
whole-definition profile at requested_size<=0; resizeInPlace also lacks borrowed
return/mutation vocabulary and Heap isLiveHandle lacks call-valued result proof.
These are explicit subsequent obligations, not permission to shrink final scope,
use a source workaround, reopen parked ownership or declare app success.

## Acceptance

- Real package issuer admits typed original formal/copy forwarded to an opaque
  guarded i64-field callee, and retains original DeclaredObject source kind.
- Typed destination also joins existing incoming source graph; exact null allowed,
  same class allowed, integer/bool/foreign class rejected even if formal ignored.
- Annotated numeric sibling/view spoof, rebind/capture/unsupported escapes reject.
- Existing opaque source/use/actual/entry families remain green; selected package
  and guard failures classified by exact name and cause.
- Guard/pointer/whitespace; selected sources below800, split tests before760.
- Physical Box header rejection remains until S1; no whole-app/goal claim.

## Review

Read-only worker audit_cost_oct5 compared both designs against production owners.
Raw nullable formal would require new exact param/copy-root provenance and would
not fit acquired-LIVE nullable_typed_object guard. Same tagged path reuses existing
Forwarded/Copy proofs and avoids that extra ABI. Worker identified the numeric
origins.contains_key hazard and unconditionally-required declared view publication;
both are mandatory obligations in this accepted mapping.

S0 entry: fc9d88108a+bcb8da56c6 pushed; result focused20/20, package617/3 exact
baseline. CLI quick llvm-boundary passes; unchanged app stops artifact-unowned-
lifecycle-site, EXE absent. Protected C/S0/Array/S2 work stays unstaged. No Cargo.

## Construction and verification

The prompt-drafting turn changed no compiler state (no progress toward the finite
pipeline). Revalidated the live worktree and original goal before continuing.
Previous Cargo83740 is terminal0, source focused4/4 PASS; no Cargo/rustc remained.
S0 original compile E0507 was this-slice and fixed by borrowing the source kind.

Worker found an Array index sibling still receiving all handle origins. Corrected
the existing classifier argument to numeric_origins, not a new classifier or
registry. The exact-source regression obtains the real receiver entry loan and
compares an opaque index positive against declared/copy index negatives; it
checks the draft's ArrayElementValue itself, before graph closure could hide the
wrong authority. Added same-class live Home/copy and direct/copy capture cases.
The live foreign-class actual already uses a local `new Other` binding, so its
class mismatch is tested rather than unsupported inline-new syntax.

Cargo59526 terminal101: new test referenced receiver proof through the wrong
private module; this-slice E0425, corrected to the existing coseal re-export.
Focused rebuild38445 terminal101: four existing tests pass, new Array control
fails because its `.set` was inside the if, outside the existing source draft's
sequential guard vocabulary. This-slice fixture error, not baseline; corrected
to the existing `if > { return }` then `.set` shape without widening vocabulary.
Focused rebuild44308 terminal0, source5/5 PASS; log
`/tmp/hako-declared-tagged-borrow-source-S0-tests.log`.
Full rebuilt package622 PASS /3 known baseline, including142 borrowed tests;
log `/tmp/hako-declared-tagged-borrow-source-S0-package.log`. Exact names/causes:
birth_receiver_non_escape_rejects_unproven_uses_before_row_publication retains
InstanceConstructors ReceiverNonEscape Capture; main_static_child_port_consumes_
all_role_rows_once retains IncompleteOrdinaryNewCoverage; qualified_call_map_
argument_reaches_the_named_capability_boundary retains MapLifecycleUndertaking
BorrowedEntryEscape. Compared against previous identity-S0 package617/3 log,
ignoring only process-local compilation ordinals. No new/unclassified package red.
Scope pins pass, full guard stops at HEAD-identical root test1351 size debt
(SHA256 5fa8091c1e29602c85d65b0e3dfe41a3444de8c48b41138ec60de295df865467);
whole guard is NOT PASS. This unchanged test-file debt does not undermine the
selected numeric/class proof; selected source/use/actual/entry remain below800.
Pointer and diff whitespace PASS. Physical entry/copy regression18/18 PASS on the
rebuilt binary; log `/tmp/hako-declared-tagged-borrow-source-S0-entry.log`.
S0 acceptance is closed; S1 remains required before production activation.
No new CLI/app evidence; physical activation and unchanged-app acceptance remain
S1/subsequent obligations. No goal completion or current frontier advancement.

## Next S1 wire decision

Read-only owner review found that optional object_view loses an ignored typed
formal's class constraint if omitted. Use the existing param field with one
closed spelling: every borrowed param has exactly value/representation/object_view;
object_view is canonical uint for class/null or explicit null for unconstrained
opaque. Nonborrowed params retain their two-key schema. No additional carrier,
domain discriminator or source registry. The original declared source still
requires a uint at the final Rust projection, even without field admissions.

C rejects a missing view key and scalar input when uint. Known New/receiver and
Forwarded inputs compare exact canonical views; nullable acquired results retain
their original LIVE proof and the callee prologue checks runtime class/null.
Their current C index has no producer-class row; default object_id0 is not class
evidence. Use existing nyash.object.type_id_h against layout.runtime_type_id,
not the canonical object ordinal. Compile-time foreign-class rejection for such
nullable results would require a separate exact producer correspondence; do not
claim it from the existing index. No hidden ownership acquisition or extra release.
The existing handwritten borrowed-ingress fixture helper is the only direct
two-key borrowed constructor found; Rust-produced witnesses must be regenerated.
External C cannot know that a supplied uint was changed to null compared with an
unavailable original AST; that source-correspondence negative belongs to the
Rust source-backed final boundary. Do not claim C can recover erased semantics.
