---
Status: consultation-question
Date: 2026-08-01
Lane: MirBuilder in-place replacement
Current row: RAW-SCRIPT-RECORD-SCHEMA-ADMISSION0-I0-R0
---

# Record schema admission: consultation question

## Question

Is the following atomic cutover sound, without changing user-visible record
diagnostics or creating a second semantic authority?

```text
CatalogSeal
-> PreparedNormalProgramDeclarationFactsV1::collect(Program) exactly once
-> immutable RecordSchemaDemandViewV1 loan
-> ScriptSemanticSeal
-> CatalogInstall
-> RootLower consumes the same prepared declaration facts exactly once
```

The goal is to make only this selected Script closure Complete:

```text
FullyExplicitRecordLiteralV1 ::= RecordLiteral {
  record_type_name = known non-generic record R,
  fields = every declared field of R exactly once,
  every value = ScriptLexicalCore expression,
}
```

This proves that the existing record owner has zero declaration-owned default
child demands. It may then consume one exact `RecordFieldValue(i)` source
receipt per explicit field while retaining all operational ownership.

## Current evidence

`PreparedNormalProgramDeclarationFactsV1::collect` is source-only. It derives
record declarations and defaults from `Program`, but selected normal currently
collects and installs it only in `prepare_program_root_lowering_state_v1`, after
`ScriptSemanticSeal`.

The existing record owner is intentionally unchanged:

```text
build_record_literal_value_with_port_v1
  -> schema preflight
  -> explicit fields in source order
  -> omitted defaults in declaration order
  -> RecordFieldContractCheck
  -> RecordValuePublish
```

The prior RecordLiteral-only D0 is closed NoSafeSlice because a literal with an
omitted default, such as `Pair {}` for `Pair { value: i64 = 9 }`, would lower a
declaration-owned default through a Program child port with no valid receipt.

## Required invariants

- Collect declaration facts once; do not add a second Program declaration scan.
- Script semantics receives immutable schema demand data only, never mutable
  `MirBuilder` or `CompilationContext`.
- The exact collected declaration product is moved once into existing RootLower
  installation; it is not cloned, reconstructed, or independently paired.
- Unknown record, generic record, duplicate/unknown field, missing required
  field, and omitted-default forms remain Deferred. Their current RootLower
  diagnostics keep their stage and order.
- `RecordUpdate`, record-constructor `New`, defaults as child receipts, fallback,
  retry, and post-preflight route reselection remain out of scope.
- `program_root_work_plan.rs` must not grow; it is at the file-size boundary.
- Every touched source/check file remains below 800 lines.

## Exact old edge to remove if accepted

```text
FullyExplicitRecordLiteralV1
-> Deferred
-> RawInvocationSourceTransportV1::script_root(())
```

Replacement:

```text
FullyExplicitRecordLiteralV1
-> Complete Script semantic source
-> RecordFieldValue(i) receipts
-> existing record owner exactly once
```

## Please decide

1. Is borrowing immutable schema demand from the one prepared declaration
   product before ScriptSemanticSeal, then moving that product to RootLower,
   a single-authority design?
2. What is the smallest product/API boundary that prevents an independent
   schema collector or mutable Builder read from reappearing?
3. Can all invalid/defaulted forms remain Deferred without accidentally moving
   their diagnostics to ScriptSemanticSeal?
4. What focused parity and failure fixtures are necessary before landing the
   atomic row?

