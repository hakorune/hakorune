---
Status: active compact card
Date: 2026-08-12
Scope: selected Dynamic callable, hako.text.scan@1, AOT/LLVM production lane
ParentHistory: docs/development/current/main/design/archive/dynamic-fault-exit-transaction-d0-history-2026-08-10.md
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/ring2-provider-link-abi-lifecycle-ssot.md
---

# Dynamic callable current card

## Current capsule

Current decision: `hako.text.scan@1` is one complete two-role capability,
not an I6-only pseudo-slot and not the full String surface. Its normalized
contract is the sole result/lifecycle authority for I6/I7.

Current implementation status: semantic exact-I64 recut, VM nonconsumer fence,
and transport-only output wire are landed. The selected-package/canonical-plan
handoff has a private input callback accessor landed; the identity co-seal
helper and production handoff are the current bounded row.

Next ordered task: resolve the production bridge design below. The installed
package loan and canonical lowering plan currently have no common production
caller, so the handoff helper must not be added as an orphan. Provider, wire,
LLVM, runtime, and VM feature code is not authorized by this stop.

Production stop line: selected package loan and canonical resolved lowering
plan/session must be co-sealed once. No raw AST/JoinIR route, selector/name
repair, second Recipe/JoinSig/CFG/SSA owner, or fallback may cross the seam.

Retirement finish line: the complete I0-B cell has one LLVM production caller,
the selected old edge is removed in that same activation commit, and all
provider/registry/selector/runtime lookup and Rust-VM DynamicV2 callers are
zero.

## Current row

```text
work_mode = design_stop
current_execution_row = DYNAMIC-V2-CALLSLOT-AOT-HANDOFF-PRODUCTION-BRIDGE-D0
```

The previous BoxShape slice landed the canonical-plan callback accessor and
the focused identity test. The next step is a design stop because the two
existing owners do not meet at one production boundary:

```text
package loan:
  NormalCallableSemanticPackagePortAdapterV1
    -> old raw AST/JoinIR route

canonical plan/session:
  resolved_lowering / callable transaction
    -> canonical session / DraftSeal

existing common production caller = 0
```

### Production bridge decision brief

```text
Decision:
  Keep the handoff Builder-free and stop until one existing production
  lowering callback can borrow both products; do not create an orphan helper.
Source authority + canonical issuer:
  Installed SelectedCallableLoweringInputRefV1 and the existing
  CanonicalTrivialBindingSsaPlanV1/session owners; the bridge is issued only
  at their first shared lowering callback.
Non-authority:
  source seed, raw AST/JoinIR, selector/name, ordinal/batch repair, a second
  plan/Completion, and any VM/provider path.
Fail-fast boundary:
  no shared caller, foreign owner/function/forest/projection/root, ordinary
  semantic variant, or plan re-verification rejects before Builder effect.
Smallest next slice:
  carry the already-created canonical plan into the selected package lowering
  callback, then perform one private identity co-seal and consume existing
  demand/session owners inside that callback.
Non-claims:
  no new semantic receipt, AOT provider cell, LLVM leaf, runtime lease, VM
  feature, fallback, retry, or selected production switch in this D0.
```

After this design stop is accepted, the implementation row may add only the
production bridge wiring described above. It may not add:

1. one crate-private callback/accessor on
   `CanonicalTrivialBindingSsaPlanV1` for its existing
   `ResolvedFunctionLoweringInputV1`;
2. one private selected-Dynamic handoff helper that borrows the selected
   package loan and canonical plan together, checks exact identity, and then
   permits the existing A-prime issuer to be called inside that callback.

It may not add a semantic `Verified*`/`Prepared*` handoff receipt, provider
artifact, admitted registry, call-in wire, LLVM hook, runtime symbol, lease,
Physical End, VM adapter, or production switch.

## Authority chain

```text
parser/resolver source
  -> exact callable membership and source identity
  -> complete Dynamic semantic package
  -> verified Recipe / JoinSig / semantic program
  -> selected package loan
  -> private handoff co-seal with canonical resolved plan
  -> existing canonical session / Completion / DraftSeal
  -> future atomic AOT/LLVM provider cell
```

The handoff compares the existing products by owner, function product,
function origin, forest allocation, and source-root identity. It never repairs
with a name, ordinal, batch slot, selector, AST rescan, or physical `ValueId`.
Foreign, ordinary, duplicate, missing, or mismatched input rejects before
session/Builder effect and never falls through to the selected raw route.

## hako.text.scan@1 contract

The contract is normalized from one source artifact. Rust, C, Python, LLVM,
`hako.toml`, and PluginLoader data are projections or link/export facts, never
semantic contract owners.

```text
hako.text.scan@1
profile: utf8-codepoint-clamped-v1
receiver: canonical Text
aliases: String | StringBox, canonicalized before admission

TextSliceRange / substring/2
  CanonicalText + ImmediateI64 + ImmediateI64 -> CanonicalText
  CP half-open range, endpoint clamp, synchronous, one EndAuthorized lease

TextFindNeedle / indexOf/1
  CanonicalText + CanonicalText -> exact ImmediateI64
  first CP index, empty needle = 0, miss = -1, lease = 0, End = 0
```

`TextFindNeedle` is the sole authority for `I7/V11 = ImmediateI64` and its
no-lease/no-End result. `calls.rs` expectations, selector/fingerprint strings,
fixture constants, and A-prime diagnostic role strings are dispatch and
cross-check evidence only. They must never refine result class or lifecycle.
I6/I7 must co-seal the same contract, provider, profile, alias branch, and
admission generation; missing/foreign/duplicate/ambiguous/byte/env-mode or
lifecycle drift is `RejectBeforeEffect`.

## Exact semantic recut already landed

```text
I6 substring/2: V0:Dynamic, V6:I64, V9:I64 -> V10:Dynamic
I7 indexOf/1:   V3:Dynamic, V10:Dynamic   -> V11:I64
I8 ConstI64(0):                              -> V12:I64
I9 CompareI64(Less): V11:I64, V12:I64      -> V13:Bool
```

Only V10 has Dynamic lifecycle. I7/V11 has no lease or End. The active
physical evidence is `13 NonFaulting / 0 Faulting / 2 ExternallyBound`; the
selected cleanup rows are I6 fault, I7 fault, inner Return, and Backedge.

## Existing landed evidence

```text
2e9348d0bc  canonical plan input callback accessor + focused identity test
38a5895d15  handoff D0 accepted; fast row opened
b5caad2ce8  handoff implementation boundary recorded
c3c4343367  capability plan impl split keeps capability.rs at 748 lines;
            trivial_plan.rs owns the 73-line plan carrier/impl
```

Focused checks currently used by this row:

```text
cargo test -q --lib main_plan_retains_exact_role_unit_and_consumable_trivial_plan
cargo test -q --lib dynamic_full_body_recipe
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/dynamic_v2_physical_input_authority_guard.sh
bash tools/checks/loop_precutover_authority_guard.sh
git diff --check
```

Known baseline warnings from the full Rust crate are non-blocking unless a
focused command fails. The active row has no provider or VM production caller.

## Next atomic AOT/LLVM cell (not yet open)

After the handoff row closes, I0-B must land in one activation commit using
small owner modules:

```text
normalized contract artifact + generated manifest
  -> one consuming ProviderAdmissionSeal
  -> immutable deterministic admitted BoxCallableRegistry
  -> I6/I7 RoutePlan and receiver-identity RuntimeExecutablePlan
  -> separate call-in admission wire (C header owner; projections checked)
  -> canonical-session I6/I7 physical receipts
  -> one strict LLVM early consumer and CodePoint AOT leaf
  -> V10 value/lifecycle aggregate and one-shot End
```

The admitted branch retains contract/provider/profile/receiver/generation,
image/entry, and plan stamp. Runtime only checks and consumes that sealed
branch; it never searches a registry or reselects a provider/image/selector.
The I0-B result class must come from the role contract, not `calls.rs`.

Required I0-B counts:

```text
contract source / normalized generator                 = 1 / 1
ProviderAdmissionSeal issuer                          = 1
complete TextScan role coverage                       = 2
immutable admitted registry / mutable overwrite       = 1 / 0
I6/I7 same contract/provider/profile                   = 1
receiver identity + generation + image/entry stamp    = present
LLVM selected early consumer / strict leaf             = 1 / 1
I6/I7 canonical-session receipt issuers                 = 1 / 1
I6 lease issuer / End consumer                         = 1 / 1
I7 lease / End                                         = 0 / 0
runtime selector/provider/registry/image lookup        = 0
selected generic/legacy fallthrough                    = 0
Rust VM DynamicV2 provider/receipt/session consumer    = 0
fallback / retry / sentinel-zero repair               = 0
```

## Non-claims

```text
full String surface
I6-only provider slot
Rust VM provider/receipt/session
Dynamic-specific registry
runtime provider/selector/image lookup
generic String compatibility route
bare-handle or sentinel-zero repair
standalone I7 receipt
physical End in isolation
new CFG/SSA/PHI/Completion authority
production cutover before the complete I0-B cell
```

## History and audit

The former 3,900-line investigation ledger is retained at
`docs/development/current/main/design/archive/dynamic-fault-exit-transaction-d0-history-2026-08-10.md`.
It is historical evidence, not a current pointer or semantic authority.
The local `AGENTS.md` was read in full; it is gitignored local guidance and
requires no edit. The tracked current card and `CURRENT_STATE.toml` are the
durable task sources.
