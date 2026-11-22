//! Decision 24's source qualification for comparing a borrowed original parameter.
use super::{
    BindingKindV1, BindingRefV1, OwnedExprSiteV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
    ResolvedLiteralSourceV1, SourceExprSiteV1, SourceStmtSiteV1,
};
use crate::ast::{ASTNode, Span};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::dynamic_operator_contract::{
    issue_dynamic_operator_execution_envelope_v1, DynamicOperatorDomainV1,
    DynamicOperatorValueClassV1,
};
use crate::mir::resolved_value_profile::expression_class::comparison_operator_family_v1;
use crate::mir::CompareOp;
use std::collections::BTreeMap;

/// Immutable source authority retained while the Lower and Verify owners
/// bind the original reads to the actual instruction sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BorrowedIntegerComparisonSourceV1 {
    site: OwnedExprSiteV1,
    read: SourceExprSiteV1,
    formal: BindingRefV1,
    ordinal: u32,
    operator: CompareOp,
    tagged_on_left: bool,
    span: [usize; 4],
}

impl BorrowedIntegerComparisonSourceV1 {
    fn at(
        input: ResolvedFunctionLoweringInputV1<'_>,
        site: &SourceExprSiteV1,
    ) -> Option<Self> {
        use DynamicOperatorValueClassV1 as Class;
        let function = input.function();
        let expressions = function.expression_source();
        let binary = expressions.binary(site)?;
        let operator = binary.operator().compare_op()?;
        if matches!(operator, CompareOp::Eq | CompareOp::Ne) { return None; }
        let family = comparison_operator_family_v1(binary.operator())?;
        let [left, right] = [binary.lhs(), binary.rhs()];
        let tagged_on_left = match (
            Self::is_exact_integer(input, left),
            Self::is_exact_integer(input, right),
        ) {
            (false, true) => true,
            (true, false) => false,
            _ => return None,
        };
        let (read, lhs, rhs) = if tagged_on_left {
            (left, Class::Dynamic, Class::I64)
        } else {
            (right, Class::I64, Class::Dynamic)
        };
        // The mixed signed-ordering envelope admits the original operand order.
        // The exact formal-borrow proof below is still required for extraction.
        issue_dynamic_operator_execution_envelope_v1(DynamicOperatorDomainV1::new(
            family, lhs, rhs,
        ))
        .ok()?;
        let ResolvedLexicalRefV1::Local(binding) = function.variable_ref(read)? else {
            return None;
        };
        let flow = expressions.value_flow()?;
        let value = flow.read(read, binding)?;
        let formal = flow.alias_entry_origin(value, |site| function.variable_ref(site))?;
        let BindingKindV1::Parameter { index: ordinal } = function.binding(formal)?.kind() else {
            return None;
        };
        if Self::declared_parameter_type(input, ordinal)?.is_some() {
            return None;
        }

        let site = OwnedExprSiteV1::new(input.owner(), site.clone());
        let span = input.source().expr_at(&site).ok()?.node().span();
        Some(Self {
            site,
            read: read.clone(),
            formal,
            ordinal,
            operator,
            tagged_on_left,
            span: [span.start, span.end, span.line, span.column],
        })
    }

    fn has_read_only_uses<E>(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        borrows_at: &mut impl FnMut(&SourceExprSiteV1, BindingRefV1, BindingRefV1) -> Result<bool, E>,
    ) -> Result<bool, E> {
        let function = input.function();
        let expressions = function.expression_source();
        let Some(flow) = expressions.value_flow() else { return Ok(false); };
        let formal = self.formal;
        let mut aliases = std::collections::BTreeSet::from([formal]);
        for (site, reference) in function.variable_refs() {
            let ResolvedLexicalRefV1::Local(binding) = *reference else { continue; };
            if flow.read(site, binding).and_then(|value| {
                flow.alias_entry_origin(value, |site| function.variable_ref(site))
            }) != Some(formal) {
                continue;
            }
            aliases.insert(binding);
            let local_read = expressions.binaries().any(|binary| {
                binary.operator().compare_op().is_some()
                    && (binary.lhs() == site || binary.rhs() == site)
            }) || expressions.initializers().any(|row| {
                row.declared_type_name().is_none() && row.initializer_site() == Some(site)
            });
            if !local_read && !borrows_at(site, binding, formal)? {
                return Ok(false);
            }
        }
        let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else {
            return Ok(false);
        };
        Ok(aliases.iter().all(|binding| ledger.uncaptured_binding(*binding))
            && !function.assignment_targets().any(|(_, target)| {
                matches!(target, ResolvedAssignmentTargetV1::BindingRebind(binding) if aliases.contains(binding))
            }))
    }

    fn declared_parameter_type(
        input: ResolvedFunctionLoweringInputV1<'_>,
        index: u32,
    ) -> Option<Option<&str>> {
        let ASTNode::FunctionDeclaration {
            params,
            param_decls,
            ..
        } = input.source().root()
        else {
            return None;
        };
        match param_decls.get(index as usize) {
            Some(parameter) => Some(parameter.declared_type_name.as_deref()),
            None if param_decls.is_empty() => params.get(index as usize).map(|_| None),
            None => None,
        }
    }

    fn is_exact_integer(
        input: ResolvedFunctionLoweringInputV1<'_>,
        site: &SourceExprSiteV1,
    ) -> bool {
        scalar_comparison_class_v1(input, site) == Some(DynamicOperatorValueClassV1::I64)
    }

    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn read(&self) -> &SourceExprSiteV1 {
        &self.read
    }
    pub(crate) fn formal(&self) -> BindingRefV1 {
        self.formal
    }
    pub(crate) fn ordinal(&self) -> u32 {
        self.ordinal
    }
    pub(crate) fn operator(&self) -> CompareOp {
        self.operator
    }
    pub(crate) fn tagged_on_left(&self) -> bool {
        self.tagged_on_left
    }
    pub(crate) fn span(&self) -> Span {
        let [start, end, line, column] = self.span;
        Span {
            start,
            end,
            line,
            column,
        }
    }
}

/// Exact scalar source evidence shared by comparison demand and ordered-formal admission.
pub(crate) fn scalar_comparison_class_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
) -> Option<DynamicOperatorValueClassV1> {
    use DynamicOperatorValueClassV1::{Bool, I64};
    let function = input.function();
    let expressions = function.expression_source();
    let literal_class = |site| match expressions.literal(site) {
        Some(ResolvedLiteralSourceV1::Integer(_)) => Some(I64),
        Some(ResolvedLiteralSourceV1::Bool(_)) => Some(Bool),
        _ if expressions.negative_integer_immediate(site).is_some() => Some(I64),
        _ => None,
    };
    if let Some(class) = literal_class(site) {
        return Some(class);
    }
    let ResolvedLexicalRefV1::Local(binding) = function.variable_ref(site)? else {
        return None;
    };
    let declared = match function.binding(binding)?.kind() {
        BindingKindV1::Parameter { index } =>
            BorrowedIntegerComparisonSourceV1::declared_parameter_type(input, index).flatten(),
        _ => expressions.initializers().find(|row| row.binding() == binding)?.declared_type_name(),
    };
    match declared {
        Some("i64") => Some(I64),
        Some("Bool") => Some(Bool),
        Some(_) => None,
        None => {
            let flow = expressions.value_flow()?;
            let value = flow.read(site, binding)?;
            let root = flow.alias_origin(value, |site| function.variable_ref(site))?;
            let super::source_value_flow::SourceValueOriginV1::Expression(source) = flow.origin(root)? else {
                return None;
            };
            literal_class(source)
        }
    }
}

pub(crate) fn dynamic_scalar_comparison_demand_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    read: &SourceExprSiteV1,
    equality_only: bool,
    scalar: impl FnOnce(&SourceExprSiteV1) -> Option<DynamicOperatorValueClassV1>,
) -> bool {
    use crate::mir::dynamic_operator_contract::DynamicOperatorFamilyV1::{Equal, NotEqual};
    use DynamicOperatorValueClassV1::Dynamic;
    use super::{SourceNodeSiteV1, SourcePathSegmentV1};
    let Some((side, parent)) = read.node().segments().split_last() else { return false; };
    if !matches!(side, SourcePathSegmentV1::Lhs | SourcePathSegmentV1::Rhs) { return false; }
    let parent = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(parent.to_vec()));
    let Some(binary) = input.function().expression_source().binary(&parent) else { return false; };
    if binary.lhs() != read && binary.rhs() != read { return false; }
    let Some(family) = comparison_operator_family_v1(binary.operator()) else { return false; };
    if equality_only && !matches!(family, Equal | NotEqual) { return false; }
    let domain = if binary.lhs() == read {
        scalar(binary.rhs()).map(|class| DynamicOperatorDomainV1::new(family, Dynamic, class))
    } else {
        scalar(binary.lhs()).map(|class| DynamicOperatorDomainV1::new(family, class, Dynamic))
    };
    domain.is_some_and(|domain| issue_dynamic_operator_execution_envelope_v1(domain).is_ok())
}

type LiveHome = super::home_new_prefix::LiveHomeV1;
pub(in crate::mir) type SourcesByStatement =
    BTreeMap<SourceStmtSiteV1, Vec<BorrowedIntegerComparisonSourceV1>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct ComparisonPrefix {
    pub(crate) source: BorrowedIntegerComparisonSourceV1,
    pub(crate) homes: Box<[LiveHome]>,
}
pub(in crate::mir) type ComparisonPrefixHomes = BTreeMap<OwnedExprSiteV1, Option<ComparisonPrefix>>;

pub(super) fn sources_by_statement<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    borrows_at: &mut impl FnMut(&SourceExprSiteV1, BindingRefV1, BindingRefV1) -> Result<bool, E>,
) -> Result<SourcesByStatement, E> {
    let function = input.function();
    if !function.declaration_sites().any(|site| matches!(site,
        super::SourceBindingSiteV1::Parameter { index }
            if BorrowedIntegerComparisonSourceV1::declared_parameter_type(input, *index) == Some(None)))
    {
        return Ok(BTreeMap::new());
    }
    let inventory = function.source_site_inventory();
    let expression = function.expression_source();
    let mut by_statement = SourcesByStatement::new();
    for binary in expression.binaries() {
        if let Some(source) = BorrowedIntegerComparisonSourceV1::at(input, binary.site()) {
            if !source.has_read_only_uses(input, borrows_at)? {
                continue;
            }
            if let Some(statement) = inventory.containing_statement(binary.site()) {
                by_statement
                    .entry(statement.clone())
                    .or_default()
                    .push(source);
            }
        }
    }
    // The Home walk observes each reached statement before evaluation.
    // A statement whose expression can change this prefix needs an exact
    // expression ownership point; only passive reads/literals/operators
    // can borrow the statement's snapshot here.
    by_statement.retain(|statement, _| {
        inventory
            .expression_sites()
            .filter(|site| {
                site.node() != statement.node()
                    && inventory.containing_statement(site) == Some(statement)
            })
            .all(|site| {
                let site = OwnedExprSiteV1::new(input.owner(), site.clone());
                input
                    .source()
                    .expr_at(&site)
                    .is_ok_and(|row| match row.node() {
                        ASTNode::Literal { .. } | ASTNode::Variable { .. } => true,
                        ASTNode::UnaryOp { .. } => {
                            expression.negative_integer_immediate(site.site()).is_some()
                        }
                        ASTNode::BinaryOp { .. } => {
                            expression.binary(site.site()).is_some_and(|row| {
                                row.operator().compare_op().is_some()
                                    || matches!(
                                        row.operator(),
                                        super::ResolvedBinaryOperatorV1::And
                                            | super::ResolvedBinaryOperatorV1::Or
                                    )
                            })
                        }
                        _ => false,
                    })
            })
    });
    Ok(by_statement)
}

pub(super) fn record_prefix_homes(
    comparisons: &[BorrowedIntegerComparisonSourceV1],
    homes: &[LiveHome],
    observations: &mut ComparisonPrefixHomes,
) {
    for comparison in comparisons {
        observations
            .entry(comparison.site().clone())
            .and_modify(|entry| {
                if entry.as_ref().map(|entry| entry.homes.as_ref()) != Some(homes) {
                    *entry = None;
                }
            })
            .or_insert_with(|| {
                Some(ComparisonPrefix {
                    source: comparison.clone(),
                    homes: homes.into(),
                })
            });
    }
}
