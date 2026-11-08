//! Borrowed access to the Builder's existing function-wide physical owner.
use super::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::SharedFunctionControlV1;
use crate::mir::builder::MirBuilder;
use std::cell::RefMut;
use std::rc::Rc;

impl CanonicalSsaFunctionSessionV2<'_> {
    pub(in crate::mir::builder::resolved_lowering) fn bind_control(
        &self,
        builder: &MirBuilder,
    ) -> Result<(), String> {
        let actual = builder.function_control_for_owner_v1(self.owner)?;
        let selected = self.control.get_or_init(|| Rc::clone(&actual));
        if !Rc::ptr_eq(selected, &actual) {
            return Err("[freeze:contract][canonical-control/foreign-owner]".to_owned());
        }
        Ok(())
    }
    pub(in crate::mir::builder::resolved_lowering) fn control(
        &self,
    ) -> Result<SharedFunctionControlV1, String> {
        self.control
            .get()
            .cloned()
            .ok_or_else(|| "[freeze:contract][canonical-control/inactive]".to_owned())
    }
    pub(in crate::mir::builder::resolved_lowering) fn cfg(
        &self,
    ) -> Result<RefMut<'_, CanonicalCfgSessionV1>, String> {
        let control = self
            .control
            .get()
            .ok_or_else(|| "[freeze:contract][canonical-control/inactive]".to_owned())?;
        Ok(RefMut::map(control.borrow_mut(), |control| {
            &mut control.cfg
        }))
    }
    pub(in crate::mir::builder::resolved_lowering) fn cfg_ref(
        &self,
    ) -> Result<std::cell::Ref<'_, CanonicalCfgSessionV1>, String> {
        let control = self
            .control
            .get()
            .ok_or_else(|| "[freeze:contract][canonical-control/inactive]".to_owned())?;
        Ok(std::cell::Ref::map(control.borrow(), |control| {
            &control.cfg
        }))
    }
}
