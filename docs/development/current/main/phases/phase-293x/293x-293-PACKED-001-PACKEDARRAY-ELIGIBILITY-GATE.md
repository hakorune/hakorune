# 293x-293 PACKED-001 PackedArray eligibility gate

Status: complete

## Decision

Decision: accepted.

`PackedArray<T>` is a source-level request for packed residence. Stage1 must
fail fast when that residence cannot be proven. This row adds the first
Program(JSON v0) eligibility gate for declaration type metadata only; it does
not enable runtime packed storage or backend lowering.

## Scope

- Detect `PackedArray<T>` in declaration type metadata.
- Accept only same-program concrete `record` elements.
- Accept record fields only when all fields are typed integer-lane values,
  including simple `type` aliases and `brand` declarations over integer-lane
  storage.
- Reject ordinary box elements, generic record instantiations, unknown elements,
  weak fields, untyped fields, handle/string fields, and unsupported storage.
- Keep generic arity checking as the earlier failure owner.
- Guard passed locally.

## Non-goals

- Do not add local typed array literal semantics.
- Do not construct production `ArrayStorage::InlineRecord`.
- Do not add public ArrayBox packed APIs.
- Do not add record materialization.
- Do not add backend lowering.
- Do not add boxed fallback for `PackedArray<T>`.
- Do not migrate hako_alloc source to `PackedArray<T>` yet.

## Acceptance

```bash
bash tools/checks/k2_wide_packed_array_eligibility_guard.sh
```

## Next

`LOCALTYPE-001` resumes the source surface sequence by adding local type
annotation metadata before typed array literal semantics.
