//! Entry-receiver local call source observation, not argument/execution authority.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;

pub(crate) fn issue_receiver_local_call(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[LiveHomeV1],
    result: LocalCallResultClassV1,
) -> Option<LocalCallObservationV1> {
    if site.owner() != input.owner()
        || !matches!(
            result,
            LocalCallResultClassV1::Handle | LocalCallResultClassV1::Nullable
        )
    {
        return None;
    }
    let Some((observed_site, call)) = input
        .function()
        .method_calls()
        .find(|(observed_site, _)| *observed_site == site.site())
    else {
        return None;
    };
    // The sealed observation is sole membership — the route already
    // proved this site is the entry-loan `me` receiver. The inventory
    // check only retains that `me` in an instance method resolves to a
    // lexical `Local` binding; `CurrentOwner`/`Other`/`QualifiedUnbound`
    // receiver shapes can never agree with the sealed row.
    if observed_site != site.site()
        || !matches!(
            call.receiver(),
            ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(
                binding
            )) if binding.owner() == input.owner()
                && input.function().binding(binding).is_some_and(|row|
                    matches!(row.kind(), crate::mir::resolved_semantics::BindingKindV1::Receiver))
        )
    {
        return None;
    }
    Some(LocalCallObservationV1::issue(
        input.owner(),
        statement.clone(),
        site.clone(),
        declaration,
        destination,
        prior_homes.iter().cloned().collect(),
        Box::new([]),
        result,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn non_receiver_local_and_unsupported_result_issue_nothing() {
        let ast = crate::parser::NyashParser::parse_from_string(
            "function probe(other) { local item = other.make(7) return item }",
        )
        .unwrap();
        let crate::ast::ASTNode::Program { statements, .. } = ast else {
            panic!("program")
        };
        let function = statements
            .into_iter()
            .find(|node| matches!(node, crate::ast::ASTNode::FunctionDeclaration { .. }))
            .unwrap();
        let unit =
            crate::mir::compiler::VerifiedResolvedSourceUnitV1::resolve_function(function).unwrap();
        let input = unit.root_function_input().unwrap();
        let row = input
            .function()
            .expression_source()
            .initializers()
            .next()
            .unwrap();
        let SourceBindingSiteV1::Local { statement, .. } = row.declaration_site() else {
            panic!("initializer")
        };
        let site = OwnedExprSiteV1::new(input.owner(), row.initializer_site().unwrap().clone());
        for role in [
            LocalCallResultClassV1::Handle,
            LocalCallResultClassV1::Nullable,
            LocalCallResultClassV1::I64,
        ] {
            let result = issue_receiver_local_call(
                input,
                statement,
                &site,
                row.declaration_site().clone(),
                row.binding(),
                &[],
                role,
            );
            assert!(result.is_none());
        }
    }
}
