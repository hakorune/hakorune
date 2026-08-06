# Generic callable Loop S1 design stop

Status: `Decision: design stop; implementation and production selection are not authorized`

Date: `2026-08-07`

Parent: `GENERIC-CALLABLE-SEMANTIC-LOOP-HANDOFF-S0`

## Why this stop exists

S0 is a clean, bounded product, but it is only a pre-effect source envelope:

```text
callable source bridge
  -> AST-free single-use schedule
  -> located raw Loop entry
  -> exact condition/body/rebind profile check
```

It does not prove a portable Recipe, JoinSig, physical ValueId/PHI use,
completion, DraftSeal, or a production caller. The selected S0 fixture is a
single loop (`StringHelpers.int_to_str/1`); the existing Generic G0 Recipe
producer is a different nested two-loop `i64` profile. Directly mapping one to
the other would be an implementation guess and is rejected.

## Decision

The next slice is one shallow design row:

```text
GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-RECIPE-D0
```

The callable profile is a separate source profile that must project into the
common `LoopRecipeV1` vocabulary. It must not widen or relabel the existing
`generic_g0` nested profile, and it must not create a Generic-specific
physicalizer or SSA/PHI owner.

The design must close this immutable mapping before code is written:

| source obligation | portable product | physical obligation |
| --- | --- | --- |
| condition read, operator, bound | one LoopNode condition role | one BindingSSA read |
| body value read | body operation/read role | one BindingSSA read |
| assignment target/value pair | recurrence/rebind role | one exact rebind and PHI publication |
| loop carrier and declaration | carrier/edge relation | one canonical carrier materialization |
| step and `After` binding | JoinSig exit/continuation relation | one canonical CFG edge and completion disposition |
| tail/loop-out obligations | callable completion envelope, not loop Recipe | existing completion/DraftSeal owner |

The map is AST-free, immutable, owner/frame branded, and names every source
site and BindingRef exactly once. If the common Recipe schema cannot represent
the selected profile without inventing a second authority, the result is
`NoSafeSlice`; the profile is not widened during lowering.

## Authority boundaries

```text
resolver/source inventory + one-shot source lease
  -> callable source projection and role map
  -> existing LoopRecipeV1 / VerifiedLoopCoreProductV1 / JoinSig
  -> RecipeVerifier
  -> CanonicalSsaFunctionSessionV2
       (ResolvedSsaIdentityStateV2 + CanonicalCfgSessionV1 + PhiTxn)
  -> existing completion / DraftSeal / atomic publication
```

The current `CallableSemanticLoweringState` `variables`/`assignments` and
ValueId maps remain migration bridges only. They cannot be the S1 source
authority, a Recipe field, or a second physical ledger. `RawInvocationChildPort`
must not discard a receipt and then claim physical consumption; the discarded
S0 receipt remains evidence only until the real handoff is implemented.

## Required negative boundary

Every failure is typed and pre-effect:

```text
missing/partial/duplicate/foreign/nested source site
wrong role or target/value pairing
unsupported operator, carrier, JoinSig, After, or tail
cross-owner or cross-session BindingRef
CFG/BindingSSA/PHI mismatch
```

There is no retry, fallback, re-selection, AST/name lookup, synthetic
ValueId/PHI, or post-effect ledger repair.

## Shallow execution ladder

```text
S1-D0   close this source->Recipe map and NoSafeSlice boundary
S1-MAP  implement AST-free Recipe/JoinSig/effect product + immutable negatives
S1-PHYS prove canonical physicalizer, After/tail, completion, fresh-session,
        exactly-once ledger consumption, DraftSeal (production caller = 0)
S2-CUT  named production caller = 1, selected old callers = 0, strict/backend
        parity, no retry/fallback/reselection
R1      caller-zero manifest, then delete/retire legacy Generic route/composer
```

Do not create deeper D4 suffixes for this profile. Existing D4 evidence stays
historical input; this ladder is the current execution surface.

## Production selection gate

Selection remains closed until all of these are true:

```text
resolver-owned source authority is the sole issuer
complete source->Recipe/JoinSig/effect map is sealed
canonical physicalizer reaches completion and DraftSeal in a fresh session
new named production caller is connected
selected profile old route/composer/ValueId callers are zero
retry/fallback/reselection are zero
strict receipt, fresh reuse, atomic rollback, and backend parity are green
current docs and exact docs/reference entries are updated in the same cutover
commit
```

S0 focused green, a candidate envelope, a Recipe producer in isolation, or a
legacy route success is not a selection gate.

## Explicit non-claims

```text
portable callable Recipe support = 0
physical ValueId/PHI consumption = 0
production selection = 0
legacy deletion = 0
```

Every future implementation row must update the exact `docs/reference/**`
entry, immutable fixture/receipt, current pointer, and active workstream in
the same commit as its code. This design stop itself changes no runtime
behavior.
