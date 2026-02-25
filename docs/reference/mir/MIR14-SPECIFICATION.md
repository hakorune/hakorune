# MIR14 命令セット仕様（補助資料）

Status: Supplemental (Profile spec)
SSOT: `docs/reference/mir/INSTRUCTION_SET.md`

この文書は MIR14 プロファイルの補助説明だよ。実装語彙の最終値は `INSTRUCTION_SET.md` を正とする。

## 概要

- MIR14 は歴史的には「Core-13 + UnaryOp」の呼称。
- RCL-3-min3（2026-02-12）で `BoxCall`/`ExternCall` は retired。
- 現在の callsite は `Call + Callee::{Global|Method|Extern|Value}` に canonical 化されている。

## Core-14（補助プロファイル）

1. Const  
2. BinOp  
3. UnaryOp  
4. Compare  
5. TypeOp  
6. Load  
7. Store  
8. Branch  
9. Jump  
10. Return  
11. Phi  
12. NewBox  
13. Call（callee で Method/Extern/Global/Value を識別）

## 補足

- `BoxCall` は `Call(Callee::Method)` へ統合済み。
- `ExternCall` は `Call(Callee::Extern)` へ統合済み。
- 旧命令名は履歴・移行メモとしてのみ参照する。
