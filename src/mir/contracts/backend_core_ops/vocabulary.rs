use crate::mir::MirInstruction;

/// Canonical instruction tag used by contract/fail-fast diagnostics.
pub fn instruction_tag(inst: &MirInstruction) -> &'static str {
    match inst {
        MirInstruction::Const { .. } => "Const",
        MirInstruction::BinOp { .. } => "BinOp",
        MirInstruction::UnaryOp { .. } => "UnaryOp",
        MirInstruction::Compare { .. } => "Compare",
        MirInstruction::FieldGet { .. } => "FieldGet",
        MirInstruction::FieldSet { .. } => "FieldSet",
        MirInstruction::WeakFieldWrite { .. } => "WeakFieldWrite",
        MirInstruction::StaticDataLoad { .. } => "StaticDataLoad",
        MirInstruction::ArrayElementWrite { .. } => "ArrayElementWrite",
        MirInstruction::ArrayStateContractClaim { .. } => "ArrayStateContractClaim",
        MirInstruction::VariantMake { .. } => "VariantMake",
        MirInstruction::VariantTag { .. } => "VariantTag",
        MirInstruction::VariantProject { .. } => "VariantProject",
        MirInstruction::Load { .. } => "Load",
        MirInstruction::Store { .. } => "Store",
        MirInstruction::MemOp { .. } => "MemOp",
        MirInstruction::PinnedTextOp { .. } => "PinnedTextOp",
        MirInstruction::PinnedTextResidenceFinish { .. } => "PinnedTextResidenceFinish",
        MirInstruction::PinnedTextResidenceEnter { .. } => "PinnedTextResidenceEnter",
        MirInstruction::Call { .. } => "Call",
        MirInstruction::NewClosure { .. } => "NewClosure",
        MirInstruction::Branch { .. } => "Branch",
        MirInstruction::Jump { .. } => "Jump",
        MirInstruction::Return { .. } => "Return",
        MirInstruction::CheckedCallOut { .. } => "CheckedCallOut",
        MirInstruction::CheckedCallOutNormalResult { .. } => "CheckedCallOutNormalResult",
        MirInstruction::CheckedCallOutEnd { .. } => "CheckedCallOutEnd",
        MirInstruction::CheckedCallOutFault { .. } => "CheckedCallOutFault",
        MirInstruction::Phi { .. } => "Phi",
        MirInstruction::NewBox { .. } => "NewBox",
        MirInstruction::TypeOp { .. } => "TypeOp",
        MirInstruction::Copy { .. } => "Copy",
        MirInstruction::CopyOwned { .. } => "CopyOwned",
        MirInstruction::DestroyOwned { .. } => "DestroyOwned",
        MirInstruction::LocalContractWrite { .. } => "LocalContractWrite",
        MirInstruction::RecordFieldContractCheck { .. } => "RecordFieldContractCheck",
        MirInstruction::RecordValuePublish { .. } => "RecordValuePublish",
        MirInstruction::Debug { .. } => "Debug",
        MirInstruction::KeepAlive { .. } => "KeepAlive",
        MirInstruction::ReleaseStrong { .. } => "ReleaseStrong",
        MirInstruction::Throw { .. } => "Throw",
        MirInstruction::Catch { .. } => "Catch",
        MirInstruction::Safepoint => "Safepoint",
        MirInstruction::RefNew { .. } => "RefNew",
        MirInstruction::WeakRef { .. } => "WeakRef",
        MirInstruction::Barrier { .. } => "Barrier",
        MirInstruction::FutureNew { .. } => "FutureNew",
        MirInstruction::FutureSet { .. } => "FutureSet",
        MirInstruction::Await { .. } => "Await",
        MirInstruction::Select { .. } => "Select",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionDietCohort {
    Kept,
    LoweredAway,
    Removed,
}

pub const MIR_INSTRUCTION_KEPT_TAGS: &[&str] = &[
    "Await",
    "ArrayElementWrite",
    "ArrayStateContractClaim",
    "Barrier",
    "BinOp",
    "Branch",
    "Call",
    "Catch",
    "Compare",
    "Const",
    "Copy",
    "CopyOwned",
    "DestroyOwned",
    "LocalContractWrite",
    "RecordFieldContractCheck",
    "RecordValuePublish",
    "Debug",
    "FutureNew",
    "FutureSet",
    "FieldGet",
    "FieldSet",
    "WeakFieldWrite",
    "VariantMake",
    "VariantTag",
    "VariantProject",
    "Jump",
    "KeepAlive",
    "Load",
    "MemOp",
    "PinnedTextOp",
    "PinnedTextResidenceEnter",
    "PinnedTextResidenceFinish",
    "NewBox",
    "NewClosure",
    "Phi",
    "RefNew",
    "ReleaseStrong",
    "Return",
    "CheckedCallOut",
    "CheckedCallOutNormalResult",
    "CheckedCallOutEnd",
    "CheckedCallOutFault",
    "Safepoint",
    "Select",
    "Store",
    "StaticDataLoad",
    "Throw",
    "TypeOp",
    "UnaryOp",
    "WeakRef",
];

pub const MIR_INSTRUCTION_LOWERED_AWAY_TAGS: &[&str] = &[];

pub const MIR_INSTRUCTION_REMOVED_TAGS: &[&str] = &[
    "ArrayGet",
    "ArraySet",
    "BarrierRead",
    "BarrierWrite",
    "BoxCall",
    "Cast",
    "DebugLog",
    "ExternCall",
    "Nop",
    "Print",
    "PluginInvoke",
    "RefGet",
    "RefSet",
    "TypeCheck",
    "WeakLoad",
    "WeakNew",
];

pub const MIR_INSTRUCTION_VOCABULARY_COUNT: usize = MIR_INSTRUCTION_KEPT_TAGS.len()
    + MIR_INSTRUCTION_LOWERED_AWAY_TAGS.len()
    + MIR_INSTRUCTION_REMOVED_TAGS.len();

pub fn instruction_diet_cohort(inst: &MirInstruction) -> InstructionDietCohort {
    match inst {
        MirInstruction::Await { .. }
        | MirInstruction::ArrayElementWrite { .. }
        | MirInstruction::ArrayStateContractClaim { .. }
        | MirInstruction::Barrier { .. }
        | MirInstruction::BinOp { .. }
        | MirInstruction::Branch { .. }
        | MirInstruction::Call { .. }
        | MirInstruction::Catch { .. }
        | MirInstruction::Compare { .. }
        | MirInstruction::StaticDataLoad { .. }
        | MirInstruction::Const { .. }
        | MirInstruction::Copy { .. }
        | MirInstruction::CopyOwned { .. }
        | MirInstruction::DestroyOwned { .. }
        | MirInstruction::LocalContractWrite { .. }
        | MirInstruction::RecordFieldContractCheck { .. }
        | MirInstruction::RecordValuePublish { .. }
        | MirInstruction::Debug { .. }
        | MirInstruction::FutureNew { .. }
        | MirInstruction::FutureSet { .. }
        | MirInstruction::FieldGet { .. }
        | MirInstruction::FieldSet { .. }
        | MirInstruction::WeakFieldWrite { .. }
        | MirInstruction::VariantMake { .. }
        | MirInstruction::VariantTag { .. }
        | MirInstruction::VariantProject { .. }
        | MirInstruction::Jump { .. }
        | MirInstruction::KeepAlive { .. }
        | MirInstruction::Load { .. }
        | MirInstruction::MemOp { .. }
        | MirInstruction::PinnedTextOp { .. }
        | MirInstruction::PinnedTextResidenceEnter { .. }
        | MirInstruction::PinnedTextResidenceFinish { .. }
        | MirInstruction::NewBox { .. }
        | MirInstruction::NewClosure { .. }
        | MirInstruction::Phi { .. }
        | MirInstruction::RefNew { .. }
        | MirInstruction::ReleaseStrong { .. }
        | MirInstruction::Return { .. }
        | MirInstruction::CheckedCallOut { .. }
        | MirInstruction::CheckedCallOutNormalResult { .. }
        | MirInstruction::CheckedCallOutEnd { .. }
        | MirInstruction::CheckedCallOutFault { .. }
        | MirInstruction::Safepoint
        | MirInstruction::Select { .. }
        | MirInstruction::Store { .. }
        | MirInstruction::Throw { .. }
        | MirInstruction::TypeOp { .. }
        | MirInstruction::UnaryOp { .. }
        | MirInstruction::WeakRef { .. } => InstructionDietCohort::Kept,
    }
}

pub fn lowered_away_tag(inst: &MirInstruction) -> Option<&'static str> {
    if instruction_diet_cohort(inst) == InstructionDietCohort::LoweredAway {
        Some(instruction_tag(inst))
    } else {
        None
    }
}
