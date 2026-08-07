# LOOP-CALLER-ZERO-PARITY-G0-D0

Status: `Design stop / implementation closed`
Date: `2026-08-08`
Parent: `docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`
North star: `docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md`

## Decision

The next row is a parity design stop, not a second physicalizer. Callable and
Generic G0 keep distinct profile boundaries, but both feed the same common
operation demand, prepared program, recursive physicalizer, canonical CFG /
BindingSSA / PhiTxn services, and `finish_for_draft_seal` terminal.

```text
Callable profile                 Generic G0 profile
  exact source/input                exact source/input (must be preserved)
  Callable Prelude                  G0 entry inputs v0/v1
  common seven-row demand           common fifteen-row demand
  Callable Tail                     G0 After/Tail (L0.After/b1)
        \                          /
         one common physicalizer
                    -> one DraftSeal owner
```

The common side consumes the full `VerifiedLoopOperationPhysicalDemandV1`
through `prepare_all`; it has no first/select/filter API. Callable's seven
rows and G0's fifteen rows are not compared by count, order, profile label,
or evidence-vector position.

## G0-specific boundary

`VerifiedGenericRecipeProductG0` owns the logical G0 facts currently available:
the common Core, operation/effect ledger, `VerifiedGenericAfterEffectG0`
(`L0.After/b1`), semantic context, and `NumericTarget`. Its item 3 is a typed
`DerivedCarrierEntry` for child carrier `C2`; item 4 is the nested Loop
structural row and is intentionally absent from the operation evidence. The
recursive topology must preserve that distinction; the fifteen evidence rows
must not be flattened or relabeled as callable rows.

The G0 adapter may be named
`PreparedGenericG0LoopPhysicalizationV1`. It is a thin move-only execution
compatibility product, not a new semantic owner. It may retain only:

```text
exact resolver-issued input / entry capability
common VerifiedLoopOperationPhysicalDemandV1 or its Prepared program
VerifiedGenericAfterEffectG0 / G0 tail capability
NumericTarget and already-verified context compatibility
```

The adapter must not copy or reverify Recipe/Core/After facts, infer an owner
or scope, relabel `L0.After/b1` as a callable Tail, or create a G0-specific
CFG/SSA/PHI owner.

## Critical source-input gate

The current S4 logical product does not visibly retain an exact
`ResolvedFunctionLoweringInputV1` or Prelude source receipt. The implementation
row is therefore gated on an exact resolver-issued source/input/entry
capability being preserved or moved into the G0 prepared product. Re-resolving,
AST cloning, name/path inference, fabricated owner/frame/scope, or a second
resolver is forbidden. If the capability cannot be moved exactly, the row
returns typed `NoSafeSlice` and remains closed.

The two initial G0 input bindings must be materialized from that exact receipt.
They are not inferred from callable Prelude values, operation ordinals, or
the G0 After binding.

## Execution contract (future I1 canary)

After this D0 is accepted, the implementation ladder is intentionally shallow:

```text
LOOP-CALLER-ZERO-PARITY-G0-I0-R0
  cfg(test) prepared G0 ingress, exact source/entry receipt, Builder-free
  full-demand parity; missing exact input is NoSafeSlice

LOOP-CALLER-ZERO-PARITY-G0-I1-R0
  fresh-session root+child physical canary, fifteen rows, distinct G0
  After/Tail, Completion/DraftSeal, whole-session discard and fresh rerun
```

The eventual I1 order is:

```text
exact G0 input/entry receipt
 -> fresh function session; move Completion once
 -> recursive root+child topology
 -> bind and emit all fifteen operation rows exactly once
    (including DerivedCarrierEntry row 3; nested Loop row 4 is structural)
 -> seal physical After
 -> read post-loop b1 through canonical identity
 -> claim exact I64 ABI/Completion once
 -> finish_for_draft_seal -> DraftSeal
```

The common physicalizer sees only the prepared operation program, an explicit
`ReadyLoopEntryV1`, and borrowed canonical services. It never sees profile
identity, G0 Tail, ABI, Completion, Return, DraftSeal, or legacy routes.

## Acceptance and non-claims

The D0 design is accepted only when the following are frozen:

- exact source owner/origin/kind/site/frame/scope-region and two-entry receipt;
- full Recipe-structure schedule and fifteen exact operation rows, including
  the child-carrier entry, without count/order parity assumptions;
- distinct G0 After/Tail to physical After/return binding;
- one common physical owner and one terminal finish path;
- typed rejection matrix for missing, duplicate, foreign, stale, or inferred
  input capabilities;
- positive, post-emission failure, whole-session discard, and fresh-session
  equivalence criteria for I1.

This design does not open a production caller, selector, module collector,
retry, fallback, legacy deletion, M8/M9 coverage, or backend parity. Those
remain closed until G0 I0/I1 and all-route parity are complete.

Implementation and closeout of I0/I1 must update the physical-demand/session
SSOT, Generic source-to-portable-Recipe SSOT, JoinIR pipeline SSOT, MIR
reference pages, affected README, `CURRENT_STATE.toml`, `10-Now.md`, and the
active workstream in the same closeout slice. The implementation row must
also record its reference-document update before it can be marked complete.
