# NekoCode / Hakorune 実workspaceフィードバック

Date: 2026-09-08  
Status: 観測記録・ツール担当への改善依頼。compilerの実装許可や必須gateではない。

## 修正版 4d7326f の再試行

判定：この実workspaceでも連続調査の再利用に成功。必要な参照調査に任意利用できる。
初回起動の約128秒は残るため、同一CLIセッション内で複数シンボルを調べる用途に向く。
以下は一度の同一セッション観測であり、恒常的な性能保証ではない。

対象Hakoruneは `678e44f868`、開始・終了とも作業ツリーclean。
Yドライブの `nekocode-runtime/symlink-input-scope-v1` をworkspace外の
`/tmp/hakorune-nekocode-symlink-input-scope-v1` へコピーして使用。
manifest source_commit=`4d7326fd892ab732c2ad0d7e9620565eb3fbf80b`、
manifest SHA256=`bba5847348dba5b0e4e27bab5bf7e3e5114c5bf3387756f6001eb7d06c86db61`。
配布チェックサム6件、CLI 1.2.0と実backend 1.89.0の起動を確認。
GitHubとの独立照合はしていない。下記旧版と同じ位置・設定・順序で2件実行した。

|対象|秒|backend_reused|acquisition|retained|
|---|---:|---|---|---|
|try_compile_published_view_object|128.462|false|fresh_backend|true|
|emit_published_view_exe|0.478|true|reused|true|

初回reasonsは `no_cached_backend` のみ、2件目は空。
両方retention_reasonsは空、status=completed、backend health=ok、quiescent=true。
全4scan（各requestのbaseline/current）は5155入力・40333 entries・38255550 bytes、
complete=true、issuesなし・省略0。verification=match、matched=5155、変更/消失/
読取不能/未観測/新規/リンク変更はいずれも0。freshness=source_stable。

全scanの `link_scope` は verified=1 / excluded=4 / unverified=0、例の省略0：

|リンク|分類|参照先|
|---|---|---|
|.venv/lib64|verified_directory|lib（workspace内の実体を検証）|
|.venv/bin/python|outside_input_file_scope|python3 → /usr/bin/python3.12|
|.venv/bin/python3.12|outside_input_file_scope|python3 → /usr/bin/python3.12|
|.venv/bin/python3|outside_input_file_scope|/usr/bin/python3 → /usr/bin/python3.12|
|plugins/nyash-aot-plugin/libnyash_aot_plugin.so|outside_generated_output_scope|../../target/release/libnyash_aot_plugin.so（欠落した生成物）|

元の5リンクは変更していない。追加の未確認リンクなし。
semantic referencesは5件/6件、未確認text candidatesは1件/3件。
既知のassert!内caller候補も保存packetに保持されている。
backend_synchronizationは依然unverifiedで、参照網羅やcaller-zeroの証明にはしない。
外部依存・設定・生成入力・環境は、このsource freshness保証の範囲外。

Hakoruneのソース・依存選択・toolchainは試行中変更なし、前後status一致。
session終了値0、終了後nekocode/rust-analyzer/cargo/rustcプロセスなし。
生response/packet、provenance、評価JSON、5リンク観測は
`/tmp/hakorune-nekocode-symlink-observation/` に一時保存（永続性保証なし）。
今回の再利用拒否は解消したので差し戻し不要。MCP単発起動の改善や初回時間の短縮は
今回検証していない。通常のrgと併用し、必要な連続調査で使う。

## 旧版 3a5d176 の観測

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
