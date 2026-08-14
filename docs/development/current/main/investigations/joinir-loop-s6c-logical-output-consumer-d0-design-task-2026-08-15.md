---
Status: design_stop; next consumer boundary only
Date: 2026-08-15
Decision: design a product-first logical consumer without opening JoinModule/MIR
Scope: M8 LoopV0 forward ScanWithInit logical output; caller-zero only
---

# JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-LOGICAL-OUTPUT-CONSUMER-D0

## Six-line brief

```text
Decision: keep the S6C logical output product as the only consumer input.
Source authority + canonical issuer: VerifiedS6CScanWithInitLogicalOutputV1 and its private HRTB view.
Non-authority: raw Recipe/JoinSig, JoinModule/MIR, names, selectors, physical IDs, Artifact, fallback, retry.
Fail-fast boundary: consumer role/domain/transfer drift rejects before any module or backend effect.
Smallest next slice: specify one caller-zero logical consumer façade and its typed output contract.
Non-claims: no JoinModule construction, MIR lowering, physical selection, production caller, or legacy retirement.
```

## Current capsule

The preceding I0 producer consumes the combined non-Clone S6C
Facts/Recipe/Join product by value and retains it inside
`VerifiedS6CScanWithInitLogicalOutputV1`. Its private façade lends fixed
Recipe-local logical rows and borrows the already sealed source-call and Join
transfer authorities. It does not create a new key space or materialize
JoinModule/MIR.

The next design must decide only how a future caller-zero logical consumer
borrows that product and reports a typed, logical result. It must not accept a
Recipe-only or JoinSig-only input, and it must not use the compatibility
`LoopToJoinLowerer` as a semantic oracle.

## Required design checks

1. The consumer receives the combined output product or its private HRTB view;
   no constituent product can be re-paired by the caller.
2. Every output row is consumed by a fixed typed role; missing, duplicate,
   swapped, foreign-owner, and transfer-drift cases reject before effects.
3. Logical output remains distinct from JoinModule/MIR values, blocks, IDs,
   ABI, and physical layout.
4. The callable Tail `return -1` remains Facts/Completion authority and is not
   imported as a loop consumer exit.
5. The consumer has no `Option` fallback, selector, retry, or production
   route. A later physical consumer requires a separate design stop.

## Acceptance / negative matrix

```text
positive: exact 15 items, 2 calls, one If/Return, one Backedge, After L0/B0/I64
negative: row omission/duplication, call swap, wrong class, foreign owner/frame,
          branch/summary/After drift, raw Recipe input, Tail import, fallback
```

The current pointer remains here until this boundary is accepted. Then the
consumer may be implemented as one bounded caller-zero BoxShape/BoxCount row
without changing source, Facts, Recipe, Join, or physical authorities.
