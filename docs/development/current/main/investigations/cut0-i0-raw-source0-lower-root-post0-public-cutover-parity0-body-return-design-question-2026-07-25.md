# RAW public cutover PARITY0 Script body-return design question

Decision: `RAW-PUBLIC-CUTOVER-PARITY0-BODY-RETURN-D0`

After the effect repair, the next exact parity row exposed a second mismatch
in the admitted Script scalar slice:

```text
Legacy Script literal/binary:
  main return type = Integer/String
  body = Const(value); Return(value)

Raw Script literal/binary:
  main return type = Void
  body = Const(Void); Return(Void)
```

The Raw recipe already carries `RootBodyResultV1::Value(ValueId)`, but
`begin_raw_root_function_v1()` fixes every root signature to `Void` and
`finish_raw_root_function_v1()` emits a Void return when the block is open.
This is an ABI/owner-boundary mismatch, not a snapshot-normalization issue.

## Q1 — Script root return authority

The Legacy Script root function and its last-value behavior are the current
compatibility authority for the public parity row. App `Main.main/0` remains a
separate Void contract. Physical identity still means symbol `main`, arity 0;
return type is route/body evidence, not part of the symbol identity.

## Q2 — candidate owner shapes

The following candidates require consultation before implementation:

```text
A: route/body recipe supplies a sealed Script return contract and value
   materialization; Raw root skeleton consumes it before signature creation.

B: keep physical main Void and add a separate compatibility result function or
   out-of-band return lane. This risks diverging from Legacy module parity and
   adds a second root publication authority.

C: narrow public Script to empty/statement-only Void and reject scalar rows.
   This contradicts the already selected PARITY0 success matrix and is not
   accepted without changing the public NarrowV1 contract.
```

The preferred candidate is **A**, but it must prove compatibility with
ROOTBATCH0's required Main/condition pair, collector/ledger receipt identity,
App Void semantics, and later publication. Do not implement A until those
questions are sealed.

## Q3 — non-authorities

```text
snapshot normalization
MirPrinter/module JSON
postprocess inference
module symbol inspection
public compatibility adapter
```

None may convert a Raw Void main into a typed scalar return after publication.

## Q4 — fail-fast boundary

Until the return owner is fixed, PARITY0 must keep only the green empty Script
row. Scalar literal, unary, binary, Print, Local, Assignment, and App scalar
success rows stay parked. Snapshot weakening and expected-failure relabeling
are forbidden.

## Non-claims

```text
Script return ABI redesign
ROOTBATCH Main slot widening
App return policy widening
normal-entry cutover
JSON/executor/selfhost/fastmem/CUT0
old Raw retirement
```
