//! Request-local semantic scope for one constructor body.

use std::cell::RefCell;
use std::rc::Rc;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;

use super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use super::recursive_child_lowering::RawInvocationChildPortV1;

pub(super) fn with_constructor_semantic_scope<R>(
    inner: &mut RawInvocationChildPortV1<'_, '_>,
    input: ResolvedFunctionLoweringInputV1<'_>,
    source_id: &crate::parser::ConstructorSourceIdV1,
    kind: crate::parser::ConstructorSourceKindV1,
    construction: &crate::mir::normal_callable_semantic_package::ConstructionEligibilityV1,
    ordinary_new_claim_ledger: Rc<OrdinaryNewClaimLedgerV1>,
    named_array_field_providers: std::collections::BTreeMap<
        crate::mir::resolved_semantics::SourceExprSiteV1,
        hakorune_mir_defs::CanonicalFieldRefV1,
    >,
    execute: impl FnOnce(&mut RawInvocationChildPortV1<'_, '_>) -> Result<R, String>,
) -> Result<R, String> {
    let source_scope_ledger = Rc::clone(&ordinary_new_claim_ledger);
    let _source_scope = source_scope_ledger.branch_source_scope_v1(input.owner());
    let mut state = CallableSemanticLoweringState::from_exact_source(input)?;
    state.install_construction(source_id, kind, construction)?;
    state.install_named_array_field_providers(named_array_field_providers)?;
    // The shared ledger carries the birth-site index a provider `new`
    // resolves through inside this constructor's construction stores.
    state.lend_ordinary_new_claim_ledger(Rc::clone(&ordinary_new_claim_ledger));
    let state = Rc::new(RefCell::new(state));
    // The constructor body's issued source product (its body-shape rows)
    // travels with its state, as for every other callable source scope.
    let mut scoped = inner.reborrow();
    scoped.callable_ledger = Some(state.clone());
    scoped.ordinary_new_claim_ledger = Some(ordinary_new_claim_ledger);
    scoped.source_input = Some(input);
    let result = execute(&mut scoped);
    drop(scoped);
    match result {
        Ok(value) => {
            Rc::try_unwrap(state)
                .map_err(|_| "[freeze:contract][mir/constructor-semantic/ledger-loan]".to_owned())?
                .into_inner()
                .finish()?;
            Ok(value)
        }
        Err(error) => Err(error),
    }
}
