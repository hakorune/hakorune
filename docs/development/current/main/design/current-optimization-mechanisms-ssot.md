---
Status: SSOT
Decision: current
Date: 2026-04-16
Scope: current Hakorune optimization mechanisms を、legacy wording から current owner rows へ引き直して 1 枚で読むための正本
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/optimization-layer-roadmap-ssot.md
  - docs/development/current/main/design/optimization-task-card-os-ssot.md
  - docs/development/current/main/design/llvm-line-ownership-and-boundary-ssot.md
  - docs/development/current/main/design/semantic-optimization-authority-ssot.md
  - docs/development/current/main/design/effect-classification-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/optimization-tag-flow-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/lifecycle-typed-value-language-ssot.md
  - docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md
  - docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md
---

# Current Optimization Mechanisms SSOT

## Goal

- 「今の Hakorune には何の最適化機構があるか」を 1 枚で読めるようにする
- old wording と current owner row を対応付ける
- `substrate / producer / exporter / consumer` の role と、`landed mechanism / owner seam / scaffold / backlog` の status を分けて読む
- phase README を横断しないと current picture が見えない状態を避ける

## Status Legend

| Status | Meaning |
| --- | --- |
| `landed mechanism` | actual behavior-changing mechanism が live |
| `landed mechanism (narrow)` | live だが対象 lane / shape は意図的に narrow |
| `owner seam` | classification / metadata / policy owner は landed しているが widening はこれから |
| `scaffold` | artifact / policy / parse surface はあるが backend-active ではない |
| `backlog` | current lane では未実装、または docs 上 deliberately out-of-scope |

## Role Legend

| Role | Meaning |
| --- | --- |
| `substrate` | MIR / contract 側の authority owner |
| `producer` | proof / classification / language contract の生産者 |
| `exporter` | LLVM / boundary / manifest へ facts を書き出す層 |
| `consumer` | substrate facts を読んで widening / profitability / import を行う層 |
| `service` | cross-cutting verifier / diagnostics / guard の層 |

## Read Order

1. [optimization-layer-roadmap-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-layer-roadmap-ssot.md)
2. [optimization-task-card-os-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-task-card-os-ssot.md)
3. [llvm-line-ownership-and-boundary-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/llvm-line-ownership-and-boundary-ssot.md)
4. [semantic-optimization-authority-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/semantic-optimization-authority-ssot.md)
5. [perf-optimization-method-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/perf-optimization-method-ssot.md)
6. [optimization-tag-flow-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-tag-flow-ssot.md)
7. [value-repr-and-abi-manifest-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md)

## Legacy Wording Map

| Older wording | Current row / owner |
| --- | --- |
| `DCE` | `semantic simplification bundle` plus the pure DCE cleanup rerun |
| `Escape` | `memory / effect substrate` plus `LLVM fact export` |
| `Float optimization` | `float math policy` row |
| `Closure optimization` | `closure proof producer` plus `call surface substrate` |
| `dead store` / `load-store forwarding` | `memory-effect layer` |
| `alias軽減` | copy/phi alias reduction in MIR plus borrowed-handle/value-codec cuts |
| `Thin-entry / 呼び出し最適化` | `call surface substrate` |
| `handle ABI -> value ABI` | `call surface substrate` plus `value repr and ABI manifest` |
| `call を C レベルに寄せる` | `boundary / C ABI export` |
| `Escape -> LLVM attributes` | `memory / effect substrate` plus `LLVM fact export` |
| `nocapture / readonly / noalias` | `LLVM fact export`; `noalias` is still backlog |
| `Numeric loop / SIMD` | `numeric loop / SIMD consumer` |
| `induction / reduction / vectorization` | numeric-loop prepass plus `LoopSimdContract` |
| `IPO / PGO / ThinLTO` | `IPO / build-time consumer` plus `PGO hotness overlay` |

## Current Taxonomy

### Substrates

| Area | Role | Status | Current truth |
| --- | --- | --- | --- |
| `canonical simplification substrate` | `substrate` | `landed mechanism` | current optimizer schedule runs placement/effect prepass, semantic simplification, memory-effect, then a pure DCE cleanup rerun. See [src/mir/optimizer/core.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/optimizer/core.rs), [src/mir/passes/dce.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/passes/dce.rs), [docs/development/current/main/phases/phase-227x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-227x/README.md). |
| `value representation / materialization / scalarization substrate` | `substrate` | `owner seam` | old `generic placement / effect` and `agg_local scalarization` now read as one substrate family. String retained-form placement is explicit, and generic placement/effect routes exist, but the broad canonical rewrite story is still metadata-first. See [docs/development/current/main/design/birth-placement-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/birth-placement-ssot.md), [src/mir/placement_effect.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/placement_effect.rs), [src/mir/agg_local_scalarization.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/agg_local_scalarization.rs), [crates/nyash_kernel/src/exports/string_birth_placement.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_birth_placement.rs). |
| `memory / effect substrate` | `substrate` | `landed mechanism (narrow)` | current cuts cover conservative escape analysis, dead private-carrier loads, overwritten-store pruning, same-block store-to-load forwarding, and redundant load elimination. Generic `noalias`-grade widening is still absent. See [src/mir/passes/escape.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/passes/escape.rs), [src/mir/escape_barrier.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/escape_barrier.rs), [src/mir/passes/memory_effect.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/passes/memory_effect.rs), [docs/development/current/main/phases/phase-260x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-260x/README.md). |
| `call surface substrate` | `substrate` | `owner seam` | old `thin-entry actual consumer switch` and `handle ABI -> value ABI` now read as one substrate family. MIR inventories thin-entry candidates/selections, and `value_codec` already covers narrow immediate / borrowed-string classes, but there is no universal consumer or full value-ABI coverage yet. See [src/mir/thin_entry.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/thin_entry.rs), [src/mir/thin_entry_selection.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/thin_entry_selection.rs), [docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md), [crates/nyash_kernel/src/plugin/value_codec/encode.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/value_codec/encode.rs), [crates/nyash_kernel/src/plugin/value_codec/decode.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/value_codec/decode.rs). |

### Producers

| Area | Role | Status | Current truth |
| --- | --- | --- | --- |
| `loop proof producer` | `producer` | `owner seam + landed mechanism (narrow)` | vectorization policy, simple-while numeric induction, integer reduction recognition, and conservative `llvm.loop` metadata are landed. The widening is still hint/proof-first. See [docs/development/current/main/phases/phase-262x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-262x/README.md), [docs/development/current/main/phases/phase-263x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-263x/README.md), [src/llvm_py/builders/loop_simd_contract.py](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/loop_simd_contract.py). |
| `closure proof producer` | `producer` | `owner seam + landed mechanism (narrow)` | capture classification is landed, and env scalarization / thin-entry eligibility exist as owner seams, but actual closure ABI/lowering widening is still deferred. See [docs/development/current/main/phases/phase-269x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-269x/README.md), [docs/development/current/main/phases/phase-270x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-270x/README.md), [docs/development/current/main/phases/phase-271x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-271x/README.md), [src/llvm_py/builders/closure_split_contract.py](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/closure_split_contract.py). |
| `language-level optimization metadata` | `producer` | `scaffold` | `@rune Hint/Contract/IntrinsicCandidate` is documented and parsed, but backend-active use is intentionally disabled until MIR owner fields, consumers, and diagnostics are ready. See [docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md), [docs/development/current/main/design/optimization-tag-flow-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-tag-flow-ssot.md). |
| `float math policy` | `producer` | `backlog` | float no longer reads as a subtheme of early SIMD widening. Fast-math, FMA, reassociation, and FP exception model still need a dedicated language/MIR contract row. |

### Exporters And Consumers

| Area | Role | Status | Current truth |
| --- | --- | --- | --- |
| `LLVM fact export` | `exporter` | `landed mechanism (narrow)` | conservative `readonly` and `nocapture` are applied late at builder finalization; stronger attrs/metadata such as `noalias`, `parallel_accesses`, and TBAA remain backlog. See [src/llvm_py/instructions/llvm_attrs.py](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/llvm_attrs.py), [docs/development/current/main/phases/phase-261x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-261x/README.md). |
| `boundary / C ABI export` | `exporter` | `landed mechanism (narrow)` | the live perf/mainline route is `.hako -> ny-llvmc(boundary pure-first) -> C ABI`, and narrow string/user-box corridors already consume MIR-side metadata there. This remains an operational mainline route, but architecturally it is an exporter/boundary row rather than an authority row. See [docs/development/current/main/design/optimization-tag-flow-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-tag-flow-ssot.md), [docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md), [crates/nyash_kernel/src/exports/string.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string.rs). |
| `numeric loop / SIMD consumer` | `consumer` | `landed mechanism (narrow)` | integer map/sum/compare-select cuts already consume loop proofs and emit conservative loop-vectorization hints, while profitability stays downstream. See [docs/development/current/main/phases/phase-266x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-266x/README.md), [docs/development/current/main/phases/phase-267x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-267x/README.md), [docs/development/current/main/phases/phase-268x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-268x/README.md). |
| `IPO / build-time consumer` | `consumer` | `owner seam + scaffold` | callable-node/call-edge contracts and build-policy seams are landed; ThinLTO can emit a companion bitcode artifact, but actual LLVM-side widening remains narrow. See [docs/development/current/main/phases/phase-272x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-272x/README.md), [docs/development/current/main/phases/phase-273x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-273x/README.md), [docs/development/current/main/phases/phase-274x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-274x/README.md), [src/llvm_py/builders/ipo_build_policy.py](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/ipo_build_policy.py). |
| `PGO hotness overlay` | `consumer` | `scaffold` | generate/use artifacts and sidecars are resolved, but LLVM-side instrumentation/use remains out of scope. Read this as a hotness overlay, not an authority row. See [docs/development/current/main/phases/phase-275x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-275x/README.md), [docs/development/current/main/phases/phase-276x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-276x/README.md), [src/llvm_py/builders/pgo_build_policy.py](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/pgo_build_policy.py). |

### Services

| Area | Role | Status | Current truth |
| --- | --- | --- | --- |
| `optimization export verifier` | `service` | `backlog` | a cross-cutting verifier for MIR contract -> LLVM/C-boundary export consistency is still missing. This should land before widening strong attrs/metadata such as `noalias`, `parallel_accesses`, and TBAA. |
| `optimization task-card OS` | `service` | `landed mechanism` | live optimization work now reads through an explicit task-card OS with one `primary owner`, one `proof delta`, fixed verdict taxonomy, and immediate revert on reject. See [optimization-task-card-os-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-task-card-os-ssot.md), [perf-optimization-method-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/perf-optimization-method-ssot.md), [CURRENT_TASK.md](/home/tomoaki/git/hakorune-selfhost/CURRENT_TASK.md). |

## Current Strong vs Weak

Current strong points:

- owner seams are mostly explicit
- phase closeouts exist for 260x-276x
- perf/micro/asm judge is already fixed
- narrow backend consumers for string, user-box, numeric-loop, and closure/IPO metadata are real
- old flat rows can now be reread as substrates, producers, exporters, and consumers

Current weak points:

- backend-active high-level hint consumption is not live yet
- generic `noalias` / broader LLVM attr feed is not live yet
- full value-ABI coverage is not live yet
- thin-entry still lacks a universal backend consumer
- float-specific widening is still backlog
- optimization export verifier is still missing
- current LLVM lane ownership must be read through `ny-llvmc(boundary pure-first)` daily ownership; `llvm_py` and `native_driver` remain keep/bootstrap lanes rather than perf authority owners

## Phase Pointers

- `phase-137x`: current string guardrail / exact-keeper lane
- `phase-163x`: optimization roadmap regroup parent
- `phase-260x`: memory-effect owner seam
- `phase-261x`: LLVM attrs first seam
- `phase-262x`: first numeric-loop policy seam
- `phase-262x` to `phase-268x`: numeric loop / SIMD row
- `phase-269x` to `phase-271x`: closure split row
- `phase-272x` to `phase-276x`: IPO / ThinLTO / PGO row

## Reading Rule

- use this doc as the current mechanism map
- use [optimization-layer-roadmap-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-layer-roadmap-ssot.md) as the parent `substrate / producer / exporter / consumer` ordering
- use [optimization-task-card-os-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-task-card-os-ssot.md) for live optimization card operation
- use [llvm-line-ownership-and-boundary-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/llvm-line-ownership-and-boundary-ssot.md) for daily vs keep LLVM lane ownership
- use phase READMEs only for closeout detail and proof commands
- do not read `@rune` optimization metadata as backend-active until its activation rule changes
- do not read `LLVM attrs`, `C ABI corridor`, `ThinLTO`, or `PGO` as authority rows
