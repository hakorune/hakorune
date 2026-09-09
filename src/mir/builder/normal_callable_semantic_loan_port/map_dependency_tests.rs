//! Same-source callable dependency harness; never installs a package or lifts its Stop.
use super::*;
use crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1;
use crate::mir::builder::module_lowering_invocation::ModuleLoweringInvocationV1;
use crate::mir::builder::normal_callable_binding_materialization_port::{
    CallableBindingMaterializationPortV1, CallableEntryShapeV1,
};
use crate::mir::builder::raw_invocation_source_transport::RawInvocationRootLineageV1;
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use crate::parser::CallableDeclarationIdentityV1;

impl MirBuilder {
    pub(in crate::mir) fn lower_map_dependency_for_test(
        &mut self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        key: SelectedNormalCallableKeyV1,
        identity: &CallableDeclarationIdentityV1,
        observation: Option<CallableMethodSourceObservationV1>,
        ledger: Rc<OrdinaryNewClaimLedgerV1>,
        loan: Option<
            &mut crate::mir::normal_callable_semantic_package::AppMainDirectCallDispositionLoanV1,
        >,
    ) -> Result<crate::mir::MirFunction, String> {
        let body = input
            .source()
            .root_body()
            .map_err(|e| e.to_string())?
            .statements()
            .to_vec();
        let SelectedNormalCallableKeyV1::Cataloged(key) = key else {
            return Err("dependency requires actual cataloged Main".into());
        };
        self.enter_function_for_test("Main.main/0".into());
        let mut invocation =
            ModuleLoweringInvocationV1::with_collector(self, ModuleDraftCollectorV1::default());
        invocation.with_module_port(|builder, port| {
            let mut inner = RawInvocationChildPortV1::new(port);
            inner.direct_call_loan = loan;
            with_callable_source_scope(
                &mut inner,
                RawInvocationRootLineageV1::Cataloged(key),
                input,
                None,
                observation,
                Rc::clone(&ledger),
                |inner, transport| {
                    inner.with_source_transport_v1(transport, |inner, ()| {
                        inner
                            .callable_ledger
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .select_root_fault_frame()?;
                        ledger.register_app_main_root(input.owner(), identity)?;
                        inner.adopt_callable_entry_values_v1(
                            builder,
                            CallableEntryShapeV1::Static { parameter_count: 0 },
                        )?;
                        inner.lower_body(builder, body)?;
                        inner.complete_construction_stores_v1(builder)
                    })
                },
            )
        })?;
        drop(invocation);
        self.function_state
            .current_function
            .take()
            .ok_or_else(|| "dependency function missing".into())
    }
}
