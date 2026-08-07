# LOOP-COMMON-RECURSIVE-AFTER-R3

Status: accepted implementation row
Decision: accepted after worker premise audit and R3 boundary correction
Date: 2026-08-08

## Design-stop finding

R2 is an exact segment-to-old-topology adapter, not a segment allocator. The
old topology still allocates `Header/Body/Step/After` per logical Loop, while
R1's transfer authority is segment-based. A neutral writer cannot be placed on
top of that adapter: it would bypass R1's transfer graph, leave `Step`
unconnected/unsealed, and retain the selected Callable's fixed edge authority.

The implementation was stopped here, and the physical boundary is now
corrected below. This task is the executable R3-I0 contract; no code may
start from the former fixed-topology adapter.

## Source authority / non-authority

```text
authority:
  LoopRecipeV1 / LoopJoinSigV1
  PreparedLoopPhysicalLayoutV1 (R1 segments + transfer facts)
  ReadyLoopEntryV1 (exact session ingress)
  completed operation receipts (condition ValueId evidence)
  CanonicalCfgSessionV1 / canonical identity / one PhiTxn

non-authority:
  old Header/Body/Step/After topology adapter
  logical-block lookup
  fixed Callable close helper
  current block, item order, segment ordinal, or profile name
  a second CFG/SSA/PHI/transaction owner
```

## Corrected R3 boundary

```text
PreparedLoopPhysicalLayoutV1 + ReadyLoopEntryV1
  -> exact segment allocator
       one physical block per R1 segment
       one root After block
       no synthetic Step block
  -> segment-aware operation dispatch
  -> CompletedLoopSegmentProgramV1
       moved layout
       exact segment-block receipt
       completed operation receipts
  -> preflight every R1 transfer and the entry edge
  -> preheader -> exact root entry segment
  -> emit every R1 Jump/Predicate/OpenNestedLoop exactly once
  -> canonical CFG/identity/PhiTxn sealing
  -> neutral ReadyLoopAfterContinuationV1
```

The `PreparedLoopPhysicalLayoutV1` schema must retain an explicit sealed
`entry_segment`; `segments()[0]` is not an authority. Predicate conditions are
resolved from the completed operation receipt, never passed as a second outer
argument. Tail/Completion meaning is unchanged, but the thin Callable wrapper
may change its input from the fixed closure receipt to the neutral continuation
receipt. The old `close_callable_loop_after_v1` and `from_callable_layout`
authorities retire in this same R3 series; broad legacy deletion remains later.

## Failure and profile boundary

```text
semantic/ingress/transfer preflight before allocation:
  Builder effect = 0

allocation or emission failure:
  discard the whole unpublished function session
  no retry, fallback, or reselection

Callable:
  physical canary through DraftSeal; retain 7 = Pure4 + Read2 + Write1
  only in a thin profile wrapper

Generic G0:
  Builder-free recursive-transfer preflight only
  no G0 block allocation, operation emission, carrier lowering, or DraftSeal
```

## Explicit non-goals

```text
provider/selector changes
retry/fallback
collector/publication changes
M8/M9 activation
M10b/M11/M12 legacy retirement
semantic Tail/Completion redesign
```

## Acceptance gates after this design stop

```text
entry + segment + transfer coverage is exact and owner-branded
no Step block is allocated by the new segment physicalizer
condition missing/foreign/placement/type cases reject before edge emission
Callable reaches the existing DraftSeal through one neutral After receipt
G0 transfer preflight is Builder-free only
old selected fixed-edge caller and the R2 adapter path are zero
all changed source/check files remain below 800 lines
implementation commit updates docs/reference/**, README, current/workstream,
this task closeout, and the executable guards
```

The executable row is now `LOOP-COMMON-RECURSIVE-AFTER-R3-I0`. It must stop
again before G0 physical allocation or production selection.
