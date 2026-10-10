# Gate1 guarded outgoing transport D0

Status: selected design
Date: 2026-10-10
Scope: MIRBUILDER-GATE1-OBJECT-GUARDED-OUTGOING-TRANSPORT-D0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-result-new-site-provenance-d0-2026-10-06.md
  - docs/development/current/main/investigations/mirbuilder-gate1-bin-size-return-mul-home-s0-2026-10-10.md

## Production frontier and Decision

The unchanged `apps/mimalloc-lite/main.hako` pure-first EXE probe with fresh
quick CLI SHA-256 `b9079fd6ee018343cca3f7433536f6282df4100b0dfa474c69e667e018dcedde`
exits 1 before EXE at
`ordinary-new/borrowed-entry/source-only-object-actuals` on the original
`HakoAllocHeap.allocate -> HakoAllocPage.allocate` incoming call. The probe
command used `--backend mir`,
`--emit-exe`, `--emit-exe-nyrt`, `HAKO_BACKEND_COMPILE_RECIPE=pure-first`,
`HAKO_BACKEND_COMPAT_REPLAY=none`, and `NYASH_DISABLE_PLUGINS=1`; elapsed
time was 0.05 s after a 247.56 s quick CLI build. No app source changed.

Read-only audits of the physical Loop and object actual owners found that
physical `LoopBinaryI64::Mul` is a deferred separate capability, but the
production app stops before reaching it. `SourceObject` is deliberately
non-executable: `prepare_object_source_actuals_v1` retains original target
and candidates with no opaque actual proof, and `require_executable_v1`
refuses it. The typed-object input finisher can regenerate only the
zero/all-ExactI64 case; it cannot authorize the unannotated opaque formal
here. Do not promote `SourceObject` directly to executable.

The source classifier can issue `BorrowedGuardedActualV1` for a checked
call/ordinal, and the existing source-seeds owner consumes those facts. This
does not yet prove such a fact for the Page outgoing calls below. The real
imported-source test retains all 15 Heap callers and confirms candidate
Integer agreement while formal executable agreement is false. Do not issue a
second source receipt or treat global guarded-actual coverage as site proof.

The real-source diagnostic below distinguished source-draft admission from
fixed-point pruning. Its first exclusion is `LayoutBox.class_id(size)`; the
two `Page.allocate(size)` calls follow. The prior result/new-site D0's
source-only step has landed, while checked Fault/SSA/ABI transport and
executable-actual handoff remain open. Result qualification, Home success and
source argument shape are not executable proof.

## Design task and acceptance

1. Completed: distinguish initial draft from fixed-point pruning and identify
   the first exclusion while retaining the 15-caller inventory.
2. Resolved prerequisite: the original `>=` has raw MIR/JSON/C `sge`
   spelling, but the canonical Dynamic operator issuer has no
   `GreaterEqual(NormalInteger, NormalInteger)` envelope. Issue that semantic
   contract in a separate S0 before admitting any condition-call source.
   Then identify the exact CurrentOwner call issuer/consumer. Preserve exact
   owner/site/ordinal/root and guarded Normal/Fault correspondence; do not
   add a second scan or parallel classifier.
3. Reuse the real imported-source candidate/transport test and the existing
   negative family. Add only an independently missing positive/negative.
   Require all 15 caller rows and vetoes, then probe the unchanged app for
   its actual next first-stop.

Diagnostic: the existing test only sees the final definition set. A temporary
test-only trace of the SAME draft collector and fixed-point pruning, run with
`--nocapture`, may identify the first exclusion. Remove that trace before any
commit; it is evidence gathering, not a new semantic owner or acceptance.

The first diagnostic run on the original imported source passed 1/1
(316.92 s total, 0.05 s test execution). It confirmed that `Heap.allocate`
has a valid initial draft with exactly three `UnresolvedArgument` sites:
`LayoutBox.class_id(size)` at Body0 initializer and the two guarded
`Page.allocate(size)` calls at Body1/2 return values. The first fixed-point
exclusion is the Body0 static call: target `LayoutBox.class_id` has an
`OpaqueHandle` formal but is not in `transport_owners`. Thus the earlier
`UnsupportedUse` hypothesis is disproven.

The second diagnostic passed 1/1 (318.18 s total, 0.05 s test execution).
`LayoutBox.class_id` has a valid initial source draft. Its incoming inventory
contains the qualified `HakoAllocHeap.allocate -> LayoutBox.class_id` call
and an unqualified CurrentOwner call from `LayoutBox.accepts`. The existing
static transport seed requires every observation of a callee to be qualified
and excludes a target with an unsupported static context. Hence
`LayoutBox.class_id` is absent from `transport_owners`, and the existing
fixed point removes `Heap.allocate` at its Body0 static argument. The
`Page.allocate` edges are later obligations, not this first exclusion.
Both temporary traces were removed; only the original tests and the
diagnostic logs under `/tmp/hako-object-transport-d0*.log` remain.

## Integrated Decision: CurrentOwner condition call

Source authority + canonical issuer: the original Static incoming `Rc` from
`QualifiedStaticCallClaimIndex` and the one existing incoming inventory own
the call identity and complete callee cohort. `LayoutBox.accepts` contains
`if me.class_id(size) >= 0`; its `me.class_id(size)` is the sole CurrentOwner
condition-expression caller of `LayoutBox.class_id`. The real-source census
shows both this unqualified row and the qualified Heap row. The static seed
excludes `class_id` independently because any same-callee CurrentOwner row
exists and because the incoming scan marks non-initializer static contexts
unsupported (`ordinary_new_borrowed_formal_incoming.rs` at the context
classifier). The former is a complete-cohort requirement; the latter is a
physical-context requirement.

Non-authority: the retained `SourceStatic` argument list, candidate Integer
agreement, ExactI64 result claim, and the Loop-specific CurrentOwner packet
do not authorize this `if` condition call. The existing Loop packet only
covers a selected arity-one local initializer in a Loop; adapting its result
by ignoring the condition site would create a second execution meaning.

Fail-fast boundary: keep `source-only-object-actuals` while either selected
condition input, tagged actual, result/Completion, Normal/Fault, or exact
source/ABI identity is missing. Do not filter out `LayoutBox.accepts` by name
or relax either static seed predicate independently. Every source-proved
reachable qualified and CurrentOwner caller must have an executable handoff
before `class_id` joins transport owners.

Smallest next bounded series: the canonical `GreaterEqual` envelope landed at
`c480ae8c70`. Next, admit only the original
`if me.class_id(size) >= 0` Home condition observation in the existing
resolved-source walk. The exact binary root, `GreaterEqual`, left CurrentOwner
ExactI64 call, and right integer-zero literal must be checked before asking
the existing direct-value issuer for its source arguments. This first Home S0
issues one `ExpressionValue` observation with original site, ordered actuals,
and prior Homes. It does not make the Static source executable or weaken either
transport seed veto. If the condition caller is source-proved reachable, a
following physical packet S0 must connect the same original incoming `Rc`
and selected complete callee cohort to a condition
child-call lowering path, tagged formal actual, result Completion and
Normal/Fault. General `located_if` binary lowering currently has no MethodCall
child arm; the Loop-only packet and nonzero-arity qualified packet cannot be
reused as permission. Static transport admission follows only after the
source-proved reachable incoming cohort has executable handoff and the full
original inventory has been reconciled, then unchanged mimalloc-lite
is probed for its actual next first-stop. The Page edges and result contract
remain separate.

The Home focused test also proves the source-only distinction: an opaque Bool
actual can retain the original call while its candidate Integer evidence is
false. The physical packet must inspect the tagged kind and take Fault before
an Integer payload read. Rejecting Bool at Home would silently change the
OpaqueHandle contract and is not this series' design.

The missing operator contract landed in
`MIRBUILDER-GE-NORMAL-INTEGER-CONTRACT-S0`; its focused 8/8 contract family,
1/1 opaque-formal source rejection, and 1/1 `Ge -> sge` serializer test pass.
The condition-call source owner is selected as
`MIRBUILDER-CURRENTOWNER-IF-CALL-HOME-S0` and its focused original-source and
negative tests passed. Its physical consumer remains a
following design dependency. No `LayoutBox.accepts` caller appears in the
selected source corpus, so its formal's executable Integer view cannot be
inferred from callers. If `accepts` is selected in another artifact, its
physical packet needs an explicit declaration/entry contract. This D0 grants no transport
or production switch.

### Physical entry audit (2026-10-10)

`CallableParameterContractKindV1::OpaqueHandle` is the declaration-backed
source contract for both `LayoutBox.accepts(size)` and
`LayoutBox.class_id(size)`. The physical signature gives each an
`OrdinaryScalar` lane; that lane alone does not classify the tagged payload as
Integer. `checked_static_input` proves a checked Normal-use dependency, not
an Integer payload or executable entry. `accepts` has no incoming caller in
the selected mimalloc-lite corpus, so an empty caller set cannot supply its
entry proof. When selected, its body must receive the declared opaque carrier
and check its kind before any Integer payload read, with a non-Integer kind
taking Fault.

The existing `static_incoming_cohort_v1` can retain both original
`class_id` callers as one identity cohort, but `borrowed_static_packet_actuals_v1`
requires a qualified nonzero-arity call. The selected Loop packet proves a
different local-initializer site and cannot authorize this If condition. The
physical condition-call slice therefore needs its own bounded packet loan
from the original `StaticIncomingSourceV1` and Home `ExpressionValue` row,
with source-order actual, result Completion, and Normal/Fault coordinates
checked at the selected If child. Generic `located_if` currently recurses
through `lower_expr`, whose expression grammar has no MethodCall child arm.
No generic Static veto changes before the selected reachable cohort has a
complete physical handoff; the original source inventory remains visible.

### Decision revision: complete-source caller zero before new packet

Source authority + canonical issuer: keep every original declaration and
incoming call, including `accepts -> class_id`. The existing
`VerifiedWholeSourceStaticCallTargetInventoryV1` is issued before selected
mapping and owns the complete MethodCall inventory plus exact Static targets.
Its first bounded observation gap must be absent before a Static target can
be declared caller-zero. That proof can exclude the uncalled method from the
closed-world EXE's physical callable selection while retaining it in the
semantic catalog. The final physical-program walk follows published MIR from
root and births; it can corroborate selection, but cannot issue the earlier
source decision. A full root/birth reachability projection is not needed to
prove this zero-incoming Static method unreachable.

Non-authority: no-caller fixture evidence, `ExactBool`, `OrdinaryScalar`, and
post-lowering MIR reachability do not justify silently omitting `accepts`.
Today the selected catalog includes every declaration. `accepts` has no
incoming caller in this corpus and returns Bool; the physical call-result and
function-role enums have no Bool call lane. Final borrowed-call coverage also
requires every tagged callee to have a witnessed call. A fabricated caller or
Integer result is not a valid bridge. The replacement policy's zero-reachable
site rule favors caller-zero reconciliation over a new unused issuer.

Fail-fast boundary: retain both Static seed vetoes until source authority
proves the exact reachable cohort, every selected edge has a checked packet,
and the final physical walk agrees. Unresolved or ambiguous targets prevent
exclusion. Unreachable source rows remain in the full inventory; they grant
no executable packet. A publicly required callable remains available for
other artifacts and is not retired by this EXE-specific selection.

Smallest next design task: bind this inventory's caller-zero proof to the
closed-world physical selection owner, without weakening the all-declaration
semantic catalog or changing library/public selection. The diagnostic on the
unchanged mimalloc-lite source passed 1/1: 102 MethodCalls observed, no
observation gap, 26 Static target rows, zero `LayoutBox.accepts` targets, and
one distinct `SizeClassBox.accepts` target. The temporary print was removed.
Acceptance must retain the full 15-caller Heap source inventory and
`accepts -> class_id` source row while proving the original
`Heap -> class_id` call selected and `accepts` unselected physically; a corpus
that calls `LayoutBox.accepts` or has
an observation gap must not exclude it. This revises the blanket demand to
execute every source Static row. The condition-call packet remains a later
selected-case task if an actual reachable caller requires it.

Non-claims: this D0 grants no executable actual, physical payload, result
contract, EXE, Loop Mul, old-edge retirement or full MirBuilder completion.

### Integrated Decision: closed-world physical selection boundary

Source authority + canonical issuer: the existing
`VerifiedWholeSourceStaticCallTargetInventoryV1` is the only original-source
MethodCall inventory. Its declaration-catalog brand, all observed calls,
exact Static targets, and first observation gap must travel together to the
EXE physical-selection issuer. The present `QualifiedStaticCallClaimIndexV1`
calls `into_targets()` before selected mapping, losing the complete-inventory
evidence; the first bounded change must preserve a loan or projection of that
same inventory rather than rescan source. Keep the complete declaration and
result catalogs unchanged. Physical selection is a separate, same-brand
projection of selected identities, selected source inventory, selected batch
map, work plan, and package coverage obligations.

Selection rule: a method may be omitted from a *closed-world App EXE* only
when it is not the entry point or a required public/exported callable, every
declaration's MethodCall observation completed, and no exact or plausible
unresolved/ambiguous incoming call can target that method. A missing exact
target row alone does not prove zero callers: `seal_static_targets` can leave
reserved, rejected, or noncandidate MethodCalls in the full inventory. If
the issuer cannot prove the possible-target exclusion, keep the method and
its obligations. Script/library/public modes retain existing selection. This
is artifact-local omission, not retirement of `LayoutBox.accepts` as an API.

Non-authority: a post-lowering root/birth MIR walk may corroborate the result
but cannot decide pre-lowering selection. Filtering only final JSON or only
the root work plan is invalid: `NormalCallableSemanticPackagePortV1::complete`
requires all selected keys consumed and the ordinary-new claim ledger empty.
Unselected declaration source rows remain evidence, but must not veto a
selected callee's executable incoming cohort. That cohort reconciliation is
the *following* slice; it cannot be hidden inside physical omission.

Fail-fast boundary and acceptance: require the same catalog brand and
consistent selected membership through work-plan lowering, package
`complete()`, and final physical-program membership. The unchanged
mimalloc-lite corpus must retain all 15 Heap source callers and the original
`accepts -> class_id` source row, physically omit only source-proved uncalled
`LayoutBox.accepts`, and still select `Heap -> class_id`. Adding an incoming
`LayoutBox.accepts` call, an observation gap, or a plausible ambiguous target
must retain the method or fail closed. Public/library mode must not omit it.
The selected-caller cohort must later stop treating the omitted caller as a
physical transport obligation without deleting its source evidence.

Smallest next slice: bind the existing complete inventory to an EXE-only
physical-selection issuer and co-project selected identities, work-plan
sources, map, and coverage ledger. Do not issue a new condition packet or
relax Static seed vetoes in this slice. Confirm the exact public/export and
possible-target classifiers before changing code; if either is unavailable,
remain in `design_stop` and specify that missing authority rather than
guessing from names.

Read-only owner check: `source_backed.rs` records App Main and every other
callable identity; `selected_mapping.rs` maps every recorded identity;
`program_root_work_plan_production.rs` currently defers every static method.
The AST retains declaration runes including `Public`, `Internal`, `FfiSafe`,
and `Symbol`, but this catalog has no sealed public/export-required selection
classifier. The source inventory also lacks a possible-target disposition for
non-exact MethodCalls. These are the two concrete missing authorities before
the physical-selection issuer can omit a method safely. The next D0 step is
to locate or define their single source-backed classifiers, then freeze the
projection and rejection cases above; a missing exact target remains only a
diagnostic, not omission permission.

Resolution for S0 selection: the App lifecycle publishes a closed EXE rooted
at Main/births and transitive ordinary calls; it has no separate callable
export manifest. `Public` is declaration visibility metadata, not evidence of
an EXE symbol export. Conservatively preserve any explicitly visibility/ABI
annotated method while the physical export story remains unsealed. The
parser-backed App relation protects Main. For a remaining static method,
the existing complete inventory itself can prove exclusion: compare every
observed call with the same method and arity. An exact *other* canonical
target is harmless; an exact target to the candidate, or any non-exact row,
retains the candidate. Any observation gap retains all candidates. This
distinguishes `SizeClassBox.accepts/1` from `LayoutBox.accepts/1` without a
name-only rule. The source-backed catalog and inventory are the issuer;
physical selection consumes their same-brand projection. S0 is closed in the
linked card. The source cohort veto remains unchanged until a later slice.

### After closed-App selection S0

The physical selection S0 is closed in
`mirbuilder-closed-app-static-selection-s0-2026-10-10.md`. The unchanged
mimalloc-lite probe still stops at `source-only-object-actuals`; the original
`accepts -> class_id` source row remains even though `LayoutBox.accepts` is
not a selected physical callable. The next design decision must define the
single selected-caller cohort consumed by Static transport: its source is the
same complete inventory plus the same-brand physical selected membership.
It must retain every original row for diagnostics, include all selected
reachable callers, and exclude an omitted caller only from executable
transport obligations. Freeze the exact issuer/consumer and failure boundary
before changing either Static seed veto. Page edges and return outcome are
separate.

## 2026-10-10 post-selection handoff

`8b4339ecfb` confined closed-App omission to explicit EXE requests, and
`1d8817ea39` removed only omitted Static callers from executable incoming
obligations. The real imported mimalloc test now reports one selected
`LayoutBox.class_id` incoming caller and candidate integer agreement while
retaining the original `accepts -> class_id` source claim and all 15 Heap
callers. The fresh quick CLI still stops at
`ordinary-new/borrowed-entry/source-only-object-actuals` in 0.05s. The
diagnostic has no owner/site in its CLI text, so the next first exclusion is
unclassified; do not infer that a Page edge is first merely from the earlier
ordering. The next design action is a read-only census of the post-selection
source graph and existing failure owner/site evidence, then one Decision
naming the precise outgoing actual issuer, consumer, and Normal/Fault
boundary. No new transport construction begins before that mapping is fixed.
The old trace naming `LayoutBox.class_id` described the pre-selection state
and is superseded for current first-exclusion order.

### Read-only audit correction: outgoing Page actual

The normal CLI token contains no owner/site. Earlier detailed logs pointed to
`HakoAllocHeap.allocate` calling `me.small_page.allocate(size)` at the
original Body(1)/IfThen(0) site; the current-source trace below confirms it.
The medium-page sibling is the next candidate. The current real-source test proves the selected Heap caller
inventory and Integer candidate agreement for `LayoutBox.class_id`, but it
does not prove the exact Page outgoing actual executable.

`Heap.allocate` branches on the **result** of `LayoutBox.class_id(size)`.
That branch does not check the tagged kind of `size`. In particular, the
existence of some `BorrowedGuardedActualV1` facts in the corpus does not
authorize either `Page.allocate(size)` site. `prepare_object_source_actuals_v1`
retains the original target/candidates while producing no executable opaque
actual; `require_executable_v1` correctly rejects `SourceObject`. Keep that
failure boundary.

The temporary trace at the existing `require_executable_v1` rejection has now
resolved the first site on the **current source**. A quick CLI build (2m39s)
with only that temporary diagnostic, followed by the unchanged mimalloc-lite
pure-first EXE command, reported `FunctionOwnerIdV1(1,33)` at
`Body(1)/IfThen(0)/Value`: original
`HakoAllocHeap.allocate -> HakoAllocPage.allocate`, stored `small_page`
receiver, ordinal 0 `SelfRooted(size)` from the Heap formal. It also reported
the `medium_page` sibling at `Body(2)/IfThen(0)/Value`. The command still
exited 1 at `source-only-object-actuals`, with no EXE. The temporary print
was removed; the tracked source is unchanged. The diagnostic build is for
site evidence only, not a verified production binary.

Next Decision: the `class_id(size)` result governs which Page call executes,
but it is **not** the original `size` actual and does not classify that
carrier's tag. Audit the existing `ForwardIdentityV1` from Heap's formal to
each exact Page call, the selected caller's entry carrier/read, and the
callee's checked comparison before any integer payload use. The tagged
carrier may be forwarded unchanged only if the original call/ordinal/binding,
Normal value, target formal, and Fault path are tied to the same authority;
otherwise keep `SourceObject` fail-closed and specify the missing mapping.
Do not borrow a guard from another call or infer a kind from caller-wide
candidate agreement. Reuse the imported-source and object-packet
positive/negative families; add a test only for an independently uncovered
condition. The unchanged app remains the first-stop probe.

Two temporary observations in the **existing** real imported-source test
passed 1/1 each under the quick profile (`9078` other tests filtered, not
counted as passes). They were removed before commit. For
`HakoAllocPage.allocate`, the original source draft and executable borrowed
transport definition are both present. Its four formal uses are the original
Body(0) `Greater` comparison, Body(8) array-element value, Body(11) add,
and Body(13) new argument. For `LayoutBox.class_id`, the original source
draft is present but its executable transport definition is absent; its
opaque formal has one Body(0) unresolved outgoing argument to
`SizeClassBox.good_size`. The existing test already proves
`HakoAllocHeap.allocate` lacks an executable definition. Thus simply lending
the Page source forward cannot settle the selected Heap entry: the upstream
Static chain is still pruned. The next read-only check must identify the
**first** missing executable owner on the `class_id -> good_size ->
size_to_bin -> normalize_size` chain using the same draft/selection owner.
Do not generalize a Static CurrentOwner call or promote `SourceObject` from
these observations alone. Once that prerequisite is closed, revisit the
exact Page outgoing handoff and Page-side tagged kind/Fault proof above.

The selected Static prerequisite is now
`mirbuilder-static-currentowner-closed-cohort-d0-2026-10-10.md`. This object
outgoing row remains open and dependent; no Page transport was promoted.
