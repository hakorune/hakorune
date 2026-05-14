# VM known limitations SSOT

Decision: accepted.

This document records VM limitations that are known, bounded, and not allowed to
silently affect LLVM/EXE acceptance.

## VM-LIM-001 object-heavy page queue/facade route

Status: active limitation.

Affected route shape:

```text
page object stored in queue-like box field
page object stored in ArrayBox then selected later
page object passed through queue helper for lifecycle selection
facade/page queue object-heavy orchestration in mimalloc lane
```

Observed during `MIMAP-010` exploration before the scalar lifecycle selection
policy was adopted. The VM process could keep running without producing proof
output when the queue owner retained or accepted page objects directly.

Current decision:

```text
MIMAP-010:
  use VM scalar lifecycle selection proof

MIMAP-011+:
  use LLVM/EXE primary acceptance for object-heavy page/facade routes
  keep VM only as small scalar smoke unless a row explicitly targets VM support
```

Not blocker for:

- MIMAP-011+ LLVM/EXE acceptance
- mimalloc page/facade object route design

Still required:

- VM guards must use timeout.
- VM timeout must fail the VM guard.
- Rows that rely on this limitation must mention LLVM/EXE primary acceptance.

Retire when:

```text
VM executes page queue/facade object-heavy lifecycle route under the standard
MIMAP VM timeout without hang and with the same output contract as LLVM/EXE.
```

### Investigation notes

Read-only worker investigation narrowed the likely VM risk to object identity and
field-route stability when a user box is retained inside `ArrayBox` and later used
as a receiver again.

Relevant implementation surfaces:

```text
lang/src/hako_alloc/memory/page_queue_box.hako
src/boxes/array/storage.rs
src/core/instance_v2.rs
src/backend/mir_interpreter/helpers.rs
src/backend/mir_interpreter/handlers/boxes_object_fields.rs
```

Risk pattern:

```text
ArrayBox.push(page object)
ArrayBox.get(i)
returned page object becomes method receiver
page fields/methods are resolved through VM object field/key route
```

This reinforces the current split: keep scalar VM proofs, but use LLVM/EXE as the
primary acceptance backend for MIMAP-011+ object-heavy allocator routes.
