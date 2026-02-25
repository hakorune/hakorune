# LAYER GUARD — lang/src

責務
- 言語側（自己ホスト）の箱・共通部品を配置する領域。
- Rust エンジン層（engine/）への直接依存は禁止。Box 経由・公開 ABI のみ利用可。

禁止事項
- engine/runtime への直接参照（相互依存の発生）
- 外部 I/O や OS 依存（テストしにくい副作用）

許可事項
- selfhost/shared の段階移行（まずは `shared/` から）
- selfhost/vm や selfhost/compiler の段階移行（計画中）

Surface Policy（重要）
- 禁止: lang/src 配下で `using "selfhost/..."` の直接参照。
- 許可: lang/src の等価箱（ミラー）を参照すること。必要に応じて `hako_module.toml` でモジュール名エイリアスを定義。
- 運用: 移行中はCIチェックで検出。段階的に WARN→FAIL へ昇格する。

運用
- しばらくは selfhost/ と lang/src/ が共存（ミラー配置）。
- 参照更新は小バッチで実施し、スモークで形状と出力を固定。
