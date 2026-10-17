//! Current Text residences retain their original acquisition, never Object Home.
use super::*;
use std::rc::Rc;
use super::super::CheckedTextInitializerFrameV1;

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn text_reference(&self, site: &SourceExprSiteV1)
        -> Option<(BindingRefV1, BindingRefV1, Rc<CheckedTextInitializerFrameV1>)> {
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else { return None };
        let StoredLocal::Text { root, acquisition } = self.locals.get(&binding)? else { return None };
        // An alias cannot silently follow the same binding's newer acquisition.
        let StoredLocal::Text { root: live, acquisition: current } = self.locals.get(root)? else { return None };
        (*live == *root && *root == acquisition.binding() && Rc::ptr_eq(acquisition, current))
            .then(|| (binding, *root, Rc::clone(acquisition)))
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_text_initializer(&mut self, frame: Rc<CheckedTextInitializerFrameV1>) {
        let root = frame.binding();
        self.store(root, StoredLocal::Text { root, acquisition: frame });
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_text_alias(&mut self, binding: BindingRefV1, root: BindingRefV1,
        acquisition: Rc<CheckedTextInitializerFrameV1>) {
        self.store(binding, StoredLocal::Text { root, acquisition });
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn signed_entry_reference(&self, site: &SourceExprSiteV1) -> Option<BindingRefV1> {
        let OrdinaryObservation::TrivialLocal(binding, Some(SourceScalarKind::Integer)) = self.observe(site)? else { return None };
        let flow = self.input.function().expression_source().value_flow()?;
        let root = flow.alias_entry_origin(flow.read(site, binding)?, |site| self.input.function().variable_ref(site))?;
        self.signed_formals.contains(&root).then_some(binding)
    }
}

#[cfg(test)]
pub(crate) fn assert_identity_for_test(
    input: ResolvedFunctionLoweringInputV1<'_>, frame: Rc<CheckedTextInitializerFrameV1>,
    next: &CheckedTextInitializerFrameV1,
) {
    let read = &next.residence_reads()[0];
    let site = next.source().nodes()[read.node()].site();
    let mut current = PrefixLocalFlow::new(input);
    current.install_text_initializer(Rc::clone(&frame));
    current.install_text_alias(read.binding(), frame.binding(), Rc::clone(&frame));
    assert!(Rc::ptr_eq(&current.text_reference(site).unwrap().2, &frame));
    assert!(current.observe(site).is_none(), "Text is not an Object/Scalar/BoundValue");
    let snapshot = current.clone();
    assert!(current.join_branch(&snapshot), "same acquisition survives a branch clone");
    current.install_uninitialized(frame.binding());
    assert!(current.text_reference(site).is_none(), "invalid root cannot retain an alias");
    assert!(Rc::ptr_eq(&snapshot.text_reference(site).unwrap().2, &frame));
    let replacement = Rc::new(frame.as_ref().clone());
    current.install_text_initializer(replacement);
    assert!(current.text_reference(site).is_none(), "alias cannot follow an equal-content acquisition");
    assert!(!current.join_branch(&snapshot), "different acquisitions cannot merge by binding name");
}
