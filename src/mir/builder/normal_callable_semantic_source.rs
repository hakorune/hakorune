//! Atomic semantic source authority for one selected callable batch.

use crate::ast::ASTNode;
use crate::mir::compiler::callable_single_loop_recipe_coseal::VerifiedCallableSingleLoopRecipeProductV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    CallableSemanticSourceLedgerView, FunctionOwnerIdV1, FunctionSemanticResolverSessionV1,
    FunctionSyntaxViewV1, ReceiverPolicyV1, ResolveSelectedCallableForestsOutcomeV1,
    VerifiedSemanticOwnerForestV1,
};

use super::callable_declaration_catalog::{
    SameModuleCallableCatalogBrandV1, SameModuleCallableNamespaceV1, SelectedNormalCallableKeyV1,
    SelectedNormalCallableSourceSiteV1, VerifiedSelectedNormalCallableSourceInventoryV1,
};

#[derive(Debug)]
struct VerifiedNormalCallableSemanticSourceRowV1 {
    key: SelectedNormalCallableKeyV1,
    site: SelectedNormalCallableSourceSiteV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedNormalCallableSemanticSourceV1<'source> {
    program: &'source ASTNode,
    catalog_brand: SameModuleCallableCatalogBrandV1,
    rows: Box<[VerifiedNormalCallableSemanticSourceRowV1]>,
}

#[cfg(test)]
#[path = "normal_callable_prepared_ingress_tests.rs"]
mod normal_callable_prepared_ingress_tests;

pub(in crate::mir) struct VerifiedNormalCallableSemanticLoanV1<'source, 'loan> {
    pub(super) lineage: super::raw_invocation_source_transport::RawInvocationRootLineageV1,
    pub(super) catalog_brand: SameModuleCallableCatalogBrandV1,
    _function: &'source ASTNode,
    pub(super) source_ingress: VerifiedNormalCallableSourceIngressReceiptV1<'loan>,
}

/// Exact source-only ingress carried by an already-issued callable loan.
///
/// This is a transport receipt over the resolver forest/projection owners. It
/// is intentionally not a Recipe, Prepared physicalization, or Builder state;
/// the future ingress may consume it once, while the current raw host simply
/// drops it after preserving its existing behavior.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedNormalCallableSourceIngressReceiptV1<'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    ledger: CallableSemanticSourceLedgerView<'source>,
}

/// One-shot, Builder-free assembly of the exact callable source receipt and
/// the already-issued logical Loop product.
///
/// This is deliberately narrower than a physicalization request: it carries
/// no ABI, completion, CFG, SSA, PHI, ValueId, BasicBlockId, selector, or
/// publication state.  The source receipt and logical product are consumed
/// together so a later row cannot accidentally retain two independent owners.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedCallableLoopIngressV1<'source> {
    source: VerifiedNormalCallableSourceIngressReceiptV1<'source>,
    logical: VerifiedCallableSingleLoopRecipeProductV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum PreparedCallableLoopIngressRejectV1 {
    SourceLoopIdentityUnavailable,
    SourceOwnerMismatch,
    LogicalCoreOwnerMismatch,
    LogicalPreludeOwnerMismatch,
    LogicalTailOwnerMismatch,
    LogicalContinuationOwnerMismatch,
    LogicalContextOwnerMismatch,
    LogicalOriginMismatch,
    LogicalSourceKindMismatch,
    LogicalLoopSiteMismatch,
    LogicalFrameMismatch,
    LogicalScopeRegionMismatch,
}

impl<'source> VerifiedNormalCallableSourceIngressReceiptV1<'source> {
    pub(in crate::mir) const fn input(&self) -> ResolvedFunctionLoweringInputV1<'_> {
        self.input
    }

    pub(in crate::mir) const fn ledger(&self) -> &CallableSemanticSourceLedgerView<'_> {
        &self.ledger
    }

    pub(in crate::mir) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.input.owner()
    }

    /// Test-only bridge for an already resolved callable-module input.  The
    /// physical canary must use the exact input/index/header owner pair; this
    /// helper does not resolve source or issue a second semantic owner.
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_resolved_input_for_test(
        input: ResolvedFunctionLoweringInputV1<'source>,
    ) -> Result<Self, String> {
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .map_err(|error| format!("callable source ledger: {error:?}"))?;
        Ok(Self { input, ledger })
    }
}

#[derive(Debug)]
pub(in crate::mir) enum NormalCallableSemanticAdmissionV1<'source> {
    Complete(VerifiedNormalCallableSemanticSourceV1<'source>),
    Deferred,
}

impl<'source> VerifiedNormalCallableSemanticSourceV1<'source> {
    pub(in crate::mir) fn seal(
        program: &'source ASTNode,
        inventory: &VerifiedSelectedNormalCallableSourceInventoryV1,
        is_app_mode: bool,
        resolver: &mut FunctionSemanticResolverSessionV1,
    ) -> Result<NormalCallableSemanticAdmissionV1<'source>, String> {
        if !is_app_mode && !inventory.blockers().is_empty() {
            return Ok(NormalCallableSemanticAdmissionV1::Deferred);
        }
        let ASTNode::Program { statements, .. } = program else {
            return Err("[freeze:contract][mir/callable-semantic/program-required]".to_owned());
        };
        let mut candidates = Vec::with_capacity(inventory.len());
        for (key, site) in inventory.entries() {
            let function = function_at_site(statements, key, site)?;
            let view = view_for_key(function, key)?;
            candidates.push((key.clone(), site.clone(), function, view));
        }
        let views = candidates
            .iter()
            .map(|(_, _, _, view)| *view)
            .collect::<Vec<_>>();
        let forests = match resolver
            .resolve_selected_callable_forests(&views)
            .map_err(|error| format!("[freeze:contract][mir/callable-semantic/forest] {error:?}"))?
        {
            ResolveSelectedCallableForestsOutcomeV1::Complete(forests) => forests,
            ResolveSelectedCallableForestsOutcomeV1::Deferred => {
                return Ok(NormalCallableSemanticAdmissionV1::Deferred)
            }
        };
        if forests.len() != candidates.len() {
            return Err("[freeze:contract][mir/callable-semantic/cardinality]".to_owned());
        }
        let mut rows = Vec::with_capacity(candidates.len());
        for ((key, site, function, view), forest) in candidates.into_iter().zip(forests) {
            let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
                function,
                &forest,
                view.root_profile(),
            )
            .map_err(|error| {
                format!("[freeze:contract][mir/callable-semantic/projection] {error}")
            })?;
            rows.push(VerifiedNormalCallableSemanticSourceRowV1 {
                key,
                site,
                forest,
                projection,
            });
        }
        Ok(NormalCallableSemanticAdmissionV1::Complete(Self {
            program,
            catalog_brand: inventory.brand().clone(),
            rows: rows.into_boxed_slice(),
        }))
    }

    pub(in crate::mir) fn loan<'loan>(
        &'loan self,
        key: &SelectedNormalCallableKeyV1,
    ) -> Result<VerifiedNormalCallableSemanticLoanV1<'source, 'loan>, String> {
        let row = self
            .rows
            .iter()
            .find(|row| &row.key == key)
            .ok_or_else(|| "[freeze:contract][mir/callable-semantic/missing-loan]".to_owned())?;
        let [root] = row.forest.roots() else {
            return Err("[freeze:contract][mir/callable-semantic/root-cardinality]".to_owned());
        };
        let ASTNode::Program { statements, .. } = self.program else {
            unreachable!("seal retained a Program")
        };
        let function = function_at_site(statements, &row.key, &row.site)?;
        let projected = row
            .projection
            .owner_root(function, *root)
            .map_err(|error| {
                format!("[freeze:contract][mir/callable-semantic/owner-root] {error}")
            })?;
        if !std::ptr::eq(projected, function) {
            return Err("[freeze:contract][mir/callable-semantic/root-identity]".to_owned());
        }
        let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
            function,
            &row.forest,
            &row.projection,
        )
        .map_err(|error| format!("[freeze:contract][mir/callable-semantic/input] {error:?}"))?;
        let ledger = row.forest.callable_source_ledger(*root).map_err(|error| {
            format!("[freeze:contract][mir/callable-semantic/ledger] {error:?}")
        })?;
        if input.owner() != ledger.owner() || !std::ptr::eq(input.forest(), &row.forest) {
            return Err("[freeze:contract][mir/callable-semantic/input-owner]".to_owned());
        }
        let source_ingress = VerifiedNormalCallableSourceIngressReceiptV1 { input, ledger };
        let lineage = match &row.key {
            SelectedNormalCallableKeyV1::TopLevel(key) => {
                super::raw_invocation_source_transport::RawInvocationRootLineageV1::TopLevel(
                    key.clone(),
                )
            }
            SelectedNormalCallableKeyV1::Cataloged(key) => {
                super::raw_invocation_source_transport::RawInvocationRootLineageV1::Cataloged(
                    key.clone(),
                )
            }
        };
        Ok(VerifiedNormalCallableSemanticLoanV1 {
            lineage,
            catalog_brand: self.catalog_brand.clone(),
            _function: function,
            source_ingress,
        })
    }

    /// Borrow one exact cataloged callable without exposing the mixed
    /// top-level/catalog selection key outside this authority.
    pub(in crate::mir) fn cataloged_loan<'loan>(
        &'loan self,
        key: &super::CanonicalSameModuleCallableKeyV1,
    ) -> Result<VerifiedNormalCallableSemanticLoanV1<'source, 'loan>, String> {
        self.loan(&SelectedNormalCallableKeyV1::Cataloged(key.clone()))
    }

    pub(in crate::mir) fn keys(&self) -> impl Iterator<Item = &SelectedNormalCallableKeyV1> {
        self.rows.iter().map(|row| &row.key)
    }
}

impl<'source, 'loan> VerifiedNormalCallableSemanticLoanV1<'source, 'loan> {
    pub(super) fn into_source_ingress(self) -> VerifiedNormalCallableSourceIngressReceiptV1<'loan> {
        self.source_ingress
    }

    /// Consume this loan together with one already-issued logical product.
    /// No Builder/session effect occurs here; identity mismatches are rejected
    /// before any physical ingress can be opened.
    pub(super) fn prepare_loop_ingress(
        self,
        logical: VerifiedCallableSingleLoopRecipeProductV1,
    ) -> Result<PreparedCallableLoopIngressV1<'loan>, PreparedCallableLoopIngressRejectV1> {
        let source = self.source_ingress;
        let source_owner = source.owner();
        if source.input().owner() != source_owner || source.ledger().owner() != source_owner {
            return Err(PreparedCallableLoopIngressRejectV1::SourceOwnerMismatch);
        }

        let co_seal = logical.co_seal();
        if co_seal.core().owner() != source_owner {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalCoreOwnerMismatch);
        }
        if logical.prelude().owner() != source_owner {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalPreludeOwnerMismatch);
        }
        if logical.tail().owner() != source_owner {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalTailOwnerMismatch);
        }
        if co_seal.continuation().owner() != source_owner {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalContinuationOwnerMismatch);
        }

        let context = co_seal.context();
        if context.owner() != source_owner {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalContextOwnerMismatch);
        }
        if context.origin() != source.ledger().function_origin() {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalOriginMismatch);
        }
        if context.source_kind() != source.ledger().source_kind() {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalSourceKindMismatch);
        }
        let membership = source
            .ledger()
            .only_loop_site()
            .map_err(|_| PreparedCallableLoopIngressRejectV1::SourceLoopIdentityUnavailable)?;
        if context.loop_site() != membership.source().site() {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalLoopSiteMismatch);
        }
        if context.frame() != membership.frame() {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalFrameMismatch);
        }
        if context.scope_region() != membership.scope_region() {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalScopeRegionMismatch);
        }

        Ok(PreparedCallableLoopIngressV1 { source, logical })
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        super::raw_invocation_source_transport::RawInvocationRootLineageV1,
        VerifiedNormalCallableSourceIngressReceiptV1<'loan>,
    ) {
        (self.lineage, self.source_ingress)
    }

    pub(super) fn lineage(
        &self,
    ) -> &super::raw_invocation_source_transport::RawInvocationRootLineageV1 {
        &self.lineage
    }
}

impl<'source> PreparedCallableLoopIngressV1<'source> {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.source.owner()
    }

    pub(super) fn source(&self) -> &VerifiedNormalCallableSourceIngressReceiptV1<'_> {
        &self.source
    }

    pub(super) fn logical(&self) -> &VerifiedCallableSingleLoopRecipeProductV1 {
        &self.logical
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        VerifiedNormalCallableSourceIngressReceiptV1<'source>,
        VerifiedCallableSingleLoopRecipeProductV1,
    ) {
        (self.source, self.logical)
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn from_source_for_test(
        source: VerifiedNormalCallableSourceIngressReceiptV1<'source>,
        logical: VerifiedCallableSingleLoopRecipeProductV1,
    ) -> Self {
        Self { source, logical }
    }
}

fn function_at_site<'source>(
    statements: &'source [ASTNode],
    key: &SelectedNormalCallableKeyV1,
    site: &SelectedNormalCallableSourceSiteV1,
) -> Result<&'source ASTNode, String> {
    let function = match site {
        SelectedNormalCallableSourceSiteV1::ProgramFunction { statement_index } => {
            statements.get(*statement_index)
        }
        SelectedNormalCallableSourceSiteV1::ProgramBoxMethod {
            statement_index,
            method_key,
        } => match statements.get(*statement_index) {
            Some(ASTNode::BoxDeclaration { methods, .. }) => {
                methods.get_declaration(method_key.as_ref())
            }
            _ => None,
        },
    }
    .ok_or_else(|| "[freeze:contract][mir/callable-semantic/source-site]".to_owned())?;
    if !matches!(function, ASTNode::FunctionDeclaration { .. }) {
        return Err("[freeze:contract][mir/callable-semantic/source-kind]".to_owned());
    }
    match (key, site) {
        (
            SelectedNormalCallableKeyV1::TopLevel(_),
            SelectedNormalCallableSourceSiteV1::ProgramFunction { .. },
        )
        | (
            SelectedNormalCallableKeyV1::Cataloged(_),
            SelectedNormalCallableSourceSiteV1::ProgramBoxMethod { .. },
        ) => Ok(function),
        _ => Err("[freeze:contract][mir/callable-semantic/key-site]".to_owned()),
    }
}

fn view_for_key<'source>(
    function: &'source ASTNode,
    key: &SelectedNormalCallableKeyV1,
) -> Result<FunctionSyntaxViewV1<'source>, String> {
    let ASTNode::FunctionDeclaration { params, body, .. } = function else {
        unreachable!("function_at_site checked the kind")
    };
    let receiver = match key {
        SelectedNormalCallableKeyV1::TopLevel(_) => ReceiverPolicyV1::Absent,
        SelectedNormalCallableKeyV1::Cataloged(key) => match key.namespace() {
            SameModuleCallableNamespaceV1::StaticBoxMethod => ReceiverPolicyV1::StaticCurrentOwner,
            SameModuleCallableNamespaceV1::InstanceBoxMethod => ReceiverPolicyV1::DeclaredInstance,
        },
    };
    Ok(FunctionSyntaxViewV1::from_borrowed_function_parts(
        params, body, receiver,
    ))
}

#[cfg(test)]
mod tests {
    use super::{NormalCallableSemanticAdmissionV1, VerifiedNormalCallableSemanticSourceV1};
    use crate::mir::builder::callable_declaration_catalog::{
        SameModuleCallableNamespaceV1, SelectedNormalCallableKeyV1,
        VerifiedSameModuleCallableDeclarationCatalogV1,
    };
    use crate::mir::resolved_semantics::{FunctionSemanticResolverSessionV1, SourcePathV1};
    use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};
    use crate::parser::NyashParser;

    fn loop_program() -> crate::ast::ASTNode {
        NyashParser::parse_from_string(
            r#"
                static box StringHelpers {
                    int_to_str(n) {
                        local value = me.to_i64(n)
                        local i = 0
                        loop(i < 1) { i = i + 1 }
                        return value
                    }
                    to_i64(x) { return x + 1 }
                }
            "#,
        )
        .expect("callable loop source")
    }

    fn assert_callable_materialization_parity(source: &str) {
        let legacy = MirCompiler::with_options(false)
            .compile_with_source(
                NyashParser::parse_from_string(source).unwrap(),
                Some("callable-materialization.hako"),
            )
            .unwrap();
        let normal = MirCompiler::with_options(false)
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    NyashParser::parse_from_string(source).unwrap(),
                    Some("callable-materialization.hako"),
                    std::collections::HashMap::new(),
                )
                .unwrap(),
            )
            .unwrap();
        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(normal.verification_result, legacy.verification_result);
    }

    #[test]
    fn callable_entry_local_variable_and_rebind_materialization_keeps_parity() {
        assert_callable_materialization_parity(
            "function helper(x) { local y = x y += 1 return y }\n\
             static box Tools { add(x) { local y = x y += 1 return y } }\n\
             box Page { show(x) { local y = x y += 1 return y } }\n\
             function capture(first, second) {\n\
                 local f = fn(){ first + second }\n\
                 return first\n\
             }",
        );

        let mut compiler = MirCompiler::with_options(false);
        assert!(compiler
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    NyashParser::parse_from_string(
                        "function bad(x) { local y = missing return y }",
                    )
                    .unwrap(),
                    Some("callable-materialization-failure.hako"),
                    std::collections::HashMap::new(),
                )
                .unwrap(),
            )
            .is_err());
        compiler
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    NyashParser::parse_from_string(
                        "function good(x) { local y = x y += 1 return y }",
                    )
                    .unwrap(),
                    Some("callable-materialization-reuse.hako"),
                    std::collections::HashMap::new(),
                )
                .unwrap(),
            )
            .unwrap();
    }

    #[test]
    fn mixed_callable_batch_seals_and_reacquires_exact_program_sites() {
        let program = NyashParser::parse_from_string(
            "function helper(x) { return x }\n\
             static box Tools { add(x) { return x } }\n\
             box Page { show(x) { return x } }",
        )
        .unwrap();
        let catalog =
            VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
        let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
        let NormalCallableSemanticAdmissionV1::Complete(source) =
            VerifiedNormalCallableSemanticSourceV1::seal(
                &program,
                catalog.selected_source_inventory(),
                false,
                &mut resolver,
            )
            .unwrap()
        else {
            panic!("mixed callable batch deferred")
        };
        for (key, _) in catalog.selected_source_inventory().entries() {
            source.loan(key).unwrap();
        }
        assert_eq!(source.keys().count(), 3);
    }

    #[test]
    fn callable_loop_handoff_issues_exact_resolver_sites_before_lowering() {
        let program = loop_program();
        let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program)
            .expect("callable catalog");
        let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
        let NormalCallableSemanticAdmissionV1::Complete(source) =
            VerifiedNormalCallableSemanticSourceV1::seal(
                &program,
                catalog.selected_source_inventory(),
                false,
                &mut resolver,
            )
            .unwrap()
        else {
            panic!("callable semantic source deferred")
        };
        let key = source
            .keys()
            .find(|key| {
                matches!(
                    key,
                    SelectedNormalCallableKeyV1::Cataloged(key)
                        if key.owner() == "StringHelpers" && key.name() == "int_to_str"
                )
            })
            .expect("loop callable key")
            .clone();
        let ingress = source.loan(&key).unwrap().into_source_ingress();
        assert_eq!(ingress.owner(), ingress.input().owner());
        assert_eq!(ingress.owner(), ingress.input().source().owner());
        assert_eq!(ingress.owner(), ingress.ledger().owner());
        assert!(ingress.input().callable_index().is_none());
        assert!(ingress.input().callable_header().is_none());

        // The source rows remain reusable: issuing a fresh loan creates a
        // fresh receipt without rewalking or mutating the resolver forest.
        let (_, ingress) = source.loan(&key).unwrap().into_parts();
        let state = super::super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState::from_exact_source(
            ingress.input(),
        )
        .unwrap();
        let schedule = state
            .loop_binding_source_projection()
            .project(SourcePathV1::root_body(2).node())
            .expect("loop schedule");
        assert_eq!(schedule.receipt_count(), 3);
        assert_eq!(
            schedule
                .receipts()
                .filter(|receipt| {
                    matches!(
                        receipt.role(),
                        super::super::normal_callable_loop_handoff::CallableLoopBindingRoleV1::ConditionRead
                            | super::super::normal_callable_loop_handoff::CallableLoopBindingRoleV1::BodyRead
                    )
                })
                .count(),
            2
        );
        assert_eq!(
            schedule
                .receipts()
                .filter(|receipt| {
                    matches!(
                        receipt.role(),
                        super::super::normal_callable_loop_handoff::CallableLoopBindingRoleV1::BodyRebind
                    )
                })
                .count(),
            1
        );
    }

    #[test]
    fn function_call_defers_before_unresolved_argument_child() {
        let program =
            NyashParser::parse_from_string("function helper() { return unknown(missing) }")
                .unwrap();
        let catalog =
            VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
        let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
        assert!(matches!(
            VerifiedNormalCallableSemanticSourceV1::seal(
                &program,
                catalog.selected_source_inventory(),
                false,
                &mut resolver,
            )
            .unwrap(),
            NormalCallableSemanticAdmissionV1::Deferred
        ));
    }

    #[test]
    fn main_methods_are_absent_from_callable_semantic_batch() {
        let program = NyashParser::parse_from_string(
            "static box Main { main() { return 0 } helper() { return 1 } }\n\
             static box Tools { helper() { return 2 } }",
        )
        .unwrap();
        let catalog =
            VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
        let inventory = catalog.selected_source_inventory();
        assert_eq!(inventory.len(), 1);
        let tools = catalog
            .declaration_for(
                SameModuleCallableNamespaceV1::StaticBoxMethod,
                "Tools",
                "helper",
                0,
            )
            .unwrap()
            .key()
            .clone();
        assert!(inventory
            .site(&SelectedNormalCallableKeyV1::Cataloged(tools))
            .is_some());
    }

    #[test]
    fn nonplain_instance_blocker_defers_the_whole_mixed_batch_before_resolution() {
        let program = NyashParser::parse_from_string(
            "function helper(x) { return x }\n\
             record Pair { value: i64 }",
        )
        .unwrap();
        let catalog =
            VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
        assert_eq!(catalog.selected_source_inventory().len(), 1);
        assert_eq!(catalog.selected_source_inventory().blockers().len(), 1);
        let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
        assert!(matches!(
            VerifiedNormalCallableSemanticSourceV1::seal(
                &program,
                catalog.selected_source_inventory(),
                false,
                &mut resolver,
            )
            .unwrap(),
            NormalCallableSemanticAdmissionV1::Deferred
        ));
    }

    #[test]
    fn nonplain_blocker_is_script_only_and_app_remains_complete_eligible() {
        let program = NyashParser::parse_from_string(
            "function helper(x) { return x } record Pair { value: i64 }",
        )
        .unwrap();
        let catalog =
            VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
        let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
        assert!(matches!(
            VerifiedNormalCallableSemanticSourceV1::seal(
                &program,
                catalog.selected_source_inventory(),
                true,
                &mut resolver,
            )
            .unwrap(),
            NormalCallableSemanticAdmissionV1::Complete(_)
        ));
    }

    #[test]
    fn mixed_nonplain_batch_keeps_selected_and_legacy_lowering_in_parity() {
        let text = "function helper() { return 1 }\n\
                    record Pair { value: i64 }\n\
                    Pair { value: 1 }";
        let mut legacy = MirCompiler::with_options(false);
        let legacy = legacy
            .compile_with_source(
                NyashParser::parse_from_string(text).unwrap(),
                Some("callable-nonplain"),
            )
            .unwrap();
        let normal = MirCompiler::with_options(false)
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    NyashParser::parse_from_string(text).unwrap(),
                    Some("callable-nonplain"),
                    std::collections::HashMap::new(),
                )
                .unwrap(),
            )
            .unwrap();
        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(normal.verification_result, legacy.verification_result);
    }

    #[test]
    fn callable_parameter_and_local_bodies_do_not_borrow_the_script_ledger() {
        let text = "function helper(x) { local y = x return y }\n\
                    static box Tools { add(x) { local y = x return y } }\n\
                    box Page { show(x) { local y = x return y } }\n\
                    0";
        let mut legacy = MirCompiler::with_options(false);
        let legacy = legacy
            .compile_with_source(
                NyashParser::parse_from_string(text).unwrap(),
                Some("callable-ledger-scope"),
            )
            .unwrap();
        let normal = MirCompiler::with_options(false)
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    NyashParser::parse_from_string(text).unwrap(),
                    Some("callable-ledger-scope"),
                    std::collections::HashMap::new(),
                )
                .unwrap(),
            )
            .unwrap();
        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(normal.verification_result, legacy.verification_result);
    }
}
