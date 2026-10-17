//! Original ordinary Core source issuance; no Loop or physical capability.
use std::rc::Rc;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::ClosedOriginalCoreSourceV1;
use crate::mir::resolved_semantics::{
    CoreMethodInstanceTargetIssuerV1, ResolverCoreMethodCallableContractIssuerV1,
    VerifiedResolverOrdinaryCoreMethodSourceV1,
};

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct OrdinaryCoreSourceCallV1 {
    operands: Rc<ClosedOriginalCoreSourceV1>,
    source: VerifiedResolverOrdinaryCoreMethodSourceV1,
}
impl OrdinaryCoreSourceCallV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn source(&self) -> &VerifiedResolverOrdinaryCoreMethodSourceV1 { &self.source }
    pub(in crate::mir::normal_callable_semantic_package) fn operands(&self) -> &Rc<ClosedOriginalCoreSourceV1> { &self.operands }
}

pub(in crate::mir::normal_callable_semantic_package) fn issue_ordinary_core_source_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    operands: &Rc<ClosedOriginalCoreSourceV1>,
) -> Result<Rc<OrdinaryCoreSourceCallV1>, String> {
    let reject = || "[freeze:contract][ordinary-core/source-identity]".to_owned();
    let method = operands.method();
    if input.owner() != method.owner()
        || !input.function().method_call_source_v1(method.site()).is_some_and(|original| std::sync::Arc::ptr_eq(original, method))
    { return Err(reject()); }
    let ledger = input.forest().callable_source_ledger(input.owner()).map_err(|_| reject())?;
    let mut issuer = CoreMethodInstanceTargetIssuerV1::string_box_text(operands.manifest().brand())
        .map_err(|error| format!("[freeze:contract][ordinary-core/target] {error:?}"))?;
    let target = issuer.issue(operands.manifest())
        .map_err(|error| format!("[freeze:contract][ordinary-core/target] {error:?}"))?;
    if target.checked_code_point_law_v1().is_none() {
        return Err("[freeze:contract][ordinary-core/semantic-law]".to_owned());
    }
    let source = ResolverCoreMethodCallableContractIssuerV1::issue_ordinary_source(&ledger, method, target)
        .map_err(|error| format!("[freeze:contract][ordinary-core/source] {error:?}"))?;
    if !std::sync::Arc::ptr_eq(source.source(), method) { return Err(reject()); }
    Ok(Rc::new(OrdinaryCoreSourceCallV1 { source, operands: Rc::clone(operands) }))
}
