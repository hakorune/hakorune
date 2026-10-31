//! Packet-backed binding-group constructors: Shared initializer and array append proofs.
use super::*;

impl RootLifecyclePrefixBindingGroupV1 {
    pub(in super::super) fn new_shared_initializer(packet: Rc<SharedLocalCommitV1>) -> Result<Self, String> {
        if packet.bindings.is_empty() { return Err(freeze("shared-initializer/bindings-empty")); }
        Ok(Self {
            site: OwnedExprSiteV1::new(packet.source.owner(), packet.source.nodes()[0].site().clone()),
            bindings: packet.bindings.clone(),
            proof: PrefixPhysicalProofV1::SharedInitializer(packet),
            projected: false,
        })
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn shared_initializer_v1(&self) -> Option<Rc<SharedLocalCommitV1>> {
        match &self.proof { PrefixPhysicalProofV1::SharedInitializer(packet) => Some(Rc::clone(packet)), _ => None }
    }

    pub(in super::super) fn new_append(
        packet: Rc<owned_array_append_projection::EmittedOwnedArrayAppendV1>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<Self, String> {
        packet.validate_recorded(&bindings, None)?;
        Ok(Self {
            site: packet.source().site().clone(),
            bindings,
            proof: PrefixPhysicalProofV1::OwnedArrayAppend(packet),
            projected: false,
        })
    }

    pub(in super::super) fn field_array_append_packet_v1(
        &self,
    ) -> Option<&Rc<field_array_append_projection::EmittedFieldArrayAppendV1>> {
        match &self.proof {
            PrefixPhysicalProofV1::FieldArrayAppend(packet) => Some(packet),
            _ => None,
        }
    }
    pub(in super::super) fn new_field_append(
        packet: Rc<field_array_append_projection::EmittedFieldArrayAppendV1>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<Self, String> {
        packet.validate_recorded(&bindings, None)?;
        Ok(Self {
            site: packet.source().site().clone(),
            bindings,
            proof: PrefixPhysicalProofV1::FieldArrayAppend(packet),
            projected: false,
        })
    }
}
