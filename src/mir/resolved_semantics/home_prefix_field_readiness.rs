//! Branch-local Normal field readiness of the original selected allocation.
//! Each value retains its exact successful source frame; it grants no physical pin.
use super::*;
use std::rc::Rc;
use crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1;
use crate::mir::resolved_semantics::home_new_prefix::CheckedFieldWriteFrameV1;

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn owned_allocation_source(
        &self, root: BindingRefV1,
    ) -> Option<(OwnedExprSiteV1, Rc<ConstructionSourceStructureV1>)> {
        let StoredLocal::Home { acquisition, construction_source: Some(source), .. } = self.locals.get(&root)? else { return None };
        source.is_allocation_only_i64().then(|| (acquisition.clone(), Rc::clone(source)))
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn record_normal_field_write(
        &mut self, frame: Rc<CheckedFieldWriteFrameV1>,
    ) -> bool {
        let Some(local) = frame.owned_local() else { return false };
        let Some(StoredLocal::Home { acquisition, construction_source: Some(source), ready }) = self.locals.get_mut(&local.root()) else { return false };
        if acquisition != local.acquisition() || !Rc::ptr_eq(source, local.construction())
            || source.allocation_only_i64_field(local.original().write().field()) != Some(local.field())
        { return false; }
        ready.insert(local.field(), frame);
        true
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn ready_field_read(
        &self, root: BindingRefV1, name: &str,
    ) -> Option<Option<Rc<CheckedFieldWriteFrameV1>>> {
        let Some(StoredLocal::Home { construction_source: Some(source), ready, .. }) = self.locals.get(&root) else { return Some(None) };
        if !source.is_allocation_only_i64() { return Some(None); }
        let field = source.allocation_only_i64_field(name)?;
        ready.get(&field).map(|frame| Some(Rc::clone(frame)))
    }
}
