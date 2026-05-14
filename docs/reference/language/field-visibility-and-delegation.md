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
- Duplicate exposed names fail-fast unless resolved by an explicit alias.
- Wildcard exposes are not MVP.

## Field Visibility

Current field visibility is still a separate design area. Do not use field
visibility proposals to imply inheritance semantics.

Current practical rules:

- `box` owns identity and fields.
- `record` owns identity-free aggregate data.
- `delegate` forwards behavior only.
- Public field/property exposure should be explicit and documented by the
  owning box surface.

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
