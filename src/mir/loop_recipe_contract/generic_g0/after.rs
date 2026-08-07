use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::loop_structural_facts::generic_g0::VerifiedGenericG0PostLoopReadV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1};

use super::super::ids::{LoopBindingKeyV1, LoopNodeKeyV1};
use super::super::join_sig::VerifiedLoopAfterBindingV1;
use super::super::schema::LoopValueClassV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0AfterRejectV1 {
    WrongLoop,
    WrongBinding,
    WrongClass,
    PostLoopBindingMismatch,
    OwnerMismatch,
    ReturnAbiMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericAfterEffectG0 {
    after_binding: VerifiedLoopAfterBindingV1,
    post_loop_read: VerifiedGenericG0PostLoopReadV1,
    return_abi: ExactTrivialReturnAbiV1,
    owner: FunctionOwnerIdV1,
    frame: LoopExecutionFrameKeyV1,
}

impl VerifiedGenericAfterEffectG0 {
    pub(crate) fn after_binding(&self) -> &VerifiedLoopAfterBindingV1 {
        &self.after_binding
    }

    pub(crate) fn into_after_binding(self) -> VerifiedLoopAfterBindingV1 {
        self.after_binding
    }

    pub(crate) fn post_loop_read(&self) -> &VerifiedGenericG0PostLoopReadV1 {
        &self.post_loop_read
    }

    pub(crate) const fn return_abi(&self) -> ExactTrivialReturnAbiV1 {
        self.return_abi
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }
}

pub(super) fn issue_after(
    after_binding: VerifiedLoopAfterBindingV1,
    post_loop_read: VerifiedGenericG0PostLoopReadV1,
    return_abi: ExactTrivialReturnAbiV1,
    owner: FunctionOwnerIdV1,
    frame: LoopExecutionFrameKeyV1,
    expected_source_binding: BindingRefV1,
) -> Result<VerifiedGenericAfterEffectG0, GenericG0AfterRejectV1> {
    if after_binding.loop_key() != LoopNodeKeyV1::new(0) {
        return Err(GenericG0AfterRejectV1::WrongLoop);
    }
    if after_binding.binding() != LoopBindingKeyV1::new(1) {
        return Err(GenericG0AfterRejectV1::WrongBinding);
    }
    if after_binding.class() != LoopValueClassV1::I64 {
        return Err(GenericG0AfterRejectV1::WrongClass);
    }
    if post_loop_read.binding() != expected_source_binding {
        return Err(GenericG0AfterRejectV1::PostLoopBindingMismatch);
    }
    if post_loop_read.binding().owner() != owner {
        return Err(GenericG0AfterRejectV1::OwnerMismatch);
    }
    if return_abi != ExactTrivialReturnAbiV1::I64 {
        return Err(GenericG0AfterRejectV1::ReturnAbiMismatch);
    }
    Ok(VerifiedGenericAfterEffectG0 {
        after_binding,
        post_loop_read,
        return_abi,
        owner,
        frame,
    })
}
