# 296x-1623 MIRBUILDER-CLEANUP-SAFETY-INVENTORY-001

Status: landed
Date: 2026-06-22

## Purpose

Phase-cut the MirBuilder cleanup discussion into a safe inventory row.
The large deletion ideas are not yet safe: the active MirBuilder tree still
has live references, so this row classifies cleanup surfaces instead of
removing code.

## Scope

```text
BoxCount: one cleanup safety inventory contract
owner: MirBuilder cleanup safety inventory
input: repo reference scan + live/dead candidate review
output: safe cleanup boundary + parked removal candidates
```

## Inventory Result

```text
safe_to_document:
  pointer refreshes
  current-task / now mirror alignment
  cleanup stop-lines

parked:
  lang/src/compiler/mirbuilder/**
  lang/src/compiler/pipeline_v2/mir_builder_box.hako
  lang/src/mir/builder/MirBuilderMinBox.hako
  loop_cond consolidation / loop_cond subdir collapse
```

The parked surfaces still have live references or are later structural
refactors, so they stay out of the deletion lane for now.

## Acceptance

```text
current pointers point to the cleanup safety inventory
no destructive delete is committed
parked surfaces remain retained
next cleanup action requires explicit reference-removal proof
```

## Stop Line

```text
do_not_delete_large_mirbuilder_dirs_yet=1
do_not_remove_pipeline_v2_mir_builder_box_hako_yet=1
do_not_remove_mirbuilder_minbox_yet=1
do_not_collapse_loop_cond_tree_in_this_row=1
```
