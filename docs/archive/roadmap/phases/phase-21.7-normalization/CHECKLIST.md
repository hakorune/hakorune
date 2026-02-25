Phase 21.7 — Normalization Checklist (Methodization)

Targets (must be green)
- Naming: all user functions canonicalized as Box.method/N
- Arity: params.len == call.args.len (or explicit mapping when defs absent)
- Methodization (dev toggle): Global("Box.method") → Method(receiver=static singleton)
- VM: Method calls execute via ensure_static_box_instance for static boxes
- LLVM: mir_call(Method) lowering produces correct IR; rc parity preserved

Canaries
- tools/dev/phase216_chain_canary_call.sh — remains PASS when OFF, PASS when ON
- tools/dev/phase217_methodize_canary.sh (dev) — compile-run rc=5（セマンティクス）
- tools/dev/phase217_methodize_json_canary.sh (dev) — v1 root（schema_version）+ mir_call present（Methodが望ましい、Globalは経過容認）

Toggles
- HAKO_MIR_BUILDER_METHODIZE=1 (new)
- HAKO_STAGEB_FUNC_SCAN=1 / HAKO_MIR_BUILDER_FUNCS=1 / HAKO_MIR_BUILDER_CALL_RESOLVE=1 (existing)
- 一軍（devプロファイルで既定ON）: 上記3つ + NYASH_JSON_SCHEMA_V1=1 + NYASH_MIR_UNIFIED_CALL=1
- 診断（既定OFF）: HAKO_BRIDGE_METHODIZE=1（core_bridge側の補助。Hako既定化後に撤去）

Rollback
- Disable HAKO_MIR_BUILDER_METHODIZE; remove methodization rewrite; keep Global path active.
 - core_bridgeの methodize ブリッジは Hako側が既定化され次第、撤去（タグ: [bridge/methodize:*] を一時観測可能にして差分検知）
