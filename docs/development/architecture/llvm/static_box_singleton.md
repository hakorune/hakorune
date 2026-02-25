# 静的Box（LLVM命令系）の self 先頭規約と互換トグル

目的
- LLVM 命令系の静的Box（例: `LLVMPhiInstructionBox`）のメソッド呼び出しにおける引数規約を一本化し、Verifier/Runner/Bridge での期待を一致させる。

方針（規約）
- 静的Boxのメソッドは「self（Singleton）を先頭引数」に持つ。
  - 例: `static box LLVMPhiInstructionBox { lower_phi(self, dst, incoming_list) { … } }`
- 呼び出し側の見た目は従来通りでよい（糖衣）。
  - 呼び出し例: `PhiInst.lower_phi(dst, incoming)`
  - Bridge/Runner が必要に応じて先頭に Singleton を注入して実行系へ渡す。

互換トグル（既定OFF）
- `HAKO_BRIDGE_INJECT_SINGLETON=1`（alias: `NYASH_BRIDGE_INJECT_SINGLETON`）
  - 役割: 旧スタイル（self 省略呼び出し）を受理し、実行前に `Singleton(LLVMPhiInstructionBox)` を先頭引数として補完。
  - 範囲: LLVM 命令系の静的Box（phi/const/binop/compare/branch/jump/ret…）。未対応は Fail‑Fast（静かなフォールバック禁止）。
  - TTL: 移行期限定の開発補助。完成後は削除または既定OFFのまま維持。

Fail‑Fastポリシー
- Verifier/Bridge は期待 arity と不一致の場合に明確な診断で失敗する。
  - 代表メッセージ（例）: `[bridge/singleton] static-box call missing receiver: LLVMPhiInstructionBox.lower_phi/2 (expected self+2)`

最小スモーク（設計）
1) 正常（self 統一後）
   - `PhiInst.lower_phi(5, incoming)` が PASS。
2) 互換（self 未追加だがトグルON）
   - `HAKO_BRIDGE_INJECT_SINGLETON=1` で PASS。
3) 失敗（トグルOFF・self なし）
   - 安定化メッセージで FAIL。

関連
- `docs/private/roadmap/phases/phase-20.33/README.md`（Stage‑B 全体方針）
- `lang/src/vm/README.md`（Core/Gate‑C/Bridge 概観）

