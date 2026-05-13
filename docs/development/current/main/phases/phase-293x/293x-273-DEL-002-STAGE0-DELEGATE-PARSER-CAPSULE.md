# 293x-273 DEL-002 Stage0 Delegate Parser Capsule

Status: complete
Date: 2026-05-14

## Scope

Add the Stage0 parser/metadata capsule for explicit delegation:

```hako
delegate p2p exposes {
    connect
    send as p2pSend
}
```

## Landed changes

- Added `DelegateDecl` / `DelegateExposeDecl` AST metadata.
- Parsed `delegate <field> exposes { method [as alias] ... }` inside box members.
- Kept `exposes` and `as` contextual to the delegate member.
- Preserved delegate metadata through AST JSON roundtrip/JoinIR-compatible JSON.
- Exposed delegate metadata in Program JSON v0 user-box declarations.
- Added parser tests for explicit expose list and empty-list rejection.

## Non-goals

- No forwarding method generation.
- No collision checking.
- No interface conformance.
- No wildcard exposes.
- No inherited fields or `super` route.

## Guard

```bash
bash tools/checks/k2_wide_delegate_parser_capsule_guard.sh
```

## Next selected row

`DEL-003 Stage1 delegate exposes lowering`.

`LOOP-003 Stage1 LoopRange lowering` remains open, but it requires a JoinIR/CorePlan route because legacy LoopBuilder has been removed. Do not lower LoopRange by a source-level desugar that breaks `continue` semantics.
