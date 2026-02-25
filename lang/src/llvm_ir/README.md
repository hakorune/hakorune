# LLVM Script Builder (opt-in, Phase 20.11)

目的
- Python llvmlite ハーネスで行っている IR 構築を、Hakorune スクリプトの薄い箱で段階的に置き換える。
- 責務は「IR 構築」に限定し、リンクおよび実行は小ライブラリ（libhako_aot）/AotBox に委譲する。

ゲート
- HAKO_LLVM_SCRIPT_BUILDER=1 で有効化（既定OFF）
- 厳格化（未実装はFAIL）: HAKO_LLVM_SCRIPT_BUILDER_STRICT=1（既定はFAIL推奨）

責務境界（Box）
- LLVMModuleBox: モジュール作成・型/レイアウト設定・関数登録
- LLVMFunctionBox: 関数定義・基本ブロック追加
- LLVMBuilderBox: 命令構築（v0: const/binop/ret から開始）
- LLVMTypesBox: 代表的なプリミティブ型クエリ
- LLVMEmitBox: オブジェクト出力（当面は AotBox へ委譲予定）

Fail‑Fast
- 未実装/未対応は `UNSUPPORTED: <op>` を短文で出力して負値を返す（将来は統一エラーへ）。

将来拡張
- v1: compare/branch/phi、v2: call/extern（hako_* の C-ABI のみ）
- MIR→IR の対応は SSOT に集約し、Builder は小さな純関数にまとめる。

