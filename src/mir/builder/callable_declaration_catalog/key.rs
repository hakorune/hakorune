#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum SameModuleCallableNamespaceV1 {
    StaticBoxMethod,
    InstanceBoxMethod,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CanonicalSameModuleCallableKeyV1 {
    namespace: SameModuleCallableNamespaceV1,
    owner: Box<str>,
    name: Box<str>,
    arity: u32,
}

impl CanonicalSameModuleCallableKeyV1 {
    pub(super) fn static_box_method(owner: &str, name: &str, arity: u32) -> Self {
        Self {
            namespace: SameModuleCallableNamespaceV1::StaticBoxMethod,
            owner: owner.into(),
            name: name.into(),
            arity,
        }
    }

    pub(super) fn instance_box_method(owner: &str, name: &str, arity: u32) -> Self {
        Self {
            namespace: SameModuleCallableNamespaceV1::InstanceBoxMethod,
            owner: owner.into(),
            name: name.into(),
            arity,
        }
    }

    pub(crate) const fn namespace(&self) -> SameModuleCallableNamespaceV1 {
        self.namespace
    }

    pub(crate) fn owner(&self) -> &str {
        &self.owner
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn arity(&self) -> u32 {
        self.arity
    }

    pub(crate) fn mir_symbol_projection(&self) -> String {
        format!("{}.{}/{}", self.owner, self.name, self.arity)
    }

    /// Project an already catalog-selected static declaration into the
    /// structural global carrier.  This is deliberately limited to the
    /// declaration key owner; callers cannot manufacture a target by
    /// reparsing a MIR symbol string.
    pub(crate) fn canonical_global_target_v1(
        &self,
    ) -> Result<hakorune_mir_defs::CanonicalGlobalTargetV1, String> {
        if self.namespace != SameModuleCallableNamespaceV1::StaticBoxMethod {
            return Err("only static box methods have a global target".to_owned());
        }
        hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
            self.owner.clone(),
            self.name.clone(),
            self.arity,
        )
        .map_err(|error| format!("invalid catalog global target: {error:?}"))
    }

    #[cfg(test)]
    pub(crate) fn test_static_box_method(owner: &str, name: &str, arity: usize) -> Self {
        Self::static_box_method(owner, name, arity as u32)
    }

    #[cfg(test)]
    pub(crate) fn test_instance_box_method(owner: &str, name: &str, arity: usize) -> Self {
        Self::instance_box_method(owner, name, arity as u32)
    }
}
