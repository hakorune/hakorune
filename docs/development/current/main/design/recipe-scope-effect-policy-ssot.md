---
Status: SSOT
Decision: provisional
Date: 2026-03-28
Scope: user box 最適化を benchmark 名ではなく `recipe family` 単位で読むために、`recipe / scope / effect / policy / leaf` の責務境界を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/design/helper-boundary-policy-ssot.md
  - docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md
  - docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md
  - lang/src/runtime/collections/README.md
---

# Recipe / Scope / Effect / Policy (SSOT)

## Purpose

- user box 最適化の共通化単位を `benchmark` や `one-off shape` ではなく `recipe family` に固定する。
- `.hako` semantic owner と native hidden leaf の間で、何を共通機構にし、何を narrow leaf recipe に閉じるかを 1 枚で読む。
- `string/array` の current perf work と、将来の allocator / user box migration を同じ語彙で扱えるようにする。

## Final Reading

- semantic owner lives in `.hako`
- policy / route / control structure live in `.hako`
- native keeps hidden leaf only
- common optimization unit is `recipe family`
- benchmark name must never be the optimization owner

## Five Boxes

### 1. Meaning Box

- owns user-visible semantics
- examples:
  - `get`
  - `set`
  - `push`
  - `has`
  - `len`
  - `indexOf`
- questions answered here:
  - visible fallback/error contract
  - bounds / missing-key behavior
  - method alias meaning

### 2. Scope Box

- owns lifetime / borrow / reuse facts
- canonical classes:
  - `lexical_local`
  - `borrowed_view`
  - `owned_handle`
  - `reused_handle`
  - `escaped_or_shared`
- questions answered here:
  - when does the value die
  - may this handle escape
  - can this value stay borrowed

### 3. Effect Box

- owns side-effect contract
- canonical classes:
  - `memory_none`
  - `memory_arg_read`
  - `memory_arg_readwrite`
  - `may_alloc`
  - `may_barrier`
  - `cold_dynamic`
- questions answered here:
  - what memory may be touched
  - may this route allocate
  - may this route hit GC / host bridge

### 4. Policy Box

- owns tuning and selection knobs
- examples:
  - cache admission
  - reuse order
  - threshold choice
  - remote-free routing
  - GC trigger policy
- rule:
  - thresholds and ordering must not be hardcoded in helpers

### 5. Leaf Box

- owns hidden native leaf
- examples:
  - raw alloc/free/realloc
  - TLS/atomics/GC body
  - raw byte scan/copy
  - raw slot load/store/probe
- rule:
  - leaf is hidden substrate, not semantic owner

## Common Recipe Unit

The common unit is:

`DispatchRecipe = { receiver_family, method_family, arg_value_classes, scope_class, effect_profile }`

### receiver_family

- `ArrayLike`
- `MapLike`
- `TextLike`
- `AllocatorLike`
- `UserBoxLike`

### method_family

- `observer`
- `mutator_local`
- `grow_alloc`
- `bulk`
- `cold_dynamic`

### scope_class

- `lexical_local`
- `borrowed_view`
- `owned_handle`
- `reused_handle`
- `escaped_or_shared`

### effect_profile

- `memory_none`
- `memory_arg_read`
- `memory_arg_readwrite`
- `may_alloc`
- `may_barrier`
- `cold_dynamic`

## SSOT Split Rule

- `scope` answers "when does it die"
- `effect` answers "what can it touch"
- `policy` answers "how do we choose"
- `meaning` answers "what does the method mean"
- `leaf` answers "how is the substrate actually executed"

Do not mix these.

## Current Generic vs Narrow Reading

### Generic enough to keep growing

- collection visible semantics in `.hako` ring1
- raw substrate naming boundary (`slot_load/store/probe/rehash`)
- handle/string-span policy isolation into PolicyBox-style seams
- `hako_alloc` policy/state owner split

### Narrow leaf, keep isolated

- string pointer fast paths
- exact `indexOf` leaf rewrites
- array/string hot observers tied to current lowered shape
- current `kilo` leaf proof routes

### Do not promote as generic owner

- benchmark-name keyed routing
- `RuntimeDataBox` as semantics owner
- host/provider/plugin cold lanes as hot-path owner
- ad-hoc helper-local thresholds

## Allocator Reading

For allocator-like user boxes, keep:

- `.hako` owner:
  - size/bin classification
  - page/slab state machine
  - local vs remote free routing
  - reclaim/purge policy
  - telemetry / trigger policy
- native hidden leaf:
  - final alloc/free entry
  - TLS / atomics / GC body
  - syscall glue

This is the preferred `mimalloc-like` split.

## Fixed Order

1. define recipe family vocabulary first
2. classify current string/array optimizations into `generic / narrow leaf / do-not-promote`
3. move tuning knobs into PolicyBox docs/APIs
4. widen `.hako` owner only through recipe family rows
5. keep native code at hidden leaf only

## Non-goals

- adding benchmark-specific permanent branches
- making `RuntimeDataBox` the optimization owner
- moving allocator metal body into `.hako` in the same wave
- widening current string pilots into generic substrate without recipe/effect proof
