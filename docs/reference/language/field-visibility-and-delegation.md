# Field Visibility and Delegation

Status: Current reference for the no-inheritance delegation surface.

Design SSOT:

- `docs/development/current/main/design/delegation-no-inheritance-ssot.md`
- `docs/development/current/main/design/language-minimal-surface-ssot.md`

## Decision

Hakorune does not use class inheritance as a canonical language model.

Do not introduce or document new code using:

```text
extends
super
origin
inherited fields
protected
implicit override
field merge
property forwarding
```

Legacy `from` / `override` names may still exist in old parser/model internals
or historical examples, but they are not the current canonical surface.

## Canonical Delegation

Behavior reuse is explicit composition:

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

    send(intent, data, target) {
        me.logger.log("send")
        return me.p2p.send(intent, data, target)
    }
}
```

Rules:

- Delegation is declared on a concrete field.
- Only explicitly exposed methods are forwarded.
- Delegate fields are never imported into the owner box.
- Parent/delegate state is accessed through an explicit field path such as
  `me.p2p.state`.
- Local method calls to the delegate use `me.<field>.<method>(...)`.
- Each exposed name occupies the same flat method namespace as the host Box's
  direct methods and every other delegate exposure.
- A collision with a host method or another exposure rejects the whole
  unpublished forwarding batch. Declaration order and last-write-wins never
  choose a winner.
- `as` resolves a collision only by choosing a distinct exposed name. An alias
  cannot overwrite, shadow, or override an existing host method.
- The same target method spelling on different delegate fields is valid when
  called through its explicit field path, for example `me.left.run()` and
  `me.right.run()`.
- Wildcard exposes are not MVP.

These are the accepted language-target rules. The Stage0 parser currently owns
syntax and metadata transport; complete forwarding target issuance and
Builder/MIR activation remain later Stage1 work.

## Field Visibility

Current field visibility is still a separate design area. Do not use field
visibility proposals to imply inheritance semantics.

Current practical rules:

- `box` owns identity and fields.
- `record` owns identity-free aggregate data.
- `delegate` forwards behavior only.
- Public field exposure and public method behavior should be explicit and
  documented by the owning box surface. Computed Property is not a canonical
  member kind.

## Legacy Quarantine

Historical examples such as:

```hako
box Child from Parent {
    override save() {
        from Parent.save()
    }
}
```

are legacy / historical and should not be copied into new code. New code should
use an explicit field plus `delegate field exposes { ... }`.
