//! Pattern 1 Normalization - Simple While Loops
//!
//! Normalizes Pattern 1 JoinIR to Normalized form.
//! Pattern 1 is the simplest while loop pattern (no break/continue).
//!
//! ## Responsibilities
//! - Convert Structured JoinIR → Normalized JoinIR (Pattern 1)
//! - Convert Normalized JoinIR → Structured JoinIR (reverse)
//! - Verify normalized module invariants

use std::collections::{BTreeMap, HashSet};

use crate::mir::join_ir::{
    JoinContId, JoinFuncId, JoinFunction, JoinInst, JoinIrPhase,
    JoinModule, MirLikeInst,
};
use crate::mir::ValueId;

use super::{EnvField, EnvLayout, JpFuncId, JpFunction, JpInst, JpOp, NormalizedModule};

/// Pattern 1 Normalizer
///
/// Normalizes Pattern 1 (simple while loops) to Normalized form.
///
/// Pattern 1 constraints:
/// - Simple while loop (no break/continue)
/// - Entry/loop_step/k_exit function structure
/// - Only Compute/Call/Ret/Join instructions
pub struct Pattern1Normalizer;

impl Pattern1Normalizer {
    /// Normalize Pattern 1 Structured JoinIR to Normalized form
    ///
    /// # Arguments
    /// * `structured` - Structured JoinIR module (must be Pattern 1)
    ///
    /// # Returns
    /// Normalized module with backup
    ///
    /// # Panics
    /// - If structured.phase is not Structured
    /// - If loop_step function is not found
    pub fn normalize(structured: &JoinModule) -> NormalizedModule {
        assert!(
            structured.is_structured(),
            "normalize_pattern1_minimal: expected Structured JoinIR"
        );

        // entry/loop_step/k_exit を前提に、loop_step を拾う
        let loop_func = structured
            .functions
            .values()
            .find(|f| f.name == "loop_step")
            .or_else(|| structured.functions.get(&JoinFuncId::new(1)))
            .expect("normalize_pattern1_minimal: loop_step not found");

        // EnvLayout をざっくり作る（フィールド名は field0, field1,... で代用）
        let env_layout = EnvLayout {
            id: 0,
            fields: loop_func
                .params
                .iter()
                .enumerate()
                .map(|(idx, vid)| EnvField {
                    name: format!("field{}", idx),
                    ty: None,
                    value_id: Some(*vid),
                })
                .collect(),
        };

        // loop_step の Compute を Let に写経（Pattern1 では Compute/Call/Ret のみ想定）
        let mut extra_konts: HashSet<JpFuncId> = HashSet::new();
        let mut jp_body = Vec::new();
        for inst in &loop_func.body {
            match inst {
                JoinInst::Compute(MirLikeInst::Const { dst, value }) => {
                    jp_body.push(JpInst::Let {
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
                        jp_body.push(JpInst::Let {
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
                    jp_body.push(JpInst::Let {
                        dst: *dst,
                        op: JpOp::BinOp(*op),
                        args: vec![*lhs, *rhs],
                    })
                }
                JoinInst::Compute(MirLikeInst::UnaryOp { dst, op, operand }) => {
                    jp_body.push(JpInst::Let {
                        dst: *dst,
                        op: JpOp::Unary(*op),
                        args: vec![*operand],
                    })
                }
                JoinInst::Compute(MirLikeInst::Compare { dst, op, lhs, rhs }) => {
                    jp_body.push(JpInst::Let {
                        dst: *dst,
                        op: JpOp::Compare(*op),
                        args: vec![*lhs, *rhs],
                    })
                }
                // Tail recursion / exit は TailCall と If でざっくり表現
                JoinInst::Jump { cont, args, cond } => {
                    if let Some(cond_val) = cond {
                        extra_konts.insert(JpFuncId(cont.0));
                        jp_body.push(JpInst::If {
                            cond: *cond_val,
                            then_target: JpFuncId(cont.0),
                            else_target: JpFuncId(loop_func.id.0),
                            env: args.clone(),
                        });
                    } else {
                        extra_konts.insert(JpFuncId(cont.0));
                        jp_body.push(JpInst::TailCallKont {
                            target: JpFuncId(cont.0),
                            env: args.clone(),
                        });
                    }
                }
                JoinInst::Call { func, args, .. } => jp_body.push(JpInst::TailCallFn {
                    target: JpFuncId(func.0),
                    env: args.clone(),
                }),
                JoinInst::Ret { value } => {
                    if let Some(v) = value {
                        let kont_id = JpFuncId(loop_func.id.0 + 1);
                        extra_konts.insert(kont_id);
                        jp_body.push(JpInst::TailCallKont {
                            target: kont_id,
                            env: vec![*v],
                        });
                    }
                }
                _ => {
                    // Pattern1 の最小変換なので他は無視（将来拡張）
                }
            }
        }

        let loop_fn = JpFunction {
            id: JpFuncId(loop_func.id.0),
            name: loop_func.name.clone(),
            env_layout: Some(env_layout.id),
            body: jp_body,
            is_kont: false,
        };

        let mut functions = BTreeMap::new();
        functions.insert(loop_fn.id, loop_fn);

        for kont_id in extra_konts {
            functions
                .entry(kont_id)
                .or_insert_with(|| JpFunction {
                    id: kont_id,
                    name: format!("kont_{}", kont_id.0),
                    env_layout: Some(env_layout.id),
                    body: Vec::new(),
                    is_kont: true,
                });
        }

        let norm = NormalizedModule {
            functions,
            entry: Some(JpFuncId(loop_func.id.0)),
            env_layouts: vec![env_layout],
            phase: JoinIrPhase::Normalized,
            structured_backup: Some(structured.clone()),
        };

        #[cfg(feature = "normalized_dev")]
        {
            Self::verify(&norm).expect("normalized verifier");
        }

        norm
    }

    /// Convert Normalized Pattern 1 back to Structured JoinIR
    ///
    /// # Arguments
    /// * `norm` - Normalized module (must be Pattern 1, Normalized phase)
    ///
    /// # Returns
    /// Structured JoinIR module
    ///
    /// # Panics
    /// - If norm.phase is not Normalized
    /// - If env layout is missing
    pub fn to_structured(norm: &NormalizedModule) -> JoinModule {
        assert_eq!(
            norm.phase,
            JoinIrPhase::Normalized,
            "normalized_pattern1_to_structured expects Normalized phase"
        );

        let env_layout = norm
            .env_layouts
            .get(0)
            .expect("normalized_pattern1_to_structured: missing env layout");

        let mut module = JoinModule::new();

        for (jp_id, jp_fn) in &norm.functions {
            let params: Vec<ValueId> = jp_fn
                .env_layout
                .and_then(|id| norm.env_layouts.iter().find(|layout| layout.id == id))
                .unwrap_or(env_layout)
                .fields
                .iter()
                .enumerate()
                .map(|(idx, f)| f.value_id.unwrap_or(ValueId(idx as u32)))
                .collect();

            let mut func =
                JoinFunction::new(JoinFuncId(jp_id.0), jp_fn.name.clone(), params);

            for inst in &jp_fn.body {
                match inst {
                    JpInst::Let { dst, op, args } => match op {
                        JpOp::Const(v) => func
                            .body
                            .push(JoinInst::Compute(MirLikeInst::Const {
                                dst: *dst,
                                value: v.clone(),
                            })),
                        JpOp::BoxCall { box_name, method } => {
                            func.body.push(JoinInst::Compute(MirLikeInst::BoxCall {
                                dst: Some(*dst),
                                box_name: box_name.clone(),
                                method: method.clone(),
                                args: args.clone(),
                            }))
                        }
                        JpOp::BinOp(op) => func
                            .body
                            .push(JoinInst::Compute(MirLikeInst::BinOp {
                                dst: *dst,
                                op: *op,
                                lhs: args.get(0).copied().unwrap_or(ValueId(0)),
                                rhs: args.get(1).copied().unwrap_or(ValueId(0)),
                            })),
                        JpOp::Unary(op) => func
                            .body
                            .push(JoinInst::Compute(MirLikeInst::UnaryOp {
                                dst: *dst,
                                op: *op,
                                operand: args.get(0).copied().unwrap_or(ValueId(0)),
                            })),
                        JpOp::Select => func
                            .body
                            .push(JoinInst::Compute(MirLikeInst::Select {
                                dst: *dst,
                                cond: args.get(0).copied().unwrap_or(ValueId(0)),
                                then_val: args.get(1).copied().unwrap_or(ValueId(0)),
                                else_val: args.get(2).copied().unwrap_or(ValueId(0)),
                            })),
                        JpOp::Compare(op) => func
                            .body
                            .push(JoinInst::Compute(MirLikeInst::Compare {
                                dst: *dst,
                                op: *op,
                                lhs: args.get(0).copied().unwrap_or(ValueId(0)),
                                rhs: args.get(1).copied().unwrap_or(ValueId(0)),
                            })),
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
                        // Jump to then_target on cond, else jump to else_target (Pattern1 minimal)
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
                        // Not used in Pattern1 minimal; ignore for now
                    }
                }
            }

            module.add_function(func);
        }

        module.entry = norm.entry.map(|e| JoinFuncId(e.0));
        module.phase = JoinIrPhase::Structured;
        module
    }

    /// Verify Pattern 1 normalized module invariants
    ///
    /// # Arguments
    /// * `module` - Normalized module to verify
    ///
    /// # Returns
    /// Ok(()) if invariants hold, Err(String) otherwise
    ///
    /// # Checks
    /// - Phase must be Normalized
    /// - Env field bounds checking
    /// - Only valid instruction types
    /// - Functions end with tail call or if
    #[cfg(feature = "normalized_dev")]
    pub fn verify(module: &NormalizedModule) -> Result<(), String> {
        if module.phase != JoinIrPhase::Normalized {
            return Err("[joinir/normalized-dev] pattern1: phase must be Normalized".to_string());
        }

        // Env field bounds check
        if let Some(env) = module.env_layouts.get(0) {
            let field_count = env.fields.len();
            for func in module.functions.values() {
                for inst in &func.body {
                    match inst {
                        JpInst::EnvLoad { field, .. } | JpInst::EnvStore { field, .. } => {
                            if *field >= field_count {
                                return Err(format!(
                                    "[joinir/normalized-dev] pattern1: env field out of range: {} (fields={})",
                                    field, field_count
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        for func in module.functions.values() {
            for inst in &func.body {
                match inst {
                    JpInst::Let { .. }
                    | JpInst::EnvLoad { .. }
                    | JpInst::EnvStore { .. }
                    | JpInst::TailCallFn { .. }
                    | JpInst::TailCallKont { .. }
                    | JpInst::If { .. } => {}
                }
            }

            // Tail: allow TailCall or If only
            if let Some(last) = func.body.last() {
                match last {
                    JpInst::TailCallFn { .. } | JpInst::TailCallKont { .. } | JpInst::If { .. } => {}
                    _ => {
                        return Err(format!(
                            "[joinir/normalized-dev] pattern1: function '{}' does not end with tail call/if",
                            func.name
                        ))
                    }
                }
            }
        }

        Ok(())
    }
}

// Re-export the original functions for backward compatibility
#[allow(deprecated)]
pub use self::Pattern1Normalizer as Pattern1NormalizerBox;

/// Legacy function wrapper for backward compatibility
#[deprecated(note = "Use Pattern1Normalizer::normalize instead")]
pub fn normalize_pattern1_minimal(structured: &JoinModule) -> NormalizedModule {
    Pattern1Normalizer::normalize(structured)
}

/// Legacy function wrapper for backward compatibility
#[deprecated(note = "Use Pattern1Normalizer::to_structured instead")]
pub fn normalized_pattern1_to_structured(norm: &NormalizedModule) -> JoinModule {
    Pattern1Normalizer::to_structured(norm)
}

/// Legacy function wrapper for backward compatibility
#[deprecated(note = "Use Pattern1Normalizer::verify instead")]
#[cfg(feature = "normalized_dev")]
pub fn verify_normalized_pattern1(module: &NormalizedModule) -> Result<(), String> {
    Pattern1Normalizer::verify(module)
}
