# DEL-003 Stage1 Delegate Exposes Lowering SSOT

Status: accepted
Date: 2026-05-14
Lane: phase-293x language minimal surface lane

## Decision

Lower explicit delegate exposure metadata into concrete forwarding methods after
parsing the full Program.

```hako
box MeshNode {
    p2p: P2PBox = new P2PBox()

    delegate p2p exposes {
        connect
        send as p2pSend
    }
}
```

If `p2p` has declared type `P2PBox`, and `P2PBox` declares `send(value)`, the
lowering creates a local method equivalent to:

```hako
p2pSend(value) {
    return me.p2p.send(value)
}
```

## Stage1 owns

- Resolve the delegate field on the containing box.
- Require the delegate field to have a declared type.
- Resolve the declared type to a sibling box declaration in the same Program.
- Resolve each exposed source method uniquely on the target box.
- Reject local method collisions.
- Reject duplicate exposed names within the same box.
- Generate forwarding methods with copied params, param declarations, and return type metadata.
- Keep the original delegate metadata for tooling and diagnostics.

## Stage1 does not own yet

- Interface method-set conformance.
- `delegate field implements Interface`.
- Wildcard exposes.
- Field/property forwarding.
- Cross-module target resolution.
- Generic substitution for target method signatures.
- Legacy `from` / `override` migration.

## Fail-fast cases

```text
delegate field missing
field has no declared type
declared type is not a known box in the current Program
source method missing on target box
source method ambiguous on target box
exposed name duplicates another delegate exposed name
exposed name collides with a local method
```

## Stop lines

```text
no super/origin
no inherited fields
no wildcard exposes in MVP
no silent skip for unresolved target boxes
no forwarding without a copied target signature
```

## Retire condition

Retire this Rust Stage1 lowering once selfhost owns delegate method resolution
and emits the same forwarding shape before MIR/Program JSON consumption.

## Next rows

- `DEL-004 legacy quarantine migration` for internal naming/status cleanup.
- `BRAND-001 Stage0 brand declaration metadata capsule` as the next selected language capsule if delegation cleanup is parked.
- `LOOP-003 Stage1 LoopRange lowering` remains open and requires JoinIR/CorePlan route work.
