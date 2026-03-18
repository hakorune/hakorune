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
  - keep it narrow; no widening to the rest of the string kernel yet

## Next Narrow Op

- `split_once_index(hay, needle) -> i64`
  - add only after prefix/suffix checks are fixed

## Examples

- concat/substring/indexOf specialization policy
- materialize/view boundary rules
- `find_index` / `contains` / `starts_with_at` の control structure

## Non-goals

- raw memory leaf を今すぐ `.hako` へ移すこと
- `NyashBox` / host handle に transient token を混ぜること
- boxed helper ABI の上にそのまま low-level search を積むこと
