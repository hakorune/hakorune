//! One pre-emission control selection from the already co-sealed Script source.
//! No AST, physical IDs, backend admission, or new semantic capability is issued.
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum ArrayReleaseRoleV1 {
    IncompleteResidence,
    Home(BindingRefV1),
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ArrayLocalRecipeV1 {
    relation: ResolvedInitializerRelationV1,
    spec: ArrayElementContractSpec,
    elements: Box<[SourceExprSiteV1]>,
    allocation_fault: Box<[ArrayReleaseRoleV1]>,
    acquired_fault: Box<[ArrayReleaseRoleV1]>,
}

impl ArrayLocalRecipeV1 {
    pub(in crate::mir::builder) fn relation(&self) -> &ResolvedInitializerRelationV1 {
        &self.relation
    }
    pub(in crate::mir::builder) fn spec(&self) -> ArrayElementContractSpec {
        self.spec
    }
    pub(in crate::mir::builder) fn elements(&self) -> &[SourceExprSiteV1] {
        &self.elements
    }
    pub(in crate::mir::builder) fn allocation_fault(&self) -> &[ArrayReleaseRoleV1] {
        &self.allocation_fault
    }
    pub(in crate::mir::builder) fn acquired_fault(&self) -> &[ArrayReleaseRoleV1] {
        &self.acquired_fault
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct ArrayReturnRecipeV1 {
    site: crate::mir::resolved_semantics::SourceStmtSiteV1,
    result: root_terminal::RootResult,
    releases: Box<[ArrayReleaseRoleV1]>,
}
impl ArrayReturnRecipeV1 {
    pub(in crate::mir::builder) fn site(
        &self,
    ) -> &crate::mir::resolved_semantics::SourceStmtSiteV1 {
        &self.site
    }
    pub(in crate::mir::builder) fn result(&self) -> &root_terminal::RootResult {
        &self.result
    }
    pub(in crate::mir::builder) fn releases(&self) -> &[ArrayReleaseRoleV1] {
        &self.releases
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct ScriptArrayLifecycleRecipeV1 {
    locals: BTreeMap<SourceNodeSiteV1, Option<ArrayLocalRecipeV1>>,
    terminal: Option<ArrayReturnRecipeV1>,
}
impl ScriptArrayLifecycleRecipeV1 {
    pub(super) fn issue(source: &ArraySourceLifecycleRows) -> Result<Self, String> {
        source.require_root()?;
        let mut locals = BTreeMap::new();
        for (site, coverage) in &source.rows {
            let ArraySourceCoverage::Available(row) = coverage else {
                return Err(freeze("recipe-unavailable-local"));
            };
            if row.progress != LocalProgress::Pending {
                return Err(freeze("recipe-after-emission"));
            }
            // Consume the complete source cutpoint shape, not a Written-only
            // filter that would hide an absent evaluation/commit/unwind obligation.
            let cuts = &row.cutpoints;
            if cuts.len() < 6
                || cuts[..3]
                    != [
                        Cutpoint::BeforeAllocation,
                        Cutpoint::AcquiredBeforeClaim,
                        Cutpoint::Claimed,
                    ]
                || cuts[cuts.len() - 3..]
                    != [
                        Cutpoint::FaultUnwind,
                        Cutpoint::LocalCommit,
                        Cutpoint::CallerExit,
                    ]
            {
                return Err(freeze("recipe-cutpoint-shape"));
            }
            let mut elements = Vec::new();
            let mut chunks = cuts[3..cuts.len() - 3].chunks_exact(3);
            for chunk in &mut chunks {
                match chunk {
                    [Cutpoint::ChildEvaluating(a), Cutpoint::ChildReady(b), Cutpoint::Written(c)]
                        if a == b && b == c =>
                    {
                        elements.push(a.clone())
                    }
                    _ => return Err(freeze("recipe-child-cutpoint")),
                }
            }
            if !chunks.remainder().is_empty() {
                return Err(freeze("recipe-child-cutpoint"));
            }
            let allocation_fault: Box<[_]> = row
                .caller_homes
                .iter()
                .copied()
                .map(ArrayReleaseRoleV1::Home)
                .collect();
            let acquired_fault = std::iter::once(ArrayReleaseRoleV1::IncompleteResidence)
                .chain(allocation_fault.iter().cloned())
                .collect();
            locals.insert(
                site.clone(),
                Some(ArrayLocalRecipeV1 {
                    relation: row.initializer.clone(),
                    spec: row.spec,
                    elements: elements.into_boxed_slice(),
                    allocation_fault,
                    acquired_fault,
                }),
            );
        }
        let terminal = source.terminal()?.map(|terminal| ArrayReturnRecipeV1 {
            site: terminal.site().clone(),
            result: terminal.result().clone(),
            releases: terminal
                .homes()
                .iter()
                .copied()
                .map(ArrayReleaseRoleV1::Home)
                .collect(),
        });
        Ok(Self { locals, terminal })
    }

    pub(in crate::mir::builder) fn take_local(
        &mut self,
        relation: &ResolvedInitializerRelationV1,
    ) -> Result<Option<ArrayLocalRecipeV1>, String> {
        let SourceBindingSiteV1::Local {
            statement,
            ordinal: 0,
        } = relation.declaration_site()
        else {
            return Err(freeze("recipe-local-site"));
        };
        let Some(slot) = self.locals.get_mut(statement.node()) else {
            return Ok(None);
        };
        let row = slot
            .as_ref()
            .ok_or_else(|| freeze("recipe-local-already-taken"))?;
        if row.relation != *relation {
            return Err(freeze("recipe-local-source-drift"));
        }
        Ok(slot.take())
    }

    pub(in crate::mir::builder) fn take_return(
        &mut self,
        site: &SourceNodeSiteV1,
    ) -> Result<Option<ArrayReturnRecipeV1>, String> {
        if self.locals.is_empty() {
            return Ok(None);
        }
        if self.locals.values().any(Option::is_some) {
            return Err(freeze("recipe-return-before-locals"));
        }
        let terminal = self
            .terminal
            .as_ref()
            .ok_or_else(|| freeze("recipe-return-already-taken"))?;
        if terminal.site.node() != site {
            return Err(freeze("recipe-return-site"));
        }
        Ok(self.terminal.take())
    }

    pub(in crate::mir::builder) fn finish(&self) -> Result<(), String> {
        if self.locals.values().any(Option::is_some) || self.terminal.is_some() {
            return Err(freeze("recipe-unconsumed"));
        }
        Ok(())
    }
}

impl ArraySourceLifecycleRows {
    pub(in crate::mir::builder) fn lowering_recipe(
        &self,
    ) -> Result<ScriptArrayLifecycleRecipeV1, String> {
        ScriptArrayLifecycleRecipeV1::issue(self)
    }
}

#[cfg(test)]
#[path = "normal_script_array_lifecycle_recipe_tests.rs"]
mod tests;
