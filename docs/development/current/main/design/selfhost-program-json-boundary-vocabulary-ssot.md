---
Status: SSOT
Date: 2026-06-15
Scope: daily selfhost vocabulary around the Program(JSON v0) boundary.
Related:
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - tools/selfhost/README.md
  - lang/README.md
  - tools/selfhost/stage3_same_result_check.sh
  - tools/selfhost_identity_check.sh
---

# Selfhost Program(JSON v0) Boundary Vocabulary

## Purpose

Keep the selfhost migration vocabulary small enough that agents can tell which
side owns a bug without reading every historical stage document.

This document is a vocabulary SSOT. It does not change compiler behavior,
artifact contracts, or selfhost gates.

## Decision

Daily selfhost vocabulary uses one compiler boundary:

```text
Program(JSON v0)
```

Daily stage vocabulary is:

```text
stage0:
  Rust authority side.
  Built by cargo as the current bootstrap/reference binary.
  Owns the boundary today.

stage1:
  .hako side crossing the Program(JSON v0) boundary.
  Partial by design.
  Grows one accepted shape per row.
```

Mental model:

```text
Rust owns the boundary today.
.hako crosses that boundary one shape at a time.
```

This is a vocabulary and ownership boundary, not a claim that
`Program(JSON v0)` is the preferred day-to-day build/run route. Current runner
and build routes may still be MIR-first; `json-v0-route-map-ssot.md` owns that
route distinction.

This replaces daily bug-owner selection by the three-axis reading:

```text
stage0/stage1/stage2-mainline/stage2+
K0/K1/K2/K2-core/K2-wide
owner/substrate
```

with:

```text
Program(JSON v0) boundary
+ owner checklist
```

The older stage2 / K-axis vocabulary remains valid only as roadmap or
historical vocabulary. Do not use it to decide the current bug owner.

## Boundary Reading

```text
             Program(JSON v0)
                    |
      +-------------+-------------+
      |                           |
  stage0                        stage1
  Rust authority                .hako frontier
```

Stage0 includes:

```text
Rust parser
Rust MIRBuilder
Rust optimizer / verifier
Rust VM / ny-llvmc route
Rust runtime / scheduler / thread substrate
```

Stage1 includes:

```text
lang/src/compiler/**
lang/src/runner/**
.hako parser / stage1 CLI
.hako MIRBuilder slices that are already accepted
```

Compat / boundary glue includes:

```text
src/stage1/**
src/runner/json_v0_bridge/**
src/runner/stage1_bridge/**
tools/selfhost/compat/**
tools/selfhost/lib/stage1_contract.sh
```

## MirBuilder Frontier

`.hako` MirBuilder is intentionally partial.

That is not a hidden fallback.

Rules:

```text
accepted shape:
  .hako builder lowers it itself

unsupported shape:
  fail-fast with [freeze:contract][hako_mirbuilder]

mainline fallback to Rust MirBuilder:
  locked off
```

Mainline lock-off examples:

```text
HAKO_SELFHOST_NO_DELEGATE=1
HAKO_MIR_BUILDER_DELEGATE=0
mirbuilder_delegate_forbidden()
```

Compatibility routes may still exist for harnesses, probes, or historical
diagnostics. When they are used, call them compatibility routes explicitly.
Do not read them as the daily stage1 authority.

## Parser Split-Brain

Parser and MirBuilder are both partial stage1 areas, but they are not the same
kind of partiality.

```text
parser:
  split-brain is expected for longer.
  Rust parser remains authority while the .hako parser catches up.

MirBuilder:
  failure-driven frontier.
  one accepted shape per row, with fixture and gate.
```

Do not collapse these two into a single completion claim. Use the owner
checklist below.

## Owner Checklist

Use file path and route to decide the owner.

| Area | Current owner reading | Daily owner |
| --- | --- | --- |
| Source parser authority | `src/parser/**`, AST / Program JSON emission | stage0 |
| `.hako` parser frontier | `lang/src/compiler/**` parser code | stage1 |
| MIRBuilder authority | `src/mir/builder/**` | stage0 |
| `.hako` MIRBuilder frontier | `lang/src/compiler/**` builder code | stage1 |
| Program JSON v0 boundary | `src/stage1/program_json_v0/**`, `src/runner/json_v0_bridge/**` | compat boundary |
| Stage1 bridge / runner glue | `src/runner/stage1_bridge/**`, `tools/selfhost/compat/**` | compat boundary |
| Runtime / scheduler / thread substrate | `src/runtime/**`, `src/boxes/**` runtime primitives | stage0 |
| Backend / ny-llvmc | `src/backend/**`, `crates/nyash-llvm-compiler/**` | stage0 |
| Stage1 CLI / .hako runner | `lang/src/runner/**` | stage1 frontier |

## Route Labels That Must Remain

Some stage-like words are script or artifact contracts. Do not rename them as
part of vocabulary cleanup unless a dedicated tool migration row owns every
caller.

Keep as tool labels:

```text
stage1-cli
launcher-exe
stage2-bin
stage3-bin
stageb-delegate
compat-direct-emit
k2_wide_* guard names
```

Reading:

```text
script label != daily selfhost stage axis
compat route != daily stage1 authority
```

## Retired Daily Vocabulary

Do not use these terms in daily restart instructions, active bug routing, or
new task names unless the task is explicitly a roadmap vocabulary cleanup:

```text
stage2-mainline
stage2+
K0
K1
K2
K2-core
K2-wide
```

Allowed uses:

```text
roadmap documents
historical phase cards
artifact/distribution end-state documents
archive notes
```

If a current task needs one of these words, it must also name the concrete file
owner or artifact route.

## Program(JSON v0) Proofs

Current proof / diagnostic surfaces:

```text
tools/selfhost_identity_check.sh:
  compares Program JSON v0 and MIR JSON v0 between stage1/stage2 artifacts

tools/selfhost/stage3_same_result_check.sh:
  compares same-result snapshots across stage artifacts
```

These prove boundary consistency. They do not prove that all stage1 compiler
domains are complete, and they do not turn `Program(JSON v0)` into the preferred
runner route.

Completion must be answered by the owner checklist plus the relevant per-domain
gate.

## Stop Lines

```text
no silent delegate fallback in mainline
no use of stage1 success as proof of full stage2 / K2 completion
no use of K-axis vocabulary for current bug owner selection
no claim that .hako MirBuilder is complete without per-shape evidence
no claim that parser split-brain is gone until both parser routes are pinned
```

## Task Breakdown

### SELFHOST-VOCAB-001: Add boundary vocabulary SSOT

Status: this document.

Acceptance:

```text
Program(JSON v0) boundary is named as the daily selfhost boundary
stage0 / stage1 daily meanings are fixed
stage2 / K-axis terms are classified as roadmap/historical vocabulary
owner checklist exists
delegate fallback caveat exists
```

### SELFHOST-VOCAB-002: Wire doc entrypoints

Scope:

```text
DOCS_LAYOUT.md
lang/README.md
tools/selfhost/README.md
CURRENT_TASK.md only if restart order changes
```

Acceptance:

```text
new selfhost vocabulary SSOT is discoverable from docs layout
lang/README.md points here before K-axis wording
tools/selfhost/README.md points here before stage2 wording
thin restart mirrors do not grow landed history
```

### SELFHOST-VOCAB-003: Quarantine daily K-axis wording

Scope:

```text
replace daily guidance paragraphs that use K0/K1/K2 or stage2-mainline/stage2+
with links to this SSOT or the historical roadmap owner
```

Non-goal:

```text
do not rewrite historical phase cards
do not delete roadmap docs
do not rename script options or artifact filenames
```

Acceptance:

```text
current daily entry docs no longer require K-axis to choose bug owner
roadmap docs remain reachable
script-visible terms remain unchanged
```

### SELFHOST-VOCAB-004: Delegate-route wording audit

Scope:

```text
HAKO_SELFHOST_NO_DELEGATE
HAKO_MIR_BUILDER_DELEGATE
mirbuilder_delegate_forbidden
stageb-delegate
compat-direct-emit
```

Acceptance:

```text
mainline delegate lock-off is documented
compat/probe routes are explicitly called compatibility routes
no doc says compat delegate route is daily stage1 authority
```

### SELFHOST-VOCAB-005: Owner-checklist guard candidate

Scope:

```text
optional docs/check only
```

Acceptance:

```text
rg-based audit can report daily docs that mention retired daily vocabulary
without failing historical roadmap docs
```

Do not implement a new guard until SELFHOST-VOCAB-002/003 show repeated drift.

### SELFHOST-VOCAB-006: Historical Stage-2 tool wording quarantine

Scope:

```text
tools/ny_parser_mvp.py
tools/ny_stage2_shortcircuit_smoke.sh
tools/selfhost_stage2_bridge_smoke.sh
```

Acceptance:

```text
old Stage-2 wording is labeled as historical parser/MVP bridge vocabulary
script behavior and executable names are unchanged
no current docs read those tools as stage2-mainline authority
```
