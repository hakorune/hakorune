---
Status: Current
Date: 2026-06-05
Scope: cut a parser parity catch-up phase before accepting `fastmem ContractName { ... }` as source syntax.
Blocker: FASTMEM-PARSER-PARITY-CATCHUP-296X-001
Related:
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
  - src/parser/
  - lang/src/compiler/parser/
---

# 296x-416 Fastmem Parser Parity Catch-up

## Purpose

Before `fastmem ContractName { ... }` becomes a language source surface, catch
up the `.hako` selfhost parser with the Rust parser for the subset needed by
the memory fast-path lane.

This phase exists because `fastmem` is not only an allocator report word. Once
it becomes source syntax, it must obey the parser syntax extension contract:
Rust parser and `.hako` parser need an aligned parse-only surface before
execution, lowering, or benchmark-front behavior changes.

## Decision

```text
fastmem_source_syntax_rust_only=0
fastmem_source_syntax_requires_dual_parser_parity=1
fastmem_contractless_region_allowed=0
fastmem_parse_only_before_execution=1
fastmem_lowering_open=0
fastmem_runtime_execution_open=0
provider_activation=0
replacement_activation=0
hook_installed=0
global_allocator_claim=0
winner_claim=0
```

`fastmem` remains usable as docs/report/hako_check vocabulary while this phase
is active. The source syntax pilot waits for parser parity.

## Current Gap Snapshot

The `.hako` parser is behind the Rust parser in several source areas:

```text
build_gate:
  rust_parser=present
  hako_parser=missing_or_not_visible_in_main_dispatch

rune_gate_inline:
  rust_parser=present
  hako_parser=contract_name_gap

nowait_task_scope_context_sync_box:
  rust_parser=present
  hako_parser=missing_or_not_visible_in_stmt_dispatch

general_bitwise_shift_expr:
  rust_parser=present
  hako_parser=static_const_only_or_incomplete

type_alias_typed_decl_surface:
  rust_parser=present
  hako_parser=incomplete_or_not_proven
```

Do not fix all of these in one commit. This phase uses a narrow catch-up order
and only promotes one accepted parser shape at a time.

## Task Order

```text
PARSER-FMEM-001:
  parser parity inventory contract
  status=current
  output=stable list of Rust/.hako parser gaps needed before fastmem syntax
  behavior_change=0

PARSER-FMEM-002:
  parser parity gate surface
  status=done
  output=one reusable smoke/probe route for Rust parser and .hako parser
  behavior_change=0
  gate=tools/smokes/v2/profiles/integration/parser/fastmem_parser_parity_smoke.sh

PARSER-FMEM-003:
  general bitwise/shift expression parity
  status=done
  scope=<< >> & | ^ parse parity for ordinary expressions
  reason=fastmem PageKey examples need shift/mask without externcall escape

PARSER-FMEM-004:
  rune contract-name parity
  status=next
  scope=Gate/Inline/FastMemory names as metadata parse-only where applicable
  reason=do not let Rust-only rune metadata become the accepted source truth

PARSER-FMEM-005:
  fastmem block parse-only dual parser pilot
  status=pending
  scope=fastmem IDENT { ... } parses in Rust and .hako parser
  behavior_change=0
  execution=0
  lowering=0

PARSER-FMEM-006:
  fastmem contractless fail-fast parity
  status=pending
  scope=fastmem { ... } and unsafe { ... } are rejected by both parsers

PARSER-FMEM-007:
  remaining Rust-parser catch-up backlog split
  status=pending
  scope=build gate, nowait/task_scope/context/sync box, type alias/typed decl
  rule=split into separate BoxCount rows only when a fixture needs the shape
```

Only after `PARSER-FMEM-005` and `PARSER-FMEM-006` pass can the older
`MIM-FMEM-008 fastmem source syntax pilot` reopen as an implementation row.

## Stop Line

- no Rust-only active grammar for `fastmem`
- no broad `unsafe {}`
- no source-level raw pointer type
- no parser catch-up batch that mixes unrelated accepted shapes
- no lowering or runtime behavior in parse-only rows
- no Type ABI hot lookup
- no Provider ABI replacement-front hot dispatch
- no product allocator activation

## Acceptance

The phase is ready to hand back to `MIM-FMEM-008` when:

```text
rust_parser_fastmem_parse_only=1
hako_parser_fastmem_parse_only=1
rust_parser_contractless_fastmem_reject=1
hako_parser_contractless_fastmem_reject=1
fastmem_execution_open=0
fastmem_lowering_open=0
parser_parity_gate_documented=1
```

Proof should stay light:

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/smokes/v2/profiles/integration/parser/fastmem_parser_parity_smoke.sh
```

Add heavier selfhost parser gates only when `PARSER-FMEM-002` defines the
reusable entry.
