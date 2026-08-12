---
Status: active compact card
Date: 2026-08-12
Scope: selected Dynamic callable, canonical session admission, hako.text.scan@1,
  AOT/LLVM production activation
ParentHistory: docs/development/current/main/design/archive/dynamic-fault-exit-transaction-d0-history-2026-08-10.md
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/ring2-provider-link-abi-lifecycle-ssot.md
  - docs/reference/mir/loop-recipe-contract.md
---

# Dynamic callable current card

## Current capsule

Current decision: the final `VerifiedDynamicExitTransactionCoSealV1` is the
selected cohort's sole semantic plan. `CanonicalTrivialBindingSsaPlanV1` is a
different family and must not be extended to accept this Loop. The installed
package port remains the exactly-once transport owner; the existing A-prime
demand/emission plan opens the existing canonical CFG/SSA/PHI session inside
that scoped loan.

Current implementation status: exact-I64 semantic recut, exact-two Completion
and DraftSeal machinery, the Rust-VM nonconsumer fence, neutral output wire,
the I8 unpublished canary, and the complete R0 canonical-session projection
series are landed. The selected emitter now consumes the exact package input,
borrows the final Dynamic program only through its private HRTB authority,
snapshots Completion/control expectations, and opens its unpublished canonical
session internally. Production still uses the selected raw AST/JoinIR edge.

Next ordered task: implement the already-accepted AOT physical activation cell
(`DYNAMIC-V2-AOT-PHYSICAL-ACTIVATION-I0`) with the complete provider contract,
admission, strict LLVM leaf, I6/I7 receipts, End/lifecycle, and atomic selected
production switch. No provider/runtime/LLVM implementation is implied by the
completed R0 BoxShape row itself.

Production stop line: provider/AOT/runtime activation and the selected
production switch remain closed until the complete activation cell is green. No
trivial-plan widening, second Completion/If/profile, raw AST repair, arbitrary
session pairing, fallback, retry, or Rust-VM DynamicV2 consumer may cross the
seam.

Retirement finish line: one atomic AOT activation consumes the selected package
loan through exact-two DraftSeal, removes the selected old edge in the same
commit, and leaves provider/selector/registry/image reselection, legacy
fallthrough, fallback, retry, and Rust-VM DynamicV2 callers at zero.

## Accepted design decision

```text
Decision:
  Build one atomic AOT activation from the final Dynamic program and the
  existing A-prime demand/emission plan. Do not widen the trivial family or
  land a provider/session fragment as a selectable route.
Source authority + canonical issuer:
  Installed package same-batch loan + VerifiedDynamicExitTransactionCoSealV1;
  retained CoreMethod rows own callable result/effect, the normalized TextScan
  contract owns the complete role/profile/lifecycle contract, and one neutral
  AOT export artifact owns the strict physical entry/ABI declarations.
Non-authority:
  generic trivial analysis, mutable compatibility registry, selector/name
  lookup, generic String, raw AST/MIR inference, LLVM, and Rust VM.
Fail-fast boundary:
  complete TextScan/provider symbolic AOT admission and session authority
  validate before Builder mutation. Exact image/digest/symbol validation occurs
  only at link and must succeed before executable publication.
Smallest next slice:
  DYNAMIC-V2-AOT-PHYSICAL-ACTIVATION-I0, assembled through the seven bounded
  subrows below and landed as one production replacement.
Non-claims:
  no Dynamic registry, runtime lookup, VM feature, generic fallback, retry,
  legacy collector key, or production switch before all subrows are green.
```

### Why the former trivial-plan premise is rejected

The target is a mixed typed/Dynamic Loop with an inner If Return. The generic
trivial verifier has no Loop arm, rejects Return inside If, and owns its own
trivial profile, If control, and Completion. Widening it would create a second
semantic planner beside the already-complete Dynamic Recipe/JoinSig program.

The correct products already exist:

```text
SelectedCallableLoweringInputRefV1
  -> VerifiedDynamicExitTransactionCoSealV1
       owns source / mixed Recipe / JoinSig / Completion / cleanup / exits
  -> VerifiedAPrimeI64PhysicalDemandV1
  -> PreparedSelectedDynamicV2EmissionPlanV1
```

The landed pre-activation seam is:

```text
borrowed sole Completion
+ Dynamic-program-owned Loop control disposition
+ exact same-source resolved input
+ move-only A-prime emission plan
  -> existing canonical CFG/SSA/PHI session
```

`PreparedProgramRootWorkPlanV1` stays a root scheduling owner. It does not gain
a borrowed canonical-plan field, avoiding a self-reference and foreign-plan
pairing surface.

### Landed preflight invariants and one remaining transport correction

The body-free physical header/effect work is already landed and remains a hard
precondition of activation:

```text
catalog physical symbol + arity
+ exact declared parameter/return representations
+ verified operation-program EffectMask projection
  -> APrimePhysicalFunctionHeaderV1
  -> create_resolved_function_skeleton(header facts)
```

The selected route never formats a symbol from the raw function name, scans a
body with `contains_value_return`, or supplies a Builder-fixed effect mask.
Header/Completion/control/executable validation finishes before the unpublished
Builder session opens. `PureRead` is a callable semantic effect; it does not
erase the Dynamic invocation outcome, Fault, suspension, or lifecycle axes.

One transport correction remains in the activation cell. The cataloged-method
adapter already owns one move-only `NormalCatalogedBoxMethodDraftAdmissionV1`,
but the current A-prime issuer reseals another admission from a cloned selected
key. The selected adapter must move its existing admission into the scoped
package loan/A-prime demand, and the issuer-side `seal(source_key)` call must
become zero. The same admission then supplies the physical header and the final
`CanonicalCallable` collector key; no raw key is resealed downstream.

## Final owner graph

```text
InstalledNormalCallableSemanticPackageV1
  owns batch + selected mapping + parameter contracts + final Dynamic program
             |
             | NormalCallableSemanticPackagePortV1
             | exactly-once HRTB selected loan
             v
SelectedCallableLoweringInputRefV1::Dynamic
  same-source ResolvedFunctionLoweringInputV1
  + &VerifiedDynamicExitTransactionCoSealV1
             |
             +-> borrowed canonical-session authority
             |     sole Completion
             |     Dynamic-owned Loop If disposition
             |     common outer If rows = 0
             |
             +-> issue_selected_a_prime_i64_physical_demand
                    |
                    v
             PreparedSelectedDynamicV2EmissionPlanV1
                    |
                    | private TextScan loan
                    | + normalized ProviderSlot artifact
                    | + neutral embedded-AOT export facts
                    | + canonical String alias projection
                    | -> consuming admission
                    v
             PreparedSelectedDynamicV2AotActivationV1
               immutable admitted rows + PreparedAotExecutableAdmissionV1
               strict I6/I7 entry IDs/lanes + PlanStamp + V10 lease capability
                    |
                    | validates, then opens one scoped session
                    v
             CanonicalSsaFunctionSessionV2
               sole CFG / Binding SSA / PHI owner
                    |
                    v
             site-keyed Completion claims
             -> DraftSeal prepare: Return x 2
             -> DraftSeal commit
             -> Collector / atomic MIR-module candidate publish
                (not executable publication)
                    |
                    v
             LLVM object + AOT link finalizer
             -> exact ProviderImageId / artifact digest / resolved entries
             -> RuntimeExecutablePlanV1
             -> executable publication; link failure publishes no executable
```

The scoped loan may yield a private view, not a durable semantic receipt. The
view cannot escape the callback and exposes no raw AST, Recipe, JoinSig,
Completion parts, `ValueId`, or `BasicBlockId`.

## Ordered implementation DAG

The former session-admission Decision and its canonical-session/I8 BoxShape
series are closed. Their detailed evidence lives in `ParentHistory` and git.
The only active parent row is the following atomic production replacement.

### 1. `DYNAMIC-V2-AOT-PHYSICAL-ACTIVATION-I0` — active BoxCount

Change:
  consume the retained I6/I7 CoreMethod rows into one complete TextScan
  AOT admission, lower the whole selected Loop through the existing canonical
  session and exact-two DraftSeal, finalize the exact linked executable plan,
  admit the completed draft with a `CanonicalCallable` key, and delete the
  selected raw AST/JoinIR edge in the same activation unit.

Contract:
  `CoreMethodContractBox` is the sole callable result/effect authority. The
  normalized TextScan contract owns only the complete two-role grouping,
  shared CodePoint profile, and lifecycle requirements. A neutral AOT export
  artifact owns symbolic strict entry IDs and ABI declarations; the runtime
  type registry owns String/StringBox vocabulary. `ProviderAdmissionSeal`
  owns provider/ABI admission and issues one canonical Text branch plus a
  symbolic `PreparedAotExecutableAdmissionV1`. Only the post-object AOT link
  finalizer may issue an exact `RuntimeExecutablePlanV1` with image digest,
  resolved entry and the carried compile-session `PlanStamp`. The session owns
  physical values, CFG/SSA/PHI, cleanup, and Completion claims. No layer
  re-searches selector, name, generated rows, registry, provider, or image.

Acceptance (not yet landed):
  exact two-role admission, immutable deterministic admitted registry,
  receiver-bearing symbolic AOT admission, strict AOT/LLVM I6/I7 leaf, exact
  link-time RuntimeExecutablePlan, one V10 lease and End, I7 ImmediateI64 with
  no lease, full I0-I16/control/backedge close, two Completion claims and
  physical Returns, one CanonicalCallable collector handoff, selected
  canonical caller=1, selected old edge=0, and all focused tests/guards green.

Stop:
  missing/foreign/duplicate Core row, incomplete role coverage, alias
  ambiguity, wrong symbolic entry/stamp/lane/lifecycle, or Builder mutation
  before pre-MIR validation rejects the MIR candidate. A stale/foreign linked
  image, digest, ABI, or symbol rejects executable publication. Synthetic
  return join/PHI, legacy key, generic fallthrough, fallback, retry, sentinel
  repair, or Rust-VM dependency rejects the cell.

The implementation is one activation product assembled by seven bounded
subrows. These are work-branch checkpoints, not independent authorities or
mainline production routes.

| Order | Subrow | Sole responsibility | Output consumed by |
| --- | --- | --- | --- |
| 1 | `CANONICAL-CHILD-ADMISSION-I0-A` | move the adapter-owned catalog admission through the selected package loan; delete A-prime resealing | physical header and collector |
| 2 | `TEXT-SCAN-EXPORT-CONTRACT-I0-B` | add one normalized ProviderSlot artifact, one neutral symbolic AOT export artifact, and checked ABI projections; defer String/StringBox alias co-seal | provider admission |
| 3 | `TEXT-SCAN-ADMISSION-I0-C` | borrow exact retained I6/I7 rows; co-seal profile/export/aliases; consume draft facts into immutable rows and symbolic AOT admission | physical activation aggregate |
| 4 | `STRICT-AOT-LEAF-I0-D` | implement the declared CodePoint entries and one LLVM early consumer; carry the symbolic admission unchanged | AOT object/link finalizer |
| 5 | `PHYSICAL-SESSION-I0-E` | consume one move-only activation aggregate; emit I0-I16, outcomes, V10 End, carrier PHI/backedge and profile close | DraftSeal terminal |
| 6 | `EXACT-TWO-COLLECTOR-I0-F` | claim retained Completion sites; prepare/commit two Returns; admit the draft as `CanonicalCallable` | package cutover |
| 7 | `SELECTED-CUTOVER-I0-G` | route the installed loan to the new cell, require link-plan finalization for AOT executable publication, and delete the raw AST/JoinIR edge atomically | production |

#### `CANONICAL-CHILD-ADMISSION-I0-A`

At the cataloged static-method adapter, move the already-sealed
`NormalCatalogedBoxMethodDraftAdmissionV1` into the exactly-once selected loan.
The A-prime issuer consumes that admission while cross-checking the package
key/source identity; it must not call `seal(source_key)` itself. The same
move-only admission supplies the `APrimePhysicalFunctionHeaderV1` and is
retained until the canonical collector handoff. A foreign key, wrong namespace
or arity, double consume, or missing admission rejects before Builder effect.

I0-A receipt (2026-08-12): the package loan now carries the admission through
static and instance adapter scopes, the selected A-prime tests consume it by
value, and the physical-input guard rejects issuer-side resealing and raw
symbol reconstruction. This remains a pre-cutover transport/BoxShape change;
the production caller is still zero.

#### `TEXT-SCAN-EXPORT-CONTRACT-I0-B`

The semantic source is one normalized artifact rooted at
`lang/src/runtime/meta/provider_slot_contract_box.hako` with a generated
manifest under `lang/src/runtime/meta/generated/`. It references the existing
generated CoreMethod row identities rather than restating result/effect. It
owns only `hako.text.scan@1`, the two roles, canonical Text, the fixed
CodePoint/clamp profile, ordered lane demands, sync policy, and lifecycle.

The physical provider-fact owner is one neutral header,
`include/nyrt_dynamic_text_scan_v1.h`, with checked Rust/kernel/Python
projections. It declares provider/version, ABI revision, opaque
`ProviderExecutableEntryIdV1` values, exact symbol spellings, physical lane
shapes, CodePoint capability, and lease capability. It does not restate
CoreMethod result/effect. The existing generic String exports and allocator
provider manifests are not inputs.

The runtime type registry remains the sole String/StringBox vocabulary source.
It lends a checked alias projection; `ProviderAdmissionSeal` is the only issuer
of the normalized one-branch CanonicalText decision. Missing, duplicate, or
conflicting aliases reject. No first-row-wins rule is permitted.

I0-B implementation receipt (2026-08-12): the normalized Hako contract,
deterministic manifest codegen, neutral C export facts, Rust/Python projections,
negative tests, and closed-mode parity guard are landed. The manifest retains
only role/op/arity/profile/lifecycle policy; CoreMethod result/effect remains
the sole callable authority. ProviderAdmission, runtime registry admission,
strict leaf, LLVM caller, session, VM, and production callers remain zero.

#### `TEXT-SCAN-ADMISSION-I0-C`

Use one private HRTB callback over the already-retained operation refs. It
must find I6 and I7 exactly once, verify owner/program, CallSlot operands,
`CoreMethodOp`/arity/result/effect, and immediately pass the borrowed pair to
the consuming admission. The view has no public constructor, clone, raw row
getter, or escapable lifetime. The existing generated by-op lookup remains in
the initial call co-seal only; provider admission performs zero lookup.

The current mutable `BoxCallableRegistry` remains a compatibility/draft input.
Admitted state is a distinct immutable deterministic product with duplicate
overwrite and mutable insert APIs absent. `String` and `StringBox` export facts
canonicalize to one Text receiver branch only when provider/profile/ABI/entry
all agree; ambiguity rejects before publication.

Admission produces one move-only pre-MIR cell containing semantic route,
canonical receiver, provider identity, registry generation, symbolic entry
IDs, ABI/wire revisions, I6/I7 lane contracts, V10 lease capability, and one
`PlanStamp` projected from the owning compile/module-invocation brand. It does
not contain a guessed image, artifact digest, runtime address, registry
reference, selector/name, Core result/effect copy, or raw provider getter.
`drop_epoch`, host handles, filesystem timestamps, and an unrelated global
counter are forbidden PlanStamp sources.

The session entry becomes conceptually:

```text
DynamicV2PhysicalEmissionSessionV1::begin(
  builder,
  PreparedSelectedDynamicV2AotActivationV1,
)
```

The aggregate owns the emission plan, compare/cleanup demands, and executable
admission together so a caller cannot re-pair them. It validates all symbolic
entry, ABI, generation, lane, lifecycle, and semantic brands before opening
Builder state.

##### I0-C implementation-preparation contract

This is a work-branch BoxCount checkpoint, not a new production route. Before
editing, freeze the following order and API boundary:

```text
retained DynamicFullLoopCallRelationV2 rows
  -> one private HRTB TextScan view (I6 + I7, borrowed)
  -> consuming ProviderAdmissionSeal
  -> immutable deterministic admitted rows
  -> move-only PreparedAotExecutableAdmissionV1
```

The view is created only inside the existing selected physical-capability
consumer. It must use the `core_method` references already retained by the
call relations and must not call a generated-row lookup, selector lookup, or
surface scan. The callback returns only an owned admission cell; no borrowed
row, raw registry, selector, provider function, or `CoreMethod` object may
escape. The admitted rows are a canonical `Text` branch: `String` and
`StringBox` are checked aliases, never two independently selectable branches.

Required pre-effect checks are: exactly one I6 `StringSubstring/2` row and one
I7 `StringIndexOf/1` row; same program/owner; CallSlot receiver/argument/result
lanes; generated result/effect identity; one CodePoint profile; one ABI/export
revision; matching alias slots; no duplicate or ambiguous export; and a
compile/module-invocation `PlanStamp` that is borrowed from an existing owner.
Any missing, foreign, swapped, stale, duplicate, or conflicting fact rejects
before Builder/session mutation. The cell contains symbolic entry IDs only;
image digest, resolved address, and `RuntimeExecutablePlanV1` are post-link
products and are not invented here.

Acceptance for this checkpoint is `admission issuer = 1`, immutable admitted
registry = 1, canonical receiver branch = 1, symbolic executable cell = 1,
and all production callers/LLVM hooks/Rust-VM consumers = 0. The next strict
leaf/session consumer must be named before this cell is allowed to land on
main; a provider-only or registry-only production commit is forbidden.

#### `STRICT-AOT-LEAF-I0-D`

Implement exactly the two entry IDs declared by the neutral artifact in a new
kernel module; do not add logic to the near-limit generic `exports/string.rs`.
The strict leaf uses fixed UTF-8 CodePoint semantics and strict handle/lane
validation. It never calls the environment-selected/generic String surface,
compat forwarding, parse/default-zero helpers, or the Rust VM.

The LLVM selected early hook consumes the symbolic admission before generic
method lowering and emits only its declared symbols. The existing neutral
`DynamicV2CallOutV1` remains the result-wire layout owner and does not gain
provider/selector meaning. Selected malformed or unsupported metadata is
terminal and never falls through.

MIRBuilder runs before object generation and link, so it cannot own a final
artifact digest or function address. After object generation, the AOT link
preflight consumes the symbolic admission, hashes/pins the exact
`libnyash_kernel` artifact, verifies both declared entries and ABI, and alone
issues `RuntimeExecutablePlanV1 { ProviderImageId, resolved entries,
PlanStamp }`. Link failure or stale/foreign artifacts reject before executable
publication; no runtime lookup or reselection is introduced.

The two publication boundaries are deliberately distinct:

```text
pre-Builder:
  PreparedAotExecutableAdmissionV1 only

post-DraftSeal/collector:
  atomic MIR-module candidate (intermediate compiler artifact)

post-object/link:
  RuntimeExecutablePlanV1 + successful link
  -> executable publication
```

A failed link may leave discardable MIR/object diagnostics, but it publishes no
executable and cannot fall back to the generic route. The selected compiler
cutover is permitted only after a positive strict-link canary and every
negative link gate are green.

#### `PHYSICAL-SESSION-I0-E`

Consume the complete Recipe-order cursor once. Preserve the landed six-block
topology and formal adoption: `Enter != Header`, then BodyPrelude,
ThenTerminal, Continuation, and After. Emit all 15 operations plus If/Return
control, exact I64 induction PHI/backedge, I6 Normal/Fault and one V10 lease,
I7 Normal/Fault with ImmediateI64/no lease, and exactly four cleanup rows.
Backedge and I7 Fault both End the live V10 carrier exactly once. A complete
cursor, not a collection of public `take_*` helpers, authorizes profile close.

#### `EXACT-TWO-COLLECTOR-I0-F`

Derive inner/outer sites from the retained A-prime relation, claim both through
the existing site-keyed Completion consumer, and let DraftSeal remain the only
physical Return writer. The session terminal passes the completed canonical
draft directly to the existing collector with
`FunctionDraftKeyV1::CanonicalCallable`; it never calls
`into_legacy_collector_parts`, reopens a function session, or reruns type,
return, signature, or name-based finalization.

#### `SELECTED-CUTOVER-I0-G`

The installed package adapter consumes the selected Dynamic program instead
of forwarding only a source seed. The located Loop keeps its retained method
and admission evidence, invokes the activation cell once, and cannot enter
`lower_loop_or_freeze_v1`. Ordinary/foreign callables keep their existing
route. The same activation commit establishes new selected caller=1, old
selected edge=0, fallback=0, and retry=0.

#### Work-branch and main landing boundary

Implementation may use seven internal commits on this feature branch:

```text
W0  move existing catalog admission through package/A-prime; delete reseal
W1  normalized TextScan contract + neutral export facts + alias projection
W2  consuming admission + immutable registry + symbolic AOT aggregate
W3  strict runtime leaf + LLVM selected early hook + link-plan finalizer
W4  full physical session
W5  exact-two DraftSeal + CanonicalCallable collector
W6  package cutover + old-edge deletion + guards/docs
```

Every W0-W5 checkpoint keeps production callers at zero and the capability
closed. Main receives only the complete activation unit (squashed or otherwise
presented as one indivisible activation commit). A provider-only, registry-only,
LLVM-only, link-plan-only, lease-only, PHI-only, or partial-cursor main landing
is forbidden.

Required counts at the I0 terminal:

```text
TextScan roles / provider profile                              = 2 / 1
normalized contract artifact / neutral AOT export artifact    = 1 / 1
provider executable entry IDs                                 = 2
ProviderAdmissionSeal / immutable admitted registry           = 1 / 1
mutable admitted insert / duplicate overwrite                 = 0 / 0
canonical Text receiver branch                                = 1
symbolic AOT admission / strict entries / LLVM consumer        = 1 / 2 / 1
link finalizer / RuntimeExecutablePlan / exact image pin       = 1 / 1 / 1
I6 receipt / lease / End                                      = 1 / 1 / 1
I7 ImmediateI64 / lease / End                                 = 1 / 0 / 0
Completion expected / claimed / physical Return               = 2 / 2 / 2
synthetic return join / Return PHI                             = 0 / 0
CanonicalCallable collector / legacy collector key            = 1 / 0
adapter admission move / A-prime catalog reseal                = 1 / 0
new selected caller / selected old edge                       = 1 / 0
runtime lookup / generic fallthrough / fallback / retry        = 0 / 0 / 0 / 0
Rust VM DynamicV2 production consumer                         = 0
```

### 2. `DYNAMIC-V2-SELECTED-LEGACY-RETIREMENT-R0` — after cutover

Delete only after caller-zero evidence:

```text
selected source-seed-only route
selected raw JoinIR edge and legacy finalizer edge
test-side Completion/If reissuance helpers
superseded I8-only canary shell
diagnostic-only raw role/fingerprint authority uses
selected old topology callers
```

Global fixed-topology deletion waits for all remaining callers to reach zero.
H2/H3/H5 parity and the AOT mimalloc gate then run as independent siblings;
both must be green before Hako producer activation.

### 3. `MIRBUILDER-MODULE-DRAIN-CONVERGENCE-D0 -> I0` — after selected cutover

This is a post-cutover publication cleanup, not a second module authority.
First census every production lowering route, then converge the routes onto the
existing one-shot `ModuleLoweringInvocationDrainOwnerV1` and post-drain
finalization owner. The disconnected `module_invocation_cut0_p0` candidate is
not production truth and must not be activated as a parallel drain.

Done requires:

```text
production route census                                      = complete
one production drain owner                                   = 1
one production post-drain finalizer                          = 1
duplicate drain/finalizer callers                            = 0
legacy finalize_function_draft production callers             = 0
candidate-only drain path promoted                           = 0
one-shot drain / atomic publish                              = green
```

The row opens only after the selected AOT caller switch and old-edge
retirement. It does not change semantic source, Recipe, JoinSig, Completion,
provider, or VM ownership.

### 4. `LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-R0` — after legacy retirement

After `DYNAMIC-V2-SELECTED-LEGACY-RETIREMENT-R0`, perform a full production
and test caller census for the fixed-role topology and its old issue APIs.
Only when every caller is zero may the fixed-role receipt, legacy boundary
receipt, and old `issue(...)` compatibility path be hard-deleted. A segment
route is not considered complete merely because a new caller exists; the
retirement row requires proof that no remaining route depends on the old
topology.

Done requires:

```text
fixed-role production callers                             = 0
fixed-role test/guard callers                              = 0
old issue(...) callers                                     = 0
segment route completeness                                 = green
fixed-role receipt / boundary types                        = deleted
compatibility fallback                                     = 0
```

This is a post-cutover BoxShape/retirement row. It cannot be used to bypass
the AOT capability gate or to delete an old path while it still owns a live
production edge.

## hako.text.scan@1 semantic contract

```text
profile: utf8-codepoint-clamped-v1
receiver: canonical Text
aliases: String | StringBox, canonicalized before admission

TextSliceRange / substring/2
  CanonicalText + ImmediateI64 + ImmediateI64 -> CanonicalText
  CP half-open range, endpoint clamp, synchronous
  Normal result = one EndAuthorized lease

TextFindNeedle / indexOf/1
  CanonicalText + CanonicalText -> exact ImmediateI64
  first CP index, empty needle = 0, miss = -1
  Normal result lease = 0, End = 0
```

Selector and diagnostic strings only cross-check dispatch keys. They do not
decide result class, representation, lifecycle, provider, or executable entry.
The strict leaf does not call the environment-selected/generic String surface,
string-to-i64 compatibility parsing, or sentinel-zero helpers.

## Activation file boundaries

Keep one public/session entry per owner and put the new logic in small modules.
Exact filenames may vary only within the owning directories; ownership and
line budgets may not move into the near-limit files.

```text
lang/src/runtime/meta/provider_slot_contract_box.hako
tools/provider_slot_contract_manifest_codegen.py
lang/src/runtime/meta/generated/provider_slot_contract_manifest.json
  normalized TextScan semantic contract; generated Core row identities only

include/nyrt_dynamic_text_scan_v1.h
src/abi/text_scan_aot_export_facts.rs
src/llvm_py/builders/dynamic_v2_text_scan_export_facts.py
  one physical export/ABI owner plus checked projections

src/box_callable/provider_admission/
  seal.rs                 consuming TextScan/alias/export co-seal
  admitted_registry.rs    immutable deterministic selected rows
  aot_admission.rs        symbolic entry IDs/generation/PlanStamp aggregate

crates/nyash_kernel/src/exports/dynamic_v2_text_scan.rs
  strict CodePoint I6/I7 entries and one-shot lease owner

src/llvm_py/instructions/mir_call/selected_dynamic_v2.py
  short early hook; no provider lookup or generic fallback

src/mir/builder/resolved_lowering/selected_dynamic_physical_emitter/
  executable.rs / call_slots.rs / lifecycle.rs / terminal.rs
  move-only aggregate consumption and full session realization

src/bin/ny_mir_builder_aot_provider_plan.rs
  post-object artifact digest/symbol verification and link-plan finalization
```

The AOT driver must pass the exact `--nyrt` artifact path into the link
finalizer explicitly; production code may not rediscover it from an environment
variable. `ProviderImageId` is derived from the verified artifact digest, while
`PlanStamp` remains the carried compile/module-invocation stamp. Static link
success alone is insufficient if the descriptor, ABI, or required symbols do
not match.

## Mandatory cleanup and line-budget gates

These are BoxShape boundaries, not substitutes for the active BoxCount.

Closed and archived in `ParentHistory`/git:

```text
CURRENT-STATE-LIVE-SCHEMA-I0           CURRENT_STATE = compact live pointer
MIRBUILDER-WORKSTREAM-ARCHIVE-R0       closed chronology archived
MIRBUILDER-BUILDER-BUILD-SPLIT-R0      thin facade + four responsibility files
MIRBUILDER-LINE-BUDGET-R0              collector/completion test splits landed
MIRBUILDER-COMPLETION-COMMENT-CLEANUP  site-keyed exact-two wording current
```

Pre-cutover freeze:

```text
src/mir/builder.rs                         794 lines; additions forbidden
src/mir/resolved_value_profile/analyzer.rs 769 lines; freeze or private seam split
crates/nyash_kernel/src/exports/string.rs  694 lines; strict leaf additions forbidden
new/touched Rust source                    design split at 760, hard stop at 800
new activation owner modules              target below 650, mandatory split by 700
LLVM method_call.py / ny_mir_builder.rs    short hook only; plan logic in new module
```

Post-cutover queue:

```text
MIRBUILDER-EMIT-INSTRUCTION-PHASE-SPLIT-R0
  keep one public writer; split private prepare/validate/commit/post-metadata

MIRBUILDER-MODULE-REGISTRY-CLASSIFY-R0
  after caller/cfg census, keep one MirBuilder facade and classify modules as
  state/session, source admission, semantic plans, physical lowering,
  collection/publication, compatibility, and tests/migration. Preserve paths,
  visibility, cfg, and re-exports; delete only caller-zero modules.
```

Each pending BoxShape is a two-to-five-commit refactor series with unchanged
behavior and callers, focused parity/failure tests, `git diff --check`, and all
touched Rust files below 760 lines. It cannot overlap the activation BoxCount.

## Common negative matrix

```text
Ordinary or foreign selected loan                         -> reject/not selected
owner/function/forest/projection/root drift              -> reject
missing/duplicate/extra Completion site                  -> reject
wrong function target or return operand                  -> reject
Loop/local If/inner Return shape drift                   -> reject
catalog admission resealed below the package adapter     -> guard failure
raw physical symbol/body Return/effect inference         -> guard failure
missing/duplicate ProviderSlot or AOT export row         -> RejectBeforeEffect
String/StringBox alias/profile/entry disagreement        -> RejectBeforeEffect
symbolic entry/ABI/generation/PlanStamp drift            -> RejectBeforeEffect
final image/digest/address requested before AOT link     -> guard failure
stale/foreign linked artifact or missing strict symbol   -> link reject
second Completion/If/profile issuance                    -> guard failure
raw AST/Recipe/JoinSig/ValueId handoff                    -> guard failure
arbitrary canonical-session pairing                      -> API/guard failure
borrow escaping package callback                         -> compile failure
provider/LLVM/End incomplete at activation               -> RejectBeforeEffect
selected legacy/generic fallthrough                      -> guard failure
name/ordinal/selector repair, fallback, retry             -> guard failure
Rust VM DynamicV2 provider/receipt/session                -> guard failure
```

## Focused gates

```bash
cargo test -q --lib normal_callable_semantic_package
cargo test -q --lib dynamic_full_body_recipe
cargo test -q --lib selected_dynamic_physical_emitter
cargo test -q --lib completion
cargo check -q --lib

bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/dynamic_v2_physical_input_authority_guard.sh
bash tools/checks/dynamic_v2_callslot_wire_authority_guard.sh
bash tools/checks/dynamic_v2_vm_nonconsumer_fence_guard.sh
bash tools/checks/loop_precutover_authority_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

The activation implementation must add one reusable
`tools/checks/dynamic_v2_aot_activation_authority_guard.sh` during W0/W1 and
make it green only at W6. Before W6 it runs in closed mode and requires
production caller=0; at W6 the same guard flips atomically to new=1/old=0. It
owns export/header projection parity, single admission/alias/PlanStamp issuers,
pre-link versus post-link plan boundaries, strict symbols, CanonicalCallable
handoff, and zero VM/lookup/fallback/retry assertions. It is not claimed to
exist or be green in the current docs-only task.

Gate classification (2026-08-12): the focused `completion` command has one
known parent-baseline failure in
`mir::compiler::canonical_physical_completion_p0::compiler_bridge_drains_a_plus_single_route`
(`ReturnValueTypeMissing`, `ValueId(12)`). It reproduces at parent
`b69f5e11fe` and is outside the selected Dynamic activation diff; it remains
recorded as baseline debt, not a green production claim. All currently
existing selected Dynamic/package/Recipe/emitter checks and authority guards
pass; the activation guard described above is a future acceptance item.

## Non-claims

```text
CanonicalTrivialBindingSsaPlanV1 Dynamic expansion
generic all-V2 Loop admission
full String surface or I6-only provider slot
Dynamic-specific registry
runtime provider/selector/image lookup
pre-link final image/address fabrication
generic String compatibility route
Rust VM provider/receipt/session
new Recipe/JoinSig/CFG/SSA/PHI/Completion authority
production cutover before the complete atomic activation cell
fallback / retry / legacy dual-production
```
## History

Detailed landed chronology lives in git history and the historical archive
named in `ParentHistory`. This card owns only the live Decision, next slice,
activation boundary, retirement conditions, and cleanup queue.
