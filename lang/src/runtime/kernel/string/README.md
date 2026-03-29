# runtime/kernel/string

このディレクトリは `.hako` string kernel の owner map だよ。

## Core Position

- surface / public boundary は `Everything is Box`
- low-level string algorithm control structure は `.hako` / docs 側が owner
- raw byte scan / compare / copy / allocation / freeze leaf は substrate 側に残す

つまり、ここに置くのは「boxed helper を何度も呼ぶ実装」ではなく、algorithm/control structure の truth だよ。

## This Directory Owns

- `indexOf` / `contains` / `startsWith` / `endsWith` の algorithm contract
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
str.slice
str.concat3
str.len
str.find_byte_from
str.eq_at
freeze.str
```

`.hako` 側では internal spelling として `__str.*` を使ってよい。
ただし docs/SSOT では contract 名を上の canonical 形で揃える。

## Current Module

- `search.hako`
  - `find_index(hay, needle) -> i64`
  - `contains(hay, needle) -> i64`
  - `starts_with(hay, needle) -> i64`
  - `ends_with(hay, needle) -> i64`
  - `split_once_index(hay, needle) -> i64`
  - keep it narrow; no widening to the rest of the string kernel yet
- `chain_policy.hako`
  - `boundary_kind_store() -> "Store"`
  - `post_store_use_len_observer() -> "LenObserver"`
  - `substring_retained_form(...) -> retained-form tag`
  - `concat_pair_route(...) -> route tag`
  - `insert_mid_route(...) -> route tag`
  - `concat3_route(...) -> route tag`
  - first stage2 semantic-owner landing for concat / substring / post-store observer vocabulary
  - current compiler-side mirror lives in `lang/c-abi/shims/hako_llvmc_ffi_string_chain_policy.inc`

## Current Narrow Frontier

- `search.hako` is at the v0 landing point for the current string-search pilot
- current authoring lane is helper extraction / control-structure cleanup only
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
