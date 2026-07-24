# RAW public cutover PARITY0 Script body-return design question

Decision: `RAW-BODY-RETURN-prime-r1`

Status: closed as migration evidence. The A′ owner boundary below was
implemented in the paired S0 task, but its Legacy any-statement-tail rule is
not canonical language authority.

`FUNCTION-EXIT-SEMANTICS-prime-r1` supersedes the semantic interpretation of
this card. Canonical Script evaluation is `ScriptLastExpressionOrUnit`;
`Print`, `Local`, `Assignment`, and `CompoundAssignment` remain Unit. The
landed BODY owner/witness structure remains implementation evidence until
`SCRIPT-RESULT-TAIL0-S0` replaces the Legacy tail classifier.

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

The following decisions are now closed:

```text
1. Script starts with provisional `Unknown`; App starts with fixed `Void`.
2. `InstalledRawRootEnvironmentV1::drive_root_body` is the single BODY owner
   that consumes `RootBodyResultV1` and performs the paired
   Return/signature seal? `drive_root_body` and
   `finish_raw_root_function_v1` is removed.
3. BODY prepares one exit plan and emits completion Value(v), physical Return(v),
   and Builder type(v) are the same fact? BODY is the preferred boundary;
   ROOTBATCH validates only the borrowed witness.
4. Missing/Unknown/unsupported type, undefined value, route drift, and tracker
   non-closure are typed fail-fast errors retaining the unpublished owner.
5. Legacy App also infers a last-value return through the common
   `finalize_module` path. Keeping App `Void` therefore requires an explicit
   divergence: Raw App is FixedVoid here and scalar App parity is parked in a
   separate `BODY-RETURN-APP-D0` row.
```

Implementation authorization is limited to the paired S0 task. No App scalar
parity, normal-entry cutover, or adapter repair is implied.

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
