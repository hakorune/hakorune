# LOOP-CALLER-ZERO-PARITY-G0-I1-D0

Status: `Accepted after top-down review; direct G0 I1 is superseded by a common recursive segment prerequisite`
Date: `2026-08-08`
Parent: `docs/development/current/main/investigations/loop-caller-zero-parity-g0-design-2026-08-08.md`
North star: `docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md`

## Decision

The exact G0 ingress -> fresh session -> common physicalizer -> distinct G0
Tail -> canonical finish direction remains accepted. Direct I1 implementation
does not open yet.

The common physical path must first learn one missing neutral concept:

```text
portable Recipe / JoinSig             sole logical authority
        -> private derived segment/resume layout
        -> segment-aware canonical CFG allocation
        -> common leaf operation emission
        -> neutral recursive After receipt
        -> profile Tail / Completion adapter
```

The layout is a mechanical physical compatibility product. It may retain or
consume Recipe provenance, but it must not reinterpret control meaning,
reorder by item key, inspect AST, or become a second Recipe.

## Counterexample that fixes the boundary

Generic G0 root body `B1` contains, in semantic order:

```text
item 3       read child carrier entry
item 4       nested child Loop
items 12-15  root update after the child Loop
```

The current topology maps one logical block to one physical block, while the
current operation preflight flattens operation rows and skips structural Loop
items. That cannot represent:

```text
parent segment before child
  -> child entry/body/After
  -> parent resume segment
```

It may emit items 12-15 before the child or place both sides in one physical
block. Therefore a profile-neutral recursive closure alone is insufficient;
segment/resume layout is a prerequisite.

## Sole owners

| Truth | Sole owner |
| --- | --- |
| Loop/Block/Item order, nested control, carriers, After obligations | `LoopRecipeV1` + `LoopJoinSigV1` |
| mechanically derived physical segments and transfers | private `PreparedLoopPhysicalLayoutV1` target |
| BasicBlock creation, edges, terminators, predecessor sealing | `CanonicalCfgSessionV1` |
| BindingRef reaching values and assignment publication | function-owned canonical identity/Binding SSA |
| provisional PHI lifecycle | existing `PhiTxn` |
| callable or G0 Tail, ABI, Completion | outer profile adapter |
| function finish and Return/DraftSeal | existing `finish_for_draft_seal` + DraftSeal |

New semantic owner count is zero.

## Private target shape

Names are provisional; the ownership boundary is normative.

```text
PreparedLoopPhysicalLayoutV1
  - moved/retained full prepared operation program
  - exact Recipe-item coverage receipt
  - ordered PreparedLoopControlSegmentV1 rows
  - exact item -> segment placement
  - exact nested entry / After -> parent resume transfer

PreparedLoopControlSegmentV1
  - logical Loop and Block provenance
  - segment ordinal within that logical block
  - ordered operation item keys
  - one verified transfer
```

The first transfer vocabulary is restricted to what the accepted Recipes
require, for example `Jump`, `Predicate`, and `OpenRootAfter`. Unsupported
`If`/`Exit` structure is typed `NoSafeSlice` before Builder effect; it is not
silently flattened or admitted while this row is open.

The physical block receipt becomes segment-aware:

```text
segment key -> BasicBlockId
operation item -> exact segment -> BasicBlockId
```

Logical-block-only placement is not an execution authority after the
segment-aware cutover.

## Profile-neutral and profile-specific boundaries

The common recursive owner sees only:

```text
PreparedLoopPhysicalLayoutV1
ReadyLoopEntryV1
borrowed canonical CFG / identity / PhiTxn services
```

It does not see Callable/G0 identity, Tail, ABI, Completion, Return,
DraftSeal, module collector, selector, or legacy route handles.

The common ready-After receipt carries neutral owner/root/After/predecessor
and exact common coverage. Callable's fixed `7 = Pure4 + Read2 + Write1`
proof remains outside it. G0's fifteen-operation proof and Tail remain in a
G0 profile-close adapter.

## G0-only capabilities after the common prerequisite

1. Two exact parameter entries are installed from I0 BindingRefs and existing
   formal values through canonical identity. No AST/name lookup or direct
   ValueId fabrication is permitted.
2. `DerivedCarrierEntry` is not an ordinary source read. A dedicated
   `PreparedLoopDerivedCarrierSeedV1` target reads the child-entry BindingRef
   through canonical identity, publishes the Recipe value key once, and owns
   no new SSA/PHI authority.
3. The G0 Tail adapter compares the neutral After source BindingRef with
   `VerifiedGenericG0TailCapabilityV1`, reads it through canonical identity,
   claims Completion once, and reaches only the existing typed finish and
   DraftSeal terminals.

## Finite task order

This is one bounded objective, but BoxShape and new G0 acceptance do not share
an implementation commit.

1. `LOOP-COMMON-RECURSIVE-SEGMENT-PLAN-R1`
   - Builder-free layout derivation and exact coverage/order tests.
   - Callable and G0 counterexamples only; Builder effect remains zero.
2. `LOOP-COMMON-SEGMENT-BLOCK-CUTOVER-R2`
   - allocate/lookup physical blocks by segment and move the Callable canary
     to the same plan;
   - delete the selected logical-block-only execution lookup in the same
     refactor series.
3. `LOOP-COMMON-RECURSIVE-AFTER-R3`
   - replace fixed one-loop Callable edge/coverage authority with the neutral
     recursive edge writer and ready-After receipt;
   - retain Callable coverage only in its outer profile close.
4. `LOOP-CALLER-ZERO-PARITY-G0-I1-R0`
   - exact G0 parameters, derived-carrier seed, all fifteen operations,
     distinct G0 Tail/Completion, canonical finish/DraftSeal, late-failure
     whole-session discard, and fresh rerun;
   - no G0-specific physicalizer.
5. Only afterward: existing M8/M9 coverage, production-selection
   consultation, M10b atomic caller switch, then M11/M12 retirement.

Each implementation cell updates its exact `docs/reference/**`, owning
README, current state, and this task/SSOT in the same commit. The final
cutover cell also removes stale legacy claims from references; a final docs
audit is not a substitute for implementation-coupled updates.

## Stop lines

Until R1-R3 close, G0 physical emission is not authorized. This design does
not activate a selector/caller, M8/M9/all-19 coverage, backend parity, module
publication, retry/fallback retirement, or legacy deletion. It adds no public
Recipe, CFG, SSA, or PHI owner; no AST reread, by-name inference, partial
fallback, or same-session retry is allowed.
