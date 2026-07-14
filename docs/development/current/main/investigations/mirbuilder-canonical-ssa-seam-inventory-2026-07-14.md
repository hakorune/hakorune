# Canonical Binding SSA Seam Inventory

Status: Closed evidence for SSA-P0
Date: 2026-07-14
Decision: D′ — one function-owned Binding SSA value authority
Machine table:
`../../../../../tools/checks/fixtures/canonical_ssa_seam_inventory_v1.json`
Validator:
`../../../../../tools/checks/lib/resolved_binding_ssa_inventory.py`

## Purpose

This is the behavior-neutral source/caller inventory required before any
canonical value-authority cutover. The machine table contains 92 required rows
and exactly four dispositions:

```text
move to Binding SSA
control-only retain
legacy isolate
caller-zero delete
```

The table is guarded by stable anchors and a hard-coded required row set. A
row cannot disappear merely by deleting it from the data file.

## Closed counts

| Category | Rows |
| --- | ---: |
| canonical binding/value | 18 |
| CFG/predecessor | 12 |
| PHI lifecycle | 23 |
| RC/lifetime | 7 |
| finish/publication | 10 |
| current terminal Return | 10 |
| old A+ If value authority | 12 |
| total | 92 |

| Disposition | Rows |
| --- | ---: |
| move to Binding SSA | 21 |
| control-only retain | 28 |
| legacy isolate | 25 |
| caller-zero delete | 18 |

Production behavior and accepted grammar change are both zero.

## Current canonical surface

The currently accepted resolved family is one static, non-main owner with:

```text
Parameter / Local / Outbox
Variable read / BindingRef assignment
Literal / eager BinaryOp / BlockExpr
fallthrough statement If
function-root final Return or implicit fallthrough completion
```

Receiver, Upvar storage, Loop, Calls, nested Return, Break, Continue, QMark,
Throw, Try, Lambda execution, ProgramV0 source authority, and the default
source route are not active. Repository non-test callers of
`compile_resolved()` remain zero.

## Binding and RC findings

All canonical reaching values are concentrated in one
`BTreeMap<BindingRefV1, ValueId>` inside `ResolvedValueEnvironmentV1`. The
complete production operations are declaration publication, variable read,
assignment old-value read/rebind, old If branch snapshot/restore/join, and
scope retirement.

This concentration makes an atomic D′ cutover structurally feasible. The old
map must be replaced, not synchronized with the new SSA owner.

Three RC gaps require explicit tasks rather than mechanical migration:

1. Successful scope exit currently removes ValueIds and discards them; no
   canonical scope-exit `ReleaseStrong` is emitted.
2. `x = x` has no explicit ownership rule and can release the same ValueId
   that is immediately republished.
3. A BlockExpr tail ValueId may escape the lexical scope whose local ownership
   is closing. Tail escape and outer-binding alias transfer need a fixed law
   before scope-exit release is activated.

Error scope close discards the unpublished function draft. It must restore
semantic state but must not manufacture duplicate runtime releases for code
that will never be published.

## CFG findings

Current canonical If emits exactly one conditional and two merge jumps. The
route is:

```text
resolved If
  -> emission::branch
  -> cf_common::set_branch / set_jump
  -> BasicBlock terminator + cached predecessor mutation
```

The current setters are infallible, can partially mutate, and do not reject an
edge into a sealed target. The existing `BasicBlock::seal()` bit has no real
production protocol and does not protect `add_predecessor()`.

`compute_predecessors()` reads cached `successors`; it is not an independent
terminator-truth proof. SSA-C1 therefore must derive predecessors directly
from terminators and compare that result with the cached witness.

PHI input materialization also calls `update_cfg()` internally. Canonical PHI
definition cannot be allowed to repair CFG truth as an analysis side effect.

## PHI findings

Current canonical If uses only final PHIs:

```text
next_value_id
  -> define_phi_final
  -> hold unpublished batch
  -> verify entry values
  -> publish batch into flat environment
```

Canonical provisional/patch/rollback callers are zero today. The provisional
transaction is used by legacy routes and has a real cleanup defect: its abort
path stops at the first rollback failure and can replace the primary failure.
SSA-P1 must correct this before Binding SSA uses it.

After canonical function publication, `finalize_module` runs
`materialize_all_phi_inputs()`. That pass updates CFG, deletes unused PHIs,
and fabricates missing self-carried inputs. It is a legacy repair pass, not a
valid completion mechanism for the new SSA owner. Its canonical caller must be
zero at SSA-I1; explicit legacy callers may remain isolated.

The mandatory SSA-L0 physical split is:

```text
ssa/phi_input_materializer.rs
  facade only

ssa/phi_input_materializer/edge_rematerialization.rs
  analysis, diagnostics, recursive rematerialization, for_pred

ssa/phi_input_materializer/function_repair.rs
  whole-function repair, pruning, missing-input completion

separate focused test modules for the two responsibilities
```

The split changes no API, behavior, or grammar.

## Return findings

The only explicit accepted Return is the final statement of the function root
body. Nested Return is rejected before Builder effects. The exact source exit,
function-region target, projection, and coverage already exist, but Lower
currently claims only the site; it does not consume an explicit target and
cleanup contract.

SSA-E0 must cover both existing completion forms:

```text
explicit final Return:
  exact target + exact site + explicit empty crossed-scope cleanup

implicit fallthrough completion:
  Void value + Return emitted during function draft finalization
```

It must also prove zero unreachable suffix and keep nested Return activation at
zero. The old RegionFlow copy of the final-only policy is then caller-zero.

## Publication finding

The highest-risk omission is the final module barrier. Today
`finish_built_module()` stores MIR verifier failure in
`MirCompileResult.verification_result` but still returns `Ok`; the canonical
module session can commit immediately afterward. `MirModule::add_function`
also silently overwrites an existing same-name function.

This requires a dedicated pre-SSA production prerequisite, SSA-V0:

```text
post-RC verifier failure -> typed compile failure
candidate module commit -> unreachable on verifier failure
duplicate canonical function publication -> typed failure
function/module publication before seal/SSA completion -> zero
```

SSA-V0 changes no accepted source grammar. It closes invalid-publication
behavior before SSA-S1/S3 are connected.

## Old If split

Retain:

```text
exact If/IfThen/optional IfElse topology
else=None versus else=Some(empty)
exact source coverage
scope/region sessions
family-specific CFG layout
actual predecessor verification, rebuilt on terminator truth
```

Delete after atomic cutover:

```text
condition/whole effect rows
may_rebind_outer
join-source rows
active effect stack
flat branch snapshot/capture/restore
effect-directed PHI creation
flat join publication adapters
```

The separate legacy `lower_if_form()` remains isolated until its explicit
legacy callers reach zero.

## Updated prerequisite order

```text
SSA-P0  exhaustive seam inventory                 closed here
SSA-L0  split oversized PHI helper                 closed
SSA-C1  fallible canonical edge + real seal
SSA-P1  all-attempt PHI cleanup transaction
SSA-V0  verifier/duplicate-publication veto
SSA-S1  disconnected Binding SSA
SSA-S2  identity/value separation
SSA-E0  explicit + implicit current Return contract
SSA-S3  disconnected carrier-free If product
SSA-I1  atomic current-owner production cutover
SSA-R1  physical old If value-authority deletion
```

Loop, nesting, typed exits, owner-family expansion, default-route cutover, and
legacy retirement then follow the final-form taskboard in order. No Loop row
is allowed to bypass these prerequisites.

## Stop conditions added by this inventory

Stop if any implementation:

```text
uses cached successors as terminator-truth predecessor proof
repairs canonical CFG as a PHI materialization side effect
runs whole-function missing-PHI-input fabrication after SSA-I1
stops PHI rollback after the first cleanup failure
commits a canonical module with verification_result = Err
silently overwrites a same-name canonical function
emits scope-exit release without resolving tail/alias ownership
forgets implicit fallthrough completion while sealing Return
adapts Binding SSA back into old branch snapshot/join interfaces
```

These are task boundaries, not new runtime claims.
