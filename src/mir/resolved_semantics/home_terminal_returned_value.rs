//! Exact returned-value source identity for the existing terminal Home walk.
//! Lifecycle acquisition and physical result admission remain separate owners.
use super::{
    ASTNode, BindingRefV1, FunctionOwnerIdV1, OrdinaryObservation, OwnedExprSiteV1,
    PrefixLocalFlow, ResolvedFunctionLoweringInputV1, ResolvedLiteralSourceV1, SourceExprSiteV1,
    SourceStmtSiteV1,
};

/// The exact source value a non-i64 terminal `return <expr>` hands to the
/// caller. Binding classes carry the root identity; ownership transfer is
/// decided by the lifecycle contract, never by this row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TerminalReturnedSourceV1 {
    /// `return %{...}` — construction lives in the literal's MapHomeFlow
    /// row; this site joins the relation and the flow.
    MapLiteral(OwnedExprSiteV1),
    /// `return <binding>` — a live map-installed local or alias root.
    MapLocal(BindingRefV1),
    /// `return <binding>` — a live selected-New Home local; `acquisition`
    /// is its exact `new` site.
    Home {
        binding: BindingRefV1,
        acquisition: OwnedExprSiteV1,
    },
    /// `return <binding>` — a borrowed self-rooted handle root (parameter
    /// or declared handle); the caller keeps ownership.
    Handle(BindingRefV1),
    /// `return new <class>(...)` — the returned value is the fresh
    /// construction at this exact site; the fresh object's ownership
    /// transfers to the caller at the Return edge. This records only the
    /// returned-source identity — the lifecycle authority is the
    /// result-position claim row, and the physical Handle result ABI is a
    /// separate downstream family.
    Construction(OwnedExprSiteV1),
    /// `return "<literal>"` — a string literal value.
    StringLiteral,
    /// `return null` — the null literal value.
    NullLiteral,
    /// `return <float>` — a float literal value.
    FloatLiteral,
}

/// Exact source relation for a non-i64 terminal `return <value>`. Records
/// which source value the site hands to the caller; no physical value, ABI,
/// recipe, JSON, or backend authority is issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalValueReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    returned: TerminalReturnedSourceV1,
}

impl TerminalValueReturnV1 {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn issue(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        value_site: SourceExprSiteV1,
        returned: TerminalReturnedSourceV1,
    ) -> Self {
        Self {
            owner,
            return_site,
            value_site,
            returned,
        }
    }
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }
    pub(crate) fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }
    pub(crate) fn returned(&self) -> &TerminalReturnedSourceV1 {
        &self.returned
    }
}

/// Classify the exact source value a non-i64 `return <expr>` hands to the
/// caller. Literal classes come from the sealed expression-source inventory;
/// binding classes come from the running local flow. Trivial scalar locals
/// and i64-class forms are handled by `return_scalar` before this hook.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn terminal_returned_source(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
) -> Option<TerminalReturnedSourceV1> {
    match input.function().expression_source().literal(site) {
        Some(ResolvedLiteralSourceV1::String(_)) => {
            return Some(TerminalReturnedSourceV1::StringLiteral);
        }
        Some(ResolvedLiteralSourceV1::Null) => {
            return Some(TerminalReturnedSourceV1::NullLiteral);
        }
        Some(ResolvedLiteralSourceV1::Float) => {
            return Some(TerminalReturnedSourceV1::FloatLiteral);
        }
        _ => {}
    }
    if matches!(
        input
            .source()
            .expr_at(&OwnedExprSiteV1::new(input.owner(), site.clone()))
            .map(|expr| expr.node()),
        Ok(ASTNode::New { .. })
    ) {
        return Some(TerminalReturnedSourceV1::Construction(
            OwnedExprSiteV1::new(input.owner(), site.clone()),
        ));
    }
    let OrdinaryObservation::Handle(root) = locals.observe(site)? else {
        return None;
    };
    if locals.is_map_local(root) {
        Some(TerminalReturnedSourceV1::MapLocal(root))
    } else if locals.is_self_rooted_handle(root) {
        Some(TerminalReturnedSourceV1::Handle(root))
    } else {
        locals
            .home_acquisition(root)
            .map(|acquisition| TerminalReturnedSourceV1::Home {
                binding: root,
                acquisition: acquisition.clone(),
            })
    }
}
