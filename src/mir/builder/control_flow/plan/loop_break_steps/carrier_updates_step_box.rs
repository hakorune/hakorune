//! CarrierUpdatesStepBox (Phase 176+, extracted in Phase 5)
//!
//! Responsibility:
//! - Analyze carrier updates for loop_break route (or use policy override).
//! - Filter carriers to only those required by updates / condition-only / loop-local-zero.
//! - Ensure JoinValue env has join-ids for carriers referenced only from body updates.

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::carrier_info::{CarrierInfo, CarrierInit, CarrierRole};
use crate::mir::join_ir::lowering::condition_env::ConditionBinding;
use crate::mir::join_ir::lowering::loop_update_analyzer::{LoopUpdateAnalyzer, UpdateExpr};

use super::super::loop_break_prep_box::{LoopBreakDebugLog, LoopBreakPrepInputs};
use crate::mir::builder::control_flow::plan::common::{
    decide_carrier_binding_policy, CarrierBindingPolicy,
};

use std::collections::BTreeMap;

pub(crate) struct CarrierUpdatesStepBox;

impl CarrierUpdatesStepBox {
    pub(crate) fn analyze_and_filter(
        analysis_body: &[ASTNode],
        inputs: &mut LoopBreakPrepInputs,
        verbose: bool,
    ) -> BTreeMap<String, UpdateExpr> {
        let carrier_updates = if let Some(override_map) = inputs.carrier_updates_override.take() {
            override_map
        } else {
            LoopUpdateAnalyzer::analyze_carrier_updates(
                analysis_body,
                &inputs.carrier_info.carriers,
            )
        };
        LoopBreakDebugLog::new(verbose).log(
            "updates",
            format!(
                "Phase 176-3: Analyzed {} carrier updates",
                carrier_updates.len()
            ),
        );

        let original_carrier_count = inputs.carrier_info.carriers.len();
        filter_carriers_for_updates(&mut inputs.carrier_info, &carrier_updates);
        LoopBreakDebugLog::new(verbose).log(
            "updates",
            format!(
                "Phase 176-4: Filtered carriers: {} → {} (kept only carriers with updates/condition-only/loop-local-zero)",
                original_carrier_count,
                inputs.carrier_info.carriers.len()
            ),
        );

        // Ensure env has join-ids for carriers that are referenced only from body updates.
        for carrier in &inputs.carrier_info.carriers {
            if inputs.env.get(&carrier.name).is_none() {
                let join_value = carrier
                    .join_id
                    .unwrap_or_else(|| inputs.join_value_space.alloc_param());
                inputs.env.insert(carrier.name.clone(), join_value);

                match decide_carrier_binding_policy(carrier) {
                    CarrierBindingPolicy::BindFromHost => {
                        inputs.condition_bindings.push(ConditionBinding {
                            name: carrier.name.clone(),
                            host_value: carrier.host_id,
                            join_value,
                        });
                    }
                    CarrierBindingPolicy::SkipBinding => {
                        LoopBreakDebugLog::new(verbose).log(
                            "updates",
                            format!(
                                "Phase 29ab: Skipping host binding for carrier '{}' (init={:?}, role={:?})",
                                carrier.name, carrier.init, carrier.role
                            ),
                        );
                    }
                }
            }
        }

        carrier_updates
    }
}

fn filter_carriers_for_updates(info: &mut CarrierInfo, updates: &BTreeMap<String, UpdateExpr>) {
    // Keep carriers that:
    // - have updates
    // - are condition-only (used for break condition)
    // - are loop-local-zero (policy-injected for derived computations)
    info.carriers.retain(|c| {
        updates.contains_key(&c.name)
            || c.role == CarrierRole::ConditionOnly
            || c.init == CarrierInit::LoopLocalZero
    });
}
