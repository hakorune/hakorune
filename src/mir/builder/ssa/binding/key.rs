//! SSA identities supplied by the original source resolver or physical plan.
//! A planned PHI identity has no Binding/Home authority.
use crate::mir::resolved_semantics::BindingRefV1;
use crate::mir::{BasicBlockId, ValueId};

pub(in crate::mir::builder) trait SsaKeyV1:
    Copy + Ord + std::fmt::Debug
{
    fn source_binding(self) -> Option<BindingRefV1>;
}
impl SsaKeyV1 for BindingRefV1 {
    fn source_binding(self) -> Option<BindingRefV1> {
        Some(self)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum FunctionSsaKeyV1 {
    Source(BindingRefV1),
    PlannedPhi {
        block: BasicBlockId,
        destination: ValueId,
    },
}
impl SsaKeyV1 for FunctionSsaKeyV1 {
    fn source_binding(self) -> Option<BindingRefV1> {
        match self {
            Self::Source(binding) => Some(binding),
            Self::PlannedPhi { .. } => None,
        }
    }
}
