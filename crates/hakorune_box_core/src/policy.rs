/// Factory priority policy for Box creation.
///
/// Determines the order in which different Box factories are consulted during
/// Box creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactoryPolicy {
    /// Plugins > user-defined boxes > builtins.
    StrictPluginFirst,

    /// Plugins > builtins > user-defined boxes.
    CompatPluginFirst,

    /// Builtins > user-defined boxes > plugins.
    BuiltinFirst,
}

/// Factory type classification for policy-based ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactoryType {
    /// Built-in factory.
    Builtin,
    /// User-defined Box factory.
    User,
    /// Plugin-provided Box factory.
    Plugin,
}
