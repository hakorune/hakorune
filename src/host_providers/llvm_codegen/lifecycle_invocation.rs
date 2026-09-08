//! One invocation-owned binding of issued physical input and selected runtime.
//! No source facts are issued here; binding precedes temporary files/artifacts.
use super::runtime_abi_descriptor::LifecycleRuntimeSessionV1;
use crate::mir::PublishedLifecyclePhysicalAbiInputV1;
use std::path::Path;

pub(super) struct LifecycleInvocationInputV1<'module, 'session> {
    physical: PublishedLifecyclePhysicalAbiInputV1<'module>,
    session: &'session LifecycleRuntimeSessionV1,
}

impl<'module, 'session> LifecycleInvocationInputV1<'module, 'session> {
    pub(super) fn bind(
        physical: PublishedLifecyclePhysicalAbiInputV1<'module>,
        session: &'session LifecycleRuntimeSessionV1,
    ) -> Result<Self, String> {
        session.require_input(
            physical.runtime_requirements(),
            physical.fault_abi_version(),
        )?;
        Ok(Self { physical, session })
    }

    pub(super) fn serialize(&self) -> Result<String, String> {
        crate::mir::emit_lifecycle_physical_abi_json(&self.physical)
    }

    pub(super) fn session(&self) -> &'session LifecycleRuntimeSessionV1 {
        self.session
    }

    pub(super) fn runtime_archive(&self) -> &'session Path {
        self.session.runtime_archive()
    }
}
