use std::collections::{BTreeMap, BTreeSet};

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::function_view::ReceiverPolicyV1;
use crate::mir::resolved_semantics::source_site::{
    FunctionOriginV1, SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};
use crate::mir::resolved_semantics::FunctionSyntaxViewV1;
use crate::mir::resolved_semantics::{
    EnumMatchDemandV1, EnumVariantAdmissionV1, EnumVariantDemandV1, RecordSchemaDemandV1, ScriptSyntaxViewV1,
    VerifiedScriptRootDemandWindowV1,
};

use super::ids::{ShadowBindingOrdinalV0, ShadowRegionIdV0, ShadowScopeIdV0};
use super::owner_boundary::ShadowLambdaSyntaxV0;
use super::path::ShadowSourcePathV0;
use super::product::{
    ShadowAncestorCaptureAccessV0, ShadowAncestorCaptureEventV0, ShadowAssignmentTargetV0,
    ShadowBindingKindV0, ShadowBindingRecordV0, ShadowControlExitV0, ShadowDirectCallUseV0,
    ShadowExitOriginV0, ShadowExitRecordV0, ShadowLexicalRefV0, ShadowMethodCallObservationV0,
    ShadowMethodCallReceiverV0, ShadowQualifiedReceiverDispositionV0, ShadowRegionKindV0,
    ShadowRegionRecordV0, ShadowResolveErrorV0, ShadowResolvedFunctionV0, ShadowResolvedOwnerV0,
    ShadowScopeKindV0, ShadowScopeRecordV0,
};
use super::root_traversal::ShadowRootTraversalInputV1;
use super::traversal_profile::ShadowTraversalProfileV1;
#[derive(Debug)]
struct ResolverScopeFrameV0 {
    id: ShadowScopeIdV0,
    names: BTreeMap<String, ShadowBindingOrdinalV0>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShadowLambdaModeV0 {
    Reject,
    Inventory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShadowMethodCallObservationModeV0 {
    Disabled,
    All,
}

pub(super) struct ShadowResolverV0<'ast, 'schema> {
    function_scope: ShadowScopeIdV0,
    function_region: ShadowRegionIdV0,
    next_binding: u32,
    next_scope: u32,
    next_region: u32,
    receiver: Option<ShadowBindingOrdinalV0>,
    scope_stack: Vec<ResolverScopeFrameV0>,
    region_stack: Vec<ShadowRegionIdV0>,
    loop_stack: Vec<ShadowRegionIdV0>,
    bindings: BTreeMap<ShadowBindingOrdinalV0, ShadowBindingRecordV0>,
    scopes: BTreeMap<ShadowScopeIdV0, ShadowScopeRecordV0>,
    regions: BTreeMap<ShadowRegionIdV0, ShadowRegionRecordV0>,
    declarations: BTreeMap<SourceBindingSiteV1, ShadowBindingOrdinalV0>,
    variable_uses: BTreeMap<SourceExprSiteV1, ShadowLexicalRefV0>,
    assignment_targets: BTreeMap<SourceExprSiteV1, ShadowAssignmentTargetV0>,
    ancestor_capture_events: Vec<ShadowAncestorCaptureEventV0>,
    direct_calls: BTreeMap<SourceExprSiteV1, ShadowDirectCallUseV0>,
    resolved_exits: BTreeMap<SourceStmtSiteV1, ShadowExitRecordV0>,
    lambda_mode: ShadowLambdaModeV0,
    lambdas: Vec<ShadowLambdaSyntaxV0<'ast>>,
    ancestor_names: BTreeSet<Box<str>>,
    qualified_receiver_requests: BTreeSet<SourceExprSiteV1>,
    qualified_receiver_dispositions:
        BTreeMap<SourceExprSiteV1, ShadowQualifiedReceiverDispositionV0>,
    receiver_policy: ReceiverPolicyV1,
    method_call_observation_mode: ShadowMethodCallObservationModeV0,
    traversal_profile: ShadowTraversalProfileV1,
    method_call_observations: BTreeMap<SourceExprSiteV1, ShadowMethodCallObservationV0>,
    pub(super) record_schema_demand: Option<&'schema dyn RecordSchemaDemandV1>,
    pub(super) enum_variant_demand: Option<&'schema dyn EnumVariantDemandV1>,
    pub(super) enum_match_demand: Option<&'schema dyn EnumMatchDemandV1>,
    pub(super) record_literal_demands: BTreeMap<SourceExprSiteV1, u32>,
    pub(super) enum_variant_demands: BTreeMap<SourceExprSiteV1, EnumVariantAdmissionV1>,
    pub(super) enum_match_demands: BTreeSet<SourceExprSiteV1>,
    pub(super) qmark_propagation_sites: BTreeSet<SourceExprSiteV1>,
    pub(super) match_control_sites: BTreeSet<SourceExprSiteV1>,
}

pub(super) fn resolve_function_shadow_v0(
    _function_origin: FunctionOriginV1,
    function: &ASTNode,
) -> Result<ShadowResolvedFunctionV0, ShadowResolveErrorV0> {
    let Some(view) = FunctionSyntaxViewV1::from_ast(function) else {
        return Err(ShadowResolveErrorV0::ExpectedFunctionDeclaration);
    };
    resolve_function_shadow_view_v0(view)
}

pub(in crate::mir::resolved_semantics) fn resolve_function_shadow_view_v0(
    view: FunctionSyntaxViewV1<'_>,
) -> Result<ShadowResolvedFunctionV0, ShadowResolveErrorV0> {
    resolve_shadow_view(
        view,
        ShadowLambdaModeV0::Reject,
        BTreeSet::new(),
        ShadowMethodCallObservationModeV0::Disabled,
    )
    .map(|owner| owner.function)
}

pub(in crate::mir::resolved_semantics) fn resolve_owner_shadow_view_v0<'ast>(
    view: FunctionSyntaxViewV1<'ast>,
    ancestor_names: BTreeSet<Box<str>>,
) -> Result<ShadowResolvedOwnerV0<'ast>, ShadowResolveErrorV0> {
    resolve_owner_shadow_view_with_profile_v0(
        view,
        ancestor_names,
        ShadowTraversalProfileV1::FullFunctionV1,
    )
}

pub(in crate::mir::resolved_semantics) fn resolve_owner_shadow_view_with_profile_v0<'ast>(
    view: FunctionSyntaxViewV1<'ast>,
    ancestor_names: BTreeSet<Box<str>>,
    traversal_profile: ShadowTraversalProfileV1,
) -> Result<ShadowResolvedOwnerV0<'ast>, ShadowResolveErrorV0> {
    resolve_shadow_view_with_profile(
        view,
        ShadowLambdaModeV0::Inventory,
        ancestor_names,
        ShadowMethodCallObservationModeV0::Disabled,
        traversal_profile,
    )
}

pub(in crate::mir::resolved_semantics) fn resolve_script_shadow_view_v0<'ast>(
    view: ScriptSyntaxViewV1<'ast>,
    window: &'ast VerifiedScriptRootDemandWindowV1,
    record_schemas: &dyn RecordSchemaDemandV1,
    enum_variants: &dyn EnumVariantDemandV1,
    enum_matches: &dyn EnumMatchDemandV1,
) -> Result<ShadowResolvedFunctionV0, ShadowResolveErrorV0> {
    let input =
        ShadowRootTraversalInputV1::sparse_script(
            view,
            window,
            record_schemas,
            enum_variants,
            enum_matches,
        );
    let profile = input.root_profile();
    traverse_shadow_root_v1(
        input,
        ShadowLambdaModeV0::Reject,
        BTreeSet::new(),
        BTreeSet::new(),
        ShadowMethodCallObservationModeV0::Disabled,
    )
    .map(|resolver| resolver.finish_owner(profile).function)
}

pub(in crate::mir::resolved_semantics) fn resolve_script_owner_shadow_view_v0<'ast>(
    view: ScriptSyntaxViewV1<'ast>,
    window: &'ast VerifiedScriptRootDemandWindowV1,
    record_schemas: &dyn RecordSchemaDemandV1,
    enum_variants: &dyn EnumVariantDemandV1,
    enum_matches: &dyn EnumMatchDemandV1,
) -> Result<ShadowResolvedOwnerV0<'ast>, ShadowResolveErrorV0> {
    let input =
        ShadowRootTraversalInputV1::sparse_script(
            view,
            window,
            record_schemas,
            enum_variants,
            enum_matches,
        );
    let profile = input.root_profile();
    traverse_shadow_root_v1(
        input,
        ShadowLambdaModeV0::Inventory,
        BTreeSet::new(),
        BTreeSet::new(),
        ShadowMethodCallObservationModeV0::Disabled,
    )
    .map(|resolver| resolver.finish_owner(profile))
}

fn resolve_shadow_view<'ast>(
    view: FunctionSyntaxViewV1<'ast>,
    lambda_mode: ShadowLambdaModeV0,
    ancestor_names: BTreeSet<Box<str>>,
    method_call_observation_mode: ShadowMethodCallObservationModeV0,
) -> Result<ShadowResolvedOwnerV0<'ast>, ShadowResolveErrorV0> {
    resolve_shadow_view_with_profile(
        view,
        lambda_mode,
        ancestor_names,
        method_call_observation_mode,
        ShadowTraversalProfileV1::FullFunctionV1,
    )
}

fn resolve_shadow_view_with_profile<'ast>(
    view: FunctionSyntaxViewV1<'ast>,
    lambda_mode: ShadowLambdaModeV0,
    ancestor_names: BTreeSet<Box<str>>,
    method_call_observation_mode: ShadowMethodCallObservationModeV0,
    traversal_profile: ShadowTraversalProfileV1,
) -> Result<ShadowResolvedOwnerV0<'ast>, ShadowResolveErrorV0> {
    let input = ShadowRootTraversalInputV1::dense_with_profile(view, traversal_profile);
    let root_profile = input.root_profile();
    traverse_shadow_root_v1(
        input,
        lambda_mode,
        ancestor_names,
        BTreeSet::new(),
        method_call_observation_mode,
    )
    .map(|resolver| resolver.finish_owner(root_profile))
}

/// Reuses the one shadow lexical traversal for exact qualified receivers.
///
/// This entry deliberately has no `FunctionOriginV1`: it publishes only
/// Bound/ProvenUnbound observations and never constructs a semantic owner.
pub(in crate::mir) fn observe_qualified_receiver_shadow_view_v0(
    view: FunctionSyntaxViewV1<'_>,
    requested_sites: BTreeSet<SourceExprSiteV1>,
) -> Result<BTreeMap<SourceExprSiteV1, ShadowQualifiedReceiverDispositionV0>, ShadowResolveErrorV0>
{
    traverse_shadow_root_v1(
        ShadowRootTraversalInputV1::dense(view),
        ShadowLambdaModeV0::Reject,
        BTreeSet::new(),
        requested_sites,
        ShadowMethodCallObservationModeV0::Disabled,
    )?
    .finish_qualified_receiver_observations()
}

/// Reuses the sole shadow traversal to inventory every MethodCall site.
pub(in crate::mir) fn observe_method_calls_shadow_view_v0(
    view: FunctionSyntaxViewV1<'_>,
) -> Result<BTreeMap<SourceExprSiteV1, ShadowMethodCallObservationV0>, ShadowResolveErrorV0> {
    traverse_shadow_root_v1(
        ShadowRootTraversalInputV1::dense(view),
        ShadowLambdaModeV0::Reject,
        BTreeSet::new(),
        BTreeSet::new(),
        ShadowMethodCallObservationModeV0::All,
    )?
    .finish_method_call_observations()
}

fn traverse_shadow_root_v1<'ast, 'schema>(
    input: ShadowRootTraversalInputV1<'ast, 'schema>,
    lambda_mode: ShadowLambdaModeV0,
    ancestor_names: BTreeSet<Box<str>>,
    qualified_receiver_requests: BTreeSet<SourceExprSiteV1>,
    method_call_observation_mode: ShadowMethodCallObservationModeV0,
) -> Result<ShadowResolverV0<'ast, 'schema>, ShadowResolveErrorV0> {
    let receiver_policy = input.receiver_policy();

    let mut resolver = ShadowResolverV0::new(
        lambda_mode,
        ancestor_names,
        qualified_receiver_requests,
        receiver_policy,
        method_call_observation_mode,
        input.traversal_profile(),
        input.record_schema_demand(),
        input.enum_variant_demand(),
        input.enum_match_demand(),
    );
    if receiver_policy == ReceiverPolicyV1::DeclaredInstance {
        resolver.declare_binding(
            "me",
            ShadowBindingKindV0::Receiver,
            SourceBindingSiteV1::Receiver,
        )?;
        resolver.receiver = resolver.lookup("me");
    }
    for (index, name) in input.params().iter().enumerate() {
        resolver.declare_binding(
            name,
            ShadowBindingKindV0::Parameter {
                index: index as u32,
            },
            SourceBindingSiteV1::Parameter {
                index: index as u32,
            },
        )?;
    }
    let body_path = input.body_path();
    let (body_region, _) = resolver.enter_region_scope(
        ShadowRegionKindV0::Sequence,
        ShadowScopeKindV0::LexicalBlock,
        &body_path,
    );
    let body_result = input.resolve_body(&mut resolver);
    resolver.leave_region_scope(body_region);
    body_result?;
    Ok(resolver)
}

impl<'ast, 'schema> ShadowResolverV0<'ast, 'schema> {
    pub(super) fn allows_statement(&self, statement: &ASTNode) -> bool {
        self.traversal_profile.allows_statement(statement)
    }

    pub(super) fn allows_expression(&self, expression: &ASTNode) -> bool {
        self.traversal_profile.allows_expression(expression)
    }

    pub(super) const fn is_script_lexical_core(&self) -> bool {
        matches!(
            self.traversal_profile,
            ShadowTraversalProfileV1::ScriptLexicalCoreV1
        )
    }

    fn new(
        lambda_mode: ShadowLambdaModeV0,
        ancestor_names: BTreeSet<Box<str>>,
        qualified_receiver_requests: BTreeSet<SourceExprSiteV1>,
        receiver_policy: ReceiverPolicyV1,
        method_call_observation_mode: ShadowMethodCallObservationModeV0,
        traversal_profile: ShadowTraversalProfileV1,
        record_schema_demand: Option<&'schema dyn RecordSchemaDemandV1>,
        enum_variant_demand: Option<&'schema dyn EnumVariantDemandV1>,
        enum_match_demand: Option<&'schema dyn EnumMatchDemandV1>,
    ) -> Self {
        let function_scope = ShadowScopeIdV0::new(0);
        let function_region = ShadowRegionIdV0::new(0);
        let mut scopes = BTreeMap::new();
        scopes.insert(
            function_scope,
            ShadowScopeRecordV0 {
                kind: ShadowScopeKindV0::Function,
                parent: None,
                declarations: Box::new([]),
                origin: None,
            },
        );
        let mut regions = BTreeMap::new();
        regions.insert(
            function_region,
            ShadowRegionRecordV0 {
                kind: ShadowRegionKindV0::Function,
                parent: None,
                lexical_scope: Some(function_scope),
                origin: None,
            },
        );
        Self {
            function_scope,
            function_region,
            next_binding: 0,
            next_scope: 1,
            next_region: 1,
            receiver: None,
            scope_stack: vec![ResolverScopeFrameV0 {
                id: function_scope,
                names: BTreeMap::new(),
            }],
            region_stack: vec![function_region],
            loop_stack: Vec::new(),
            bindings: BTreeMap::new(),
            scopes,
            regions,
            declarations: BTreeMap::new(),
            variable_uses: BTreeMap::new(),
            assignment_targets: BTreeMap::new(),
            ancestor_capture_events: Vec::new(),
            direct_calls: BTreeMap::new(),
            resolved_exits: BTreeMap::new(),
            lambda_mode,
            lambdas: Vec::new(),
            ancestor_names,
            qualified_receiver_requests,
            qualified_receiver_dispositions: BTreeMap::new(),
            receiver_policy,
            method_call_observation_mode,
            traversal_profile,
            method_call_observations: BTreeMap::new(),
            record_schema_demand,
            enum_variant_demand,
            enum_match_demand,
            record_literal_demands: BTreeMap::new(),
            enum_variant_demands: BTreeMap::new(),
            enum_match_demands: BTreeSet::new(),
            qmark_propagation_sites: BTreeSet::new(),
            match_control_sites: BTreeSet::new(),
        }
    }

    fn finish_owner(
        self,
        root_profile: super::super::SemanticOwnerRootProfileV1,
    ) -> ShadowResolvedOwnerV0<'ast> {
        ShadowResolvedOwnerV0 {
            function: ShadowResolvedFunctionV0 {
                root_profile,
                function_scope: self.function_scope,
                function_region: self.function_region,
                bindings: self.bindings,
                scopes: self.scopes,
                regions: self.regions,
                declarations: self.declarations,
                variable_uses: self.variable_uses,
                assignment_targets: self.assignment_targets,
                ancestor_capture_events: self.ancestor_capture_events.into_boxed_slice(),
                direct_calls: self.direct_calls,
                resolved_exits: self.resolved_exits,
                record_literal_demands: self.record_literal_demands,
                enum_variant_demands: self.enum_variant_demands,
                enum_match_demands: self.enum_match_demands,
                qmark_propagation_sites: self.qmark_propagation_sites,
                match_control_sites: self.match_control_sites,
            },
            lambdas: self.lambdas.into_boxed_slice(),
        }
    }

    fn finish_qualified_receiver_observations(
        self,
    ) -> Result<
        BTreeMap<SourceExprSiteV1, ShadowQualifiedReceiverDispositionV0>,
        ShadowResolveErrorV0,
    > {
        let missing = self
            .qualified_receiver_requests
            .iter()
            .filter(|site| !self.qualified_receiver_dispositions.contains_key(*site))
            .cloned()
            .collect::<Vec<_>>();
        let extra = self
            .qualified_receiver_dispositions
            .keys()
            .filter(|site| !self.qualified_receiver_requests.contains(*site))
            .cloned()
            .collect::<Vec<_>>();
        if !missing.is_empty() || !extra.is_empty() {
            return Err(
                ShadowResolveErrorV0::QualifiedReceiverObservationCoverageMismatch {
                    missing: missing.into_boxed_slice(),
                    extra: extra.into_boxed_slice(),
                },
            );
        }
        Ok(self.qualified_receiver_dispositions)
    }

    fn finish_method_call_observations(
        self,
    ) -> Result<BTreeMap<SourceExprSiteV1, ShadowMethodCallObservationV0>, ShadowResolveErrorV0>
    {
        Ok(self.method_call_observations)
    }

    pub(super) fn record_lambda(
        &mut self,
        lambda: &'ast ASTNode,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        if self.lambda_mode == ShadowLambdaModeV0::Reject {
            return Err(ShadowResolveErrorV0::UnsupportedExpression {
                kind: "Lambda",
                site: path.expr(),
            });
        }
        let mut visible_bindings = BTreeMap::new();
        for frame in &self.scope_stack {
            for (name, binding) in &frame.names {
                visible_bindings.insert(name.clone().into_boxed_str(), *binding);
            }
        }
        self.lambdas.push(ShadowLambdaSyntaxV0::new(
            path.expr(),
            self.current_scope(),
            visible_bindings,
            lambda,
        ));
        Ok(())
    }

    pub(super) fn current_scope(&self) -> ShadowScopeIdV0 {
        self.scope_stack.last().expect("function scope exists").id
    }

    pub(super) fn current_region(&self) -> ShadowRegionIdV0 {
        *self.region_stack.last().expect("function region exists")
    }

    pub(super) fn function_region(&self) -> ShadowRegionIdV0 {
        self.function_region
    }

    pub(super) fn nearest_loop(&self) -> Option<ShadowRegionIdV0> {
        self.loop_stack.last().copied()
    }

    pub(super) fn receiver(&self) -> Option<ShadowBindingOrdinalV0> {
        self.receiver
    }

    pub(super) fn lookup(&self, name: &str) -> Option<ShadowBindingOrdinalV0> {
        self.scope_stack
            .iter()
            .rev()
            .find_map(|frame| frame.names.get(name).copied())
    }

    pub(super) fn ancestor_is_visible(&self, name: &str) -> bool {
        self.ancestor_names.contains(name)
    }

    pub(super) fn record_use(&mut self, site: SourceExprSiteV1, lexical_ref: ShadowLexicalRefV0) {
        self.variable_uses.insert(site, lexical_ref);
    }

    pub(super) fn record_ancestor_capture(
        &mut self,
        site: SourceExprSiteV1,
        name: &str,
        access: ShadowAncestorCaptureAccessV0,
    ) {
        self.ancestor_capture_events
            .push(ShadowAncestorCaptureEventV0 {
                site,
                name: name.into(),
                access,
            });
    }

    pub(super) fn qualified_receiver_is_requested(&self, site: &SourceExprSiteV1) -> bool {
        self.qualified_receiver_requests.contains(site)
    }

    pub(super) fn observes_all_method_calls(&self) -> bool {
        self.method_call_observation_mode == ShadowMethodCallObservationModeV0::All
    }

    pub(super) const fn receiver_policy(&self) -> ReceiverPolicyV1 {
        self.receiver_policy
    }

    pub(super) fn request_qualified_receiver(&mut self, site: SourceExprSiteV1) {
        self.qualified_receiver_requests.insert(site);
    }

    pub(super) fn qualified_receiver_disposition(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<ShadowQualifiedReceiverDispositionV0> {
        self.qualified_receiver_dispositions.get(site).copied()
    }

    pub(super) fn record_method_call_observation(
        &mut self,
        site: SourceExprSiteV1,
        receiver_site: SourceExprSiteV1,
        receiver: ShadowMethodCallReceiverV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let observation = ShadowMethodCallObservationV0::new(receiver_site, receiver);
        if self
            .method_call_observations
            .insert(site.clone(), observation)
            .is_some()
        {
            return Err(ShadowResolveErrorV0::DuplicateMethodCallObservation { site });
        }
        Ok(())
    }

    pub(super) fn record_qualified_receiver_disposition(
        &mut self,
        site: SourceExprSiteV1,
        disposition: ShadowQualifiedReceiverDispositionV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        if self
            .qualified_receiver_dispositions
            .insert(site.clone(), disposition)
            .is_some()
        {
            return Err(ShadowResolveErrorV0::DuplicateQualifiedReceiverObservation { site });
        }
        Ok(())
    }

    pub(super) fn record_assignment(
        &mut self,
        site: SourceExprSiteV1,
        target: ShadowAssignmentTargetV0,
    ) {
        self.assignment_targets.insert(site, target);
    }

    pub(super) fn record_direct_call(
        &mut self,
        site: SourceExprSiteV1,
        name: &str,
        arity: usize,
    ) -> Result<(), ShadowResolveErrorV0> {
        let arity = u32::try_from(arity)
            .map_err(|_| ShadowResolveErrorV0::FunctionCallArityOverflow { site: site.clone() })?;
        let record = ShadowDirectCallUseV0 {
            name: name.into(),
            arity,
        };
        if self.direct_calls.insert(site.clone(), record).is_some() {
            return Err(ShadowResolveErrorV0::DuplicateDirectCallSite { site });
        }
        Ok(())
    }

    pub(super) fn record_exit(
        &mut self,
        site: SourceStmtSiteV1,
        origin: ShadowExitOriginV0,
        transfer: ShadowControlExitV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let record = ShadowExitRecordV0 {
            source_region: self.current_region(),
            origin,
            transfer,
        };
        if self.resolved_exits.insert(site.clone(), record).is_some() {
            return Err(ShadowResolveErrorV0::DuplicateExitSite { site });
        }
        Ok(())
    }

    pub(super) fn declare_binding(
        &mut self,
        name: &str,
        kind: ShadowBindingKindV0,
        origin: SourceBindingSiteV1,
    ) -> Result<ShadowBindingOrdinalV0, ShadowResolveErrorV0> {
        let frame = self.scope_stack.last_mut().expect("function scope exists");
        if frame.names.contains_key(name) {
            return Err(ShadowResolveErrorV0::SameScopeRedeclaration { name: name.into() });
        }
        let binding = ShadowBindingOrdinalV0::new(self.next_binding);
        self.next_binding += 1;
        frame.names.insert(name.to_owned(), binding);
        self.bindings.insert(
            binding,
            ShadowBindingRecordV0 {
                diagnostic_name: name.into(),
                kind,
                owner_scope: frame.id,
                origin: origin.clone(),
            },
        );
        self.declarations.insert(origin, binding);
        let scope = self.scopes.get_mut(&frame.id).expect("scope record exists");
        let mut declarations = scope.declarations.to_vec();
        declarations.push(binding);
        scope.declarations = declarations.into_boxed_slice();
        Ok(binding)
    }

    pub(super) fn enter_region_scope(
        &mut self,
        region_kind: ShadowRegionKindV0,
        scope_kind: ShadowScopeKindV0,
        path: &ShadowSourcePathV0,
    ) -> (ShadowRegionIdV0, ShadowScopeIdV0) {
        self.enter_region_scope_with_origins(region_kind, scope_kind, path, path)
    }

    pub(super) fn enter_region_scope_with_origins(
        &mut self,
        region_kind: ShadowRegionKindV0,
        scope_kind: ShadowScopeKindV0,
        region_path: &ShadowSourcePathV0,
        scope_path: &ShadowSourcePathV0,
    ) -> (ShadowRegionIdV0, ShadowScopeIdV0) {
        let parent_region = self.current_region();
        let parent_scope = self.current_scope();
        let region = ShadowRegionIdV0::new(self.next_region);
        self.next_region += 1;
        let scope = ShadowScopeIdV0::new(self.next_scope);
        self.next_scope += 1;
        self.regions.insert(
            region,
            ShadowRegionRecordV0 {
                kind: region_kind,
                parent: Some(parent_region),
                lexical_scope: Some(scope),
                origin: Some(region_path.node()),
            },
        );
        self.scopes.insert(
            scope,
            ShadowScopeRecordV0 {
                kind: scope_kind,
                parent: Some(parent_scope),
                declarations: Box::new([]),
                origin: Some(scope_path.node()),
            },
        );
        self.region_stack.push(region);
        self.scope_stack.push(ResolverScopeFrameV0 {
            id: scope,
            names: BTreeMap::new(),
        });
        (region, scope)
    }

    pub(super) fn leave_region_scope(&mut self, expected_region: ShadowRegionIdV0) {
        let scope = self.scope_stack.pop().expect("nested scope exists");
        debug_assert_ne!(scope.id, self.function_scope);
        let region = self.region_stack.pop().expect("nested region exists");
        debug_assert_eq!(region, expected_region);
    }

    pub(super) fn enter_control_region(
        &mut self,
        kind: ShadowRegionKindV0,
        path: &ShadowSourcePathV0,
    ) -> ShadowRegionIdV0 {
        let region = ShadowRegionIdV0::new(self.next_region);
        self.next_region += 1;
        self.regions.insert(
            region,
            ShadowRegionRecordV0 {
                kind,
                parent: Some(self.current_region()),
                lexical_scope: None,
                origin: Some(path.node()),
            },
        );
        self.region_stack.push(region);
        region
    }

    pub(super) fn leave_control_region(&mut self, expected: ShadowRegionIdV0) {
        let actual = self.region_stack.pop().expect("nested region exists");
        debug_assert_eq!(actual, expected);
    }

    pub(super) fn push_loop(&mut self, loop_region: ShadowRegionIdV0) {
        self.loop_stack.push(loop_region);
    }

    pub(super) fn pop_loop(&mut self, expected: ShadowRegionIdV0) {
        debug_assert_eq!(self.loop_stack.pop(), Some(expected));
    }
}
