---
Status: Landed
Date: 2026-06-15
Task: RECORD-METHODS-GATE-000
Scope: Guard that record methods remain disabled in v0.
Related:
  - docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md
  - docs/reference/language/EBNF.md
---

# RECORD-METHODS-GATE-000

## Result

```text
output_contract=hako-record-methods-gate-v0
record_methods_enabled=0
record_fini_enabled=0
record_dynamic_dispatch_enabled=0
record_member_grammar_excludes_method_decl=1
box_owns_behavior_surface=1
selected_next=RECORD-WITH-BOX-GATE-000
summary=ok
```

## Stop Line

```text
do not add record methods in v0
do not add record fini
do not add record dynamic dispatch
```
