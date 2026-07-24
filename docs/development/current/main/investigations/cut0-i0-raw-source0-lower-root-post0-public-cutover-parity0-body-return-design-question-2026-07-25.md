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

## Worker audit refinement: A is only admissible as A′

The read-only authority audit confirms that the original wording of A is too
strong. The recipe cannot own an exact `MirType` or a lowered `ValueId` before
Builder lowering. `ValueId` and its type are created in the function-owned
Builder state. The safe candidate is therefore:

```text
A′:
  recipe seals only the route/exit policy:
    Script = last-value return
    App    = the separately selected App policy

  Builder lowers the recipe and owns the resulting ValueId/type.
  One BODY finalization boundary consumes RootBodyResultV1, resolves the
  type from the Builder type authority, emits Return(Value), updates the
  root signature, and emits one paired exit witness before state cleanup.
```

The following decisions remain open and are prerequisites for implementation:

```text
1. Does the root skeleton start with a route-aware provisional contract, or
   does it remain open until the first BODY result is known?
2. Which single BODY owner consumes RootBodyResultV1 and performs the paired
   Return/signature seal? `drive_root_body` and
   `finish_raw_root_function_v1` must not remain competing authorities.
3. Where is the co-seal checked that completion Value(v), physical Return(v),
   and Builder type(v) are the same fact? BODY is the preferred boundary;
   ROOTBATCH may only validate a borrowed witness.
4. What is the exact fail-fast error and rejected-owner retention when type or
   exit sealing fails after lowering?
5. Legacy App also infers a last-value return through the common
   `finalize_module` path. Keeping App `Void` therefore requires an explicit
   divergence/parked-row decision, or a separate App return-policy row.
```

This is a design refinement, not an implementation authorization. No source
or parity fixture may widen until these five owner questions are selected.

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
