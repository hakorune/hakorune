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

## Follow-up investigation task

Task id: `VM-LIM-001-FOLLOWUP`.

Focus:

```text
ArrayBox-held InstanceBox identity across push/get
object_key_for Arc ptr dependency
returned user/page object as method receiver
```

Scope:

- read-only or diagnostic-first unless a dedicated VM row is opened
- use shell timeout and VM step budget for every reproduction attempt
- do not make this a MIMAP-011+ blocker

Candidate files:

```text
src/boxes/array/storage.rs
src/core/instance_v2.rs
src/backend/mir_interpreter/helpers.rs
src/backend/mir_interpreter/handlers/boxes_object_fields.rs
lang/src/hako_alloc/memory/page_queue_box.hako
```

Retire follow-up when `VM-LIM-001` itself is retired or a dedicated VM fix row is
opened with its own guard and acceptance contract.

### Follow-up result: identity hypothesis narrowed

The broad identity hypothesis was probed with:

```text
apps/vm-lim-object-queue-identity-probe/main.hako
tools/checks/vm_lim_001_object_queue_identity_probe.sh
```

The VM completed the minimal route:

```text
ArrayBox.push(page object)
ArrayBox.get(0)
returned page object becomes method receiver
```

Existing M166 page queue also completed under an external timeout:

```text
timeout --kill-after=2s 25s bash tools/checks/k2_wide_mimalloc_page_queue_guard.sh
```

Therefore `VM-LIM-001` is narrowed: current evidence does not prove a general
`ArrayBox-held InstanceBox` identity failure. Keep the limitation as a caution for
MIMAP prototype object-heavy lifecycle/facade routes, but treat it as a retirement
candidate if MIMAP-012 LLVM/EXE and a bounded VM probe agree.
