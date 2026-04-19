## next fixed order (resume point)

1. `shadow_adopt` 縮退（step-2）:
   - minimal cluster は撤去済み（`fd26729ff`）、nested minimal の registry route 化も完了（`66ddbce40`）。
   - `generic_loop_body/helpers.rs` の nested fallback は `shadow_adopt` 呼び出しを外し、`nested_loop_minimal` 明示 compose + `strict_nested_loop_guard` 直評価へ移行済み（trace path: `recipe_nested_loop_minimal` / `nested_loop_guard_error`）。
   - router 側の pre-plan `shadow_adopt` 採用 lane は撤去済み。`shadow_pre_plan_guard_error`（guard-only）へ縮退し、entry 例外導線を最小化した。
   - active SSOT/docs（entry_route / exception inventory）は guard-only 契約へ同期済み。
2. Surface/trace の semantic 語彙統一（step-1 継続）:
   - `joinir/routing.rs` / `joinir/trace.rs` / `parity_checker.rs` の主語外しは実施済み（`c76bb7884`, `51234e1b6`）。
   - `parity_checker` の parity mismatch 文言と unit test 期待は route semantic label 主語へ同期済み（`6a166e62a`）。
   - `LoopRouteContext` rename sweep は src 側完了（`30c94f450`, `c5ca36791`, `0738b745b`）。`LoopPatternContext` alias は撤去済み。
   - 残りは補助ログの route/rule 主語統一（必要最小限）。
   - 既存 gate sentinel は維持しつつ label を route/rule 主語へ段階移行。
3. `phase29bq_fast_gate_vm --only bq` と `phase29x-probe` を各 cleanup で継続し、`unexpected_emit_fail=0` / `route_blocker=0` を維持。
4. `DomainPlan` 縮退（step-3）: 1-variant 現状を label-only 化し、normalizer 直通依存を段階撤去。
   - `single_planner` / router / nested-loop helper の tuple API は撤去完了（`07c72a9e5`）。
   - `DomainPlanKind` 撤去（`1e70bf85e`）と `DomainPlan` 単一payload alias 化（`22e5d69cf`）まで完了。
   - planner candidate 経路の 1-variant 縮退も完了（`0df74eaa5`）、関連SSOT語彙同期も完了（`53d59a7f0`）。
   - `DomainPlan` alias 撤去（`fa1efcb21`）と `src/mir/builder/control_flow/{plan,joinir}` 内の残語彙撤去（`27cbe50d2`, `5af900fe3`）まで完了。
   - `normalizer` の pattern minimal helper 再公開は撤去済み（`e90d5074a`）、composer facade 隔離（`809088903`）まで完了。
   - `normalizer` 側窓口は撤去済み、pattern minimal は composer 側へ集約後に folderごと撤去完了（`96591f62b`, `ea8ffeab3`, `fd26729ff`）。
   - planner/normalizer 側の payload-era dead comments は同期済み（`adc600d92`）。
   - test-only wiring の payload 語彙縮退を開始（`5ea18acd6`）。
   - 次は test-only wiring（payload 前提コメント/配線）の残りを段階撤去する。
5. 進捗ログの時系列は archive 側へ寄せ、root pointer は fixed order と blocker だけを更新。
