//! Map prefix groups share the existing ordered publication pool.
use super::*;
impl RootLifecyclePrefixBindingGroupV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn stored_map_get(
        &self,
    ) -> Option<&StoredMapGetLocalEmissionV1> {
        match &self.proof {
            PrefixPhysicalProofV1::StoredMapGet(packet) => Some(packet),
            _ => None,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn initializer_copy_receipt_v1(
        &self,
    ) -> Option<(ValueId, ValueId, &(BasicBlockId, MirInstruction))> {
        if let Some(packet) = self.stored_map_get() {
            return Some(packet.copy_receipt());
        }
        let packet = self.initializer()?;
        Some((
            packet.local_value(),
            packet.value(),
            packet.receipt_copy_binding_v1()?,
        ))
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn new_stored_map_get(
        packet: Rc<StoredMapGetLocalEmissionV1>,
    ) -> Result<Self, String> {
        packet.validate_recorded(packet.bindings(), None)?;
        Ok(Self {
            site: packet.frame().site().clone(),
            bindings: packet.bindings().to_vec(),
            proof: PrefixPhysicalProofV1::StoredMapGet(packet),
            projected: false,
        })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn map_field_install(
        &self,
    ) -> Option<&MapFieldInstallEmissionV1> {
        match &self.proof {
            PrefixPhysicalProofV1::FieldMapInstall(packet) => Some(packet),
            _ => None,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn map_field_install_rc(&self) -> Option<Rc<MapFieldInstallEmissionV1>> {
        match &self.proof { PrefixPhysicalProofV1::FieldMapInstall(packet) => Some(Rc::clone(packet)), _ => None }
    }
    fn new_map_field_install(packet: Rc<MapFieldInstallEmissionV1>) -> Result<Self, String> {
        packet.validate_recorded(packet.bindings(), None)?;
        Ok(Self {
            site: packet.frame().site().clone(),
            bindings: packet.bindings().to_vec(),
            proof: PrefixPhysicalProofV1::FieldMapInstall(packet),
            projected: false,
        })
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn validate_stored_map_get_local_source_v1(
        &self,
        frame: &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::MapFieldGetLocalV1>,
    ) -> Result<(), String> {
        let retained = self
            .stored_map_get_local_source_v1(frame.site().owner(), frame.statement().node())?
            .ok_or_else(|| freeze("map-get-source-missing"))?;
        let next = self.next_prefix_source_v2(frame.site().owner(), frame.site())?;
        if !std::rc::Rc::ptr_eq(&retained, frame)
            || !matches!(next.as_ref(), Some(crate::mir::resolved_semantics::home_new_prefix::LifecyclePrefixSourceV1::StoredMapGet { frame: original, .. }) if std::rc::Rc::ptr_eq(&original, frame))
        {
            return Err(freeze("initializer-expression/source-prefix-order"));
        }
        Ok(())
    }
    pub(crate) fn record_stored_map_get_local_bindings_v1(
        &self,
        packet: std::rc::Rc<StoredMapGetLocalEmissionV1>,
    ) -> Result<(), String> {
        self.prepare_local_call_binding_group(
            packet.frame().site().owner(),
            RootLifecyclePrefixBindingGroupV1::new_stored_map_get(packet)?,
        )?
        .commit()
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn validate_map_field_install_source_v1(
        &self,
        frame: &Rc<crate::mir::resolved_semantics::home_new_prefix::MapFieldInstallV1>,
    ) -> Result<(), String> {
        let retained = self
            .map_field_install_source_v1(frame.site())?
            .ok_or_else(|| freeze("map-get-source-missing"))?;
        let next = self.next_prefix_source_v2(frame.site().owner(), frame.site())?;
        if !Rc::ptr_eq(&retained, frame)
            || !matches!(next.as_ref(), Some(LifecyclePrefixSourceV1::FieldMapInstall { frame: original, .. }) if Rc::ptr_eq(original, frame))
        {
            return Err(freeze("initializer-expression/source-prefix-order"));
        }
        Ok(())
    }
    pub(crate) fn record_map_field_install_bindings_v1(
        &self,
        packet: Rc<MapFieldInstallEmissionV1>,
    ) -> Result<(), String> {
        self.prepare_local_call_binding_group(
            packet.frame().site().owner(),
            RootLifecyclePrefixBindingGroupV1::new_map_field_install(packet)?,
        )?
        .commit()
    }
}
