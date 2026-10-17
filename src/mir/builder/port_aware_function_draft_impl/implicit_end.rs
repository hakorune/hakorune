//! Body completion keeps the original ledger and the body's existing Fault frame.
use std::{cell::RefCell, rc::Rc};
use crate::mir::{MirBuilder, MirInstruction, ValueId, ConstValue};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::ordinary_new_admission::selected;

#[derive(Debug)]
pub(in crate::mir::builder) struct ImplicitCallableEndV1 {
    owner: FunctionOwnerIdV1,
    completion: Rc<VerifiedFunctionCompletionV1>,
    ledger: Rc<OrdinaryNewClaimLedgerV1>,
    state: Rc<RefCell<CallableSemanticLoweringState>>,
}
impl ImplicitCallableEndV1 {
    pub(in crate::mir::builder) fn new(
        owner: FunctionOwnerIdV1, completion: Rc<VerifiedFunctionCompletionV1>,
        ledger: Rc<OrdinaryNewClaimLedgerV1>, state: Rc<RefCell<CallableSemanticLoweringState>>,
    ) -> Result<Self, String> {
        if completion.owner() != owner || state.borrow().owner() != owner {
            return Err("[freeze:contract][implicit-exit/body-owner-mismatch]".into());
        }
        Ok(Self { owner, completion, ledger, state })
    }
    pub(in crate::mir::builder) fn owner(&self) -> FunctionOwnerIdV1 { self.owner }

    pub(in crate::mir::builder) fn emit_source_body_end_v1(
        &self, builder: &mut MirBuilder,
        source: &crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceContextV1,
        end: usize,
    ) -> Result<(), String> {
        let (body, original_end) = self.completion.implicit_body_end()
            .ok_or("[freeze:contract][implicit-exit/source-end-missing]")?;
        if body.owner() != self.owner || source.site() != Some(&body.root())
            || body.kind() != crate::mir::compiler::located::SourceBodyKindV1::Function
            || u32::try_from(end).ok() != Some(original_end) {
            return Err("[freeze:contract][implicit-exit/body-end-drift]".into());
        }
        let value = crate::mir::builder::emission::constant::emit_void(builder)?;
        self.emit(builder, value)
    }

    pub(in crate::mir::builder) fn validate_finalized_body_v1(&self, builder: &MirBuilder) -> Result<(), String> {
        let function = builder.function_state.current_function.as_ref()
            .ok_or("[freeze:contract][implicit-exit/no-function]")?;
        self.ledger.validate_implicit_callable_body_end_v1(self.owner, function)?;
        self.state.borrow().validate_fault_frame(function)
    }

    fn emit(&self, builder: &mut MirBuilder, value: ValueId) -> Result<(), String> {
        let block = builder.function_state.current_block
            .ok_or("[freeze:contract][implicit-exit/no-block]")?;
        let producer = (block, MirInstruction::Const { dst: value, value: ConstValue::Void });
        if self.state.borrow().owner() != self.owner || !builder.function_state.current_function.as_ref()
            .and_then(|function| function.get_block(block))
            .is_some_and(|block| block.instructions.iter().any(|instruction| instruction == &producer.1)) {
            return Err("[freeze:contract][implicit-exit/void-producer-missing]".into());
        }
        selected::emit_implicit_root_home_exit_v1(builder, &mut *self.state.borrow_mut(),
            &self.ledger, self.owner, self.completion.clone(), producer, value)
    }
}
