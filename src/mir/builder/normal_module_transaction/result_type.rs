//! Sole MIR-type projection for the sealed canonical Main result.

use crate::mir::compiler::normal_source_plan::VerifiedNormalMainThunkResultV1;
use crate::mir::MirType;

pub(super) fn normal_main_result_mir_type(result: VerifiedNormalMainThunkResultV1) -> MirType {
    match result {
        VerifiedNormalMainThunkResultV1::Unit { .. } => MirType::Void,
        VerifiedNormalMainThunkResultV1::Integer => MirType::Integer,
        VerifiedNormalMainThunkResultV1::Bool => MirType::Bool,
        VerifiedNormalMainThunkResultV1::Float => MirType::Float,
    }
}
