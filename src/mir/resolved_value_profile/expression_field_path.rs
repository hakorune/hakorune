//! Original field geometry shared by authority-specific expression consumers.
//! A path proves syntax identity only, never class, value, liveness or layout.
use super::*;
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1};

#[derive(Debug, Clone)]
pub(crate) struct OriginalExpressionFieldPathV1 {
    plan: Rc<ExpressionSourcePlanV1>,
    node: usize,
    root: BindingRefV1,
    fields: Box<[usize]>,
}
impl PartialEq for OriginalExpressionFieldPathV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.plan, &other.plan) && self.node == other.node
    }
}
impl Eq for OriginalExpressionFieldPathV1 {}
impl OriginalExpressionFieldPathV1 {
    pub(crate) fn issue(plan: Rc<ExpressionSourcePlanV1>, site: &SourceExprSiteV1) -> Option<Self> {
        let mut nodes = plan.nodes().iter().enumerate().filter(|(_, node)| node.site() == site);
        let (node, _) = nodes.next()?;
        if nodes.next().is_some() { return None; }
        let (root, fields) = plan.field_path_v1(site)?;
        if root.owner() != plan.owner() || fields.last() != Some(&node) { return None; }
        Some(Self { plan, node, root, fields: fields.into_boxed_slice() })
    }
    pub(crate) fn plan(&self) -> &Rc<ExpressionSourcePlanV1> { &self.plan }
    pub(crate) fn node(&self) -> usize { self.node }
    pub(crate) fn fields(&self) -> &[usize] { &self.fields }
    pub(crate) fn root(&self) -> BindingRefV1 { self.root }
    pub(crate) fn site(&self) -> OwnedExprSiteV1 {
        OwnedExprSiteV1::new(self.plan.owner(), self.plan.nodes()[self.node].site().clone())
    }
    pub(crate) fn receiver_site(&self) -> &SourceExprSiteV1 {
        match self.plan.nodes()[self.node].source() {
            ExpressionNodeSourceV1::Shape(BodyExpressionShapeV1::FieldAccess { object, .. }) => object,
            _ => unreachable!("issued original field path"),
        }
    }
    pub(crate) fn name(&self) -> &str {
        match self.plan.nodes()[self.node].source() {
            ExpressionNodeSourceV1::Shape(BodyExpressionShapeV1::FieldAccess { field, .. }) => field,
            _ => unreachable!("issued original field path"),
        }
    }
    pub(crate) fn first_site(&self) -> &SourceExprSiteV1 {
        self.plan.nodes()[self.fields[0]].site()
    }
    pub(crate) fn root_site(&self) -> &SourceExprSiteV1 {
        match self.plan.nodes()[self.fields[0]].source() {
            ExpressionNodeSourceV1::Shape(BodyExpressionShapeV1::FieldAccess { object, .. }) => object,
            _ => unreachable!("issued original field path"),
        }
    }
    pub(crate) fn parent(&self) -> Option<OwnedExprSiteV1> {
        self.fields.len().checked_sub(2).map(|index|
            OwnedExprSiteV1::new(self.plan.owner(), self.plan.nodes()[self.fields[index]].site().clone()))
    }
    pub(crate) fn is_intermediate(&self) -> bool {
        self.plan.nodes().iter().any(|node| matches!(node.source(),
            ExpressionNodeSourceV1::Shape(BodyExpressionShapeV1::FieldAccess { object, .. })
                if object == self.plan.nodes()[self.node].site()))
    }
    pub(crate) fn is_nested_component(&self) -> bool { self.fields.len() > 1 || self.is_intermediate() }
}
