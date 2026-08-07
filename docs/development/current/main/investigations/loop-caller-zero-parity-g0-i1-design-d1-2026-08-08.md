# LOOP-CALLER-ZERO-PARITY-G0-I1-D1

Status: `Accepted design stop; implementation remains closed until the two common contracts land`
Date: `2026-08-08`
Parent: `docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`
Worker audit: `/root/g0_i1_design_audit`

## Decision

R3-I0 closed the selected Callable canary, but its recursive After writer is
Callable-shaped in two places that cannot be reused for Generic G0:

1. it carries one condition value and would branch a child Predicate with the
   root Predicate value;
2. the common ReadBinding path accepts only expression anchors, while G0 item
   3 is a `DerivedCarrierEntry` source anchor.

Do not add a G0 physicalizer. First add the two profile-neutral common
contracts in one small BoxShape implementation row, then run one G0
caller-zero canary in the following row.

```text
common predicate-value receipt + common carrier-seed operation
    -> exact G0 ingress
    -> 5 R1 segments + root After
    -> 15 G0 operation rows
    -> distinct G0 Tail / Completion / DraftSeal
```

## Sole authorities

| Concern | Owner |
| --- | --- |
| logical order, nested control, transfer and After obligations | `LoopRecipeV1` + `LoopJoinSigV1` |
| source/effect and exact G0 ingress | existing resolver ledger and `VerifiedGenericRecipeProductG0` |
| derived segment/parent-resume layout | private `PreparedLoopPhysicalLayoutV1` |
| per-segment blocks, edges, terminators | `CanonicalCfgSessionV1` |
| BindingRef reaching values and the derived carrier seed | canonical identity/Binding SSA; use `read_entry_receipt` |
| provisional PHI lifecycle | the one session-local `PhiTxn` |
| G0 Tail, exact I64 ABI, Completion, finish and DraftSeal | outer G0 adapter and existing terminals |

No new semantic, CFG, SSA, PHI, Tail, or publication owner is introduced.

## Common contract A: per-transfer Predicate values

`LoopPhysicalTransferV1::Predicate.condition` is the authority for which
condition value a transfer consumes. Before any instruction emission, the
common physicalizer must build a sealed receipt table from the completed
segment program:

```text
transfer.condition
  -> exact Recipe value key
  -> one completed ValueId
  -> same owner
  -> Bool class / MirType::Bool
  -> physical block equal to the condition's source segment
```

The neutral After receipt contains only owner, root After, predecessor, and
common transfer/coverage receipts. It does not contain one `condition_key`,
Callable operation counts, or a profile label. Callable keeps its
`7 = Pure4 + Read2 + Write1` and one-condition proof in its outer profile
close. Generic G0 validates both its root and child condition receipts.

## Common contract B: derived carrier seed

The common operation family gains a profile-neutral prepared variant for a
`DerivedCarrierEntry` anchor, for example:

```text
PreparedLoopDerivedCarrierSeedV1
  source loop/carrier provenance
  source BindingRef
  Recipe value key
```

Its emitter delegates to canonical identity's non-claiming
`read_entry_receipt(builder, phis, block, binding)`. It does not fabricate an
expression site, create a G0-name branch, or create a second SSA/PHI owner.
The G0 adapter supplies the exact capability; the common dispatcher emits the
same prepared variant for any future profile with the same source role.

## Two implementation cells

### A — `LOOP-COMMON-PREDICATE-CARRIER-I0-R0`

One common BoxShape row:

- per-transfer condition receipt table and neutral After boundary;
- `DerivedCarrierEntry` prepared/emitter variant using canonical identity;
- Callable regression remains green;
- no G0 allocation, production selection, selector, fallback, retry, or
  legacy deletion;
- all affected source/README/test/guard/reference-current docs update in the
  same commit.

### B — `LOOP-CALLER-ZERO-PARITY-G0-I1-R0`

One cfg(test) caller-zero canary:

- exact compiler-side G0 ingress and two parameter entries;
- five R1 segments plus one root After, allocated by the common allocator;
- all fifteen G0 operation rows exactly once (item 3 uses the carrier seed;
  item 4 is structural nested Loop, not an emitted operation);
- root and child Predicate values use their own receipts;
- G0 post-loop `b1` is read through canonical identity, then exact I64
  Tail/Completion reaches `finish_for_draft_seal` and DraftSeal;
- post-emission failure discards the whole unpublished session and a fresh
  session reproduces the same semantic shape.

Both cells remain test-only caller-zero evidence. Production selection,
M8/M9, M10b, M11/M12, retry/fallback retirement, collector publication, and
broad legacy deletion remain later rows.

## Non-claims and stop lines

```text
G0-specific physicalizer                    = 0
single-operation extraction from full demand = 0
first Predicate reused for every transfer   = 0
expression fabrication for DerivedCarrier   = 0
AST reread / name lookup / ValueId fabricate = 0
same-session retry / provider fallback       = 0
production caller switch                    = 0
```

If either common contract cannot be issued from exact existing evidence, the
row returns typed `NoSafeSlice` before Builder effect. After allocation or
emission, any failure uses whole-session discard and restores the caller once.

Each implementation row must keep touched source/check files below 800 lines,
update the exact `docs/reference/**` pages and affected README in the same
commit, and record migration/retirement conditions before it is marked done.
