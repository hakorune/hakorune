//! Checked loop source products and the V2 MIR lowering support.

mod call_source;
pub(in crate::mir::builder) mod expression_port;
pub(in crate::mir::builder) mod facts;
pub(in crate::mir) use facts::LoopFactsPolicyFrameV1;
pub(in crate::mir::builder) mod lowerer;
pub(in crate::mir::builder) mod planner;

pub(in crate::mir::builder) use self::call_source::CoreCallSourceV1;
pub(in crate::mir::builder) use self::expression_port::{
    LoopPlanExpressionPortErrorV1, LoopPlanExpressionPortV1,
};
