# Phase 127: unknown-read strict Fail-Fast（DONE）

## 目的

- Phase 126 で `available_inputs`（function params + CapturedEnv）を Normalized builder に配線できた。
- 次に、`reads` に出てくるのに `writes`/`inputs` のどちらにも解決できない変数を “unknown-read” として検出し、strict では Fail-Fast にする。

## Scope

- 対象: if-only Normalized（dev-only）
- 既定挙動は不変: `joinir_dev_enabled()` のときだけチェック・検証する

## 契約（SSOT）

- unknown-read の定義:
  - `unknown_reads = reads - (writes ∪ inputs)`  
    - `writes`: StepTreeContract.writes
    - `inputs`: reads ∩ available_inputs（Phase 125/126）
- strict（`joinir_strict_enabled()`）:
  - unknown_reads が 1 つでもあれば `freeze_with_hint` で停止（hint必須・1行）
- non-strict/dev:
  - 理由ログ（tag + count + 先頭数件）までで継続

## 受け入れ基準

- `cargo test --lib` が PASS
- Phase 121–126 の smokes が退行しない
- 新規 fixture（例: `return missing_x`）が strict で確実に Fail-Fast する

## Status

- DONE: unknown-read を strict で Fail-Fast 固定（dev-only）

## 関連

- Phase 125: EnvLayout（writes + inputs）
  - `docs/development/current/main/phases/phase-125/README.md`
- Phase 126: available_inputs SSOT wiring
  - `docs/development/current/main/phases/phase-126/README.md`
