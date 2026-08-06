//! AST-free structural witness for the first Generic G0 profile.
//!
//! The compiler-side projector owns syntax navigation. This module owns only
//! the exact source shape, binding relations, and coverage seal. It does not
//! inspect AST, numeric values, policy, Recipe keys, or physical identities.

use std::collections::BTreeSet;

use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::numeric_substrate::generic_g0::VerifiedGenericNumericFactLeaseG0;
use crate::mir::numeric_substrate::NumericTarget;
use crate::mir::resolved_semantics::generic_g0::VerifiedGenericSourceTypeInventoryG0;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOriginV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1,
    SourceExprSiteV1, SourceNodeSiteV1, SourceStmtSiteV1, VerifiedLoopFamilyWindowLeaseV1,
    VerifiedResolvedLoopSourceForestV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum GenericG0ConditionOperatorV1 {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum GenericG0UpdateOperatorV1 {
    Add,
    Subtract,
    Other,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0ConditionSitesV1 {
    pub(crate) operator: GenericG0ConditionOperatorV1,
    pub(crate) condition: SourceExprSiteV1,
    pub(crate) lhs: SourceExprSiteV1,
    pub(crate) rhs: SourceExprSiteV1,
    pub(crate) binding: BindingRefV1,
}

impl GenericG0ConditionSitesV1 {
    pub(crate) const fn operator(&self) -> GenericG0ConditionOperatorV1 {
        self.operator
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0UpdateSitesV1 {
    pub(crate) operator: GenericG0UpdateOperatorV1,
    pub(crate) statement: SourceStmtSiteV1,
    pub(crate) target: SourceExprSiteV1,
    pub(crate) value: SourceExprSiteV1,
    pub(crate) lhs: SourceExprSiteV1,
    pub(crate) rhs: SourceExprSiteV1,
    pub(crate) binding: BindingRefV1,
}

impl GenericG0UpdateSitesV1 {
    pub(crate) const fn operator(&self) -> GenericG0UpdateOperatorV1 {
        self.operator
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0TailSitesV1 {
    pub(crate) statement: SourceStmtSiteV1,
    pub(crate) value: SourceExprSiteV1,
    pub(crate) binding: BindingRefV1,
}

/// AST-free input produced by the one compiler-side source projector.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0StructuralObservationV1 {
    pub(crate) owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    pub(crate) origin: FunctionOriginV1,
    pub(crate) source_kind: SemanticOwnerSourceKindV1,
    pub(crate) forest: VerifiedResolvedLoopSourceForestV1,
    pub(crate) expected_root_frame: LoopExecutionFrameKeyV1,
    pub(crate) function_body: Box<[SourceStmtSiteV1]>,
    pub(crate) root_body: Box<[SourceStmtSiteV1]>,
    pub(crate) child_body: Box<[SourceStmtSiteV1]>,
    pub(crate) root_loop: SourceStmtSiteV1,
    pub(crate) child_loop: SourceStmtSiteV1,
    pub(crate) outer_condition: GenericG0ConditionSitesV1,
    pub(crate) inner_condition: GenericG0ConditionSitesV1,
    pub(crate) outer_update: GenericG0UpdateSitesV1,
    pub(crate) inner_update: GenericG0UpdateSitesV1,
    pub(crate) tail: GenericG0TailSitesV1,
    pub(crate) coverage: Box<[SourceNodeSiteV1]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0StructuralRejectV1 {
    WrongSourceKind,
    ForestShape,
    ForestIdentity,
    FunctionBodySchedule,
    RootBodySchedule,
    ChildBodySchedule,
    BindingRelation,
    Coverage,
}

/// Move-only S0A product. It is intentionally not `Clone`: later rows must
/// consume the exact structural lease instead of rebuilding it from names.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericStructuralFactsG0 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    forest: VerifiedResolvedLoopSourceForestV1,
    function_body: Box<[SourceStmtSiteV1]>,
    root_body: Box<[SourceStmtSiteV1]>,
    child_body: Box<[SourceStmtSiteV1]>,
    root_loop: SourceStmtSiteV1,
    child_loop: SourceStmtSiteV1,
    outer_condition: GenericG0ConditionSitesV1,
    inner_condition: GenericG0ConditionSitesV1,
    outer_update: GenericG0UpdateSitesV1,
    inner_update: GenericG0UpdateSitesV1,
    tail: GenericG0TailSitesV1,
    coverage: Box<[SourceNodeSiteV1]>,
    root_frame: LoopExecutionFrameKeyV1,
}

impl VerifiedGenericStructuralFactsG0 {
    pub(crate) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn origin(&self) -> FunctionOriginV1 {
        self.origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn forest(&self) -> &VerifiedResolvedLoopSourceForestV1 {
        &self.forest
    }

    pub(crate) fn function_body(&self) -> &[SourceStmtSiteV1] {
        &self.function_body
    }

    pub(crate) fn root_body(&self) -> &[SourceStmtSiteV1] {
        &self.root_body
    }

    pub(crate) fn child_body(&self) -> &[SourceStmtSiteV1] {
        &self.child_body
    }

    pub(crate) fn root_loop(&self) -> &SourceStmtSiteV1 {
        &self.root_loop
    }

    pub(crate) fn child_loop(&self) -> &SourceStmtSiteV1 {
        &self.child_loop
    }

    pub(crate) fn outer_condition(&self) -> &GenericG0ConditionSitesV1 {
        &self.outer_condition
    }

    pub(crate) fn inner_condition(&self) -> &GenericG0ConditionSitesV1 {
        &self.inner_condition
    }

    pub(crate) fn outer_update(&self) -> &GenericG0UpdateSitesV1 {
        &self.outer_update
    }

    pub(crate) fn inner_update(&self) -> &GenericG0UpdateSitesV1 {
        &self.inner_update
    }

    pub(crate) fn tail(&self) -> &GenericG0TailSitesV1 {
        &self.tail
    }

    pub(crate) fn coverage(&self) -> &[SourceNodeSiteV1] {
        &self.coverage
    }

    pub(crate) fn root_frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.root_frame
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::mir::resolved_semantics::FunctionOwnerIdV1,
        FunctionOriginV1,
        SemanticOwnerSourceKindV1,
        VerifiedResolvedLoopSourceForestV1,
        Box<[SourceStmtSiteV1]>,
        Box<[SourceStmtSiteV1]>,
        Box<[SourceStmtSiteV1]>,
        SourceStmtSiteV1,
        SourceStmtSiteV1,
        GenericG0ConditionSitesV1,
        GenericG0ConditionSitesV1,
        GenericG0UpdateSitesV1,
        GenericG0UpdateSitesV1,
        GenericG0TailSitesV1,
        Box<[SourceNodeSiteV1]>,
        LoopExecutionFrameKeyV1,
    ) {
        (
            self.owner,
            self.origin,
            self.source_kind,
            self.forest,
            self.function_body,
            self.root_body,
            self.child_body,
            self.root_loop,
            self.child_loop,
            self.outer_condition,
            self.inner_condition,
            self.outer_update,
            self.inner_update,
            self.tail,
            self.coverage,
            self.root_frame,
        )
    }
}

pub(crate) fn issue_generic_g0_structural_facts_v1(
    observation: GenericG0StructuralObservationV1,
) -> Result<VerifiedGenericStructuralFactsG0, GenericG0StructuralRejectV1> {
    if observation.source_kind != SemanticOwnerSourceKindV1::DeclaredFunction {
        return Err(GenericG0StructuralRejectV1::WrongSourceKind);
    }
    let members = observation.forest.members();
    if members.len() != 2
        || members[0].parent_index().is_some()
        || members[1].parent_index() != Some(0)
    {
        return Err(GenericG0StructuralRejectV1::ForestShape);
    }
    if members[0].source().site() != &observation.root_loop
        || members[1].source().site() != &observation.child_loop
        || !members[0].source().matches_identity(
            observation.origin,
            observation.source_kind,
            &observation.root_loop,
        )
        || !members[1].source().matches_identity(
            observation.origin,
            observation.source_kind,
            &observation.child_loop,
        )
        || !members[0]
            .source()
            .frame_key()
            .matches(&observation.expected_root_frame)
    {
        return Err(GenericG0StructuralRejectV1::ForestIdentity);
    }
    if observation.function_body.len() != 2
        || observation.function_body[0] != observation.root_loop
        || observation.function_body[1] != observation.tail.statement
    {
        return Err(GenericG0StructuralRejectV1::FunctionBodySchedule);
    }
    if observation.root_body.len() != 2
        || observation.root_body[0] != observation.child_loop
        || observation.root_body[1] != observation.outer_update.statement
        || observation.child_body.len() != 1
        || observation.child_body[0] != observation.inner_update.statement
    {
        return Err(GenericG0StructuralRejectV1::RootBodySchedule);
    }
    let bindings = [
        observation.outer_condition.binding,
        observation.inner_condition.binding,
        observation.outer_update.binding,
        observation.inner_update.binding,
        observation.tail.binding,
    ];
    if bindings
        .iter()
        .any(|binding| binding.owner() != observation.owner)
        || observation.outer_condition.binding != observation.outer_update.binding
        || observation.inner_condition.binding != observation.inner_update.binding
        || observation.inner_condition.binding != observation.tail.binding
        || observation.outer_condition.binding == observation.inner_condition.binding
    {
        return Err(GenericG0StructuralRejectV1::BindingRelation);
    }
    let expected = expected_coverage(&observation);
    let actual = observation
        .coverage
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if actual.len() != observation.coverage.len()
        || actual.len() != expected.len()
        || actual != expected
    {
        return Err(GenericG0StructuralRejectV1::Coverage);
    }
    let root_frame = observation.expected_root_frame;
    Ok(VerifiedGenericStructuralFactsG0 {
        owner: observation.owner,
        origin: observation.origin,
        source_kind: observation.source_kind,
        forest: observation.forest,
        function_body: observation.function_body,
        root_body: observation.root_body,
        child_body: observation.child_body,
        root_loop: observation.root_loop,
        child_loop: observation.child_loop,
        outer_condition: observation.outer_condition,
        inner_condition: observation.inner_condition,
        outer_update: observation.outer_update,
        inner_update: observation.inner_update,
        tail: observation.tail,
        coverage: observation.coverage,
        root_frame,
    })
}

/// Cumulative move-only source lease for S0B.
///
/// The bundle belongs to the neutral structural-facts boundary so policy and
/// later source observers do not depend on compiler projection modules.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericSourceBundleG0 {
    structural: VerifiedGenericStructuralFactsG0,
    source_types: VerifiedGenericSourceTypeInventoryG0,
}

impl VerifiedGenericSourceBundleG0 {
    pub(crate) fn new(
        structural: VerifiedGenericStructuralFactsG0,
        source_types: VerifiedGenericSourceTypeInventoryG0,
    ) -> Self {
        Self {
            structural,
            source_types,
        }
    }

    pub(crate) fn structural(&self) -> &VerifiedGenericStructuralFactsG0 {
        &self.structural
    }

    pub(crate) fn source_types(&self) -> &VerifiedGenericSourceTypeInventoryG0 {
        &self.source_types
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedGenericStructuralFactsG0,
        VerifiedGenericSourceTypeInventoryG0,
    ) {
        (self.structural, self.source_types)
    }
}

/// S0C's cumulative move-only source/type/numeric/return capability.
///
/// The compiler projector issues this product, but it cannot own or
/// reconstruct it after the boundary. Consumers move the exact lease onward.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericTypedSourceBundleG0 {
    source: VerifiedGenericSourceBundleG0,
    numeric: VerifiedGenericNumericFactLeaseG0,
    return_abi: ExactTrivialReturnAbiV1,
}

impl VerifiedGenericTypedSourceBundleG0 {
    pub(crate) fn new(
        source: VerifiedGenericSourceBundleG0,
        numeric: VerifiedGenericNumericFactLeaseG0,
        return_abi: ExactTrivialReturnAbiV1,
    ) -> Self {
        Self {
            source,
            numeric,
            return_abi,
        }
    }

    pub(crate) fn source(&self) -> &VerifiedGenericSourceBundleG0 {
        &self.source
    }

    pub(crate) fn numeric(&self) -> &VerifiedGenericNumericFactLeaseG0 {
        &self.numeric
    }

    pub(crate) const fn return_abi(&self) -> ExactTrivialReturnAbiV1 {
        self.return_abi
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedGenericSourceBundleG0,
        VerifiedGenericNumericFactLeaseG0,
        ExactTrivialReturnAbiV1,
    ) {
        (self.source, self.numeric, self.return_abi)
    }
}

/// Opaque resolver/source brand retained by the Generic policy handoff.
///
/// The brand is minted from the resolver-owned common-window lease.  It is
/// intentionally not reconstructible from names or loose source coordinates.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0SourceBrandV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    root_site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
    _seal: GenericG0SourceBrandSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericG0SourceBrandSealV1;

impl GenericG0SourceBrandV1 {
    pub(crate) fn from_window_lease(lease: &VerifiedLoopFamilyWindowLeaseV1) -> Self {
        Self {
            owner: lease.owner(),
            origin: lease.function_origin(),
            source_kind: lease.source_kind(),
            root_site: lease.site().clone(),
            frame: lease.frame(),
            _seal: GenericG0SourceBrandSealV1,
        }
    }

    pub(crate) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn origin(&self) -> FunctionOriginV1 {
        self.origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn root_site(&self) -> &SourceStmtSiteV1 {
        &self.root_site
    }

    pub(crate) fn frame(&self) -> LoopExecutionFrameKeyV1 {
        self.frame.clone()
    }

    pub(crate) fn matches_window(&self, lease: &VerifiedLoopFamilyWindowLeaseV1) -> bool {
        self.owner == lease.owner()
            && self.origin == lease.function_origin()
            && self.source_kind == lease.source_kind()
            && self.root_site == *lease.site()
            && self.frame.matches(&lease.frame())
    }
}

/// Exact completion relation for the function-tail `return` after the root
/// loop.  This is separate from loop exits and keeps the post-loop read's
/// resolver `BindingRef` alive for later policy/Recipe consumers.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericG0PostLoopReadV1 {
    statement: SourceStmtSiteV1,
    value: SourceExprSiteV1,
    binding: BindingRefV1,
    _seal: GenericG0PostLoopReadSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericG0PostLoopReadSealV1;

impl VerifiedGenericG0PostLoopReadV1 {
    pub(crate) fn new(
        statement: SourceStmtSiteV1,
        value: SourceExprSiteV1,
        binding: BindingRefV1,
    ) -> Self {
        Self {
            statement,
            value,
            binding,
            _seal: GenericG0PostLoopReadSealV1,
        }
    }

    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 {
        &self.statement
    }

    pub(crate) fn value(&self) -> &SourceExprSiteV1 {
        &self.value
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0PolicyHandoffSealRejectV1 {
    BrandMismatch,
    WindowSiteMismatch,
    RootFrameMismatch,
    BundleOwnerMismatch,
    TargetMismatch,
    ReturnBindingMismatch,
    ReturnSiteMismatch,
}

/// The one AST-free, move-only source-to-policy capability for Generic G0.
///
/// The compiler-side projector co-seals every field. Policy consumes this
/// product by value; it must not downgrade it to a bare typed bundle.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericG0PolicyHandoffV1 {
    brand: GenericG0SourceBrandV1,
    bundle: VerifiedGenericTypedSourceBundleG0,
    post_loop_read: VerifiedGenericG0PostLoopReadV1,
    target: NumericTarget,
    _seal: GenericG0PolicyHandoffSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericG0PolicyHandoffSealV1;

impl VerifiedGenericG0PolicyHandoffV1 {
    pub(crate) fn seal(
        window_lease: &VerifiedLoopFamilyWindowLeaseV1,
        bundle: VerifiedGenericTypedSourceBundleG0,
        post_loop_read: VerifiedGenericG0PostLoopReadV1,
        target: NumericTarget,
    ) -> Result<Self, GenericG0PolicyHandoffSealRejectV1> {
        let structural = bundle.source().structural();
        let brand = GenericG0SourceBrandV1::from_window_lease(&window_lease);
        if brand.owner() != structural.owner()
            || brand.origin() != structural.origin()
            || brand.source_kind() != structural.source_kind()
        {
            return Err(GenericG0PolicyHandoffSealRejectV1::BrandMismatch);
        }
        if window_lease.site() != structural.root_loop()
            || brand.root_site() != structural.root_loop()
        {
            return Err(GenericG0PolicyHandoffSealRejectV1::WindowSiteMismatch);
        }
        if !window_lease.frame().matches(structural.root_frame())
            || !brand.frame().matches(structural.root_frame())
        {
            return Err(GenericG0PolicyHandoffSealRejectV1::RootFrameMismatch);
        }
        if post_loop_read.statement() != &structural.tail().statement
            || post_loop_read.value() != &structural.tail().value
        {
            return Err(GenericG0PolicyHandoffSealRejectV1::ReturnSiteMismatch);
        }
        if post_loop_read.binding() != structural.tail().binding {
            return Err(GenericG0PolicyHandoffSealRejectV1::ReturnBindingMismatch);
        }
        if bundle.numeric().target() != target {
            return Err(GenericG0PolicyHandoffSealRejectV1::TargetMismatch);
        }
        Ok(Self {
            brand,
            bundle,
            post_loop_read,
            target,
            _seal: GenericG0PolicyHandoffSealV1,
        })
    }

    pub(crate) fn brand(&self) -> &GenericG0SourceBrandV1 {
        &self.brand
    }

    pub(crate) fn bundle(&self) -> &VerifiedGenericTypedSourceBundleG0 {
        &self.bundle
    }

    pub(crate) fn post_loop_read(&self) -> &VerifiedGenericG0PostLoopReadV1 {
        &self.post_loop_read
    }

    pub(crate) const fn target(&self) -> NumericTarget {
        self.target
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        GenericG0SourceBrandV1,
        VerifiedGenericTypedSourceBundleG0,
        VerifiedGenericG0PostLoopReadV1,
        NumericTarget,
    ) {
        (self.brand, self.bundle, self.post_loop_read, self.target)
    }
}

fn expected_coverage(observation: &GenericG0StructuralObservationV1) -> BTreeSet<SourceNodeSiteV1> {
    let mut sites = BTreeSet::new();
    for site in observation
        .function_body
        .iter()
        .chain(observation.root_body.iter())
        .chain(observation.child_body.iter())
    {
        sites.insert(site.node().clone());
    }
    sites.insert(observation.outer_condition.condition.node().clone());
    sites.insert(observation.outer_condition.lhs.node().clone());
    sites.insert(observation.outer_condition.rhs.node().clone());
    sites.insert(observation.inner_condition.condition.node().clone());
    sites.insert(observation.inner_condition.lhs.node().clone());
    sites.insert(observation.inner_condition.rhs.node().clone());
    for update in [&observation.outer_update, &observation.inner_update] {
        sites.insert(update.target.node().clone());
        sites.insert(update.value.node().clone());
        sites.insert(update.lhs.node().clone());
        sites.insert(update.rhs.node().clone());
    }
    sites.insert(observation.tail.value.node().clone());
    sites
}
