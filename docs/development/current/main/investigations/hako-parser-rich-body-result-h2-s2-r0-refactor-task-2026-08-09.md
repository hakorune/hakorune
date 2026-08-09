---
Status: ready implementation task
Date: 2026-08-09
Row: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-R0`
Parent: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-D0`
Mode: BoxShape / behavior-neutral refactor
---

# HAKO-PARSER-RICH-BODY-RESULT-H2-S2-R0

## Goal

Create safe physical room for the later same-pass rich body result by keeping
`ParserBox` a thin facade and moving one cohesive helper responsibility to a
dedicated owner.

The current file is 787 lines. Adding rich-result state or delegation directly
would cross the repository's 800-line hard limit and further mix facade,
state, JSON utility, and grammar delegation responsibilities.

## Scope

```text
allowed:
  behavior-neutral physical extraction
  stable ParserBox public API
  unchanged ProgramJSON output and parser position behavior
  explicit new owner module and README boundary
  focused structural/regression guard

forbidden:
  ParserNodeProductV1 connection
  typed integer lexical product
  parse_stmt_product / parse_block_product activation
  source vocabulary changes
  ordinary Box method connection
  Take syntax or Home meaning
  FuncScanner / StageB / JSON reconstruction
```

## Structure

Audit `ParserBox` helpers by cohesive responsibility before moving anything.
Prefer a stateless/static compatibility utility owner whose methods can be
delegated without copying parser state. Do not move grammar policy merely to
reduce line count.

```text
parser_box.hako
  coordination state
  grammar entry facade
  tiny delegations

new small owner
  one cohesive extracted helper family
  no parser state duplication
  no new acceptance decision
```

The exact helper family is selected from code evidence during implementation.
If no cohesive extraction preserves API and state ownership, stop with
`NoSafeSlice`; do not compress formatting or combine statements to fake room.

## Acceptance

```text
git diff proves behavior-neutral movement
ParserBox public entry signatures unchanged
existing parser focused gates green
H1, H2-S0, and H2-S1 guards green
current-state pointer guard green
naming guard green
parser_box.hako materially below 760 lines
all new/touched Hako files below 800 lines
no new source kind, language acceptance, or semantic authority
owner README and task receipt updated in the same commit
```

## Verification

At minimum:

```bash
bash tools/checks/hako_parser_box_declaration_h1_guard.sh
bash tools/checks/hako_parser_parameter_list_h2_s1_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/task_name_naming_guard.sh
```

Also run the narrowest existing parser regression named by
`docs/tools/check-scripts-index.md` after the exact extracted helper family is
known. Do not substitute a broad expensive suite for a missing focused test.

## Closeout

Implementation, focused tests/guard, module README, landed receipt, current
pointers, commit, and push close together. The next row is
`HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S0`, the typed integer lexical-parts
product; it remains closed until this behavior-neutral refactor is green.
