//! Receiver ('me'/'this') normalization and binding
//!
//! Responsibilities:
//! - Classify 'this'/'me' receivers before member-call emission
//! - Handle 'this' in static vs instance context
//! - Keep 'me' semantics on the instance lane
//!
//! Key functions:
//! - classify_this_me_method_call: classify this/me before member-call emission
//!
//! Design notes:
//! - Uses comp_ctx.current_static_box to determine static context
//! - Leaves actual emission to the caller-selected member route

use super::super::MirBuilder;
use crate::ast::ASTNode;

#[derive(Clone, Debug)]
pub(in crate::mir::builder) enum ReceiverNormalizationPlan {
    MeCall,
    StaticThis { box_name: String },
}

impl MirBuilder {
    pub(in crate::mir::builder) fn classify_this_me_method_call(
        &mut self,
        object: &ASTNode,
    ) -> Result<Option<ReceiverNormalizationPlan>, String> {
        match object {
            ASTNode::Me { .. } => Ok(Some(ReceiverNormalizationPlan::MeCall)),
            ASTNode::Variable { name, .. } if name == "me" => {
                Ok(Some(ReceiverNormalizationPlan::MeCall))
            }
            ASTNode::This { .. } => {
                if let Some(box_name) = self.comp_ctx.current_static_box.clone() {
                    if crate::config::env::builder_trace_normalize() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[trace:normalize] this receiver classified as static {}",
                            box_name
                        ));
                    }
                    return Ok(Some(ReceiverNormalizationPlan::StaticThis { box_name }));
                }
                Ok(Some(ReceiverNormalizationPlan::MeCall))
            }
            _ => Ok(None),
        }
    }
}
