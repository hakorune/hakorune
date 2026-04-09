# MIR Unified Reference

Status: thin navigation pointer

このファイルは「全部入りの MIR 解説」ではなく、現在の正本へ案内する薄い入口だよ。
古い 35 命令 tier 表や retired 命令の説明はここでは持たない。

## Read in this order

1. `docs/reference/mir/INSTRUCTION_SET.md`
   - 現在の kept / removed ledger
   - Core profile と実装語彙の区別
   - canonical sum op の位置づけ
2. `docs/reference/mir/metadata-facts-ssot.md`
   - `functions[].metadata` の JSON 契約
   - thin-entry / sum-placement metadata の shape
3. `docs/reference/mir/call-instructions-current.md`
   - current callsite 契約 (`Call + Callee`)
4. `docs/reference/mir/mir-dumper-guide.md`
   - text dump / verbose dump / JSON の読み方

## Implementation anchors

- `src/mir/instruction.rs`
- `src/mir/contracts/backend_core_ops.rs`
- `src/runner/mir_json_emit/mod.rs`

## Historical note

- Core-26 / MIR14 / retired callsite history は補助資料として残っている
- ただし current 実装の authority は上の 4 本に集約する
