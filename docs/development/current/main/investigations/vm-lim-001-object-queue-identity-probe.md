# VM-LIM-001 object queue identity probe

Decision: accepted as diagnostic probe.

This investigation narrows `VM-LIM-001` to the smallest object-heavy route that
matters for mimalloc page queues:

```text
ArrayBox.push(page object)
ArrayBox.get(0)
returned page object becomes method receiver
```

Probe app:

```text
apps/vm-lim-object-queue-identity-probe/main.hako
```

Probe command:

```bash
bash tools/checks/vm_lim_001_object_queue_identity_probe.sh
```

The probe is **not** a mimalloc acceptance guard. It is allowed to report the
known limitation under timeout. If it completes with `summary=ok`, `VM-LIM-001`
becomes a retirement candidate and should be rechecked against LLVM/EXE parity.

## Root-cause candidates

Read-only inspection points to these surfaces:

```text
src/boxes/array/storage.rs
src/core/instance_v2.rs
src/backend/mir_interpreter/helpers.rs
src/backend/mir_interpreter/handlers/boxes_object_fields.rs
```

Current risk:

- `ArrayBox` returns retained `InstanceBox` values through `share_box()`.
- `InstanceBox::share_box()` is currently `clone_box()`.
- VM object field keys can depend on `Arc` pointer identity.

That combination can destabilize object identity/field ownership when a user box
is stored in an `ArrayBox`, retrieved, then used as a method receiver.

## MIMAP impact

This remains non-blocking for:

```text
MIMAP-011+ LLVM/EXE acceptance
```

MIMAP rows should continue to use scalar VM smokes and LLVM/EXE primary acceptance
for object-heavy page/facade routes.

## Local probe results

Observed on the current branch:

```text
bash tools/checks/vm_lim_001_object_queue_identity_probe.sh
```

Result:

```text
vm-lim-object-queue-identity-probe
values=2,1,1,1
summary=ok
```

Also checked the existing M166 page queue VM guard under an external timeout:

```text
timeout --kill-after=2s 25s bash tools/checks/k2_wide_mimalloc_page_queue_guard.sh
```

Result:

```text
mimalloc-page-queue-proof
entries=0,1,2
ids=10,11,-1,12
direct=1,2,12
counts=3,4,2,2,1
shape=10
summary=ok
```

Conclusion:

```text
The broad hypothesis "ArrayBox-held InstanceBox identity fails across push/get"
is not reproduced by the minimal probe or by the existing M166 page queue proof.
```

The remaining limitation should be treated as narrower until reproduced again:

```text
prototype lifecycle queue/facade object-heavy route before module export and guard timeout hardening
```

MIMAP-012 can therefore proceed with LLVM/EXE primary acceptance while using VM
probes only as non-blocking diagnostics.
