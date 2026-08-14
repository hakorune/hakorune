---
Status: design_stop; ConsumerResultDialectMissing
Date: 2026-08-15
Decision: design a product-first logical consumer without opening JoinModule/MIR
Scope: M8 LoopV0 forward ScanWithInit logical output; caller-zero only
---

# JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-LOGICAL-OUTPUT-CONSUMER-D0

## Six-line brief

```text
Decision: keep the S6C logical output product as the only consumer input and define one canonical typed result view.
Source authority + canonical issuer: VerifiedS6CScanWithInitLogicalOutputV1 and its private HRTB view.
Non-authority: raw Recipe/JoinSig, JoinModule/MIR, names, selectors, physical IDs, Artifact, fallback, retry.
Fail-fast boundary: canonical call-role co-seal, row/domain/transfer drift, and result-shape drift reject before any module or backend effect.
Smallest next slice: specify one caller-zero consumer façade, canonical call view, and typed terminal result/reject.
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

## Required correction before consumer I0

The landed output façade currently exposes `rows()`, `input()`, and
`logical_transfer()` as separate read surfaces. That is safe for inspection but
not yet a consumer contract: a caller could pair the `rows().calls()` array
with a different source-call view. The next design must choose one canonical
call projection, preferably a private role-wise view that co-seals the
`Length`/`Substring` row with its retained source contract. The independent
`calls` array must either become that sole canonical view or be removed from
the consumer-facing façade.

The consumer result dialect is also not fixed yet. A consumer must return one
typed logical result or one named terminal reject; `()`/`Option`/legacy
`JoinModule` are not acceptable authority boundaries. If that result introduces
new semantic meaning rather than borrowing/verifying the existing product, it
is a separate BoxCount design stop.

## Required design checks

1. The consumer receives the combined output product only through its private
   HRTB view;
   no constituent product can be re-paired by the caller.
2. Every output row and source-call role is consumed by one fixed typed role;
   missing, duplicate,
   swapped, foreign-owner, and transfer-drift cases reject before effects.
3. Logical output remains distinct from JoinModule/MIR values, blocks, IDs,
   ABI, and physical layout.
4. The callable Tail `return -1` remains Facts/Completion authority and is not
   imported as a loop consumer exit.
5. The consumer returns a named typed result/reject and has no `Option`
   fallback, selector, retry, or production
   route. A later physical consumer requires a separate design stop.

## Acceptance / negative matrix

```text
positive: exact 15 items, one canonical Length/Substring call view, one If/Return,
          one Backedge, After L0/B0/I64, typed result
negative: row omission/duplication, call-role re-pair, wrong class, foreign owner/frame,
          branch/summary/After drift, raw Recipe input, Tail import, fallback,
          untyped/Option result
```

The current pointer remains here until this boundary is accepted. Then the
consumer may be implemented as one bounded caller-zero row without changing
source, Facts, Recipe, Join, or physical authorities.
