//! Immutable resolver roles shared by source observation and affine execution.
//! Borrowing this payload grants neither a Home After nor physical permission.
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{ResolvedAssignmentTargetV1, ResolvedLexicalRefV1};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) struct CallableLoopBindingSourceV1 {
    pub(super) owner: FunctionOwnerIdV1,
    pub(super) loop_site: SourceNodeSiteV1,
    pub(super) rows: Box<[CallableLoopReadyBindingRowV1]>,
    pub(super) body_only_rows: Box<[CallableLoopOutsideRowV1]>,
    pub(super) local_declarations: BTreeMap<BindingRefV1, SourceBindingSiteV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum CallableLoopSourceBindingClassV1 {
    Carrier,
    ReadOnlyOperand,
    IterationLocal,
}

impl CallableLoopBindingSourceV1 {
    pub(in crate::mir) fn owner(&self) -> FunctionOwnerIdV1 { self.owner }
    pub(in crate::mir) fn loop_site(&self) -> &SourceNodeSiteV1 { &self.loop_site }

    /// The original ready/body-only partition remains available to its strict
    /// callers. All LoopCond consumers borrow this same normalized role view.
    pub(in crate::mir) fn bindings(&self)
        -> impl Iterator<Item = (BindingRefV1, CallableLoopSourceBindingClassV1)> + '_ {
        use CallableLoopSourceBindingClassV1 as Class;
        self.rows.iter().map(|row| (row.binding(), match row.class() {
            CallableLoopReadyBindingClassV1::Carrier => Class::Carrier,
            CallableLoopReadyBindingClassV1::ReadOnlyOperand => Class::ReadOnlyOperand,
            CallableLoopReadyBindingClassV1::IterationLocal => Class::IterationLocal,
        })).chain(self.body_only_rows.iter().map(|row| (row.binding(),
            if self.local_declarations.contains_key(&row.binding()) { Class::IterationLocal }
            else { Class::Carrier })))
    }

    pub(in crate::mir) fn local_declarations(&self) -> &BTreeMap<BindingRefV1, SourceBindingSiteV1> {
        &self.local_declarations
    }

    pub(super) fn validate_pre_effect(&self, parent: &SourceNodeSiteV1,
        condition: &SourceNodeSiteV1, body: &SourceNodeSiteV1) -> Result<(), String> {
        if parent != &self.loop_site { return Err(freeze("loop-owner-site-mismatch")); }
        if !is_direct_child(parent, condition, SourcePathSegmentV1::LoopCondition) {
            return Err(freeze("condition-source-mismatch"));
        }
        if !is_direct_child(parent, body, SourcePathSegmentV1::LoopBodyRoot) {
            return Err(freeze("body-source-mismatch"));
        }
        Ok(())
    }
}

impl CallableLoopBindingProjectionDispositionV1 {
    pub(in crate::mir::builder) fn source(&self) -> Result<&Rc<CallableLoopBindingSourceV1>, String> {
        match self {
            Self::Ready(row) => Ok(&row.source),
            Self::ReadyWithBodyOnly(row) => Ok(&row.source),
            Self::Outside(_) => Err(freeze("outside-first-cohort")),
        }
    }

    /// Adapt original resolver rows to the existing projector. No role is
    /// classified here, and no physical state or inferred type is consulted.
    pub(in crate::mir::builder) fn from_resolved_input(input: ResolvedFunctionLoweringInputV1<'_>,
        site: &SourceStmtSiteV1, loop_true: bool) -> Result<Self, String> {
        let function = input.function();
        let mut locals = BTreeMap::<_, BTreeMap<u32, BindingRefV1>>::new();
        for declaration in function.declaration_sites() {
            let SourceBindingSiteV1::Local { statement, ordinal } = declaration else { continue; };
            let binding = function.declaration_binding(declaration)
                .ok_or_else(|| freeze("missing-declaration-binding"))?;
            if binding.owner() != input.owner() { return Err(freeze("foreign-local-declaration")); }
            if locals.entry(statement.node().clone()).or_default().insert(*ordinal, binding).is_some() {
                return Err(freeze("duplicate-local-declaration-site"));
            }
        }
        let locals = locals.into_iter().map(|(site, bindings)| {
            if bindings.keys().copied().ne(0..bindings.len() as u32) {
                return Err(freeze("local-declaration-ordinal-gap"));
            }
            Ok((site, bindings.into_values().collect::<Box<[_]>>()))
        }).collect::<Result<BTreeMap<_, _>, String>>()?;
        let variables = function.variable_refs().filter_map(|(site, reference)| match reference {
            ResolvedLexicalRefV1::Local(binding) => Some((site.node().clone(), *binding)),
            _ => None,
        }).collect();
        let assignments = function.assignment_targets().filter_map(|(site, target)| match target {
            ResolvedAssignmentTargetV1::BindingRebind(binding) => Some((site.node().clone(), *binding)),
            _ => None,
        }).collect();
        let projection = CallableLoopSourceProjectionV1::new(input.owner(), &locals, &variables, &assignments);
        if loop_true { projection.project_loop_true_disposition(site.node().clone()) }
        else { projection.project_disposition(site.node().clone()) }
    }
}
