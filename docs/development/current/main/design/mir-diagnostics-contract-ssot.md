# MIR Diagnostics Contract (SSOT)

Status: SSOT  
Scope: MIR fail-fast diagnostics (builder / control-flow utilities)  
Last updated: 2026-02-06

## Purpose

Freeze diagnostics の書式を 1 箇所に寄せ、caller/mir_dump の表現 drift を止める。

## Contract

- Freeze メッセージは `crate::mir::diagnostics::FreezeContract` で組み立てる。
  - 形式: `[freeze:contract][<tag>] key=value ...`（1 行）
- caller は `crate::mir::diagnostics::caller_string(...)` を通して `file:line:column` で固定する。
- mir_dump は `crate::mir::diagnostics::mir_dump_value(...)` を通して以下の値のみ許可する。
  - `/tmp/...`（dump path）
  - `disabled`
  - `write_failed`
- 同一タグの callsite で手組み `format!("[freeze:contract][...]" ...)` は禁止する。

## Pinned tags (CLEAN-E)

- `builder/emit_missing_block`
- `builder/non_dominating_copy`
- `builder/binop_operand_out_of_function_scope`
- `builder/capture_jump_without_function`
- `builder/phi_insert_without_function_context`

## Drift check

- Script: `tools/checks/mir_diagnostics_contract.sh`
- Fast gate hook: `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`

運用ルール:
- 契約を広げる/変更する場合は、同コミットでこの SSOT と drift check の両方を更新する。
