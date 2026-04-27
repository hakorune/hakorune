//! Phase 273 P3: PlanLowerer - CorePlan → MIR 生成 (SSOT)
//!
//! # Orchestrator Architecture (Phase 29bq+ Modularization)
//!
//! This module is the orchestrator for 6 specialized lowering modules:
//!
//! - **core**: Public API (lower()) and dispatcher (lower_with_stack())
//! - **exit_lowering**: Return/Break/Continue → MIR (~180 lines)
//! - **effect_emission**: CoreEffectPlan → MIR (11 variants, ~160 lines)
//! - **body_processing**: Loop body effects with control flow (~240 lines)
//! - **plan_lowering**: Seq/If/BranchN lowering (~160 lines)
//! - **loop_lowering**: 8-step loop pipeline (~300 lines, CRITICAL)
//!
//! # Responsibilities
//!
//! - Receive CorePlan from PlanNormalizer
//! - Emit MIR instructions using pre-allocated ValueIds
//! - No route-specific knowledge (route-agnostic)
//!
//! # Key Design Decision
//!
//! Lowerer processes CorePlan ONLY. It does not know about scan, split, or
//! any other route-specific semantics. All route knowledge is in Normalizer.
//!
//! # Phase 273 P3: SSOT Finalization
//!
//! - Generalized fields (block_effects/phis/frag/final_values) are now REQUIRED
//! - Legacy fallback has been removed (Fail-Fast on missing fields)
//! - Route-specific emission functions (emit_scan_with_init_edgecfg) no longer used
//!
//! # Module Dependencies
//!
//! ```text
//! core (dispatcher)
//!   ├─→ exit_lowering (no dependencies)
//!   ├─→ effect_emission (uses exit_lowering)
//!   ├─→ body_processing (uses effect_emission, exit_lowering, core::lower_with_stack)
//!   ├─→ plan_lowering (uses core::lower_with_stack)
//!   └─→ loop_lowering (uses body_processing, effect_emission)
//! ```

mod block_effect_emission;
mod body_processing;
mod core;
mod debug_ctx;
mod debug_tags;
mod effect_emission;
mod exit_lowering;
mod loop_completion;
mod loop_lowering;
mod loop_preparation;
mod loop_validation;
mod phi_processing;
mod plan_lowering;
mod span_fmt;

// Re-export LoopFrame for internal use
pub(super) use core::LoopFrame;

/// Phase 273 P1: PlanLowerer - CorePlan → MIR 生成 (SSOT)
///
/// All implementation is delegated to specialized modules.
/// See module documentation for orchestrator architecture.
pub(in crate::mir::builder) struct PlanLowerer;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::edgecfg::api::Frag;
    use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
    use crate::mir::builder::control_flow::plan::branchn::{CoreBranchArmPlan, CoreBranchNPlan};
    use crate::mir::builder::control_flow::plan::step_mode::extract_to_step_bb_explicit_step;
    use crate::mir::builder::control_flow::plan::{
        CoreEffectPlan, CoreExitPlan, CoreIfPlan, CoreLoopPlan, CorePlan,
    };
    use crate::mir::builder::MirBuilder;
    use crate::mir::{ConstValue, MirInstruction};

    fn make_ctx<'a>(condition: &'a ASTNode, body: &'a [ASTNode]) -> LoopRouteContext<'a> {
        LoopRouteContext::new(condition, body, "test_coreplan", false, false)
    }

    #[test]
    fn test_lower_exit_return_sets_terminator() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_exit".to_string());

        let ret_val = builder.alloc_value_for_test();
        builder
            .emit_for_test(MirInstruction::Const {
                dst: ret_val,
                value: ConstValue::Integer(1),
            })
            .expect("emit const");

        let cond = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let body: Vec<ASTNode> = vec![];
        let ctx = make_ctx(&cond, &body);

        let plan = CorePlan::Exit(CoreExitPlan::Return(Some(ret_val)));
        let result = PlanLowerer::lower(&mut builder, plan, &ctx);
        assert!(result.is_ok());

        let entry = builder.current_block_for_test().expect("entry block");
        let func = builder
            .scope_ctx
            .current_function
            .as_ref()
            .expect("function");
        let block = func.get_block(entry).expect("block");
        assert!(
            matches!(block.terminator, Some(MirInstruction::Return { value: Some(v) }) if v == ret_val),
            "expected Return terminator"
        );
    }

    #[test]
    fn test_lower_if_emits_branch() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_if".to_string());

        let entry = builder.current_block_for_test().expect("entry block");
        let cond_val = builder.alloc_value_for_test();
        builder
            .emit_for_test(MirInstruction::Const {
                dst: cond_val,
                value: ConstValue::Bool(true),
            })
            .expect("emit const");

        let then_val = builder.alloc_value_for_test();
        let if_plan = CoreIfPlan {
            condition: cond_val,
            then_plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                dst: then_val,
                value: ConstValue::Integer(2),
            })],
            else_plans: None,
            joins: Vec::new(),
        };

        let cond = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let body: Vec<ASTNode> = vec![];
        let ctx = make_ctx(&cond, &body);

        let result = PlanLowerer::lower(&mut builder, CorePlan::If(if_plan), &ctx);
        assert!(result.is_ok());

        let func = builder
            .scope_ctx
            .current_function
            .as_ref()
            .expect("function");
        let block = func.get_block(entry).expect("entry block");
        assert!(
            matches!(block.terminator, Some(MirInstruction::Branch { .. })),
            "expected Branch terminator"
        );
    }

    #[test]
    fn test_lower_branchn_does_not_fail() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_branchn".to_string());

        let cond1 = builder.alloc_value_for_test();
        builder
            .emit_for_test(MirInstruction::Const {
                dst: cond1,
                value: ConstValue::Bool(true),
            })
            .expect("emit cond1");

        let cond2 = builder.alloc_value_for_test();
        builder
            .emit_for_test(MirInstruction::Const {
                dst: cond2,
                value: ConstValue::Bool(false),
            })
            .expect("emit cond2");

        let arm1_val = builder.alloc_value_for_test();
        let arm2_val = builder.alloc_value_for_test();

        let branch_plan = CoreBranchNPlan {
            arms: vec![
                CoreBranchArmPlan {
                    condition: cond1,
                    plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                        dst: arm1_val,
                        value: ConstValue::Integer(1),
                    })],
                },
                CoreBranchArmPlan {
                    condition: cond2,
                    plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                        dst: arm2_val,
                        value: ConstValue::Integer(2),
                    })],
                },
            ],
            else_plans: None,
        };

        let cond = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let body: Vec<ASTNode> = vec![];
        let ctx = make_ctx(&cond, &body);

        let result = PlanLowerer::lower(&mut builder, CorePlan::BranchN(branch_plan), &ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_loop_body_seq_flattens() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_loop_body_seq".to_string());

        let preheader_bb = builder.next_block_id();
        let header_bb = builder.next_block_id();
        let body_bb = builder.next_block_id();
        let step_bb = builder.next_block_id();
        let after_bb = builder.next_block_id();
        let found_bb = builder.next_block_id();

        let eff1 = CoreEffectPlan::Const {
            dst: builder.alloc_value_for_test(),
            value: ConstValue::Integer(1),
        };
        let eff2 = CoreEffectPlan::Const {
            dst: builder.alloc_value_for_test(),
            value: ConstValue::Integer(2),
        };
        let eff3 = CoreEffectPlan::Const {
            dst: builder.alloc_value_for_test(),
            value: ConstValue::Integer(3),
        };
        let (step_mode, has_explicit_step) = extract_to_step_bb_explicit_step();

        let loop_plan = CoreLoopPlan {
            preheader_bb,
            preheader_is_fresh: false,
            header_bb,
            body_bb,
            step_bb,
            continue_target: step_bb,
            after_bb,
            found_bb,
            body: vec![CorePlan::Seq(vec![
                CorePlan::Effect(eff1),
                CorePlan::Seq(vec![CorePlan::Effect(eff2)]),
                CorePlan::Effect(eff3),
            ])],
            cond_loop: builder.alloc_value_for_test(),
            cond_match: builder.alloc_value_for_test(),
            block_effects: vec![
                (preheader_bb, vec![]),
                (header_bb, vec![]),
                (body_bb, vec![]),
                (step_bb, vec![]),
            ],
            phis: vec![],
            frag: Frag::new(header_bb),
            final_values: vec![],
            step_mode,
            has_explicit_step,
        };

        let cond = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let body: Vec<ASTNode> = vec![];
        let ctx = make_ctx(&cond, &body);

        let result = PlanLowerer::lower(&mut builder, CorePlan::Loop(loop_plan), &ctx);
        assert!(result.is_ok());

        let func = builder
            .scope_ctx
            .current_function
            .as_ref()
            .expect("function");
        let block = func.get_block(body_bb).expect("body block");
        let const_count = block
            .instructions
            .iter()
            .filter(|inst| matches!(inst, MirInstruction::Const { .. }))
            .count();
        assert_eq!(const_count, 3, "expected 3 Const effects in body");
    }

    #[test]
    fn test_lower_loop_body_if_effect_ok() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("test_loop_body_if_effect".to_string());

        let cond_val = builder.alloc_value_for_test();
        builder
            .emit_for_test(MirInstruction::Const {
                dst: cond_val,
                value: ConstValue::Bool(true),
            })
            .expect("emit cond");

        let effect = CoreEffectPlan::IfEffect {
            cond: cond_val,
            then_effects: vec![CoreEffectPlan::Const {
                dst: builder.alloc_value_for_test(),
                value: ConstValue::Integer(1),
            }],
            else_effects: None,
        };

        let preheader_bb = builder.next_block_id();
        let header_bb = builder.next_block_id();
        let body_bb = builder.next_block_id();
        let step_bb = builder.next_block_id();
        let after_bb = builder.next_block_id();
        let found_bb = builder.next_block_id();
        let (step_mode, has_explicit_step) = extract_to_step_bb_explicit_step();

        let loop_plan = CoreLoopPlan {
            preheader_bb,
            preheader_is_fresh: false,
            header_bb,
            body_bb,
            step_bb,
            continue_target: step_bb,
            after_bb,
            found_bb,
            body: vec![CorePlan::Effect(effect)],
            cond_loop: builder.alloc_value_for_test(),
            cond_match: builder.alloc_value_for_test(),
            block_effects: vec![
                (preheader_bb, vec![]),
                (header_bb, vec![]),
                (body_bb, vec![]),
                (step_bb, vec![]),
            ],
            phis: vec![],
            frag: Frag::new(header_bb),
            final_values: vec![],
            step_mode,
            has_explicit_step,
        };

        let cond = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let body: Vec<ASTNode> = vec![];
        let ctx = make_ctx(&cond, &body);

        let result = PlanLowerer::lower(&mut builder, CorePlan::Loop(loop_plan), &ctx);
        assert!(result.is_ok());
    }
}
