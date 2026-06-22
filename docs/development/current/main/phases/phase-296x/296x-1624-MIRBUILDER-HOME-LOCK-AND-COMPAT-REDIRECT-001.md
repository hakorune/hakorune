# 296x-1624 MIRBUILDER-HOME-LOCK-AND-COMPAT-REDIRECT-001

Status: landed
Date: 2026-06-22

## Purpose

Lock the canonical MirBuilder native `.hako` home and separate it from the
legacy compiler-tree compatibility surface. The cleanup discussion already
showed that a bulk delete is unsafe, so this row fixes authority first and
keeps the compat tree parked for later drain.

## Decision

```text
canonical_home:
  lang/src/mir/builder/

canonical_entry:
  lang/src/mir/builder/MirBuilderBox.hako

canonical_namespace:
  lang.mir.builder.*

compat_namespace:
  hako.mir.builder.*
```

## Scope

```text
BoxCount: one authority-lock contract
owner: MirBuilder home lock
input: current authority / compat layout
output: canonical home + compat redirect boundary
```

## Result

The native home is fixed to `lang/src/mir/builder/`. The legacy
`lang/src/compiler/mirbuilder/` tree remains compat / migration owned for
now and does not receive new authority behavior in this row.

## Acceptance

```text
canonical_home is explicit and singular
compat tree is explicit and parked
no new authority behavior is added under lang/src/compiler/mirbuilder/
generated artifacts remain execution artifacts only
```

## Stop Line

```text
do_not_add_new_authority_behavior_under_compiler_mirbuilder=1
do_not_delete_compiler_mirbuilder_yet=1
do_not_rehome_generated_artifacts_into_authority_yet=1
```
