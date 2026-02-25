# Phase 25.1 — Span Trace Mini Note

- 方針: MIR 命令に AST Span を持たせ、VMError (StepBudgetExceeded) で fn/bb/inst に加えて .hako 行番号を出す。
- 実装: MirInstruction 生成時に current_span を保存し、VM 側で last_inst_idx から Span を引いてエラーに埋め込む。Span が無い場合は従来どおり fn/bb/inst のみ。
- 状態: Stage‑1 CLI の MIR には Span 未付与なので行番号はまだ出ていないが、Span 付き MIR なら `... (file.hako:line:col)` まで表示できる。
- ダンプ: `RUST_MIR_DUMP_PATH=/tmp/foo.mir` を指定すると、VM 実行前の MirModule をファイルに出力できる（`--dump-mir` のファイル版）。Stage‑1/Stage‑B 経路でも共通で使う想定。
- JSON v0 経路: `json_v0_bridge::maybe_dump_mir` が Program(JSON v0)→MIR 直後に動くので、Span が付いていればそこで観測できる（AST 直通の MirPrinter dump では Span 表示なし）。
