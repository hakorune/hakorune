# 293x-347 VM-LIM-001 Object Queue Identity Follow-up

Status: landed.
Decision: accepted.

## Goal

Probe the suspected VM limitation around `ArrayBox` retaining a page/user object
and returning it as a method receiver.

## Probe

```bash
bash tools/checks/vm_lim_001_object_queue_identity_probe.sh
```

The probe is diagnostic, not a mimalloc acceptance guard.

## Result

The minimal route completed under VM:

```text
ArrayBox.push(page object)
ArrayBox.get(0)
returned page object becomes method receiver
```

The older M166 page queue guard also completed under an external timeout.

## Decision

Narrow `VM-LIM-001`: the broad `ArrayBox-held InstanceBox identity` hypothesis is
not reproduced by the minimal probe. Keep VM-LIM-001 as caution for prototype
object-heavy lifecycle/facade routes, but do not treat it as a blocker for
MIMAP-012 LLVM/EXE work.

## Next

Return to:

```text
MIMAP-012 object-backed lifecycle queue LLVM route pilot
```
