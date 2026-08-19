//! Request-local semantic scope for one constructor body.

use std::cell::RefCell;
use std::rc::Rc;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;

use super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use super::recursive_child_lowering::RawInvocationChildPortV1;

pub(super) fn with_constructor_semantic_scope<R>(
    inner: &mut RawInvocationChildPortV1<'_, '_>,
    input: ResolvedFunctionLoweringInputV1<'_>,
    execute: impl FnOnce(&mut RawInvocationChildPortV1<'_, '_>) -> Result<R, String>,
) -> Result<R, String> {
    let state = Rc::new(RefCell::new(
        CallableSemanticLoweringState::from_exact_source(input)?,
    ));
    let parent = inner.callable_ledger.replace(state.clone());
    let result = execute(inner);
    inner.callable_ledger = parent;
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
