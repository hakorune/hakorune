# DEL-002 Stage0 Delegate Syntax Metadata Capsule SSOT

Status: accepted
Date: 2026-05-14
Lane: phase-293x language minimal surface lane

## Decision

Accept the explicit delegation parser capsule:

```hako
box MeshNode {
    p2p: P2PBox = new P2PBox()

    delegate p2p exposes {
        connect
        send as p2pSend
    }
}
```

Stage0 owns only syntax acceptance and metadata transport.

## Stage0 owns

- Parse `delegate <field> exposes { ... }` as a box member.
- Carry `DelegateDecl { field_name, exposes }` in the AST.
- Carry `source_name` and `exposed_name` for each exposed method.
- Preserve delegate metadata through AST JSON and Program JSON v0 box metadata.
- Reject an empty `exposes` list.

## Stage0 does not own

- Field existence checking.
- Method existence checking on the delegate target.
- Forwarding method generation.
- Collision checking.
- Interface conformance.
- `delegate <field> implements Interface`.
- Wildcard exposure.
- Field/property forwarding.
- Inheritance compatibility semantics for legacy `from` / `override`.

## Canonical syntax

```ebnf
delegate_decl   := 'delegate' IDENT 'exposes' '{' delegate_expose+ '}'
delegate_expose := IDENT ( 'as' IDENT )? ','?
```

`exposes` and `as` are contextual words inside the delegate member only.

## Stop lines

```text
no extends source spelling
no super/origin
no inherited fields
no wildcard exposes in MVP
no auto collision resolution
no forwarding generation in Stage0
```

## Retire condition

Retire this Rust Stage0 capsule when the Stage1/selfhost parser and metadata
transport own the same `DelegateDecl` shape.

## Next row

`DEL-003 Stage1 delegate exposes lowering`:

- Resolve delegate field.
- Resolve source methods.
- Reject exposed-name collisions.
- Generate explicit forwarding methods.
- Keep field access explicit as `me.<delegate_field>.<field>`.
