---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Record next rust-subset app-front tasks and separate compiler loop/break acceptance backlog.
Related:
  - apps/rust-subset-to-hako/STATUS.md
  - apps/rust-subset-to-hako/tools/syn_adapter/src/main.rs
  - apps/lib/json_native/parser/parser.hako
---

# RUST-SUBSET-NEXT-TASKS-STRUCTURE-AND-LOOP-BACKLOG-001

## Decision

Keep the rust-subset app-front lane and compiler acceptance lane separate.

The next app-front task remains:

```text
RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-003
```

Before adding a larger shape such as `while`, schedule a structure task:

```text
RUST-SUBSET-SYN-ADAPTER-MODULE-SPLIT-001
```

Separately, preserve the compiler acceptance request for the previously
rejected `read_next_number_literal()` style loop:

```text
COREPLAN-LOOP-BREAK-MULTI-STAGE-RECIPE-ACCEPTANCE-001
```

## Why Separate

The `read_next_number_literal()` blocker is not a RustSubset converter feature.
It is a compiler acceptance problem: a staged scanner loop with `break` should
eventually be accepted by Recipe/CorePlan instead of forcing the JSON parser
back to token payload routes.

The rust-subset app-front still benefits from more source shapes, but those
should not be mixed with CorePlan loop/break acceptance work.

## Planned App-Front Structure Task

```text
task=RUST-SUBSET-SYN-ADAPTER-MODULE-SPLIT-001
current_file=apps/rust-subset-to-hako/tools/syn_adapter/src/main.rs
current_size=480_lines
target_modules=cli,items,functions,stmts,exprs,types
behavior_changed=0
```

Trigger:

```text
before_while_or_vec_literal_implementation=1
```

## Compiler Backlog

```text
task=COREPLAN-LOOP-BREAK-MULTI-STAGE-RECIPE-ACCEPTANCE-001
target_shape=read_next_number_literal staged scanner loop with break
owner=Recipe/CorePlan acceptance
non_goal=restore WIP parser loop before acceptance exists
current_stability_route=token payload + number materializer
```

## Stop Lines

```text
do not mix syn_adapter module split with new RustSubset shape support
do not reintroduce read_next_number_literal loop WIP before compiler acceptance is green
do not treat token payload route as the final parser number design
do not move compiler Recipe/CorePlan work into the rust-subset app-front taskboard
```

## Contract

```text
output_contract=rust-subset-next-tasks-structure-and-loop-backlog-v0

syn_adapter_module_split_task_recorded=1
loop_break_recipe_acceptance_backlog_recorded=1
rust_subset_app_front_and_compiler_acceptance_separated=1
current_app_front_blocker_unchanged=RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-003

summary=ok
```
