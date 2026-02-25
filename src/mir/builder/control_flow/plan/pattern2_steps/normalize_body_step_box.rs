//! NormalizeBodyStepBox (Trim + break condition normalization)
//!
//! Responsibility:
//! - Apply trim normalization when enabled (Phase 93+).
//! - Return the effective break condition and an optional normalized body.

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;

use super::super::pattern2_inputs_facts_box::{Pattern2DebugLog, Pattern2Inputs};

pub(crate) struct NormalizedBodyResult {
    pub effective_break_condition: ASTNode,
    pub normalized_body: Option<Vec<ASTNode>>,
}

pub(crate) struct NormalizeBodyStepBox;

impl NormalizeBodyStepBox {
    pub(crate) fn run(
        builder: &mut MirBuilder,
        condition: &ASTNode,
        body: &[ASTNode],
        inputs: &mut Pattern2Inputs,
        verbose: bool,
    ) -> Result<NormalizedBodyResult, String> {
        let log = Pattern2DebugLog::new(verbose);
        let mut alloc_join_value = || inputs.join_value_space.alloc_param();

        // Trim-like loops have their own canonicalizer path; keep existing behavior:
        // read_digits(loop(true)) disables trim handling here.
        let disable_trim = inputs.is_loop_true_read_digits;

        let effective_break_condition = if !disable_trim {
            if let Some(trim_result) =
                crate::mir::builder::control_flow::plan::trim_loop_lowering::TrimLoopLowerer::try_lower_trim_like_loop(
                    builder,
                    &inputs.scope,
                    condition,
                    &inputs.break_condition_node,
                    body,
                    &inputs.loop_var_name,
                    &mut inputs.carrier_info,
                    &mut alloc_join_value,
                )?
            {
                log.log("trim", "TrimLoopLowerer processed Trim pattern successfully");
                inputs.carrier_info = trim_result.carrier_info;
                inputs.condition_only_recipe = trim_result.condition_only_recipe;
                trim_result.condition
            } else {
                inputs.break_condition_node.clone()
            }
        } else {
            inputs.break_condition_node.clone()
        };

        use crate::mir::join_ir::lowering::complex_addend_normalizer::{
            ComplexAddendNormalizer, NormalizationResult,
        };
        let mut normalized_body = Vec::new();
        let mut has_normalization = false;

        for node in body {
            match ComplexAddendNormalizer::normalize_assign(node) {
                NormalizationResult::Normalized {
                    temp_def, new_assign, ..
                } => {
                    normalized_body.push(temp_def);
                    normalized_body.push(new_assign);
                    has_normalization = true;
                }
                NormalizationResult::Unchanged => normalized_body.push(node.clone()),
            }
        }

        Ok(NormalizedBodyResult {
            effective_break_condition,
            normalized_body: if has_normalization {
                Some(normalized_body)
            } else {
                None
            },
        })
    }
}
