# 静的Box（LLVM命令系）の self 先頭規約と旧互換経路の退役

目的
- LLVM 命令系の静的Box（例: `LLVMPhiInstructionBox`）のメソッド呼び出しにおける引数規約を一本化し、Verifier/Runner/Bridge での期待を一致させる。

方針（規約）
- 静的Boxのメソッドは「self（Singleton）を先頭引数」に持つ。
  - 例: `static box LLVMPhiInstructionBox { lower_phi(self, dst, incoming_list) { … } }`
- 呼び出し側はcanonicalなselfを含むtyped形へlowerする。
  - 呼び出し例: `LLVMPhiInstructionBox.lower_phi(self, dst, incoming_list)`
  - Bridge/Runnerは名前や`args[0]`からSingletonを補完しない。

旧互換トグル（退役済み）
- `HAKO_BRIDGE_INJECT_SINGLETON=1`（alias: `NYASH_BRIDGE_INJECT_SINGLETON`）
  - 旧JSON bridgeのSingleton injection reissuerは削除済み。
  - 設定が残っていてもJSON mutation/publication前に
    `[freeze:contract][mir-json-bridge/singleton-injection-retired]` で停止する。

Fail‑Fastポリシー
- Verifier/Bridge は期待 arity と不一致の場合に明確な診断で失敗する。
  - 代表メッセージ（例）: `[bridge/singleton] static-box call missing receiver: LLVMPhiInstructionBox.lower_phi/2 (expected self+2)`

最小スモーク（設計）
1) 正常（self 統一後）
   - typed canonical call が PASS。
2) 退役経路（トグルON）
   - `HAKO_BRIDGE_INJECT_SINGLETON=1` で上記の安定したtyped reject。
3) 失敗（self なし）
   - 安定化メッセージで FAIL。

関連
- `docs/private/roadmap/phases/phase-20.33/README.md`（Stage‑B 全体方針）
- `lang/src/vm/README.md`（Core/Gate‑C/Bridge 概観）
