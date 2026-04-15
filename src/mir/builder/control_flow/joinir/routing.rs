//! JoinIR routing logic for loop lowering

use super::trace;
use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::cell::RefCell;

thread_local! {
    static STEP_TREE_PARITY_ERROR: RefCell<Option<String>> = RefCell::new(None);
}

fn clear_step_tree_parity_error() {
    STEP_TREE_PARITY_ERROR.with(|slot| {
        *slot.borrow_mut() = None;
    });
}

fn set_step_tree_parity_error(message: String) {
    STEP_TREE_PARITY_ERROR.with(|slot| {
        *slot.borrow_mut() = Some(message);
    });
}

fn take_step_tree_parity_error() -> Option<String> {
    STEP_TREE_PARITY_ERROR.with(|slot| slot.borrow_mut().take())
}

/// Loop route classification の SSOT 入口
///
/// 既存の分散した選択ロジックをここに集約する。
/// 将来的には Canonicalizer decision に委譲する。
///
/// Phase 137-6-S1: 現時点では既存の router ロジック（LoopFeatures ベース）を使用
/// Phase 137-6-S2: dev-only で canonicalizer decision を提案として受け取る
pub(in crate::mir::builder) fn choose_route_kind(
    condition: &ASTNode,
    body: &[ASTNode],
) -> crate::mir::loop_route_detection::LoopRouteKind {
    use crate::mir::builder::control_flow::cleanup::policies::balanced_depth_scan_policy_box::BalancedDepthScanPolicyBox;
    use crate::mir::builder::control_flow::cleanup::policies::PolicyDecision;
    use crate::mir::builder::control_flow::facts::ast_feature_extractor as ast_features;
    use crate::mir::loop_route_detection;

    clear_step_tree_parity_error();

    // Phase 107: Route balanced depth-scan (return-in-loop) to loop-break recipe via policy.
    //
    // This keeps loop routing structural: no by-name dispatch, no silent fallback.
    match BalancedDepthScanPolicyBox::decide(condition, body) {
        PolicyDecision::Use(_) => {
            return loop_route_detection::LoopRouteKind::LoopBreak;
        }
        PolicyDecision::Reject(_reason) => {
            // In strict mode, treat "close-but-unsupported" as a fail-fast
            // loop-break route so the policy can surface the precise contract violation.
            if crate::config::env::joinir_dev::strict_enabled() {
                return loop_route_detection::LoopRouteKind::LoopBreak;
            }
        }
        PolicyDecision::None => {}
    }

    // Phase 193: Use AST Feature Extractor Box for break/continue detection
    let has_continue = ast_features::detect_continue_in_body(body);
    let has_break = ast_features::detect_break_in_body(body);
    let has_return = ast_features::detect_return_in_body(body);

    // Phase 188.3: Nested-loop minimal selection for 1-level nested loops.
    // SSOT: All nested-loop minimal detection happens here (no dev dependency).
    //
    // Strategy: Cheap check → StepTree → Full AST validation
    // Only select nested-loop minimal if lowering is guaranteed to work.

    // Step 1: Cheap check - does body contain any Loop node?
    let has_inner_loop = body.iter().any(|stmt| matches!(stmt, ASTNode::Loop { .. }));

    if has_inner_loop {
        // Step 2: Build StepTree to get nesting depth (cost: acceptable for nested loops only)
        use crate::ast::Span;
        use crate::mir::control_tree::StepTreeBuilderBox;

        let loop_ast = ASTNode::Loop {
            condition: Box::new(condition.clone()),
            body: body.to_vec(),
            span: Span::unknown(),
        };

        let tree = StepTreeBuilderBox::build_from_ast(&loop_ast);

        // Step 3: Check if exactly 1-level nesting (depth == 2)
        if tree.features.max_loop_depth == 2 {
            // Step 4: Full AST validation (simple-loop-compatible requirements)
            if is_nested_loop_minimal_lowerable(&tree, body) {
                // nested_loop_minimal selected - lowering MUST succeed
                trace::trace().dev(
                    "choose_route_kind",
                    "[routing] nested_loop_minimal selected: 1-level nested loop validated",
                );
                return loop_route_detection::LoopRouteKind::NestedLoopMinimal;
            }
            // Validation failed - not nested_loop_minimal, fall through to router_choice
        }
    }

    // Phase 110: StepTree parity check (structure-only SSOT).
    //
    // This is dev-only; strict mode turns mismatch into a fail-fast.
    if crate::config::env::joinir_dev_enabled() {
        use crate::ast::Span;
        use crate::mir::control_tree::StepTreeBuilderBox;

        let loop_ast = ASTNode::Loop {
            condition: Box::new(condition.clone()),
            body: body.to_vec(),
            span: Span::unknown(),
        };

        let tree = StepTreeBuilderBox::build_from_ast(&loop_ast);
        if tree.features.max_loop_depth <= 1
            && (tree.features.has_break != has_break
                || tree.features.has_continue != has_continue
                || tree.features.has_return != has_return)
        {
            let detail = ast_features::find_first_control_flow_stmt(body)
                .map(|(idx, kind)| format!("root=loop_body idx={} kind={}", idx, kind))
                .unwrap_or_else(|| "root=loop_body idx=-1 kind=none".to_string());
            let msg = format!(
                "[choose_route_kind/STEPTREE_PARITY] step_tree(break={}, cont={}, ret={}) != extractor(break={}, cont={}, ret={}) {}",
                tree.features.has_break,
                tree.features.has_continue,
                tree.features.has_return,
                has_break,
                has_continue,
                has_return,
                detail
            );

            if crate::config::env::joinir_dev::strict_enabled() {
                set_step_tree_parity_error(msg.clone());
            }
            trace::trace().dev("choose_route_kind/step_tree_parity", &msg);
        }
    }

    // Phase 193: Extract features using modularized extractor
    let features = ast_features::extract_features(condition, body, has_continue, has_break);

    // Phase 192: Classify route kind based on features (既存の router 結果)
    let router_choice = loop_route_detection::classify(&features);

    // Phase 137-6-S2: dev-only で Canonicalizer の提案を取得
    if crate::config::env::joinir_dev_enabled() {
        use crate::ast::Span;
        use crate::mir::loop_canonicalizer::canonicalize_loop_expr;

        let loop_ast = ASTNode::Loop {
            condition: Box::new(condition.clone()),
            body: body.to_vec(),
            span: Span::unknown(),
        };

        if let Ok((_skeleton, decision)) = canonicalize_loop_expr(&loop_ast) {
            if let Some(canonical_choice) = decision.chosen {
                // parity check
                if canonical_choice != router_choice {
                    let msg = format!(
                        "[choose_route_kind/PARITY] router={}, canonicalizer={}",
                        router_choice.semantic_label(),
                        canonical_choice.semantic_label()
                    );

                    if crate::config::env::joinir_dev::strict_enabled() {
                        // strict mode: 不一致は Fail-Fast
                        panic!("{}", msg);
                    } else {
                        // debug mode: ログのみ
                        trace::trace().dev("choose_route_kind/parity", &msg);
                    }
                } else {
                    // Route kinds match - success!
                    trace::trace().dev(
                        "choose_route_kind/parity",
                        &format!(
                            "[choose_route_kind/PARITY] OK: canonical and actual agree on {}",
                            canonical_choice.semantic_label()
                        ),
                    );
                }

                // TODO (Phase 137-6-S3): ここで canonical_choice を返す
                // 現時点では router_choice を維持（既定挙動不変）
                //
                // 有効化条件（将来実装）:
                // 1. joinir_dev_enabled() && 新フラグ（例: canonicalizer_preferred()）
                // 2. または joinir_dev_enabled() をそのまま使用
                //
                // 注意: 有効化時は全 route kind の parity が green であること
                //
                // 有効化後のコード例:
                // ```rust
                // if crate::config::env::canonicalizer_preferred() {
                //     return canonical_choice;
                // }
                // ```
            }
        }
    }

    router_choice
}

/// Phase 188.3: Validate nested loop meets ALL nested-loop-minimal requirements
///
/// Returns true ONLY if nested-loop-minimal lowering is guaranteed to succeed.
/// False → fall through to other routes (NOT an error)
fn is_nested_loop_minimal_lowerable(
    tree: &crate::mir::control_tree::StepTree,
    body: &[crate::ast::ASTNode],
) -> bool {
    use crate::ast::ASTNode;

    // Requirement 1: Outer loop has no break (simple-loop-compatible requirement)
    if tree.features.has_break {
        return false;
    }

    // Requirement 2: Outer loop has no continue (simple-loop-compatible requirement)
    if tree.features.has_continue {
        return false;
    }

    // Requirement 3: Extract inner loop(s) - must have exactly 1
    let mut inner_loop: Option<&ASTNode> = None;
    for stmt in body.iter() {
        if matches!(stmt, ASTNode::Loop { .. }) {
            if inner_loop.is_some() {
                // Multiple inner loops - not supported
                return false;
            }
            inner_loop = Some(stmt);
        }
    }

    let inner_loop = match inner_loop {
        Some(l) => l,
        None => return false, // No inner loop found (shouldn't happen, but defensive)
    };

    // Requirement 4: Inner loop has no break (simple-loop-compatible requirement)
    use crate::mir::control_tree::StepTreeBuilderBox;
    let inner_tree = StepTreeBuilderBox::build_from_ast(inner_loop);
    if inner_tree.features.has_break {
        return false;
    }

    // Requirement 5: Inner loop has no continue (simple-loop-compatible requirement)
    if inner_tree.features.has_continue {
        return false;
    }

    // All requirements met - nested-loop-minimal lowering will succeed
    true
}

impl MirBuilder {
    /// Phase 49: Try JoinIR Frontend for mainline integration
    ///
    /// Returns `Ok(Some(value))` if the loop is successfully lowered via JoinIR,
    /// `Ok(None)` if no JoinIR route matched (unsupported loop structure).
    /// Phase 187-2: Legacy LoopBuilder removed - all loops must use JoinIR.
    ///
    /// # Phase 49-4: Multi-target support
    ///
    /// Targets are enabled via separate dev flags:
    /// - `HAKO_JOINIR_PRINT_TOKENS_MAIN=1`: JsonTokenizer.print_tokens/0
    /// - `HAKO_JOINIR_ARRAY_FILTER_MAIN=1`: ArrayExtBox.filter/2
    ///
    /// Note: Arity in function names does NOT include implicit `me` receiver.
    /// - Instance method `print_tokens()` → `/0` (no explicit params)
    /// - Static method `filter(arr, pred)` → `/2` (two params)
    pub(in crate::mir::builder) fn try_cf_loop_joinir(
        &mut self,
        condition: &ASTNode,
        body: &[ASTNode],
    ) -> Result<Option<ValueId>, String> {
        // Get current function name
        let func_name = self
            .scope_ctx
            .current_function
            .as_ref()
            .map(|f| f.signature.name.clone())
            .unwrap_or_default();

        // Phase 195: Use unified trace
        trace::trace().routing("router", &func_name, "try_cf_loop_joinir called");

        // Phase 170-4: Structure-based routing option
        // When NYASH_JOINIR_STRUCTURE_ONLY=1, skip function name whitelist
        // and route purely based on loop structure analysis
        // Phase 196: Default to structure-first routing now that LoopBuilder is removed.
        // - Default: ON (structure_only = true) to allow JoinIR routes to run for all loops.
        // - To revert to the previous whitelist-only behavior, set NYASH_JOINIR_STRUCTURE_ONLY=0.
        let structure_only = crate::config::env::joinir_structure_only_enabled();

        if structure_only {
            trace::trace().routing(
                "router",
                &func_name,
                "Structure-only mode enabled, skipping whitelist",
            );
        } else {
            // Phase 49-4 + Phase 80: Multi-target routing (legacy whitelist)
            // - JoinIR は常時 ON。legacy LoopBuilder は削除済み。
            // - 代表2本（print_tokens / ArrayExt.filter）も常に JoinIR で試行する。
            // Note: Arity does NOT include implicit `me` receiver
            // Phase 188: Add "main" routing for loop route expansion
            // Phase 170: Add JsonParserBox methods for selfhost validation
            let is_target = match func_name.as_str() {
                "main" => true,             // Phase 188-Impl-1: Enable JoinIR for main function
                "JoinIrMin.main/0" => true, // Phase 188-Impl-2: Enable JoinIR for JoinIrMin.main/0
                "JsonTokenizer.print_tokens/0" => true,
                "ArrayExtBox.filter/2" => true,
                // Phase 170-A-1: Enable JsonParserBox methods for JoinIR routing
                "JsonParserBox._trim/1" => true,
                "JsonParserBox._skip_whitespace/2" => true,
                "JsonParserBox._match_literal/3" => true, // Phase 182: Fixed arity (s, pos, literal)
                "JsonParserBox._parse_string/2" => true,
                "JsonParserBox._parse_array/2" => true,
                "JsonParserBox._parse_object/2" => true,
                // Phase 182: Add simple loop methods
                "JsonParserBox._parse_number/2" => true, // P2 Break (s, pos)
                "JsonParserBox._atoi/1" => true,         // P2 Break (s)
                // Phase 170-A-1: Test methods (simplified versions)
                "TrimTest.trim/1" => true,
                "Main.trim/1" => true, // Phase 171-fix: Main box variant
                "Main.trim_string_simple/1" => true, // Phase 33-13: Simple trim variant
                "TrimTest.main/0" => true, // Phase 170: TrimTest.main for loop route test
                // Phase 173: JsonParser P5 expansion test
                "JsonParserTest._skip_whitespace/3" => true,
                "JsonParserTest.main/0" => true,
                // Phase 174: JsonParser complex loop P5B extension test
                "JsonParserStringTest.parse_string_min/0" => true,
                "JsonParserStringTest.main/0" => true,
                // Phase 175: P5 multi-carrier support (2 carriers: pos + result)
                "JsonParserStringTest2.parse_string_min2/0" => true,
                "JsonParserStringTest2.main/0" => true,
                _ => false,
            };

            if !is_target {
                crate::mir::builder::control_flow::facts::reject_reason::set_last_plan_reject_detail_if_absent(
                    format!(
                        "whitelist_miss func={} structure_only=false (set NYASH_JOINIR_STRUCTURE_ONLY=1 to use structure routing)",
                        func_name
                    ),
                );
                return Ok(None);
            }
        }

        // Debug log when routing through JoinIR Frontend
        // Phase 195: Check trace flags directly from JoinLoopTrace
        let debug = trace::trace().is_loopform_enabled() || trace::trace().is_mainline_enabled();
        trace::trace().routing(
            "router",
            &func_name,
            "Routing through JoinIR Frontend mainline",
        );

        // Phase 49-3: Implement JoinIR Frontend integration
        self.cf_loop_joinir_impl(condition, body, &func_name, debug)
    }

    /// Phase 49-3: JoinIR Frontend integration implementation
    ///
    /// Routes loop compilation through either:
    /// 1. Normalized shadow (Phase 131 P1) - dev-only for loop(true) break-once
    /// 2. Recipe-first loop router (Phase 194+) - preferred path
    pub(in crate::mir::builder) fn cf_loop_joinir_impl(
        &mut self,
        condition: &ASTNode,
        body: &[ASTNode],
        func_name: &str,
        debug: bool,
    ) -> Result<Option<ValueId>, String> {
        // Phase 131 P1: Try Normalized shadow first (dev-only)
        if crate::config::env::joinir_dev_enabled() {
            if let Some(result) = self.try_normalized_shadow(condition, body, func_name, debug)? {
                return Ok(Some(result));
            }
        }

        // Phase 137-2/137-4: Dev-only observation via Loop Canonicalizer
        if crate::config::env::joinir_dev_enabled() {
            use crate::ast::Span;
            use crate::mir::loop_canonicalizer::canonicalize_loop_expr;

            // Reconstruct loop AST for canonicalizer
            let loop_ast = ASTNode::Loop {
                condition: Box::new(condition.clone()),
                body: body.to_vec(),
                span: Span::unknown(),
            };

            match canonicalize_loop_expr(&loop_ast) {
                Ok((skeleton, decision)) => {
                    trace::trace().dev("loop_canonicalizer", &format!("Function: {}", func_name));
                    trace::trace().dev(
                        "loop_canonicalizer",
                        &format!("  Skeleton steps: {}", skeleton.steps.len()),
                    );
                    trace::trace().dev(
                        "loop_canonicalizer",
                        &format!("  Carriers: {}", skeleton.carriers.len()),
                    );
                    trace::trace().dev(
                        "loop_canonicalizer",
                        &format!("  Has exits: {}", skeleton.exits.has_any_exit()),
                    );
                    trace::trace().dev(
                        "loop_canonicalizer",
                        &format!(
                            "  Decision: {}",
                            if decision.is_success() {
                                "SUCCESS"
                            } else {
                                "FAIL_FAST"
                            }
                        ),
                    );
                    if let Some(route_kind) = decision.chosen {
                        trace::trace().dev(
                            "loop_canonicalizer",
                            &format!("  Chosen route kind: {}", route_kind.semantic_label()),
                        );
                    }
                    trace::trace().dev(
                        "loop_canonicalizer",
                        &format!("  Missing caps: {:?}", decision.missing_caps),
                    );
                    if decision.is_fail_fast() {
                        trace::trace().dev(
                            "loop_canonicalizer",
                            &format!("  Reason: {}", decision.notes.join("; ")),
                        );
                    }

                    // Phase 137-4: Router parity verification
                    if let Some(canonical_route_kind) = decision.chosen {
                        // Get actual route kind from router (determined by LoopRouteContext).
                        // We need to defer this check until after ctx is created
                        // Store decision for later parity check
                        trace::trace().debug(
                            "canonicalizer",
                            &format!(
                                "Phase 137-4: Canonical route kind chosen: {} (parity check pending)",
                                canonical_route_kind.semantic_label()
                            ),
                        );
                    }
                }
                Err(e) => {
                    trace::trace().dev("loop_canonicalizer", &format!("Function: {}", func_name));
                    trace::trace().dev("loop_canonicalizer", &format!("  Error: {}", e));
                }
            }
        }

        // Phase 194: Use table-driven router instead of if/else chain
        use super::route_entry::{route_loop, LoopRouteContext};

        // Phase 200-C: Pass fn_body_ast to LoopRouteContext if available
        // Clone fn_body_ast to avoid borrow checker issues
        let fn_body_clone = self.comp_ctx.fn_body_ast.clone();
        trace::trace().routing(
            "router",
            func_name,
            &format!(
                "fn_body_ast is {}",
                if fn_body_clone.is_some() {
                    "SOME"
                } else {
                    "NONE"
                }
            ),
        );
        let in_static_box = self.comp_ctx.current_static_box.is_some();
        let ctx = if let Some(ref fn_body) = fn_body_clone {
            trace::trace().routing(
                "router",
                func_name,
                &format!("Creating ctx with fn_body ({} nodes)", fn_body.len()),
            );
            LoopRouteContext::with_fn_body(
                condition,
                body,
                &func_name,
                debug,
                in_static_box,
                fn_body,
            )
        } else {
            LoopRouteContext::new(condition, body, &func_name, debug, in_static_box)
        };

        if let Some(msg) = take_step_tree_parity_error() {
            use crate::mir::join_ir::lowering::error_tags;
            return Err(error_tags::freeze(&msg));
        }

        // Phase 137-4: Router parity verification (after ctx is created)
        // Phase 92 P1-0: Skeleton setting removed - patterns retrieve skeleton internally if needed
        if crate::config::env::joinir_dev_enabled() {
            let (result, _skeleton_opt) =
                self.verify_router_parity(condition, body, func_name, &ctx);
            result?;
        }

        if let Some(result) = route_loop(self, &ctx)? {
            trace::trace().routing("router", func_name, "Loop router succeeded");
            return Ok(Some(result));
        }

        // Recipe-first/router path produced no route for this loop.
        trace::trace().routing(
            "router",
            func_name,
            "Loop router found no route (no legacy fallback)",
        );
        Ok(None)
    }

    /// Phase 131 P1: Try Normalized shadow lowering (dev-only)
    ///
    /// Returns:
    /// - Ok(Some(value_id)): Successfully lowered and merged via Normalized
    /// - Ok(None): Out of scope (not a Normalized route shape)
    /// - Err(msg): In scope but failed (Fail-Fast in strict mode)
    ///
    /// Phase 134 P0: Unified with NormalizationPlanBox/ExecuteBox
    fn try_normalized_shadow(
        &mut self,
        condition: &ASTNode,
        body: &[ASTNode],
        func_name: &str,
        debug: bool,
    ) -> Result<Option<ValueId>, String> {
        use crate::ast::Span;
        use crate::mir::builder::control_flow::normalization::{
            NormalizationExecuteBox, NormalizationPlanBox, PlanKind,
        };

        // Build loop AST for route-shape detection
        let loop_ast = ASTNode::Loop {
            condition: Box::new(condition.clone()),
            body: body.to_vec(),
            span: Span::unknown(),
        };

        // Phase 134 P0: Delegate route-shape detection to NormalizationPlanBox (SSOT)
        // Convert loop to remaining format (single-element array)
        let remaining = vec![loop_ast];

        let plan =
            match NormalizationPlanBox::plan_block_suffix(self, &remaining, func_name, debug)? {
                Some(plan) => plan,
                None => {
                    if debug {
                        trace::trace().routing(
                            "router/normalized",
                            func_name,
                            "NormalizationPlanBox returned None (not a normalized route shape)",
                        );
                    }
                    return Ok(None);
                }
            };

        // Only handle loop-only route shapes here
        // (post-statement route shapes are now handled at statement level)
        match &plan.kind {
            PlanKind::LoopOnly => {
                if debug {
                    trace::trace().routing(
                        "router/normalized",
                        func_name,
                        "Loop-only route shape detected, proceeding with normalization",
                    );
                }
            }
        }

        // Phase 134 P0: Delegate execution to NormalizationExecuteBox (SSOT)
        // Phase 141 P1.5: Pass prefix_variables (using variable_map at this point)
        // Clone to avoid borrow checker conflict (self is borrowed mutably in execute)
        let prefix_var_map = self.variable_ctx.variable_map.clone();
        match NormalizationExecuteBox::execute(
            self,
            &plan,
            &remaining,
            func_name,
            debug,
            Some(&prefix_var_map),
        ) {
            Ok(value_id) => {
                if debug {
                    trace::trace().routing(
                        "router/normalized",
                        func_name,
                        "Normalization succeeded",
                    );
                }
                Ok(Some(value_id))
            }
            Err(e) => {
                if crate::config::env::joinir_dev::strict_enabled() {
                    use crate::mir::join_ir::lowering::error_tags;
                    return Err(error_tags::freeze_with_hint(
                        "phase134/routing/normalized",
                        &e,
                        "Loop should be supported by Normalized but execution failed. \
                         Check that condition is Bool(true) and body ends with break.",
                    ));
                }
                if crate::config::env::joinir_dev::debug_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[normalization/fallback] func={} reason=execute_error err={}",
                        func_name, e
                    ));
                }
                trace::trace().routing("router/normalized/error", func_name, &e);
                Ok(None) // Non-strict: fallback
            }
        }
    }
}
