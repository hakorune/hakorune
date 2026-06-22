# 296x-1625 MIRBUILDER-COMPAT-REDIRECT-DRAIN-PLAN-001

Status: landed
Date: 2026-06-22

## Purpose

Record the next cleanup step after the MirBuilder home lock. The canonical
native home is already fixed to `lang/src/mir/builder/`; this row defines how
the legacy compiler-tree entry stays alive while being drained.

## Scope

```text
BoxShape: one compat redirect / drain plan
owner: MirBuilder compat redirect
input: legacy compiler-tree Program(JSON) entries
output: redirect-first drain plan without physical deletion
```

## Decision

```text
canonical_authority:
  lang/src/mir/builder/**

compat_tree:
  lang/src/compiler/mirbuilder/**

redirect_direction:
  lang/src/compiler/mirbuilder -> lang/src/mir/builder

forbidden_direction:
  lang/src/mir/builder -> lang/src/compiler/mirbuilder
```

## Drain Order

```text
1. document the compat-only status in both README entry points
2. keep old entry paths live until active smokes and Stage-A bridge callers move
3. replace old entries with thin forwarding wrappers after their caller proof is green
4. migrate unique behavior family by family into the canonical home or explicit compat subtree
5. delete the compiler-tree only when live imports, runtime path references, active smokes, module registry entries, and unique behavior are all zero
```

## Acceptance

```text
canonical README names lang/src/mir/builder as authority
legacy README names lang/src/compiler/mirbuilder as compat / migration owned
redirect direction is explicit
physical deletion remains parked
```

## Stop Line

```text
do_not_delete_compiler_mirbuilder_yet=1
do_not_make_mir_builder_import_compiler_mirbuilder=1
do_not_add_new_authority_behavior_to_compiler_mirbuilder=1
```
