---
Status: Provisional SSOT
Scope: helper 境界の tuning 値を 1 箇所へ集約する PolicyBox 契約（runtime/string hot lane）
Related:
- docs/development/current/main/design/optimization-ssot-string-helper-density.md
- docs/development/current/main/design/optimization-portability-classification-ssot.md
- docs/development/current/main/investigations/phase21_5-kilo-hotspot-triage-2026-02-23.md
- src/runtime/host_handles.rs
- crates/nyash_kernel/src/exports/string_span_cache.rs
---

# Helper Boundary Policy SSOT

## Goal

`kilo/text` 系の最適化で増えやすい「閾値・順序・再利用方針」を、helper 本体に散らさず 1 箱に隔離する。

対象:

- Host handle 割当/再利用方針（`alloc` / `drop_handle` 周辺）
- String span cache 方針（slot 数 / admission / promotion）

## Non-goals

- 言語仕様変更
- ベンチ専用分岐の常設
- helper 呼び出し規約そのものの変更

## PolicyBox Contract

1. helper の tuning 条件は Policy API でのみ定義する。
2. helper 実装は Policy API を呼ぶだけにする（閾値の直書き禁止）。
3. 既定 policy は「挙動不変」を原則とし、差分は測定付きで段階導入する。
4. policy 追加時は investigation に根拠（perf/asm）を残す。

## v0 Scope

### HostHandleAllocPolicyBox

- 責務:
  - reusable handle の取り出し順
  - fresh handle 発番（overflow fail-fast）
  - env policy switch (`NYASH_HOST_HANDLE_ALLOC_POLICY`)
- 実装場所:
  - `src/runtime/host_handles.rs`

### StringSpanCachePolicyBox

- 責務:
  - cache admission (`handle` / span byte length)
  - hit promotion 方針
  - slot 数・上限値の SSOT 化
  - env policy switch (`NYASH_STRING_SPAN_CACHE_POLICY`)
- 実装場所:
  - `crates/nyash_kernel/src/exports/string_span_cache.rs`

## v1 Policy Switch (ENV)

- `NYASH_HOST_HANDLE_ALLOC_POLICY`:
  - `lifo` (default): drop 済み handle を LIFO 再利用
  - `none|off|no-reuse`: 再利用せず fresh 発番のみ
- `NYASH_STRING_SPAN_CACHE_POLICY`:
  - `on|enabled|1` (default): TLS span cache 有効
  - `off|disabled|0`: TLS span cache を bypass

Invalid value は fail-fast (`freeze:contract`) とし、silent fallback を禁止する。

## Fail-Fast

- policy 契約違反（overflow / invalid slot）は panic/freeze で即時検出する。
- policy 不成立時は generic route に戻る（silent no-op 禁止）。

## Acceptance

- `cargo check --bin hakorune`
- `cargo test host_handle_alloc_policy_invalid_value_panics -- --nocapture`
- `cargo test string_span_cache_policy_invalid_value_panics -- --nocapture`
- `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture`
- `cargo test -p nyash_kernel cache_invalidates_on_drop_epoch_when_handle_is_reused -- --nocapture`
- `tools/checks/dev_gate.sh quick`
