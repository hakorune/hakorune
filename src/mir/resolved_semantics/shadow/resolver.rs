//! Function-level shadow resolution entry and mutable construction owner.

use std::collections::BTreeMap;

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::source_site::{
    FunctionOriginV1, SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};
use crate::mir::resolved_semantics::FunctionSyntaxViewV1;

use super::ids::{ShadowBindingOrdinalV0, ShadowRegionIdV0, ShadowScopeIdV0};
use super::path::ShadowSourcePathV0;
use super::product::{
    ShadowAssignmentTargetV0, ShadowBindingKindV0, ShadowBindingRecordV0, ShadowControlExitV0,
    ShadowRegionKindV0, ShadowRegionRecordV0, ShadowResolveErrorV0, ShadowResolvedFunctionV0,
    ShadowScopeKindV0, ShadowScopeRecordV0,
};

#[derive(Debug)]
struct ResolverScopeFrameV0 {
    id: ShadowScopeIdV0,
    names: BTreeMap<String, ShadowBindingOrdinalV0>,
}

pub(super) struct ShadowResolverV0 {
    function_origin: FunctionOriginV1,
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
    variable_uses: BTreeMap<SourceExprSiteV1, ShadowBindingOrdinalV0>,
    assignment_targets: BTreeMap<SourceExprSiteV1, ShadowAssignmentTargetV0>,
    control_exits: BTreeMap<SourceStmtSiteV1, ShadowControlExitV0>,
    control_exit_regions: BTreeMap<SourceStmtSiteV1, ShadowRegionIdV0>,
}

pub(super) fn resolve_function_shadow_v0(
    function_origin: FunctionOriginV1,
    function: &ASTNode,
) -> Result<ShadowResolvedFunctionV0, ShadowResolveErrorV0> {
    let Some(view) = FunctionSyntaxViewV1::from_ast(function) else {
        return Err(ShadowResolveErrorV0::ExpectedFunctionDeclaration);
    };
    resolve_function_shadow_view_v0(function_origin, view)
}

pub(in crate::mir::resolved_semantics) fn resolve_function_shadow_view_v0(
    function_origin: FunctionOriginV1,
    view: FunctionSyntaxViewV1<'_>,
) -> Result<ShadowResolvedFunctionV0, ShadowResolveErrorV0> {
    let params = view.params();
    let body = view.body();

    let mut resolver = ShadowResolverV0::new(function_origin);
    if !view.is_static() {
        resolver.declare_binding(
            "me",
            ShadowBindingKindV0::Receiver,
            SourceBindingSiteV1::Receiver,
        )?;
        resolver.receiver = resolver.lookup("me");
    }
    for (index, name) in params.iter().enumerate() {
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
    let body_path = ShadowSourcePathV0::function_body();
    let (body_region, _) = resolver.enter_region_scope(
        ShadowRegionKindV0::Sequence,
        ShadowScopeKindV0::LexicalBlock,
        &body_path,
    );
    let body_result = resolver.resolve_body(body, ShadowSourcePathV0::root_body);
    resolver.leave_region_scope(body_region);
    body_result?;
    Ok(resolver.finish())
}

impl ShadowResolverV0 {
    fn new(function_origin: FunctionOriginV1) -> Self {
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
            function_origin,
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
            control_exits: BTreeMap::new(),
            control_exit_regions: BTreeMap::new(),
        }
    }

    fn finish(self) -> ShadowResolvedFunctionV0 {
        ShadowResolvedFunctionV0 {
            function_origin: self.function_origin,
            function_scope: self.function_scope,
            function_region: self.function_region,
            bindings: self.bindings,
            scopes: self.scopes,
            regions: self.regions,
            declarations: self.declarations,
            variable_uses: self.variable_uses,
            assignment_targets: self.assignment_targets,
            control_exits: self.control_exits,
            control_exit_regions: self.control_exit_regions,
        }
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

    pub(super) fn record_use(&mut self, site: SourceExprSiteV1, binding: ShadowBindingOrdinalV0) {
        self.variable_uses.insert(site, binding);
    }

    pub(super) fn record_assignment(
        &mut self,
        site: SourceExprSiteV1,
        target: ShadowAssignmentTargetV0,
    ) {
        self.assignment_targets.insert(site, target);
    }

    pub(super) fn record_exit(&mut self, site: SourceStmtSiteV1, exit: ShadowControlExitV0) {
        self.control_exit_regions
            .insert(site.clone(), self.current_region());
        self.control_exits.insert(site, exit);
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
