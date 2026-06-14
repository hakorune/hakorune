//! Box callable model.

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BoxKey(String);

impl BoxKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CallableName(String);

impl CallableName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BoxCallableRole {
    Birth,
    Fini,
    Method,
    StaticMethod,
    PropertyGet,
    PropertySet,
    Operator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BoxCallableSource {
    TypeRegistry,
    SurfaceCatalog,
    PluginLoaderProvider,
    UserBoxProvider,
    IntrinsicProvider,
    Manual,
}

impl BoxCallableSource {
    pub fn as_str(self) -> &'static str {
        match self {
            BoxCallableSource::TypeRegistry => "type_registry",
            BoxCallableSource::SurfaceCatalog => "surface_catalog",
            BoxCallableSource::PluginLoaderProvider => "plugin_loader_provider",
            BoxCallableSource::UserBoxProvider => "user_box_provider",
            BoxCallableSource::IntrinsicProvider => "intrinsic_provider",
            BoxCallableSource::Manual => "manual",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BoxCallableKey {
    pub box_key: BoxKey,
    pub role: BoxCallableRole,
    pub name: CallableName,
    pub arity: u8,
}

impl BoxCallableKey {
    pub fn new(
        box_key: impl Into<String>,
        role: BoxCallableRole,
        name: impl Into<String>,
        arity: u8,
    ) -> Self {
        Self {
            box_key: BoxKey::new(box_key),
            role,
            name: CallableName::new(name),
            arity,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FunctionId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IntrinsicId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoxCallableTarget {
    InternalSlot {
        slot: u16,
    },
    PluginMethod {
        type_id: u32,
        method_id: u32,
        returns_result: bool,
    },
    PluginLifecycle {
        type_id: u32,
        birth_id: Option<u32>,
        fini_id: Option<u32>,
    },
    UserFunction {
        function_id: FunctionId,
    },
    Intrinsic {
        intrinsic_id: IntrinsicId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxCallableEntry {
    pub source: BoxCallableSource,
    pub target: BoxCallableTarget,
}

impl BoxCallableEntry {
    pub fn new(source: BoxCallableSource, target: BoxCallableTarget) -> Self {
        Self { source, target }
    }
}

impl BoxCallableTarget {
    pub fn id_space(&self) -> &'static str {
        match self {
            BoxCallableTarget::InternalSlot { .. } => "internal_vtable_slot",
            BoxCallableTarget::PluginMethod { .. } => "plugin_typebox_method_id",
            BoxCallableTarget::PluginLifecycle { .. } => "plugin_lifecycle_method_id",
            BoxCallableTarget::UserFunction { .. } => "user_function_id",
            BoxCallableTarget::Intrinsic { .. } => "intrinsic_id",
        }
    }
}
