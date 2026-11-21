//! The source walk records its live responsibilities at the original brace.
use super::super::HomePrefixUnavailableV1;
use super::*;
use crate::mir::compiler::located::SourceBodySiteV1;
use crate::mir::resolved_semantics::{ScopeId, SourceNodeSiteV1};

pub(crate) type LexicalBodyEndsV1 =
    BTreeMap<SourceNodeSiteV1, Result<std::rc::Rc<LexicalBodyEndV1>, HomePrefixUnavailableV1>>;

impl super::super::RootHomeFlow {
    pub(crate) fn lexical_body_ends(&self) -> &LexicalBodyEndsV1 {
        &self.lexical_ends
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LexicalBodyEndV1 {
    scope: ScopeId,
    homes: Box<[LiveHomeV1]>,
    normal: Box<[usize]>,
}

impl LexicalBodyEndV1 {
    pub(crate) fn scope(&self) -> ScopeId {
        self.scope
    }

    pub(crate) fn normal_homes(&self) -> impl Iterator<Item = &LiveHomeV1> {
        self.normal.iter().map(|&index| &self.homes[index])
    }

    pub(crate) fn homes(&self) -> &[LiveHomeV1] {
        &self.homes
    }
}

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn finish_lexical_body_v1(
        &mut self,
        body: &SourceBodySiteV1,
        homes: &mut HomesV1,
        unavailable: &mut Option<HomePrefixUnavailableV1>,
    ) -> Option<std::rc::Rc<LexicalBodyEndV1>> {
        let function = self.input.function();
        let Some(scope) = function.program_body_scope(&body.root()) else {
            unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
            return None;
        };
        let declarations = function
            .scope(scope)
            .expect("verified Program scope")
            .declarations();
        let full = homes.fault_order();
        let normal: Box<[_]> = full
            .iter()
            .enumerate()
            .filter(|(_, home)| declarations.contains(&home.binding()))
            .map(|(index, _)| index)
            .collect();
        if self.has_conditional_text_end_v1(&[scope], !normal.is_empty()) {
            unavailable.get_or_insert(HomePrefixUnavailableV1::LexicalCarrierEndUnproved);
        }
        let end = match unavailable {
            Some(issue) => Err(issue.clone()),
            None => Ok(std::rc::Rc::new(LexicalBodyEndV1 {
                scope,
                homes: full,
                normal,
            })),
        };
        let normal = end.as_ref().ok().cloned();
        if self
            .lexical_ends
            .borrow_mut()
            .insert(body.root(), end)
            .is_some()
        {
            unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        }
        homes.release_all(declarations);
        for binding in declarations {
            self.locals.remove(binding);
        }
        normal
    }

    /// A conditional Text owner without a Home needs a separate source
    /// producer proof. It matters if its scope closes or a new End can Fault.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn has_conditional_text_end_v1(
        &self,
        scopes: &[ScopeId],
        cleanup_can_fault: bool,
    ) -> bool {
        self.locals.iter().any(|(binding, stored)| {
            matches!(
                stored,
                StoredLocal::LoopText { .. } | StoredLocal::TextBranch { .. }
            ) && (cleanup_can_fault
                || self
                    .input
                    .function()
                    .binding(*binding)
                    .is_some_and(|record| scopes.contains(&record.owner_scope())))
        })
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn lexical_ends(
        &self,
    ) -> LexicalBodyEndsV1 {
        self.lexical_ends.borrow().clone()
    }
}
