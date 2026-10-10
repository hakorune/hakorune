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

The original checked guard and outgoing actual source evidence already exist:
`ordinary_new_borrowed_formal_uses.rs` records `BorrowedGuardedActualV1`
at the exact call/ordinal, and the existing source-seeds owner consumes it.
The real imported-source test retains all 15 Heap callers and confirms
candidate Integer agreement while formal executable agreement is false.
Thus another source-receipt slice would duplicate authority.

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
