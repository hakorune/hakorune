Normalization Ownership — Rust vs Hakorune

Goal
- Prevent double-normalization and keep a clear Single Source of Truth (SSOT) for where each rewrite lives.

Ownership
- Hakorune layer (Hako scripts)
  - Methodization: Global("Box.m/N") → mir_call(Method) への変換。
  - Name/arity canonicalization（Box.method/N）。
  - Function defs scan/inject（HAKO_STAGEB_FUNC_SCAN, HAKO_MIR_BUILDER_FUNCS）。
  - Emit JSON v1 + unified mir_call（NYASH_JSON_SCHEMA_V1=1, NYASH_MIR_UNIFIED_CALL=1）。
  - 可視化タグ/診断の出力（dev のみ）

- Rust layer
  - Structural/correctness: SSA/PHI、受信側ローカライズ（LocalSSA/Copy/pin）。
  - Legacy JSON v0 → minimal bridging（json_v0_bridge 内での Callee 補完など）。
  - 互換/安全弁: 未定義受信の構造的回復（同一BB直近 NewBox）など、dev ガード付きの最小範囲。
  - Optimizer は構造・副作用ベースの最適化に限定（意味論の再書換えはしない）。
  - Global 呼び出し名の canonical 化（例: `"Box.method"` → `"Box.method/N"`）は NamingBox を通じて行い、VM/LLVM/Interpreter は arity 付き名を SSOT として扱う。

Guards and Toggles
- Hako（dev 推奨セット）
  - HAKO_STAGEB_FUNC_SCAN=1
  - HAKO_MIR_BUILDER_FUNCS=1
  - HAKO_MIR_BUILDER_CALL_RESOLVE=1
  - HAKO_MIR_BUILDER_METHODIZE=1（methodize が v1+unified 出力へ寄与）
  - NYASH_JSON_SCHEMA_V1=1, NYASH_MIR_UNIFIED_CALL=1

- Rust（bridge/診断）
  - HAKO_BRIDGE_METHODIZE=1 は bring-up 用の補助。Hako 既定化後は OFF（撤去予定）。
  - mir_plugin_invoke/plugin_only は A/B 評価・診断用途。既定 OFF。

Rules of Engagement
- v1 + unified を Hako で生成した場合、Rust 側での methodize/再書換えは行わない（構造のみ）。
- json_v0_bridge は v0 入力に対する互換のために限定運用。v1 既定化が進めば縮退する。
- dev の安全弁（未定義受信の構造回復など）は、テストが十分になり次第 OFF/撤去する。

Testing
- Canary
  - tools/dev/phase217_methodize_canary.sh（rc=5）
  - tools/dev/phase217_methodize_json_canary.sh（schema_version + mir_call present、Method優先）
  - tools/dev/phase216_chain_canary_call.sh（rc=5）
- 失敗時は Hako 側（methodize）→ Rust 側（構造） の順で原因を特定する。
