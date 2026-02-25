//! Pattern 2 Normalization - Loops with Carriers
//!
//! Normalizes Pattern 2 JoinIR to Normalized form.
//! Pattern 2 extends Pattern 1 with carriers (accumulator variables) and break.
//!
//! ## Responsibilities
//! - Convert Structured JoinIR → Normalized JoinIR (Pattern 2/3/4)
//! - Convert Normalized JoinIR → Structured JoinIR (reverse)
//! - Verify normalized module invariants

use std::collections::BTreeMap;

use crate::mir::join_ir::{
    JoinContId, JoinFuncId, JoinFunction, JoinInst, JoinIrPhase,
    JoinModule, MirLikeInst,
};
use crate::mir::ValueId;

use super::{EnvField, EnvLayout, JpFuncId, JpFunction, JpInst, JpOp, NormalizedModule};

#[cfg(feature = "normalized_dev")]
use crate::mir::join_ir::normalized::shape_guard::NormalizedDevShape;

/// Pattern 2 Normalizer
///
/// Normalizes Pattern 2/3/4 (loops with carriers) to Normalized form.
///
/// Pattern 2 constraints:
/// - Loops with carriers (accumulator variables like sum, count)
/// - Break support (via k_exit)
/// - Main/loop_step/k_exit function structure
/// - Env layout for parameters
pub struct Pattern2Normalizer;

impl Pattern2Normalizer {
    /// Normalize Pattern 2 Structured JoinIR to Normalized form
    ///
    /// # Arguments
    /// * `structured` - Structured JoinIR module (must be Pattern 2/3/4)
    ///
    /// # Returns
    /// Normalized module with backup
    ///
    /// # Panics
    /// - If structured.phase is not Structured
    /// - If loop_step function is not found
    /// - If function count is not 3
    /// - If param count is out of range
    /// - If no conditional jump or tail call found
    pub fn normalize(structured: &JoinModule) -> NormalizedModule {
        assert!(
            structured.is_structured(),
            "normalize_pattern2_minimal: expected Structured JoinIR"
        );

        // Minimal guardrail: Pattern2 mini should have main/loop_step/k_exit only, with 1 loop param.
        let func_count = structured.functions.len();
        let loop_func = structured
            .functions
            .values()
            .find(|f| f.name == "loop_step")
            .or_else(|| structured.functions.get(&JoinFuncId::new(1)))
            .expect("normalize_pattern2_minimal: loop_step not found");

        assert!(
            func_count == 3,
            "normalize_pattern2_minimal: expected 3 functions (entry/loop_step/k_exit) but got {}",
            func_count
        );
        let param_max = {
            #[allow(unused_mut)]
            let mut max = 3;
            #[cfg(feature = "normalized_dev")]
            {
                use crate::mir::join_ir::normalized::shape_guard;
                let shapes = shape_guard::supported_shapes(structured);
                if shapes
                    .iter()
                    .any(|s| matches!(s, NormalizedDevShape::JsonparserAtoiMini))
                {
                    max = max.max(8);
                }
                if shapes
                    .iter()
                    .any(|s| matches!(s, NormalizedDevShape::JsonparserAtoiReal))
                {
                    max = max.max(10);
                }
                if shapes.iter().any(|s| {
                    matches!(
                        s,
                        NormalizedDevShape::JsonparserParseNumberReal
                    )
                }) {
                    max = max.max(12);
                }
                if shapes.iter().any(|s| {
                    matches!(
                        s,
                        NormalizedDevShape::JsonparserSkipWsReal
                    )
                }) {
                    max = max.max(6);
                }
                if shapes.iter().any(|s| {
                    matches!(
                        s,
                        NormalizedDevShape::Pattern4ContinueMinimal
                            | NormalizedDevShape::JsonparserParseArrayContinueSkipWs
                            | NormalizedDevShape::JsonparserParseObjectContinueSkipWs
                    )
                }) {
                    max = max.max(6);
                }
                if shapes.iter().any(|s| {
                    matches!(
                        s,
                        NormalizedDevShape::Pattern3IfSumMinimal
                            | NormalizedDevShape::Pattern3IfSumMulti
                            | NormalizedDevShape::Pattern3IfSumJson
                            | NormalizedDevShape::SelfhostIfSumP3
                            | NormalizedDevShape::SelfhostIfSumP3Ext
                            | NormalizedDevShape::SelfhostStmtCountP3
                            | NormalizedDevShape::SelfhostDetectFormatP3
                    )
                }) {
                    max = max.max(6);
                }
            }
            max
        };
        assert!(
            (1..=param_max).contains(&loop_func.params.len()),
            "normalize_pattern2_minimal: expected 1..={} params (loop var + carriers + optional host)",
            param_max
        );

        let jump_conds = loop_func
            .body
            .iter()
            .filter(|inst| matches!(inst, JoinInst::Jump { cond: Some(_), .. }))
            .count();
        let tail_calls = loop_func
            .body
            .iter()
            .filter(|inst| matches!(inst, JoinInst::Call { k_next: None, .. }))
            .count();
        assert!(
            jump_conds >= 1 && tail_calls >= 1,
            "normalize_pattern2_minimal: expected at least one conditional jump and one tail call"
        );

        let mut functions = BTreeMap::new();
        let mut env_layouts = Vec::new();

        for (fid, func) in &structured.functions {
            let env_layout_id = if func.params.is_empty() {
                None
            } else {
                let id = env_layouts.len() as u32;
                env_layouts.push(EnvLayout {
                    id,
                    fields: func
                        .params
                        .iter()
                        .enumerate()
                        .map(|(idx, vid)| EnvField {
                            name: format!("field{}", idx),
                            ty: None,
                            value_id: Some(*vid),
                        })
                        .collect(),
                });
                Some(id)
            };

            let mut body = Vec::new();
            for inst in &func.body {
                match inst {
                    JoinInst::Compute(MirLikeInst::Const { dst, value }) => {
                        body.push(JpInst::Let {
                            dst: *dst,
                            op: JpOp::Const(value.clone()),
                            args: vec![],
                        })
                    }
                    JoinInst::Compute(MirLikeInst::BoxCall {
                        dst,
                        box_name,
                        method,
                        args,
                    }) => {
                        if let Some(dst) = dst {
                            body.push(JpInst::Let {
                                dst: *dst,
                                op: JpOp::BoxCall {
                                    box_name: box_name.clone(),
                                    method: method.clone(),
                                },
                                args: args.clone(),
                            })
                        }
                    }
                    JoinInst::Compute(MirLikeInst::BinOp { dst, op, lhs, rhs }) => {
                        body.push(JpInst::Let {
                            dst: *dst,
                            op: JpOp::BinOp(*op),
                            args: vec![*lhs, *rhs],
                        })
                    }
                    JoinInst::Compute(MirLikeInst::UnaryOp { dst, op, operand }) => {
                        body.push(JpInst::Let {
                            dst: *dst,
                            op: JpOp::Unary(*op),
                            args: vec![*operand],
                        })
                    }
                    JoinInst::Compute(MirLikeInst::Compare { dst, op, lhs, rhs }) => {
                        body.push(JpInst::Let {
                            dst: *dst,
                            op: JpOp::Compare(*op),
                            args: vec![*lhs, *rhs],
                        })
                    }
                    JoinInst::Compute(MirLikeInst::Select {
                        dst,
                        cond,
                        then_val,
                        else_val,
                    }) => body.push(JpInst::Let {
                        dst: *dst,
                        op: JpOp::Select,
                        args: vec![*cond, *then_val, *else_val],
                    }),
                    JoinInst::Jump { cont, args, cond } => {
                        if let Some(cond_val) = cond {
                            body.push(JpInst::If {
                                cond: *cond_val,
                                then_target: JpFuncId(cont.0),
                                else_target: JpFuncId(loop_func.id.0),
                                env: args.clone(),
                            });
                        } else {
                            body.push(JpInst::TailCallKont {
                                target: JpFuncId(cont.0),
                                env: args.clone(),
                            });
                        }
                    }
                    JoinInst::Select {
                        dst,
                        cond,
                        then_val,
                        else_val,
                        ..
                    } => {
                        body.push(JpInst::Let {
                            dst: *dst,
                            op: JpOp::Select,
                            args: vec![*cond, *then_val, *else_val],
                        });
                    }
                    JoinInst::Call {
                        func, args, k_next, ..
                    } => {
                        if k_next.is_none() {
                            body.push(JpInst::TailCallFn {
                                target: JpFuncId(func.0),
                                env: args.clone(),
                            });
                        }
                    }
                    JoinInst::MethodCall {
                        dst,
                        receiver,
                        method,
                        args,
                        ..
                    } => {
                        let mut call_args = Vec::with_capacity(args.len() + 1);
                        call_args.push(*receiver);
                        call_args.extend(args.iter().copied());
                        body.push(JpInst::Let {
                            dst: *dst,
                            op: JpOp::BoxCall {
                                box_name: "unknown".to_string(),
                                method: method.clone(),
                            },
                            args: call_args,
                        });
                    }
                    _ => {
                        // Ret / other instructions are ignored in this minimal prototype
                    }
                }
            }

            functions.insert(
                JpFuncId(fid.0),
                JpFunction {
                    id: JpFuncId(fid.0),
                    name: func.name.clone(),
                    env_layout: env_layout_id,
                    body,
                    is_kont: func.name.starts_with("k_"),
                },
            );
        }

        let norm = NormalizedModule {
            functions,
            entry: structured.entry.map(|e| JpFuncId(e.0)),
            env_layouts,
            phase: JoinIrPhase::Normalized,
            structured_backup: Some(structured.clone()),
        };

        #[cfg(feature = "normalized_dev")]
        {
            Self::verify(&norm, param_max).expect("normalized Pattern2 verifier");
        }

        norm
    }

    /// Convert Normalized Pattern 2 back to Structured JoinIR
    ///
    /// # Arguments
    /// * `norm` - Normalized module (must be Pattern 2/3/4, Normalized phase)
    ///
    /// # Returns
    /// Structured JoinIR module
    pub fn to_structured(norm: &NormalizedModule) -> JoinModule {
        if let Some(backup) = norm.to_structured() {
            return backup;
        }

        let mut module = JoinModule::new();

        for (jp_id, jp_fn) in &norm.functions {
            let params: Vec<ValueId> = jp_fn
                .env_layout
                .and_then(|id| norm.env_layouts.iter().find(|layout| layout.id == id))
                .map(|layout| {
                    layout
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(idx, f)| f.value_id.unwrap_or(ValueId(idx as u32)))
                        .collect()
                })
                .unwrap_or_default();

            let mut func = JoinFunction::new(JoinFuncId(jp_id.0), jp_fn.name.clone(), params);

            for inst in &jp_fn.body {
                match inst {
                    JpInst::Let { dst, op, args } => match op {
                        JpOp::Const(v) => {
                            func.body.push(JoinInst::Compute(MirLikeInst::Const {
                                dst: *dst,
                                value: v.clone(),
                            }))
                        }
                        JpOp::BoxCall { box_name, method } => {
                            func.body.push(JoinInst::Compute(MirLikeInst::BoxCall {
                                dst: Some(*dst),
                                box_name: box_name.clone(),
                                method: method.clone(),
                                args: args.clone(),
                            }))
                        }
                        JpOp::BinOp(op) => {
                            func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
                                dst: *dst,
                                op: *op,
                                lhs: args.get(0).copied().unwrap_or(ValueId(0)),
                                rhs: args.get(1).copied().unwrap_or(ValueId(0)),
                            }))
                        }
                        JpOp::Unary(op) => {
                            func.body.push(JoinInst::Compute(MirLikeInst::UnaryOp {
                                dst: *dst,
                                op: *op,
                                operand: args.get(0).copied().unwrap_or(ValueId(0)),
                            }))
                        }
                        JpOp::Select => {
                            func.body.push(JoinInst::Compute(MirLikeInst::Select {
                                dst: *dst,
                                cond: args.get(0).copied().unwrap_or(ValueId(0)),
                                then_val: args.get(1).copied().unwrap_or(ValueId(0)),
                                else_val: args.get(2).copied().unwrap_or(ValueId(0)),
                            }))
                        }
                        JpOp::Compare(op) => {
                            func.body.push(JoinInst::Compute(MirLikeInst::Compare {
                                dst: *dst,
                                op: *op,
                                lhs: args.get(0).copied().unwrap_or(ValueId(0)),
                                rhs: args.get(1).copied().unwrap_or(ValueId(0)),
                            }))
                        }
                    },
                    JpInst::TailCallFn { target, env } => {
                        func.body.push(JoinInst::Call {
                            func: JoinFuncId(target.0),
                            args: env.clone(),
                            k_next: None,
                            dst: None,
                        })
                    }
                    JpInst::TailCallKont { target, env } => {
                        func.body.push(JoinInst::Jump {
                            cont: JoinContId(target.0),
                            args: env.clone(),
                            cond: None,
                        })
                    }
                    JpInst::If {
                        cond,
                        then_target,
                        else_target,
                        env,
                    } => {
                        func.body.push(JoinInst::Jump {
                            cont: JoinContId(then_target.0),
                            args: env.clone(),
                            cond: Some(*cond),
                        });
                        func.body.push(JoinInst::Jump {
                            cont: JoinContId(else_target.0),
                            args: env.clone(),
                            cond: None,
                        });
                    }
                    JpInst::EnvLoad { .. } | JpInst::EnvStore { .. } => {
                        // Not used in minimal pattern2 bridge
                    }
                }
            }

            module.add_function(func);
        }

        module.entry = norm.entry.map(|e| JoinFuncId(e.0));
        module.phase = JoinIrPhase::Structured;
        module
    }

    /// Verify Pattern 2 normalized module invariants
    ///
    /// # Arguments
    /// * `module` - Normalized module to verify
    /// * `max_env_fields` - Maximum allowed env fields (shape-dependent)
    ///
    /// # Returns
    /// Ok(()) if invariants hold, Err(String) otherwise
    ///
    /// # Checks
    /// - Phase must be Normalized
    /// - Env field count within range
    /// - Env is not empty for tail calls
    /// - Functions end with tail call or if
    #[cfg(feature = "normalized_dev")]
    pub fn verify(module: &NormalizedModule, max_env_fields: usize) -> Result<(), String> {
        if module.phase != JoinIrPhase::Normalized {
            return Err("[joinir/normalized-dev] pattern2: phase must be Normalized".to_string());
        }

        let mut layout_sizes: HashMap<u32, usize> = HashMap::new();
        for layout in &module.env_layouts {
            let size = layout.fields.len();
            if !(1..=max_env_fields).contains(&size) {
                return Err(format!(
                    "[joinir/normalized-dev] pattern2: expected 1..={} env fields, got {}",
                    max_env_fields, size
                ));
            }
            layout_sizes.insert(layout.id, size);
        }

        for func in module.functions.values() {
            let expected_env_len = func
                .env_layout
                .and_then(|id| layout_sizes.get(&id))
                .copied();

            for inst in &func.body {
                match inst {
                    JpInst::Let { .. }
                    | JpInst::EnvLoad { .. }
                    | JpInst::EnvStore { .. }
                    | JpInst::TailCallFn { .. }
                    | JpInst::TailCallKont { .. }
                    | JpInst::If { .. } => {}
                }

                match inst {
                    JpInst::TailCallFn { env, .. }
                    | JpInst::TailCallKont { env, .. }
                    | JpInst::If { env, .. } => {
                        if let Some(expected) = expected_env_len {
                            if env.is_empty() {
                                return Err(
                                    "[joinir/normalized-dev] pattern2: env must not be empty"
                                        .to_string(),
                                );
                            }
                            let _ = expected;
                        }
                    }
                    _ => {}
                }
            }

            if let Some(last) = func.body.last() {
                match last {
                    JpInst::TailCallFn { .. } | JpInst::TailCallKont { .. } | JpInst::If { .. } => {}
                    _ => {
                        return Err(format!(
                            "[joinir/normalized-dev] pattern2: function '{}' does not end with tail call/if",
                            func.name
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}

// Re-export the original functions for backward compatibility
#[allow(deprecated)]
pub use self::Pattern2Normalizer as Pattern2NormalizerBox;

/// Legacy function wrapper for backward compatibility
#[deprecated(note = "Use Pattern2Normalizer::normalize instead")]
pub fn normalize_pattern2_minimal(structured: &JoinModule) -> NormalizedModule {
    Pattern2Normalizer::normalize(structured)
}

/// Legacy function wrapper for backward compatibility
#[deprecated(note = "Use Pattern2Normalizer::to_structured instead")]
pub fn normalized_pattern2_to_structured(norm: &NormalizedModule) -> JoinModule {
    Pattern2Normalizer::to_structured(norm)
}

/// Legacy function wrapper for backward compatibility
#[deprecated(note = "Use Pattern2Normalizer::verify instead")]
#[cfg(feature = "normalized_dev")]
pub fn verify_normalized_pattern2(
    module: &NormalizedModule,
    max_env_fields: usize,
) -> Result<(), String> {
    Pattern2Normalizer::verify(module, max_env_fields)
}

/// Shape-based normalization for Pattern 2/3/4
///
/// These functions provide shape-specific entry points that delegate to
/// the core Pattern2Normalizer with proper guards.
#[cfg(feature = "normalized_dev")]
pub mod shapes {
    use super::*;

    /// Normalize with shape guard
    pub fn normalize_with_shape(
        structured: &JoinModule,
        target_shape: NormalizedDevShape,
    ) -> Result<NormalizedModule, String> {
        if !structured.is_structured() {
            return Err("[normalize_p2] Not structured JoinIR".to_string());
        }

        let shapes = crate::mir::join_ir::normalized::shape_guard::supported_shapes(structured);
        if !shapes.contains(&target_shape) {
            return Err(format!(
                "[normalize_p2] shape mismatch: expected {:?}, got {:?}",
                target_shape, shapes
            ));
        }

        Ok(Pattern2Normalizer::normalize(structured))
    }
}

// Public shape-specific normalization functions (re-exported)
#[cfg(feature = "normalized_dev")]
pub use shapes::normalize_with_shape;
