# runtime/kernel/string

このディレクトリは `.hako` string semantic kernel の owner map だよ。
`StringCoreBox` wrapper の上にある policy/control truth をここへ集約し、Rust substrate
へ route policy を戻さないのが役割だよ。

## Core Position

- surface / public boundary は `Everything is Box`
- low-level string algorithm control structure / route policy は `.hako` 側が owner
- raw byte scan / compare / copy / allocation / freeze leaf は substrate 側に残す

つまり、ここに置くのは「boxed helper を何度も呼ぶ leaf 実装」ではなく、semantic policy / algorithm control structure の truth だよ。

## This Directory Owns

- `indexOf` / `lastIndexOf` / `contains` / `startsWith` / `endsWith` の algorithm contract
- `substring` / `concat` / `length` chain の orchestration policy
- materialize/view boundary rule
- `.hako` string kernel library の entry shape
- current narrow pilot: `search.hako`

## This Directory Does Not Own

- `StringBox` / `StringViewBox`
- `BoxBase::new`
- `Registry::alloc`
- host handle registry
- raw byte scan / compare / copy
- flat string allocation / flatten
- `freeze.str` leaf 実装

これらは当面 Rust/C substrate に残す。

## Canonical Lowering Contract

SSOT 上の canonical 名は次だよ。

```text
thaw.str
lit.str
str.slice
str.concat2
str.concat3
str.len
str.find_byte_from
str.eq_at
freeze.str
```

`.hako` 側では internal spelling として `__str.*` を使ってよい。
ただし docs/SSOT では contract 名を上の canonical 形で揃える。

## Current Source-Backed Mapping

phase-148x では、canonical 名と current concrete symbol の対応を次で固定する。

- owner route tag:
  - `StringChainPolicyBox.concat_pair_route(...)->"const_suffix"`
- current compiler-side mirror:
  - `lang/c-abi/shims/hako_llvmc_ffi_string_chain_policy.inc`
  - `emit_string_concat_pair_by_policy(...)`
- current concrete executor path:
  - `nyash.string.concat_hs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
    - `concat_const_suffix_fallback(...)`
- intended canonical reading:
  - `thaw.str + lit.str + str.concat2 + freeze.str`

This mapping does not make `concat_hs` the semantic source of truth.
It only records the current executor path under a `.hako`-owned route.

## Current Module

- `search.hako`
  - `find_index(hay, needle) -> i64`
  - `find_index_from(hay, needle, start) -> i64`
 - `last_index(hay, needle) -> i64`
  - `contains(hay, needle) -> i64`
  - `starts_with(hay, needle) -> i64`
  - `ends_with(hay, needle) -> i64`
  - `split_once_index(hay, needle) -> i64`
  - keep it narrow; no widening to the rest of the string kernel yet
  - `StringCoreBox` wrapper should call into this owner vocabulary rather than
    carry search policy locally
  - VM wrapper helper names should read like wrapper-via-owner adapters, not
    like the final owner entrypoints themselves
- `chain_policy.hako`
  - `boundary_kind_store() -> "Store"`
  - `post_store_use_len_observer() -> "LenObserver"`
  - `substring_retained_form(...) -> retained-form tag`
  - `concat_pair_route(...) -> route tag`
  - `insert_mid_route(...) -> route tag`
  - `concat3_route(...) -> route tag`
  - first stage2 semantic-owner landing for concat / substring / post-store observer vocabulary
  - current compiler-side mirror lives in `lang/c-abi/shims/hako_llvmc_ffi_string_chain_policy.inc`
  - that mirror now owns route / retained-form / post-store observer names on the compiler side, so string placement traces no longer hardcode those `.hako` owner terms directly

## Current Narrow Frontier

- `search.hako` is at the v0 landing point for the current string-search pilot
- current authoring lane is semantic boundary tightening only
- further widening is paused until a new exact blocker appears; if none appears, stop the lane and move to inventory or the next fixed order

## Examples

- concat/substring/indexOf specialization policy
- concat/substring/post-store observer route vocabulary
- materialize/view boundary rules
- `find_index` / `contains` / `starts_with` の control structure

## Non-goals

- raw memory leaf を今すぐ `.hako` へ移すこと
- `NyashBox` / host handle に transient token を混ぜること
- boxed helper ABI の上にそのまま low-level search を積むこと
