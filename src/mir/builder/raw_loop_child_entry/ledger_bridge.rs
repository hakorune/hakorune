use super::PreparedLocatedRawLoopChildEntryV1;
use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;
use crate::mir::builder::module_invocation_session::UnpublishedCallableLoopRootScopeV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::cell::RefCell;
use std::rc::Rc;

impl<'source> PreparedLocatedRawLoopChildEntryV1<'source> {
    pub(in crate::mir::builder) fn lower_v1_with_root_scope_and_callable_ledger(
        self,
        builder: &mut MirBuilder,
        function_name: &str,
        debug: bool,
        in_static_box: bool,
        policy: GenericLoopFactsPolicyFrameV1,
        callable_loop_root_scope: &mut UnpublishedCallableLoopRootScopeV1,
        callable_ledger: &Rc<RefCell<CallableSemanticLoweringState>>,
    ) -> Result<ValueId, String> {
        self.lower_v1_with_optional_root_scope(
            builder,
            function_name,
            debug,
            in_static_box,
            policy,
            Some(callable_loop_root_scope),
            Some(callable_ledger),
        )
    }
}
