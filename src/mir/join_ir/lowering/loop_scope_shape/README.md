# loop_scope_shape

LoopScopeShape を箱化し、JoinIR lowering からの質問をここに集約するよ。

- `shape.rs`: 変数分類の SSOT。pinned/carrier/body_local/exit_live と API を提供。
- `builder.rs`: LoopForm + Trio を受け取って LoopScopeShape を組み立て。Case-A ルーティング込み。
- `case_a.rs`: Case-A minimal ターゲット判定（Phase 30 F-3.1 のハードコード集合）。
- `context.rs`: generic_case_a 共通の CaseAContext。
- `tests.rs`: 既存仕様の回帰テスト。

責務の境界:
- 解析・分類の仕様は shape.rs に閉じ込める（他層は API で参照）。
- 新しい入力経路を足すときは builder.rs に箱を追加し、shape.rs の SSOT を崩さないこと。
