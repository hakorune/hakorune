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
}
