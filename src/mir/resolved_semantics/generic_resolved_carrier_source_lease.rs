//! Test-only resolver-owned lease for the future Generic carrier handoff.
//!
//! This product is deliberately narrower than a semantic shape.  The
//! resolver owns source identity, BindingRef resolution, scope ancestry, and
//! the loop forest/frame brand.  The returned lease owns no AST or source
//! lifetime and cannot be cloned or reconstructed from loose coordinates.

use std::collections::BTreeSet;

use super::{
    BindingOriginV1, BindingRefV1, FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1,
    OwnedExprSiteV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, SemanticOwnerSourceKindV1,
    SourceExprSiteV1, SourceStmtSiteV1, VerifiedResolvedFunctionV1,
    VerifiedResolvedLoopSourceForestV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericSourceRoleKindV1 {
    NestedWrite,
    PostLoopRead,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GenericSourceRoleSiteV1 {
    kind: GenericSourceRoleKindV1,
    site: OwnedExprSiteV1,
}

impl GenericSourceRoleSiteV1 {
    /// Test-fixture ingress only. The owner brand is checked before any
    /// resolver map lookup; callers cannot supply an unbranded path.
    pub(crate) fn new(kind: GenericSourceRoleKindV1, site: OwnedExprSiteV1) -> Self {
        Self { kind, site }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericSourceAncestryV1 {
    SameScope,
    StrictAncestor,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericSourceRoleClaimV1 {
    kind: GenericSourceRoleKindV1,
    site: SourceExprSiteV1,
    binding: BindingRefV1,
    binding_scope: super::ScopeId,
    site_scope: super::ScopeId,
    ancestry_chain: Box<[super::ScopeId]>,
    root_anchor: SourceStmtSiteV1,
    loop_anchor: SourceStmtSiteV1,
    ancestry: GenericSourceAncestryV1,
}

impl GenericSourceRoleClaimV1 {
    pub(crate) const fn kind(&self) -> GenericSourceRoleKindV1 {
        self.kind
    }

    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn binding_scope(&self) -> super::ScopeId {
        self.binding_scope
    }

    pub(crate) const fn site_scope(&self) -> super::ScopeId {
        self.site_scope
    }

    pub(crate) fn ancestry_chain(&self) -> &[super::ScopeId] {
        &self.ancestry_chain
    }

    pub(crate) fn root_anchor(&self) -> &SourceStmtSiteV1 {
        &self.root_anchor
    }

    pub(crate) fn loop_anchor(&self) -> &SourceStmtSiteV1 {
        &self.loop_anchor
    }

    pub(crate) const fn ancestry(&self) -> GenericSourceAncestryV1 {
        self.ancestry
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericSourceLeaseRejectV1 {
    ForeignOwner,
    ForeignOrigin,
    SourceKindMismatch,
    ForestMismatch,
    ForestShape,
    FrameMismatch,
    UnsupportedRole,
    MissingRole,
    DuplicateRole,
    RolePlacementMismatch,
    MissingBinding,
    NonBindingTarget,
    UpvarOrCapture,
    BindingOwnerMismatch,
    NonSourceBinding,
    ScopeMissing,
    NotStrictAncestor,
}

/// A bounded nested-carrier source capability. No AST, source view, route,
/// facts, or Builder/MIR identity is retained after issuance. The exact
/// two-role profile is deliberate; a broader Generic shape is a later slice.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericSourceLeaseV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    root_site: SourceStmtSiteV1,
    loop_site: SourceStmtSiteV1,
    forest: VerifiedResolvedLoopSourceForestV1,
    frames: Box<[GenericSourceFrameClaimV1]>,
    roles: Box<[GenericSourceRoleClaimV1]>,
    _seal: GenericSourceLeaseSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericSourceLeaseSealV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericSourceFrameClaimV1 {
    site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
}

impl GenericSourceFrameClaimV1 {
    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }
}

impl GenericSourceLeaseV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn root_site(&self) -> &SourceStmtSiteV1 {
        &self.root_site
    }

    pub(crate) fn loop_site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(crate) fn forest(&self) -> &VerifiedResolvedLoopSourceForestV1 {
        &self.forest
    }

    pub(crate) fn frames(&self) -> &[GenericSourceFrameClaimV1] {
        &self.frames
    }

    pub(crate) fn roles(&self) -> &[GenericSourceRoleClaimV1] {
        &self.roles
    }
}

/// Issue one exact Generic source lease from the resolver's sealed function
/// product. The expected identity arguments are verification witnesses only;
/// the function product remains the sole identity authority. All bindings,
/// scopes, ancestry, forest, and frame claims are derived here before the lease
/// is published. Forest and frames are never accepted from the caller, which
/// prevents cross-session re-pairing.
#[allow(clippy::too_many_arguments)]
pub(crate) fn issue_generic_source_lease_v1(
    function: &VerifiedResolvedFunctionV1,
    expected_owner: FunctionOwnerIdV1,
    expected_origin: FunctionOriginV1,
    expected_source_kind: SemanticOwnerSourceKindV1,
    root_site: SourceStmtSiteV1,
    loop_site: SourceStmtSiteV1,
    role_sites: [GenericSourceRoleSiteV1; 2],
) -> Result<GenericSourceLeaseV1, GenericSourceLeaseRejectV1> {
    if function.owner() != expected_owner {
        return Err(GenericSourceLeaseRejectV1::ForeignOwner);
    }
    if function.function_origin() != expected_origin {
        return Err(GenericSourceLeaseRejectV1::ForeignOrigin);
    }
    if function.source_kind() != expected_source_kind
        || function.source_kind() != SemanticOwnerSourceKindV1::DeclaredFunction
    {
        return Err(GenericSourceLeaseRejectV1::SourceKindMismatch);
    }

    let forest = function
        .resolved_loop_source_forest(&root_site)
        .map_err(|_| GenericSourceLeaseRejectV1::ForestMismatch)?;
    let [root_member, child_member] = forest.members() else {
        return Err(GenericSourceLeaseRejectV1::ForestShape);
    };
    if root_member.parent_index().is_some()
        || child_member.parent_index() != Some(0)
        || root_member.source().site() != &root_site
        || child_member.source().site() != &loop_site
    {
        return Err(GenericSourceLeaseRejectV1::ForestMismatch);
    }
    // Frames are reissued from this function's forest; callers cannot provide
    // a frame from another resolver session. Keep this internal co-seal check
    // so the move-only lease never publishes a mixed source/frame product.
    let frames = forest
        .members()
        .iter()
        .map(|member| {
            let source = member.source();
            if !source.matches_identity(
                function.function_origin(),
                function.source_kind(),
                source.site(),
            ) {
                return Err(GenericSourceLeaseRejectV1::FrameMismatch);
            }
            Ok(GenericSourceFrameClaimV1 {
                site: source.site().clone(),
                frame: source.frame_key(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();

    if role_sites[0].site.site() == role_sites[1].site.site() {
        return Err(GenericSourceLeaseRejectV1::DuplicateRole);
    }
    let mut nested_write = None;
    let mut post_loop_read = None;
    for role in role_sites {
        match role.kind {
            GenericSourceRoleKindV1::NestedWrite => {
                if nested_write.is_some() {
                    return Err(GenericSourceLeaseRejectV1::DuplicateRole);
                }
                nested_write = Some(issue_role_claim(
                    function, role, &root_site, &loop_site, true,
                )?);
            }
            GenericSourceRoleKindV1::PostLoopRead => {
                if post_loop_read.is_some() {
                    return Err(GenericSourceLeaseRejectV1::DuplicateRole);
                }
                post_loop_read = Some(issue_role_claim(
                    function, role, &root_site, &loop_site, false,
                )?);
            }
            GenericSourceRoleKindV1::Unknown => {
                return Err(GenericSourceLeaseRejectV1::UnsupportedRole);
            }
        }
    }
    let nested_write = nested_write.ok_or(GenericSourceLeaseRejectV1::MissingRole)?;
    let post_loop_read = post_loop_read.ok_or(GenericSourceLeaseRejectV1::MissingRole)?;
    Ok(GenericSourceLeaseV1 {
        owner: function.owner(),
        function_origin: function.function_origin(),
        source_kind: function.source_kind(),
        root_site,
        loop_site,
        forest,
        frames,
        roles: vec![nested_write, post_loop_read].into_boxed_slice(),
        _seal: GenericSourceLeaseSealV1,
    })
}

fn issue_role_claim(
    function: &VerifiedResolvedFunctionV1,
    role: GenericSourceRoleSiteV1,
    root_site: &SourceStmtSiteV1,
    loop_site: &SourceStmtSiteV1,
    nested: bool,
) -> Result<GenericSourceRoleClaimV1, GenericSourceLeaseRejectV1> {
    if role.site.owner() != function.owner() {
        return Err(GenericSourceLeaseRejectV1::ForeignOwner);
    }
    let role_site = role.site.site();
    let role_in_loop = role_site
        .node()
        .segments()
        .starts_with(loop_site.node().segments());
    if role_in_loop != nested
        || (!nested
            && role_site
                .node()
                .segments()
                .starts_with(root_site.node().segments()))
    {
        return Err(GenericSourceLeaseRejectV1::RolePlacementMismatch);
    }
    let binding = if nested {
        match function.assignment_target(role_site) {
            Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => *binding,
            Some(ResolvedAssignmentTargetV1::UpvarRebind(_)) => {
                return Err(GenericSourceLeaseRejectV1::UpvarOrCapture)
            }
            Some(_) => return Err(GenericSourceLeaseRejectV1::NonBindingTarget),
            None => return Err(GenericSourceLeaseRejectV1::MissingBinding),
        }
    } else {
        match function.variable_ref(role_site) {
            Some(ResolvedLexicalRefV1::Local(binding)) => binding,
            Some(ResolvedLexicalRefV1::Upvar(_)) => {
                return Err(GenericSourceLeaseRejectV1::UpvarOrCapture)
            }
            None => return Err(GenericSourceLeaseRejectV1::MissingBinding),
        }
    };
    if binding.owner() != function.owner() {
        return Err(GenericSourceLeaseRejectV1::BindingOwnerMismatch);
    }
    let binding_record = function
        .binding(binding)
        .ok_or(GenericSourceLeaseRejectV1::MissingBinding)?;
    if !matches!(binding_record.origin(), BindingOriginV1::Source(_)) {
        return Err(GenericSourceLeaseRejectV1::NonSourceBinding);
    }
    let binding_scope = binding_record.owner_scope();
    let site_scope = function
        .exact_scope_containing(role_site.node())
        .ok_or(GenericSourceLeaseRejectV1::ScopeMissing)?;
    let (ancestry, ancestry_chain) = if nested {
        (
            GenericSourceAncestryV1::StrictAncestor,
            strict_ancestor_chain(function, binding_scope, site_scope)?,
        )
    } else if site_scope == binding_scope {
        (
            GenericSourceAncestryV1::SameScope,
            vec![site_scope].into_boxed_slice(),
        )
    } else {
        (
            GenericSourceAncestryV1::StrictAncestor,
            strict_ancestor_chain(function, binding_scope, site_scope)?,
        )
    };
    Ok(GenericSourceRoleClaimV1 {
        kind: role.kind,
        site: role_site.clone(),
        binding,
        binding_scope,
        site_scope,
        ancestry_chain,
        root_anchor: root_site.clone(),
        loop_anchor: loop_site.clone(),
        ancestry,
    })
}

fn strict_ancestor_chain(
    function: &VerifiedResolvedFunctionV1,
    ancestor: super::ScopeId,
    mut descendant: super::ScopeId,
) -> Result<Box<[super::ScopeId]>, GenericSourceLeaseRejectV1> {
    if ancestor.owner() != function.owner() || descendant.owner() != function.owner() {
        return Err(GenericSourceLeaseRejectV1::ScopeMissing);
    }
    let mut chain = vec![descendant];
    let mut seen = BTreeSet::new();
    seen.insert(descendant);
    while let Some(parent) = function.scope(descendant).and_then(|scope| scope.parent()) {
        if parent.owner() != function.owner() || !seen.insert(parent) {
            return Err(GenericSourceLeaseRejectV1::ScopeMissing);
        }
        chain.push(parent);
        if parent == ancestor {
            return Ok(chain.into_boxed_slice());
        }
        descendant = parent;
    }
    Err(GenericSourceLeaseRejectV1::NotStrictAncestor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ASTNode;
    use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
    use crate::mir::compiler::located::LocatedStmtV1;
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
    use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
    use crate::parser::NyashParser;

    const SOURCE: &str = r#"
function generic_both(i, j) {
    loop(i < 3) {
        loop(j < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

    const SHADOWING_SOURCE: &str = r#"
function generic_both_shadowing(i, j) {
    loop(i < 3) {
        loop(j < 3) {
            local j = 0
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

    fn parse_function(source: &str) -> ASTNode {
        let root = NyashParser::parse_from_string(source).expect("lease fixture parses");
        let ASTNode::Program { statements, .. } = root else {
            panic!("lease fixture must be a Program")
        };
        statements
            .into_iter()
            .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
            .expect("lease fixture function")
    }

    fn unit(source: &str) -> VerifiedResolvedSourceUnitV1 {
        VerifiedResolvedSourceUnitV1::resolve_function(parse_function(source))
            .expect("lease fixture resolves")
    }

    fn input_and_root(
        unit: &VerifiedResolvedSourceUnitV1,
    ) -> (ResolvedFunctionLoweringInputV1<'_>, LocatedStmtV1<'_>) {
        let input = unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("function body");
        let root = input.source().body_stmt(&body, 0).expect("outer loop");
        (input, root)
    }

    fn sites(
        input: ResolvedFunctionLoweringInputV1<'_>,
        root: &LocatedStmtV1<'_>,
    ) -> (
        SourceStmtSiteV1,
        SourceStmtSiteV1,
        SourceExprSiteV1,
        SourceExprSiteV1,
    ) {
        let source = input.source();
        let outer_body = source
            .child_body_from_stmt(root, BodyChildRoleV1::LoopBody)
            .expect("outer body");
        let inner = source.body_stmt(&outer_body, 0).expect("inner loop");
        let inner_body = source
            .child_body_from_stmt(&inner, BodyChildRoleV1::LoopBody)
            .expect("inner body");
        let write_index = inner_body
            .statements()
            .iter()
            .position(|node| matches!(node, ASTNode::Assignment { .. }))
            .expect("assignment in inner body");
        let write_stmt = source.body_stmt(&inner_body, write_index).expect("write");
        let write = source
            .child_expr_from_stmt(&write_stmt, ExprChildRoleV1::AssignmentTarget)
            .expect("write target")
            .site()
            .clone();
        let function_body = source.root_body().expect("root body");
        let return_stmt = source.body_stmt(&function_body, 1).expect("return");
        let read = source
            .child_expr_from_stmt(&return_stmt, ExprChildRoleV1::ReturnValue)
            .expect("return value")
            .site()
            .clone();
        let forest = input
            .function()
            .resolved_loop_source_forest(root.site())
            .expect("nested source forest");
        (
            forest.members()[0].source().site().clone(),
            forest.members()[1].source().site().clone(),
            write,
            read,
        )
    }

    fn positive_lease(
        input: ResolvedFunctionLoweringInputV1<'_>,
        root: &LocatedStmtV1<'_>,
    ) -> GenericSourceLeaseV1 {
        let (root_site, loop_site, write, read) = sites(input, root);
        let function = input.function();
        issue_generic_source_lease_v1(
            function,
            function.owner(),
            function.function_origin(),
            function.source_kind(),
            root_site,
            loop_site,
            [
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::NestedWrite,
                    OwnedExprSiteV1::new(function.owner(), write.clone()),
                ),
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::PostLoopRead,
                    OwnedExprSiteV1::new(function.owner(), read.clone()),
                ),
            ],
        )
        .expect("positive lease")
    }

    #[test]
    fn source_lease_is_owner_branded_and_ast_free_after_issuance() {
        let unit = unit(SOURCE);
        let (input, root) = input_and_root(&unit);
        let lease = positive_lease(input, &root);
        assert_eq!(lease.roles().len(), 2);
        assert_eq!(lease.frames().len(), 2);
        assert_eq!(
            lease.roles()[0].kind(),
            GenericSourceRoleKindV1::NestedWrite
        );
        assert_eq!(
            lease.roles()[0].ancestry(),
            GenericSourceAncestryV1::StrictAncestor
        );
        assert_eq!(
            lease.roles()[1].ancestry(),
            GenericSourceAncestryV1::StrictAncestor
        );
        assert_eq!(lease.roles()[0].root_anchor(), lease.root_site());
        assert_eq!(lease.roles()[0].loop_anchor(), lease.loop_site());
        assert_eq!(lease.frames()[0].site(), lease.root_site());
        assert_eq!(lease.frames()[1].site(), lease.loop_site());
        assert_eq!(lease.forest().members().len(), 2);
    }

    #[test]
    fn source_lease_rejects_foreign_session_before_map_lookup() {
        let first = unit(SOURCE);
        let second = unit(SOURCE);
        let (input, root) = input_and_root(&first);
        let (root_site, loop_site, write, read) = sites(input, &root);
        let foreign_input = second.root_function_input().expect("foreign input");
        let foreign = foreign_input.function();
        let function = input.function();
        let result = issue_generic_source_lease_v1(
            function,
            foreign.owner(),
            function.function_origin(),
            function.source_kind(),
            root_site,
            loop_site,
            [
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::NestedWrite,
                    OwnedExprSiteV1::new(function.owner(), write.clone()),
                ),
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::PostLoopRead,
                    OwnedExprSiteV1::new(function.owner(), read.clone()),
                ),
            ],
        );
        assert_eq!(result, Err(GenericSourceLeaseRejectV1::ForeignOwner));
    }

    #[test]
    fn source_lease_rejects_foreign_role_brand_and_shadowing() {
        let first = unit(SOURCE);
        let second = unit(SOURCE);
        let (input, root) = input_and_root(&first);
        let (root_site, loop_site, write, read) = sites(input, &root);
        let function = input.function();
        let foreign_input = second.root_function_input().expect("foreign input");
        let foreign_owner = foreign_input.owner();
        let foreign_role = issue_generic_source_lease_v1(
            function,
            function.owner(),
            function.function_origin(),
            function.source_kind(),
            root_site.clone(),
            loop_site.clone(),
            [
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::NestedWrite,
                    OwnedExprSiteV1::new(foreign_owner, write.clone()),
                ),
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::PostLoopRead,
                    OwnedExprSiteV1::new(function.owner(), read.clone()),
                ),
            ],
        );
        assert_eq!(foreign_role, Err(GenericSourceLeaseRejectV1::ForeignOwner));

        let shadow = unit(SHADOWING_SOURCE);
        let (shadow_input, shadow_root) = input_and_root(&shadow);
        let (shadow_root_site, shadow_loop_site, shadow_write, shadow_read) =
            sites(shadow_input, &shadow_root);
        let shadow_function = shadow_input.function();
        let shadow_result = issue_generic_source_lease_v1(
            shadow_function,
            shadow_function.owner(),
            shadow_function.function_origin(),
            shadow_function.source_kind(),
            shadow_root_site,
            shadow_loop_site,
            [
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::NestedWrite,
                    OwnedExprSiteV1::new(shadow_function.owner(), shadow_write),
                ),
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::PostLoopRead,
                    OwnedExprSiteV1::new(shadow_function.owner(), shadow_read),
                ),
            ],
        );
        assert_eq!(
            shadow_result,
            Err(GenericSourceLeaseRejectV1::NotStrictAncestor)
        );
    }

    #[test]
    fn source_lease_rejects_duplicate_unknown_and_mismatched_forest_roles() {
        let unit = unit(SOURCE);
        let (input, root) = input_and_root(&unit);
        let (root_site, loop_site, write, read) = sites(input, &root);
        let function = input.function();
        let duplicate = issue_generic_source_lease_v1(
            function,
            function.owner(),
            function.function_origin(),
            function.source_kind(),
            root_site.clone(),
            loop_site.clone(),
            [
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::NestedWrite,
                    OwnedExprSiteV1::new(function.owner(), write.clone()),
                ),
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::NestedWrite,
                    OwnedExprSiteV1::new(function.owner(), write.clone()),
                ),
            ],
        );
        assert_eq!(duplicate, Err(GenericSourceLeaseRejectV1::DuplicateRole));

        let unknown = issue_generic_source_lease_v1(
            function,
            function.owner(),
            function.function_origin(),
            function.source_kind(),
            root_site.clone(),
            loop_site.clone(),
            [
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::NestedWrite,
                    OwnedExprSiteV1::new(function.owner(), write.clone()),
                ),
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::Unknown,
                    OwnedExprSiteV1::new(function.owner(), read.clone()),
                ),
            ],
        );
        assert_eq!(unknown, Err(GenericSourceLeaseRejectV1::UnsupportedRole));

        let wrong_loop =
            SourceStmtSiteV1::from_node(super::super::SourcePathV1::root_body(1).node());
        let mismatch = issue_generic_source_lease_v1(
            function,
            function.owner(),
            function.function_origin(),
            function.source_kind(),
            root_site,
            wrong_loop,
            [
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::NestedWrite,
                    OwnedExprSiteV1::new(function.owner(), write),
                ),
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::PostLoopRead,
                    OwnedExprSiteV1::new(function.owner(), read),
                ),
            ],
        );
        assert_eq!(mismatch, Err(GenericSourceLeaseRejectV1::ForestMismatch));
    }

    #[test]
    fn source_lease_rejects_identity_and_role_input_mismatches() {
        let unit = unit(SOURCE);
        let (input, root) = input_and_root(&unit);
        let (root_site, loop_site, write, read) = sites(input, &root);
        let function = input.function();
        let roles = || {
            [
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::NestedWrite,
                    OwnedExprSiteV1::new(function.owner(), write.clone()),
                ),
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::PostLoopRead,
                    OwnedExprSiteV1::new(function.owner(), read.clone()),
                ),
            ]
        };
        assert_eq!(
            issue_generic_source_lease_v1(
                function,
                function.owner(),
                FunctionOriginV1::new(99, 99),
                function.source_kind(),
                root_site.clone(),
                loop_site.clone(),
                roles(),
            ),
            Err(GenericSourceLeaseRejectV1::ForeignOrigin)
        );
        assert_eq!(
            issue_generic_source_lease_v1(
                function,
                function.owner(),
                function.function_origin(),
                SemanticOwnerSourceKindV1::Script,
                root_site.clone(),
                loop_site.clone(),
                roles(),
            ),
            Err(GenericSourceLeaseRejectV1::SourceKindMismatch)
        );
        assert_eq!(
            issue_generic_source_lease_v1(
                function,
                function.owner(),
                function.function_origin(),
                function.source_kind(),
                root_site,
                loop_site,
                [
                    GenericSourceRoleSiteV1::new(
                        GenericSourceRoleKindV1::PostLoopRead,
                        OwnedExprSiteV1::new(function.owner(), write),
                    ),
                    GenericSourceRoleSiteV1::new(
                        GenericSourceRoleKindV1::PostLoopRead,
                        OwnedExprSiteV1::new(function.owner(), read),
                    ),
                ],
            ),
            Err(GenericSourceLeaseRejectV1::RolePlacementMismatch)
        );
    }
}
