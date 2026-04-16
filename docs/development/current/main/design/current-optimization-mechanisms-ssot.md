---
Status: SSOT
Decision: current
Date: 2026-04-16
Scope: current Hakorune optimization mechanisms を、legacy wording から current owner rows へ引き直して 1 枚で読むための正本
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/optimization-layer-roadmap-ssot.md
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
- `landed mechanism / owner seam / scaffold / backlog` を分けて読む
- phase README を横断しないと current picture が見えない状態を避ける

## Status Legend

| Status | Meaning |
| --- | --- |
| `landed mechanism` | actual behavior-changing mechanism が live |
| `landed mechanism (narrow)` | live だが対象 lane / shape は意図的に narrow |
| `owner seam` | classification / metadata / policy owner は landed しているが widening はこれから |
| `scaffold` | artifact / policy / parse surface はあるが backend-active ではない |
| `backlog` | current lane では未実装、または docs 上 deliberately out-of-scope |

## Read Order

1. [optimization-layer-roadmap-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-layer-roadmap-ssot.md)
2. [semantic-optimization-authority-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/semantic-optimization-authority-ssot.md)
3. [perf-optimization-method-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/perf-optimization-method-ssot.md)
4. [optimization-tag-flow-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-tag-flow-ssot.md)
5. [value-repr-and-abi-manifest-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md)

## Legacy Wording Map

| Older wording | Current row / owner |
| --- | --- |
| `DCE` | `semantic simplification bundle` plus the pure DCE cleanup rerun |
| `Escape` | `escape analysis` plus `escape / barrier -> LLVM attrs` |
| `Float optimization` | subtheme of `numeric loop / SIMD` |
| `Closure optimization` | `closure split` |
| `dead store` / `load-store forwarding` | `memory-effect layer` |
| `alias軽減` | copy/phi alias reduction in MIR plus borrowed-handle/value-codec cuts |
| `Thin-entry / 呼び出し最適化` | `thin-entry actual consumer switch` |
| `handle ABI -> value ABI` | `value repr and ABI manifest` plus `value_codec` |
| `call を C レベルに寄せる` | `.hako -> ny-llvmc(boundary) -> C ABI` corridor |
| `Escape -> LLVM attributes` | `escape / barrier -> LLVM attrs` |
| `nocapture / readonly / noalias` | LLVM attrs feed; `noalias` is still backlog |
| `Numeric loop / SIMD` | `numeric loop / SIMD` |
| `induction / reduction / vectorization` | numeric-loop prepass plus `LoopSimdContract` |
| `IPO / PGO / ThinLTO` | `IPO / build-time optimization` |

## Mechanism Rows

| Area | Status | Current truth |
| --- | --- | --- |
| `semantic simplification bundle` | `landed mechanism` | current optimizer schedule runs placement/effect prepass, semantic simplification, memory-effect, then a pure DCE cleanup rerun. See [src/mir/optimizer/core.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/optimizer/core.rs), [src/mir/passes/dce.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/passes/dce.rs), [docs/development/current/main/phases/phase-227x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-227x/README.md). |
| `escape analysis` | `landed mechanism` | conservative `NewBox`/copy/phi-root escape analysis is live and owns barrier removal. See [src/mir/passes/escape.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/passes/escape.rs), [src/mir/escape_barrier.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/escape_barrier.rs). |
| `birth placement / generic placement-effect` | `owner seam` | string retained-form/birth placement is explicit, and generic placement/effect routes exist, but the broad canonical rewrite story is still metadata-first. See [docs/development/current/main/design/birth-placement-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/birth-placement-ssot.md), [src/mir/placement_effect.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/placement_effect.rs), [crates/nyash_kernel/src/exports/string_birth_placement.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_birth_placement.rs). |
| `memory-effect layer` | `landed mechanism (narrow)` | dedicated owner seam and stats surface are landed; current cuts cover dead private-carrier loads, overwritten-store pruning, same-block store-to-load forwarding, and redundant load elimination. See [src/mir/passes/memory_effect.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/passes/memory_effect.rs), [docs/development/current/main/phases/phase-260x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-260x/README.md). |
| `dead store / load-store forwarding / alias軽減` | `landed mechanism (narrow)` | current logic is intentionally narrow: private-carrier/rooted cases are handled, copy-only alias propagation is used, but there is no generic MemorySSA-like widening and no generic `noalias` feed yet. See [src/mir/passes/dce/memory.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/passes/dce/memory.rs), [docs/development/current/main/phases/phase-199x/199x-90-generic-memory-dce-docs-facts-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-199x/199x-90-generic-memory-dce-docs-facts-ssot.md). |
| `thin-entry / call optimization` | `owner seam` | MIR inventories `thin_entry_candidates` and `thin_entry_selections`, and selective backend consumers read them, but there is no backend-wide universal thin-entry switch yet. See [src/mir/thin_entry.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/thin_entry.rs), [src/mir/thin_entry_selection.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/thin_entry_selection.rs), [docs/development/current/main/phases/phase-215x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-215x/README.md). |
| `handle ABI -> value ABI` | `owner seam` | `value_codec` already returns immediate ints/bools/f64 and borrowed string aliases in narrow cases, but full generic value ABI coverage is not there yet. The canonical truth is still manifest-first borrowed-arg / owned-return vocabulary. See [docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md), [crates/nyash_kernel/src/plugin/value_codec/encode.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/value_codec/encode.rs), [crates/nyash_kernel/src/plugin/value_codec/decode.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/value_codec/decode.rs). |
| `call を C レベルに寄せる` | `landed mechanism (narrow)` | the live perf/mainline route is `.hako -> ny-llvmc(boundary) -> C ABI`, and narrow string/user-box corridors already consume MIR-side metadata there. Coverage is still lane-specific rather than universal. See [docs/development/current/main/design/optimization-tag-flow-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-tag-flow-ssot.md), [docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md), [crates/nyash_kernel/src/exports/string.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string.rs). |
| `LLVM attrs` | `landed mechanism (narrow)` | conservative `readonly` and `nocapture` are applied late at builder finalization; `noalias` and broader attr inference are still backlog. See [src/llvm_py/instructions/llvm_attrs.py](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/llvm_attrs.py), [docs/development/current/main/phases/phase-261x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-261x/README.md). |
| `language-level hints / metadata plumbing` | `scaffold` | `@rune Hint/Contract/IntrinsicCandidate` is documented and parsed, but backend-active use is still intentionally disabled. The live backend consumers today are MIR JSON metadata rows such as `string_kernel_plans`, `thin_entry_*`, and `placement_effect_routes`. See [docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md), [docs/development/current/main/design/optimization-tag-flow-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-tag-flow-ssot.md). |
| `numeric loop / SIMD / induction / reduction / vectorization` | `owner seam + landed mechanism (narrow)` | vectorization policy, simple-while numeric induction, integer reduction recognition, and conservative `llvm.loop` metadata are landed. The widening is still hint/proof-first. See [docs/development/current/main/phases/phase-262x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-262x/README.md), [docs/development/current/main/phases/phase-263x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-263x/README.md), [src/llvm_py/builders/loop_simd_contract.py](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/loop_simd_contract.py). |
| `float optimization` | `backlog` | current docs explicitly treat float as a subtheme of the numeric-loop row, but fast-math, FMA, and floating-point reduction remain out of scope. See [docs/development/current/main/design/optimization-layer-roadmap-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-layer-roadmap-ssot.md). |
| `closure split / capture classification / env scalarization / closure thin-entry` | `owner seam + landed mechanism (narrow)` | capture classification is landed, env scalarization and closure thin-entry eligibility exist as owner seams, but actual closure ABI/lowering widening is still deferred. See [docs/development/current/main/phases/phase-269x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-269x/README.md), [docs/development/current/main/phases/phase-270x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-270x/README.md), [docs/development/current/main/phases/phase-271x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-271x/README.md), [src/llvm_py/builders/closure_split_contract.py](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/closure_split_contract.py). |
| `IPO / ThinLTO / PGO` | `owner seam + scaffold` | callable-node/call-edge contracts and build-policy seams are landed; ThinLTO can emit a companion bitcode artifact and PGO can resolve sidecars, but the actual LLVM-side widening is still narrow or no-op. See [docs/development/current/main/phases/phase-272x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-272x/README.md), [docs/development/current/main/phases/phase-273x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-273x/README.md), [docs/development/current/main/phases/phase-274x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-274x/README.md), [docs/development/current/main/phases/phase-276x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-276x/README.md), [src/llvm_py/builders/ipo_build_policy.py](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/ipo_build_policy.py), [src/llvm_py/builders/pgo_build_policy.py](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/pgo_build_policy.py). |

## Current Strong vs Weak

Current strong points:

- owner seams are mostly explicit
- phase closeouts exist for 260x-276x
- perf/micro/asm judge is already fixed
- narrow backend consumers for string, user-box, numeric-loop, and closure/IPO metadata are real

Current weak points:

- backend-active high-level hint consumption is not live yet
- generic `noalias` / broader LLVM attr feed is not live yet
- full value-ABI coverage is not live yet
- thin-entry still lacks a universal backend consumer
- float-specific widening is still backlog

## Phase Pointers

- `phase-137x`: current string guardrail / exact-keeper lane
- `phase-163x`: optimization roadmap regroup parent
- `phase-260x`: memory-effect owner seam
- `phase-261x`: LLVM attrs first seam
- `phase-262x` to `phase-268x`: numeric loop / SIMD row
- `phase-269x` to `phase-271x`: closure split row
- `phase-272x` to `phase-276x`: IPO / ThinLTO / PGO row

## Reading Rule

- use this doc as the current mechanism map
- use [optimization-layer-roadmap-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-layer-roadmap-ssot.md) as the parent row ordering
- use phase READMEs only for closeout detail and proof commands
- do not read `@rune` optimization metadata as backend-active until its activation rule changes
