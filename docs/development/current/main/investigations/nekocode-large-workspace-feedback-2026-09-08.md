# NekoCode 3a5d176 / Hakorune 実workspaceフィードバック

Date: 2026-09-08  
Status: 観測記録・ツール担当への改善依頼。compilerの実装許可や必須gateではない。

判定：参照調査には使えるが、このworkspaceでは高速な連続調査を再現できず。再利用条件の改善を担当へ返したい。

対象は Hakorune d28498d591、配布runtime large-workspace-scan-v1。manifestの source_commit は 3a5d17654ae1ef2c0119f3cda91ed46c78f868d9。配布チェックサム6件とCLI/backend起動を確認。GitHubとの独立照合はしていない。

同一 CLI context --session で2件。両方 scan_profile=large、timeout_seconds=300、text_candidates=true、budget=16000。build scripts/proc macros は無効。

|対象|秒|backend_reused|retained|backend|
|---|---:|---|---|---|
|try_compile_published_view_object|127.398|false|false|ok|
|emit_published_view_exe|128.088|false|false|ok|

両方正常完了。semantic references は5件/6件、追加text candidatesは1件/3件。既知の assert! 内callerは未確認text candidateとして保存packetに存在する。表示上限8件の外なので完全packetを確認した。semantic参照の完全性やcaller-zeroの証明には使っていない。

両リクエストの開始/終了scanとも5155ファイル、40323 entries、38255550 bytes。ファイル上限16384、entries上限262144、bytes上限268435456の範囲内。5155件のハッシュ一致、変更・消失0。ただし complete=false、freshness=unknown、backend_synchronization=unverified。

scan issues は以下の symlink_unverified 5件だけ（省略0）：

- plugins/nyash-aot-plugin/libnyash_aot_plugin.so → ../../target/release/libnyash_aot_plugin.so（リンク先なし）
- .venv/lib64 → lib
- .venv/bin/python → python3
- .venv/bin/python3.12 → python3
- .venv/bin/python3 → /usr/bin/python3

5件ともgit未追跡。Python側4件のリンク先は存在する。読み取り確認のみでリンクは変更していない。

1件目 acquisition reasons は no_cached_backend / input_scan_incomplete。2件目は source_not_stable も含む。両方の retention_reasons は input_scan_incomplete / source_not_stable。この source_not_stable は実際のソース変更を観測した意味ではなく、完全性不明の状態で出ている。

改善依頼：Rust解析の入力として関係するsymlinkと、周辺環境のsymlinkをどう判別するかを検討してほしい。実入力のsymlinkは追跡・検証し、対象外とするものは根拠とscopeを明示する方向。全symlink・全未追跡ファイルの一律除外や、鮮度不明の強制再利用は求めない。

largeで観測範囲を拡張でき、再利用不可の具体的なパスと理由を返せる点は有用。こちらでは2件目も約128秒なので、日常の連続利用の採用判定は保留し、単発調査用途に留める。

Hakoruneのソース・依存選択・toolchainは変更していない。前後git status一致、session終了値0、終了後nekocode/rust-analyzer/cargo/rustcプロセスなし。配布物・観測結果はリポジトリ外の/tmpへ保存。

観測時の生データはローカルの `/tmp/hakorune-nekocode-large-observation/` に保存：
`evaluation.json`、各request/response/packet、`provenance.json`、
`symlink-observation.json`、`session-result.json`。一時保存なので永続性は保証しない。
この文書はユーザー依頼による実測サマリの保存であり、ツール本体・生packetは追跡しない。

再試行条件：配布版は `large-workspace-scan-v1`、CLI `nekocode 1.2.0`、
backend `rust-analyzer 1.89.0 (2948388 2025-08-04)`。
実体バイナリを環境変数で指定し、一つの `context WORKSPACE --session` プロセスへ
以下の2件を順番に送った。位置は観測HEAD `d28498d591` に対するもの。

```json
{"at":"src/host_providers/llvm_codegen/published_mir_object.rs:33:15","scan_profile":"large","timeout_seconds":300,"text_candidates":true,"budget":16000}
{"at":"src/host_providers/llvm_codegen/published_mir_object.rs:143:15","scan_profile":"large","timeout_seconds":300,"text_candidates":true,"budget":16000}
```

配布元での226.4秒→2.66秒・再利用成功という報告は、こちらで再現した結果とは分ける。
ローカルworkspaceのsymlink条件が今回の再利用拒否理由として観測されている。
修正後は同じ条件で2件目が再利用されるか、対象入力の鮮度検証を維持できるかを確認したい。
