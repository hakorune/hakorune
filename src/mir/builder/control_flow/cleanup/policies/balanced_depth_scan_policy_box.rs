//! BalancedDepthScanPolicyBox (Phase 107)
//!
//! Responsibility:
//! - Provide a single SSOT entry point for the balanced depth-scan policy decision
//!   so callers (routing/apply) don't duplicate Use/Reject/None handling.

use crate::ast::ASTNode;

use crate::mir::policies::balanced_depth_scan::decide as decide_balanced;
use crate::mir::policies::balanced_depth_scan::BalancedDepthScanPolicyResult;
use crate::mir::policies::PolicyDecision;

pub(crate) struct BalancedDepthScanPolicyBox;

impl BalancedDepthScanPolicyBox {
    pub(crate) fn decide(
        condition: &ASTNode,
        body: &[ASTNode],
    ) -> PolicyDecision<BalancedDepthScanPolicyResult> {
        let decision = decide_balanced(condition, body);

        if crate::config::env::joinir_dev_enabled() {
            use crate::mir::builder::control_flow::joinir::trace;
            let summary = match &decision {
                PolicyDecision::Use(_) => "Use".to_string(),
                PolicyDecision::Reject(reason) => format!("Reject: {}", reason),
                PolicyDecision::None => "None".to_string(),
            };
            trace::trace().dev("phase107/balanced_depth_scan_policy", &summary);
        }

        decision
    }
}
