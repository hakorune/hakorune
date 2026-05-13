---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: Canonical behavior-reuse surface for boxes, replacing inheritance-style mental models with explicit delegation.
Related:
  - docs/development/current/main/design/language-minimal-surface-ssot.md
  - docs/development/current/main/design/language-feature-implementation-order-ssot.md
  - docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md
  - docs/reference/boxes-system/delegation-system.md
  - docs/reference/core-language/override-delegation-syntax.md
---

# Delegation No-Inheritance SSOT

## Decision

Hakorune does not use class inheritance as a canonical model.
Behavior reuse is explicit field composition plus explicit method delegation.

```text
object identity:
  box

state ownership:
  explicit fields

behavior reuse:
  delegate field exposes method list

contract:
  interface, later Stage1 static conformance

parent call:
  me.<delegate_field>.<method>(...)
```

The canonical model is "has a field and exposes selected methods", not "is a parent".

## Canonical MVP syntax

```hako
box MeshNode {
    p2p: P2PBox = new P2PBox()
    logger: LoggerBox = new LoggerBox()

    delegate p2p exposes {
        connect
        broadcast
        send as p2pSend
    }

    delegate logger exposes {
        log
    }

    send(intent, data, target): Result<void, SendError> {
        me.logger.log("send")
        return me.p2p.send(intent, data, target)
    }
}
```

The delegate target is a field.
The exposed surface is a method list.
The local method body calls the field explicitly.

## Rules

| Rule | Decision |
| --- | --- |
| Target | A delegate target must be a field on `me`. |
| Target type | MVP requires a known Box type for the delegate target field. |
| Exposed members | Only methods may be exposed. |
| Fields | Delegated fields are never imported. |
| Public API | Only names in `exposes` become public forwarding methods. |
| Collision | Collisions reject. |
| Alias | `source as publicName` is allowed to resolve collisions. |
| Wildcard | `exposes *` is forbidden in MVP. |
| Local wrapper | If a local method wraps a delegated method, do not expose that method from the delegate. |
| Generated forwarding | Stage1 owns forwarding generation. |

## Field rule

Fields never merge across delegation.

```hako
box Child {
    parent: Parent = new Parent()

    delegate parent exposes {
        foo
    }

    readParentState(): i64 {
        return me.parent.state
    }
}
```

`state` is not imported into `Child`.
External field exposure must be written as a normal method.

## Collision rule

This rejects:

```hako
box ComplexNode {
    a: ABox = new ABox()
    b: BBox = new BBox()

    delegate a exposes {
        save
    }

    delegate b exposes {
        save
    }
}
```

This is accepted:

```hako
box ComplexNode {
    a: ABox = new ABox()
    b: BBox = new BBox()

    delegate a exposes {
        save as saveA
    }

    delegate b exposes {
        save as saveB
    }
}
```

Local methods also collide with delegated public names.
The MVP resolution is to omit the delegated method and write a local wrapper.

## Interface relation

Delegation and interface are separate concepts.

```text
delegate:
  implementation forwarding

interface:
  method-set contract
```

Interface work remains later Stage1 work.
The MVP does not need interface conformance to use delegation.

Later bridge:

```hako
interface Sender {
    send(intent: String, data: RawBuf, target: PtrId): Result<void, SendError>
}

box MeshNode implements Sender {
    p2p: P2PBox = new P2PBox()

    delegate p2p implements Sender
}
```

`delegate field implements Interface` means:

```text
use the interface method set as the forwarding list
reject if the field does not provide every required method
reject collisions
generate forwarding in Stage1
```

This bridge is not MVP.

## Generic relation

Generic delegation should stay metadata-first.

| Step | Scope |
| --- | --- |
| G1 | generic parameter transport |
| G2 | arity check |
| G3 | generic record / interface metadata |
| G4 | simple substitution in method signatures |
| G5 | `where` constraints |
| G6 | static specialization or monomorphization only if needed |

Do not introduce `where` constraints or interface-object dispatch as part of the delegate MVP.

## Legacy quarantine

Existing docs and parser residue contain inheritance-shaped names.
They are not canonical for new design.

| Legacy surface | New reading |
| --- | --- |
| `box Child from Parent` | legacy delegation surface |
| `override` | legacy explicit override surface |
| `from Parent.method(...)` | legacy parent-call surface |
| internal `extends` field | legacy name for delegation-like metadata |
| `implements` parser sugar | provisional metadata for future static conformance |

New examples, docs, and formatter output should prefer:

```text
field + delegate exposes
me.<field>.<method>(...)
```

Retirement is a separate migration row.
Do not delete legacy support as part of the delegate MVP.

## Not canonical

These are not Hakorune canonical behavior-reuse features:

```text
extends
super
origin
inherited fields
protected
abstract class
virtual class hierarchy
default interface methods
trait object
blanket impl
multiple inheritance
wildcard delegate exposes * in MVP
property forwarding
automatic conflict resolution
```

## Stage split

| Stage | Owns |
| --- | --- |
| Stage0 | parse `delegate field exposes { ... }`, parse aliases, transport metadata, reject duplicate syntax forms early |
| Stage1 | resolve delegate target type, check method existence, check collisions, generate forwarding, expose MIR facts |
| Stage1 later | interface method-set conformance, `delegate field implements Interface`, generic arity/substitution |

Stage0 must not own dispatch semantics, conformance checking, or generated forwarding.

## Implementation order

| Order | Row | Scope |
| --- | --- | --- |
| D1 | `delegation-no-inheritance SSOT` | docs-only decision; inheritance surfaces are legacy quarantine |
| D2 | `Stage0 delegate syntax metadata capsule` | parse `delegate field exposes { method, method as alias }` |
| D3 | `Stage1 delegate exposes lowering` | method resolution, collision rejection, forwarding generation |
| D4 | `interface MVP` | method set and static conformance metadata |
| D5 | `delegate field implements Interface` | use interface method set as forwarding list |
| D6 | `generic interface metadata` | arity and substitution metadata |
| D7 | `where constraints` | deferred |

